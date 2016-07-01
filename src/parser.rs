#![allow(non_upper_case_globals)]

use std::collections::{HashMap, HashSet};
use std::cell::RefCell;
use std::rc::Rc;
use std::path::Path;
use std::cmp;

use syntax::abi;

use types as il;
use types::*;
use clang as cx;
use clang::{ast_dump, Comment, Cursor, Diagnostic, TranslationUnit, type_to_str, kind_to_str};
use clangll::*;

use super::Logger;

#[derive(Clone)]
pub struct ClangParserOptions {
    pub builtin_names: HashSet<String>,
    pub builtins: bool,
    pub match_pat: Vec<String>,
    pub emit_ast: bool,
    pub fail_on_unknown_type: bool,
    pub ignore_functions: bool,
    pub enable_cxx_namespaces: bool,
    pub class_constants: bool,
    pub override_enum_ty: Option<il::IKind>,
    pub clang_args: Vec<String>,
    pub opaque_types: Vec<String>,
    pub blacklist_type: Vec<String>,
}

struct ClangParserCtx<'a> {
    options: ClangParserOptions,
    name: HashMap<Cursor, Global>,
    builtin_defs: Vec<Cursor>,
    module_map: ModuleMap,
    current_module_id: ModuleId,
    current_translation_unit: TranslationUnit,
    logger: &'a (Logger+'a),
    err_count: i32,
    anonymous_modules_found: usize,
}

impl<'a> ClangParserCtx<'a> {
    fn module(&self, id: &ModuleId) -> &Module {
        self.module_map.get(id).expect("Module not found!")
    }

    fn current_module(&self) -> &Module {
        self.module(&self.current_module_id)
    }

    fn current_module_mut(&mut self) -> &mut Module {
        self.module_map.get_mut(&self.current_module_id).expect("Module not found!")
    }
}

fn match_pattern(ctx: &mut ClangParserCtx, cursor: &Cursor) -> bool {
    let (file, _, _, _) = cursor.location().location();

    let name = match file.name() {
        None => return ctx.options.builtins,
        Some(name) => name,
    };

    if ctx.options.match_pat.is_empty() {
        return true;
    }

    ctx.options.match_pat.iter().any(|pat| name.contains(pat))
}

fn conv_template_type_parameter(ctx: &mut ClangParserCtx, cursor: &Cursor) -> Type {
    assert_eq!(cursor.kind(), CXCursor_TemplateTypeParameter);
    let ty = conv_ty(ctx, &cursor.cur_type(), cursor);
    let layout = Layout::new(ty.size(), ty.align());
    TNamed(Rc::new(RefCell::new(TypeInfo::new(cursor.spelling(), ctx.current_module_id, TVoid, layout))))
}

fn decl_name(ctx: &mut ClangParserCtx, cursor: &Cursor) -> Global {
    let cursor = cursor.canonical();
    let override_enum_ty = ctx.options.override_enum_ty;
    let new_decl = !ctx.name.contains_key(&cursor);

    let decl = if new_decl {
        let spelling = cursor.spelling();
        let comment = cursor.raw_comment();
        let (file, _, _, _) = cursor.location().location();
        let ty = cursor.cur_type();
        let layout = Layout::new(ty.size(), ty.align());
        let filename = match Path::new(&file.name().unwrap_or("".to_owned())).file_name() {
            Some(name) => name.to_string_lossy().replace(".", "_"),
            _ => "".to_string()
        };
        let glob_decl = match cursor.kind() {
            CXCursor_UnionDecl |
            CXCursor_ClassTemplate |
            CXCursor_ClassDecl |
            CXCursor_StructDecl => {
                let kind = if cursor.kind() == CXCursor_UnionDecl {
                    CompKind::Union
                } else {
                    CompKind::Struct
                };

                let opaque = ctx.options.opaque_types.iter().any(|name| *name == spelling);
                let hide = ctx.options.blacklist_type.iter().any(|name| *name == spelling);

                let mut has_non_type_template_params = false;
                let args = match ty.num_template_args() {
                    // In forward declarations, etc, they are in the ast... sigh
                    -1 => {
                        let mut args = vec![];
                        cursor.visit(|c, _| {
                            if c.kind() == CXCursor_TemplateTypeParameter {
                                args.push(conv_template_type_parameter(ctx, c));
                            }
                            CXChildVisit_Continue
                        });
                        args
                    }
                    len => {
                        let mut list = Vec::with_capacity(len as usize);
                        for i in 0..len {
                            let arg_type = ty.template_arg_type(i);
                            if arg_type.kind() != CXType_Invalid {
                                list.push(conv_ty(ctx, &arg_type, &cursor));
                            } else {
                                has_non_type_template_params = true;
                                ctx.logger.warn("warning: Template parameter is not a type");
                            }
                        }
                        list
                    }
                };

                let mut ci = CompInfo::new(spelling, ctx.current_module_id, filename, comment, kind, vec![], layout);
                ci.parser_cursor = Some(cursor);

                // If it's an instantiation of another template,
                // find the canonical declaration to find the module
                // it belongs to and if it's opaque.
                let parent = cursor.specialized();
                if let Some(parent) = ctx.name.get(&parent) {
                    ci.ref_template = Some(parent.clone().to_type())
                }

                ci.opaque = opaque;
                ci.hide = hide;
                ci.args = args;
                ci.has_non_type_template_params = has_non_type_template_params;

                let ci = Rc::new(RefCell::new(ci));
                GCompDecl(ci)
            }
            CXCursor_EnumDecl => {
                let kind = match override_enum_ty {
                    Some(t) => t,
                    None => match cursor.enum_type().kind() {
                        CXType_SChar | CXType_Char_S => ISChar,
                        CXType_UChar | CXType_Char_U => IUChar,
                        CXType_UShort => IUShort,
                        CXType_UInt => IUInt,
                        CXType_ULong => IULong,
                        CXType_ULongLong => IULongLong,
                        CXType_Short => IShort,
                        CXType_Int => IInt,
                        CXType_Long => ILong,
                        CXType_LongLong => ILongLong,
                        _ => IInt,
                    }
                };

                let ei = Rc::new(RefCell::new(EnumInfo::new(spelling, ctx.current_module_id, filename, kind, vec!(), layout)));
                GEnumDecl(ei)
            }
            CXCursor_TypeAliasDecl | CXCursor_TypedefDecl => {
                let opaque = ctx.options.opaque_types.iter().any(|name| *name == spelling);
                let mut ti = TypeInfo::new(spelling, ctx.current_module_id, TVoid, layout);
                ti.opaque = opaque;

                let ti = Rc::new(RefCell::new(ti));
                GType(ti)
            }
            CXCursor_VarDecl => {
                let mangled = cursor.mangling();
                let is_const = ty.is_const();
                let ty = conv_ty_resolving_typedefs(ctx,  &ty, &cursor, true);
                let mut vi = VarInfo::new(spelling, mangled, comment, ty);
                vi.is_const = is_const;
                cursor.visit(|c, _: &Cursor| {
                    vi.val = visit_literal(c, &ctx.current_translation_unit);
                    CXChildVisit_Continue
                });
                GVar(Rc::new(RefCell::new(vi)))
            }
            CXCursor_MacroDefinition => {
                let vi = Rc::new(RefCell::new(VarInfo::new(spelling, String::new(), comment, TVoid)));
                GVar(vi)
            }
            CXCursor_FunctionDecl => {
                let mangled = cursor.mangling();
                let vi = Rc::new(RefCell::new(VarInfo::new(spelling, mangled, comment, TVoid)));
                GFunc(vi)
            }
            _ => GOther,
        };

        ctx.name.insert(cursor, glob_decl.clone());
        glob_decl
    } else {
        ctx.name.get(&cursor).unwrap().clone()
    };

    if new_decl && ctx.options.builtin_names.contains(&cursor.spelling()) {
        ctx.builtin_defs.push(cursor);
    }

    decl
}

fn opaque_decl(ctx: &mut ClangParserCtx, decl: &Cursor) {
    let spelling = decl.spelling();
    let hide = ctx.options.blacklist_type.iter().any(|name| *name == spelling);

    if hide {
        return;
    }

    let name = decl_name(ctx, decl);
    ctx.current_module_mut().globals.push(name);
}

fn fwd_decl<F: FnOnce(&mut ClangParserCtx)->()>(ctx: &mut ClangParserCtx, cursor: &Cursor, f: F) {
    let def = cursor.definition();
    if cursor == &def {
        f(ctx);
    } else if def.kind() == CXCursor_NoDeclFound ||
              def.kind() == CXCursor_InvalidFile {
        opaque_decl(ctx, cursor);
    }
}

fn get_abi(cc: Enum_CXCallingConv) -> abi::Abi {
    match cc {
        CXCallingConv_Default => abi::Abi::C,
        CXCallingConv_C => abi::Abi::C,
        CXCallingConv_X86StdCall => abi::Abi::Stdcall,
        CXCallingConv_X86FastCall => abi::Abi::Fastcall,
        CXCallingConv_AAPCS => abi::Abi::Aapcs,
        CXCallingConv_X86_64Win64 => abi::Abi::Win64,
        other => panic!("unsupported calling convention: {}", other),
    }
}

fn conv_ptr_ty_resolving_typedefs(ctx: &mut ClangParserCtx,
                                  ty: &cx::Type,
                                  cursor: &Cursor,
                                  is_ref: bool,
                                  layout: Layout,
                                  resolve_typedefs: bool) -> il::Type {
    let is_const = ty.is_const();
    match ty.kind() {
        CXType_Void => {
            return TPtr(Box::new(TVoid), is_const, is_ref, layout)
        }
        CXType_Unexposed |
        CXType_FunctionProto |
        CXType_FunctionNoProto => {
            let ret_ty = ty.ret_type();
            return if ret_ty.kind() != CXType_Invalid {
                TFuncPtr(mk_fn_sig(ctx, ty, cursor))
            } else if cursor.kind() == CXCursor_VarDecl {
                let can_ty = ty.canonical_type();
                conv_ty_resolving_typedefs(ctx, &can_ty, cursor, resolve_typedefs)
            } else {
                TPtr(Box::new(conv_decl_ty_resolving_typedefs(ctx, ty, cursor, resolve_typedefs)), ty.is_const(), is_ref, layout)
            };
        }
        CXType_Typedef => {
            let decl = ty.declaration();
            let def_ty = decl.typedef_type();
            if def_ty.kind() == CXType_FunctionProto ||
               def_ty.kind() == CXType_FunctionNoProto {
                return TPtr(Box::new(conv_ptr_ty_resolving_typedefs(ctx, &def_ty, cursor, is_ref, layout, resolve_typedefs)), is_const, is_ref, layout);
            } else {
                return TPtr(Box::new(conv_ty_resolving_typedefs(ctx, ty, cursor, resolve_typedefs)), is_const, is_ref, layout);
            }
        }
        _ => return TPtr(Box::new(conv_ty_resolving_typedefs(ctx, ty, cursor, resolve_typedefs)), is_const, is_ref, layout),
    }
}

fn mk_fn_sig(ctx: &mut ClangParserCtx, ty: &cx::Type, cursor: &Cursor) -> il::FuncSig {
    mk_fn_sig_resolving_typedefs(ctx, ty, cursor, &[])
}

fn mk_fn_sig_resolving_typedefs(ctx: &mut ClangParserCtx,
                                ty: &cx::Type,
                                cursor: &Cursor,
                                typedefs: &[String]) -> il::FuncSig {
    let args_lst: Vec<(String, il::Type)> = match cursor.kind() {
        CXCursor_FunctionDecl | CXCursor_CXXMethod => {
            // For CXCursor_FunctionDecl, cursor.args() is the reliable way to
            // get parameter names and types.
            cursor.args().iter().map(|arg| {
                let arg_name = arg.spelling();
                let is_class_typedef = arg.cur_type().sanitized_spelling_in(typedefs);
                (arg_name, conv_ty_resolving_typedefs(ctx, &arg.cur_type(), arg, is_class_typedef))
            }).collect()
        }
        _ => {
            // For non-CXCursor_FunctionDecl, visiting the cursor's children is
            // the only reliable way to get parameter names.
            let mut args_lst = vec!();
            cursor.visit(|c: &Cursor, _: &Cursor| {
                if c.kind() == CXCursor_ParmDecl {
                    let is_class_typedef = c.cur_type().sanitized_spelling_in(typedefs);
                    args_lst.push((c.spelling(), conv_ty_resolving_typedefs(ctx, &c.cur_type(), c, is_class_typedef)));
                }
                CXChildVisit_Continue
            });
            args_lst
        }
    };

    let ret_ty = Box::new(conv_ty(ctx, &ty.ret_type(), cursor));
    let abi = get_abi(ty.call_conv());

    // Function is presumed unsafe if it takes a pointer argument.
    let is_unsafe = args_lst.iter().any(|arg| match arg.1 {
        TPtr(..) => true,
        _ => false
    });

    il::FuncSig {
        ret_ty: ret_ty,
        args: args_lst,
        is_variadic: ty.is_variadic(),
        is_safe: !is_unsafe,
        abi: abi,
    }
}

fn conv_decl_ty_resolving_typedefs(ctx: &mut ClangParserCtx,
                                   ty: &cx::Type,
                                   cursor: &Cursor,
                                   resolve_typedefs: bool) -> il::Type {
    let ty_decl = ty.declaration();
    // println!("conv_ty_decl: `{}`, ty kind {}: {}, decl `{}` kind {}: {}", cursor.spelling(), ty.kind(), type_to_str(ty.kind()), ty_decl.spelling(), ty_decl.kind(), kind_to_str(ty_decl.kind()));
    return match ty_decl.kind() {
        CXCursor_StructDecl |
        CXCursor_UnionDecl |
        CXCursor_ClassTemplate |
        CXCursor_ClassDecl => {
            let decl = decl_name(ctx, &ty_decl);
            // NB: This will only return a number greater than 0 if this is a **full** class
            // template specialization.
            //
            // If the cursor kind is CXCursor_ClassTemplate, this will still return -1
            // and we'll have to keep traversing the cursor.
            let args = match ty.num_template_args() {
                -1 => vec!(),
                len => {
                    let mut list = Vec::with_capacity(len as usize);
                    for i in 0..len {
                        let arg_type = ty.template_arg_type(i);
                        if arg_type.kind() != CXType_Invalid {
                            list.push(conv_ty(ctx, &arg_type, &cursor));
                        } else {
                            ctx.logger.warn("warning: Template parameter is not a type");
                        }
                    }
                    list
                }
            };

            let ci = decl.compinfo();
            // NB: Args might be filled from decl_name,
            // it's important not to override
            //
            // We might incur in double borrows here. If that's the case, we're
            // already scanning the compinfo, and we'd get the args from the
            // ast.
            use std::cell::BorrowState;
            if !args.is_empty() && ci.borrow_state() == BorrowState::Unused {
                ci.borrow_mut().args = args;

                // XXX: This is a super-dumb way to get the spesialisation,
                // but it seems to be the only one that'd work here...
                cursor.visit(|c, _: &Cursor| {
                    if c.kind() == CXCursor_TemplateRef {
                        let decl = decl_name(ctx, &c.referenced());
                        ci.borrow_mut().ref_template = Some(decl.to_type());
                    }
                    CXChildVisit_Continue
                });
            }

            TComp(ci)
        }
        CXCursor_EnumDecl => {
            let decl = decl_name(ctx, &ty_decl);
            let ei = decl.enuminfo();
            TEnum(ei)
        }
        CXCursor_TypeAliasDecl |
        CXCursor_TypedefDecl => {
            if resolve_typedefs {
                return conv_ty_resolving_typedefs(ctx, &ty_decl.typedef_type(), &ty_decl.typedef_type().declaration(), resolve_typedefs);
            }

            let decl = decl_name(ctx, &ty_decl);
            let ti = decl.typeinfo();
            TNamed(ti)
        }
        CXCursor_NoDeclFound => {
            let layout = Layout::new(ty.size(), ty.align());
            TNamed(Rc::new(RefCell::new(TypeInfo::new(ty.spelling().replace("const ", ""), ctx.current_module_id, TVoid, layout))))
        }
        _ => {
            let fail = ctx.options.fail_on_unknown_type;
            log_err_warn(ctx,
                &format!("unsupported decl `{}` ({})",
                    kind_to_str(ty_decl.kind()), ty_decl.location()
                ),
                fail
            );
            TVoid
        }
    };
}

fn conv_ty(ctx: &mut ClangParserCtx,
           ty: &cx::Type,
           cursor: &Cursor) -> il::Type {
    conv_ty_resolving_typedefs(ctx, ty, cursor, false)
}

fn conv_ty_resolving_typedefs(ctx: &mut ClangParserCtx,
                              ty: &cx::Type,
                              cursor: &Cursor,
                              resolve_typedefs: bool) -> il::Type {
    let layout = Layout::new(ty.size(), ty.align());
    // println!("conv_ty: `{}` layout: {:?}, kind {}: {}", cursor.spelling(), layout, ty.kind(), type_to_str(ty.kind()));

    match ty.kind() {
        CXType_Void => TVoid,
        CXType_Invalid => {
            log_err_warn(ctx,
                &format!("invalid type `{}` ({})",
                    cursor.spelling(), cursor.location()
                ),
                false
            );
            TVoid
        }
        CXType_Bool => TInt(IBool, layout),
        CXType_SChar |
        CXType_Char_S => TInt(ISChar, layout),
        CXType_UChar |
        CXType_Char_U => TInt(IUChar, layout),
        CXType_WChar => TInt(IShort, layout),
        CXType_Char16 |
        CXType_UShort => TInt(IUShort, layout),
        CXType_UInt => TInt(IUInt, layout),
        CXType_ULong => TInt(IULong, layout),
        CXType_ULongLong => TInt(IULongLong, layout),
        CXType_Short => TInt(IShort, layout),
        CXType_Int => TInt(IInt, layout),
        CXType_Long => TInt(ILong, layout),
        CXType_LongLong => TInt(ILongLong, layout),
        CXType_Float => TFloat(FFloat, layout),
        CXType_Double => TFloat(FDouble, layout),
        CXType_LongDouble => TFloat(FDouble, layout),
        CXType_Pointer => conv_ptr_ty_resolving_typedefs(ctx, &ty.pointee_type(), cursor, false, layout, resolve_typedefs),
        CXType_LValueReference => conv_ptr_ty_resolving_typedefs(ctx, &ty.pointee_type(), cursor, true, layout, resolve_typedefs),
        // XXX DependentSizedArray is wrong
        CXType_VariableArray |
        CXType_DependentSizedArray |
        CXType_IncompleteArray => {
            conv_ptr_ty_resolving_typedefs(ctx, &ty.elem_type(), cursor, false, layout, resolve_typedefs)
        }
        CXType_FunctionProto => TFuncProto(mk_fn_sig(ctx, ty, cursor)),
        CXType_Record |
        CXType_Typedef  |
        CXType_Unexposed |
        CXType_Enum => conv_decl_ty_resolving_typedefs(ctx, ty, cursor, resolve_typedefs),
        CXType_ConstantArray => TArray(Box::new(conv_ty_resolving_typedefs(ctx, &ty.elem_type(), cursor, resolve_typedefs)), ty.array_size(), layout),
        #[cfg(not(feature="llvm_stable"))]
        CXType_Elaborated => conv_ty_resolving_typedefs(ctx, &ty.named(), cursor, resolve_typedefs),
        _ => {
            let fail = ctx.options.fail_on_unknown_type;
            log_err_warn(ctx,
                &format!("unsupported type `{}` ({})",
                    type_to_str(ty.kind()), cursor.location()
                ),
                fail
            );
            TVoid
        },
    }
}

fn opaque_ty(ctx: &mut ClangParserCtx, ty: &cx::Type) {
    if ty.kind() == CXType_Record || ty.kind() == CXType_Enum {
        let decl = ty.declaration();
        let def = decl.definition();
        if def.kind() == CXCursor_NoDeclFound ||
           def.kind() == CXCursor_InvalidFile {
            opaque_decl(ctx, &decl);
        }
    }
}

struct Annotations {
    opaque: bool,
    hide: bool,
    use_as: Option<String>,
    /// Disable deriving copy/clone on this struct.
    no_copy: bool,
}

impl Annotations {
    fn new(cursor: &Cursor) -> Annotations {
        let mut anno = Annotations {
            opaque: false,
            hide: false,
            use_as: None,
            no_copy: false,
        };

        anno.parse(&cursor.comment());
        anno
    }

    fn parse(&mut self, comment: &Comment) {
        if comment.kind() == CXComment_HTMLStartTag &&
           comment.get_tag_name() == "div" &&
           comment.get_num_tag_attrs() > 1 &&
           comment.get_tag_attr_name(0) == "rustbindgen" {
            for i in 0..comment.get_num_tag_attrs() {
                let name = comment.get_tag_attr_name(i);
                match name.as_str() {
                    "opaque" => self.opaque = true,
                    "hide" => self.hide = true,
                    "replaces" => self.use_as = Some(comment.get_tag_attr_value(i)),
                    "nocopy" => self.no_copy = true,
                    _ => (),
                }
            }
        }

        for i in 0..comment.num_children() {
            self.parse(&comment.get_child(i));
        }
    }
}

/// Recursively visits a cursor that represents a composite (struct or union)
/// type and fills members with CompMember instances representing the fields and
/// nested composites that make up the visited composite.
fn visit_composite(cursor: &Cursor, parent: &Cursor,
                   ctx: &mut ClangParserCtx,
                   ci: &mut CompInfo) -> Enum_CXVisitorResult {
    assert!(ci.parser_cursor.is_some());
    fn is_bitfield_continuation(field: &il::FieldInfo, _ty: &il::Type, width: u32) -> bool {
        match (&field.bitfields, field.ty.layout()) {
            (&Some(ref bitfields), Some(layout)) => {
                let actual_width = bitfields.iter().map(|&(_, w)| w).fold(0u32, |acc, w| acc + w);
                actual_width + width <= (layout.size * 8) as u32
            },
            _ => false
        }
    }

    match cursor.kind() {
        CXCursor_TypeAliasDecl | CXCursor_TypedefDecl => {
            ci.typedefs.push(cursor.spelling().to_owned());
        }
        CXCursor_FieldDecl => {
            let anno = Annotations::new(cursor);
            if anno.hide {
                return CXChildVisit_Continue;
            }

            let is_class_typedef = cursor.cur_type().sanitized_spelling_in(&ci.typedefs);
            let mutable = cursor.is_mutable_field();

            let cursor_ty = cursor.cur_type();

            // NB: Overwritten in the case of non-integer bitfield
            let mut ty = conv_ty_resolving_typedefs(ctx,
                                                    &cursor_ty,
                                                    cursor,
                                                    is_class_typedef);


            use std::cell::BorrowState;
            if let Some(child_ci) = ty.get_outermost_composite() {
                if let BorrowState::Unused = child_ci.borrow_state() {
                    let mut child_ci = child_ci.borrow_mut();
                    let child_cursor = child_ci.parser_cursor.unwrap();

                    // TODO: This is lame, ideally we should use cursors.
                    // The problem this loop is trying to solve is
                    // tests/headers/inner_template_self.hpp, and templates with
                    // incomplete types.
                    //
                    // The problem with this is that, in the first case (see the
                    // CXCursor_ClassDecl branch below) clang treats the *prev*
                    // field as a Class Declaration instead of a Class Template,
                    // so we have to check now for the name and the module id.
                    //
                    // Ideally, some method like `semantic_parent` or
                    // `lexical_parent` should return the reference to the
                    // class, but I've tried everything I could think about and
                    // failed miserably.
                    //
                    // Also, there could be more complex cases, like a templated
                    // type in an inner type declaration, that this is
                    // completely unable to catch.
                    //
                    // In the second case (the CXCursor_ClassTemplate branch),
                    // we're not able to retrieve the template parameters of an
                    // incomplete type via the declaration or anything like
                    // that. We can inspect the AST and deduct them though,
                    // since there's a leading CXCursor_TemplateRef.
                    if child_ci.args.is_empty() && child_cursor.kind() == CXCursor_ClassDecl {
                        // println!("child: {:?} {:?}, {:?}, {:?}", cursor.spelling(),
                        //                               type_to_str(cursor_ty.kind()),
                        //                               type_to_str(child_cursor.cur_type().kind()),
                        //                               kind_to_str(child_cursor.kind()));
                        if child_ci.name == ci.name &&
                           child_ci.module_id == ci.module_id {
                            child_ci.args = ci.args.clone();
                        }
                    }

                    if child_cursor.kind() == CXCursor_ClassTemplate {
                        // We need to take into account the possibly different
                        // type template names, so we need to clear them and
                        // re-scan.
                        child_ci.args.clear();
                        let mut found_invalid_template_ref = false;
                        cursor.visit(|c, _| {
                            // println!("ichild: {:?} {:?}, {:?}", c.spelling(),
                            //                               kind_to_str(c.kind()),
                            //                               type_to_str(c.cur_type().kind()));
                            if c.kind() == CXCursor_TemplateRef &&
                               c.cur_type().kind() == CXType_Invalid {
                                found_invalid_template_ref = true;
                            }
                            if found_invalid_template_ref &&
                               c.kind() == CXCursor_TypeRef {
                                child_ci.args.push(TNamed(Rc::new(RefCell::new(
                                    TypeInfo::new(c.spelling(),
                                                  ctx.current_module_id,
                                                  TVoid,
                                                  Layout::zero())))));
                            }
                            CXChildVisit_Continue
                        })
                    }
                }
            }

            let comment = cursor.raw_comment();

            let (name, bitfields) = match (cursor.bit_width(), ci.members.last_mut()) {
                // The field is a continuation of an exising bitfield
                (Some(width), Some(&mut il::CompMember::Field(ref mut field)))
                    if is_bitfield_continuation(field, &ty, width) => {

                    // println!("found bitfield continuation {} (width: {})", cursor.spelling(), width);

                    field.bitfields.as_mut().unwrap().push((cursor.spelling(), width));
                    return CXChildVisit_Continue;
                },
                // The field is the start of a new bitfield
                (Some(width), _) => {
                    // Bitfields containing enums are not supported by the c standard
                    // https://stackoverflow.com/questions/11983231/is-it-safe-to-use-an-enum-in-a-bit-field

                    match ty {
                        il::TInt(..) => {},
                        _ => {
                            // NOTE: We rely on the name of the type converted
                            // to rust types, and on the alignment.
                            let bits = cmp::max(width, ty.size() as u32 * 8);
                            let layout_size = cmp::max(1, bits.next_power_of_two() / 8) as usize;

                            let msg = format!("Enums in bitfields are not supported ({}::{}). Trying to recover with width: {}",
                                parent.spelling(), cursor.spelling(), layout_size * 8);
                            ctx.logger.warn(&msg);

                            let name = match layout_size {
                                1 => "uint8_t",
                                2 => "uint16_t",
                                4 => "uint32_t",
                                8 => "uint64_t",
                                _ => panic!("bitfield width not supported: {}", layout_size),
                            };

                            // NB: We rely on the ULongLong not being translated
                            // (using the common uintxx_t name)
                            let ti = TypeInfo::new(name.into(),
                                                   ctx.current_module_id,
                                                   TInt(IKind::IULongLong, Layout::new(layout_size, layout_size)),
                                                   Layout::new(layout_size, layout_size));
                            ty = TNamed(Rc::new(RefCell::new(ti)))
                        }
                    }
                    ("".to_owned(), Some(vec![(cursor.spelling(), width)]))
                },
                // The field is not a bitfield
                (None, _) => (cursor.spelling(), None)
            };

            // The Clang C api does not fully expose composite fields, but it
            // does expose them in a way that can be detected. When the current
            // field kind is TComp, TPtr or TArray and the previous member is a
            // composite type - the same type as this field - then this is a
            // composite field.  e.g.:
            //
            //     struct foo {
            //         union {
            //             int a;
            //             char b;
            //         } bar;
            //     };
            //
            //     struct foo {
            //         union {
            //             int a;
            //             char b;
            //         } **bar;
            //     };
            //
            //     struct foo {
            //         union {
            //             int a;
            //             char b;
            //         } bar[3][2];
            //     };
            //

            //let is_composite = match (inner_composite(&ty), ci.members.last()) {
            //    (Some(ty_compinfo), Some(&CompMember::Comp(ref c))) => {
            //        c.borrow().deref() as *const _ == ty_compinfo.borrow().deref() as *const _
            //    },
            //    _ => false
            //};

            if let Some(&mut CompMember::Field(ref mut info)) = ci.members.last_mut() {
                if bitfields.is_none() && info.bitfields.is_none() {
                    let should_replace = if let TComp(ref ci) = info.ty {
                        if ci.borrow().was_unnamed && ty.was_unnamed() &&
                            Some(&ci.borrow().name) == ty.name().as_ref() {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    };

                    if should_replace {
                        *info = FieldInfo::new(name, ty, comment, bitfields, mutable);
                        return CXChildVisit_Continue;
                    }
                }
            }

            let field = FieldInfo::new(name, ty, comment, bitfields, mutable);
            ci.members.push(CompMember::Field(field));
        }
        CXCursor_StructDecl |
        CXCursor_UnionDecl |
        CXCursor_ClassTemplate |
        CXCursor_ClassDecl => {
            fwd_decl(ctx, cursor, |ctx_| {
                // If the struct is anonymous (i.e. declared here) then it
                // cannot be used elsewhere and so does not need to be added
                // to globals otherwise it will be declared later and a global.
                let decl = decl_name(ctx_, cursor);
                let ci2 = decl.compinfo();

                // Mangle the name to prevent multiple definitions
                // of the same inner type to cause conflicts
                let new_name = [&*ci.name, &*ci2.borrow().name].join("_").to_owned();
                ci2.borrow_mut().name = new_name;

                // This clear() is needed because of the speculation we do on
                // incomplete types inside visit_composite() members.
                //
                // If this type ends up being complete, we're going to really
                // parse them now, so we should reset them.
                ci2.borrow_mut().args.clear();

                // Propagate template arguments and typedefs to inner structs
                ci2.borrow_mut().args.extend(ci.args.clone().into_iter());
                ci2.borrow_mut().typedefs.extend(ci.typedefs.clone().into_iter());

                cursor.visit(|c, p| {
                    let mut ci_ = ci2.borrow_mut();
                    visit_composite(c, p, ctx_, &mut ci_)
                });

                ci.members.push(CompMember::Comp(decl.compinfo()));

                // Anonymous structs are legal in both C++ and C11
                if ci2.borrow().was_unnamed {
                    let ci2b = ci2.borrow();
                    let field = FieldInfo::new(ci2b.name.clone(), TComp(ci2.clone()), ci2b.comment.clone(), None, false);
                    ci.members.push(CompMember::Field(field));
                }
            });
        }
        CXCursor_PackedAttr => {
            ci.layout.packed = true;
        }
        CXCursor_TemplateTypeParameter => {
            ci.args.push(conv_template_type_parameter(ctx, cursor));
        }
        CXCursor_EnumDecl => {
            let anno = Annotations::new(cursor);

            fwd_decl(ctx, cursor, |ctx_| {
                let decl = decl_name(ctx_, cursor);
                let ei = decl.enuminfo();
                // Mangle the name to avoid name conflicts with inner types
                let new_name = [&*ci.name, &*ei.borrow().name].join("_").to_owned();
                ei.borrow_mut().name = new_name;
                ei.borrow_mut().comment = cursor.raw_comment();
                cursor.visit(|c, _: &Cursor| {
                    let mut ei_ = ei.borrow_mut();
                    visit_enum(c, &mut ei_.items)
                });
                if anno.opaque {
                    ei.borrow_mut().items = vec!();
                }
                ci.members.push(CompMember::Enum(ei));
            });
        }
        CXCursor_CXXBaseSpecifier => {
            let ty = conv_ty(ctx, &cursor.cur_type(), cursor);
            let fieldname = if ci.members.is_empty() {
                "_base".to_string()
            } else {
                format!("_base{}", ci.members.len())
            };
            let found_virtual_base = if ci.members.is_empty() {
                false
            } else if let CompMember::Field(ref fi) = ci.members[0] {
                if let TComp(ref ci2) = fi.ty {
                    ci2.borrow().has_vtable
                } else {
                    false
                }
            } else {
                false
            };

            if let TComp(ref info) = ty {
                ci.has_nonempty_base |= !info.borrow().members.is_empty();
                ci.has_destructor |= info.borrow().has_destructor;
                ci.typedefs.extend(info.borrow().typedefs.clone().into_iter());
            }

            let field = FieldInfo::new(fieldname, ty, "".to_owned(), None, false);
            if !found_virtual_base && cursor.is_virtual_base() {
                ci.members.insert(0, CompMember::Field(field));
                ci.has_vtable = true;
            } else {
                ci.members.push(CompMember::Field(field));
            }
            ci.base_members += 1;
        }
        CXCursor_CXXMethod => {
            let linkage = cursor.linkage();
            if linkage != CXLinkage_External {
                return CXChildVisit_Continue;
            }

            let visibility = cursor.visibility();
            if visibility != CXVisibility_Default {
                return CXChildVisit_Continue;
            }

            if cursor.is_inlined_function() {
                return CXChildVisit_Continue;
            }

            // XXX no methods yet for templates
            if !ci.args.is_empty() {
                return CXChildVisit_Continue;
            }

            if cursor.access_specifier() == CX_CXXPrivate {
                return CXChildVisit_Continue;
            }

            let spelling = cursor.spelling();
            if spelling.len() > 8 &&
               &(spelling)[..8] == "operator" {
                return CXChildVisit_Continue;
            }

            fn is_override(ci: &CompInfo, sig: &Type, name: &str) -> bool {
                for vm in ci.vmethods.iter() {
                    if vm.name == name && &vm.ty == sig {
                        return true;
                    }
                }
                for base in ci.members[..ci.base_members].iter() {
                    let base = match base {
                        &CompMember::Field(ref fi) => {
                            match fi.ty {
                                TComp(ref ci) => ci.clone(),
                                _ => continue,
                            }
                        },
                        _ => unreachable!()
                    };
                    if is_override(&*base.borrow(), sig, name) {
                        return true;
                    }
                }
                return false;
            }

            if cursor.method_is_virtual() {
                ci.has_vtable = true;
            }

            let mut sig = mk_fn_sig_resolving_typedefs(ctx, &cursor.cur_type(), cursor, &ci.typedefs);
            if !cursor.method_is_static() {
                // XXX what have i done
                if cursor.method_is_virtual() {
                    sig.args.insert(0, ("this".to_string(),TPtr(Box::new(TVoid), cursor.cur_type().is_const(), false, Layout::zero())));
                } else {
                    // XXX This is weak and doesn't work if names are mangled further, but...
                    // We can't have access to the current Rc from here, so we can't pass the type
                    // here.
                    //
                    // Also, it would form an rc cycle.
                    //
                    // Possibly marking the "this" attribute with TOther or a similar marked value
                    // would be a better choice.
                    sig.args.insert(0, ("this".to_string(),
                                        TPtr(Box::new(TNamed(Rc::new(RefCell::new(TypeInfo::new(ci.name.clone(), ctx.current_module_id, TVoid, Layout::zero()))))), cursor.cur_type().is_const(), false, Layout::zero())));
                }
            }

            // XXX with final classes we can optimize a bit
            let sig = TFuncPtr(sig);
            if is_override(ci, &sig, &spelling) {
                return CXChildVisit_Continue;
            }

            let mut vi = VarInfo::new(spelling, cursor.mangling(), cursor.raw_comment(), sig);
            vi.is_static = cursor.method_is_static();
            vi.is_const = cursor.cur_type().is_const();

            if ctx.options.ignore_functions {
                return CXChildVisit_Continue;
            }

            if cursor.method_is_virtual() {
                ci.vmethods.push(vi);
            } else {
                ci.methods.push(vi);
            }
        }
        CXCursor_Destructor => {
            ci.has_destructor = true;
            // Propagate the change to the parent
            if let Some(ref t) = ci.ref_template {
                match *t {
                    TComp(ref parent_ci) => parent_ci.borrow_mut().has_destructor = true,
                    _ => {}
                }
            }
        }
        CXCursor_NonTypeTemplateParameter => {
            log_err_warn(ctx, &format!("warning: Non-type template parameter in composite member could affect layout: `{}` (kind {}) in `{}` ({})",
                              cursor.spelling(), cursor.kind(), parent.spelling(),
                              cursor.location()), false);
            ci.has_non_type_template_params = true;
        }
        CXCursor_VarDecl => {
            if !ctx.options.class_constants {
                return CXChildVisit_Continue;
            }

            let linkage = cursor.linkage();
            if linkage != CXLinkage_External && linkage != CXLinkage_UniqueExternal {
                return CXChildVisit_Continue;
            }

            let visibility = cursor.visibility();
            if visibility != CXVisibility_Default {
                return CXChildVisit_Continue;
            }

            let var = decl_name(ctx, cursor);
            ci.vars.push(var);
        }
        // Intentionally not handled
        CXCursor_CXXAccessSpecifier |
        CXCursor_CXXFinalAttr |
        CXCursor_Constructor |
        CXCursor_FunctionTemplate |
        CXCursor_ConversionFunction => {}
        _ => {
            // XXX: Some kind of warning would be nice, but this produces far
            //      too many.
            log_err_warn(ctx, &format!("unhandled composite member `{}` (kind {}) in `{}` ({})",
                              cursor.spelling(), cursor.kind(), parent.spelling(),
                              cursor.location()), false);
        }
    }
    CXChildVisit_Continue
}

fn visit_enum(cursor: &Cursor,
              items: &mut Vec<EnumItem>) -> Enum_CXVisitorResult {
    if cursor.kind() == CXCursor_EnumConstantDecl {
        let name = cursor.spelling();
        let comment = cursor.raw_comment();
        let val = cursor.enum_val();
        let item = EnumItem::new(name, comment, val);
        items.push(item);
    }
    CXChildVisit_Continue
}

fn parse_int_literal_tokens(cursor: &Cursor, unit: &TranslationUnit, which: usize) -> Option<i64> {
    match unit.tokens(cursor) {
        None => None,
        Some(tokens) => {
            if tokens.len() <= which || tokens[which].kind != CXToken_Literal {
                None
            } else {
                let ref s = tokens[which].spelling;
                let parsed = {
                    //TODO: try to preserve hex literals?
                    if s.starts_with("0x") {
                        i64::from_str_radix(&s[2..], 16)
                    } else {
                        s.parse()
                    }
                };
                match parsed {
                    Ok(i) => Some(i),
                    Err(_) => None,
                }
            }
        }
    }
}

fn visit_literal(cursor: &Cursor, unit: &TranslationUnit) -> Option<i64> {
    if cursor.kind() == CXCursor_IntegerLiteral {
        return parse_int_literal_tokens(cursor, unit, 0);
    }
    return None;
}

fn visit_top(cursor: &Cursor,
             mut ctx: &mut ClangParserCtx) -> Enum_CXVisitorResult {
    if !match_pattern(ctx, cursor) {
        return CXChildVisit_Continue;
    }

    match cursor.kind() {
        CXCursor_UnexposedDecl => {
            return CXChildVisit_Recurse;
        }
        CXCursor_StructDecl
        | CXCursor_UnionDecl
        | CXCursor_ClassDecl
        | CXCursor_ClassTemplate => {
            let anno = Annotations::new(cursor);
            fwd_decl(ctx, cursor, move |ctx_| {
                let decl = decl_name(ctx_, cursor);
                let ci = decl.compinfo();
                // This clear() is needed because of the speculation we do
                // on incomplete types inside visit_composite() members.
                ci.borrow_mut().args.clear();
                cursor.visit(|c, p| {
                    let mut ci_ = ci.borrow_mut();
                    visit_composite(c, p, ctx_, &mut ci_)
                });

                if anno.opaque {
                    ci.borrow_mut().opaque = true;
                }

                if anno.hide {
                    ci.borrow_mut().hide = true;
                }

                if anno.no_copy {
                    ci.borrow_mut().no_copy = true;
                }

                // If we find a previous translation, we take it now and carry
                // on.
                //
                // XXX: This clone is spurious and could be avoided with another
                // scope I think.
                let name = ci.borrow().name.clone();
                if let Some(translation) = ctx_.current_module_mut().translations.remove(&name) {
                    println!("*** {}: found previous translation", name);
                    if let GComp(ref translated) = translation {
                        *ci.borrow_mut() = translated.borrow().clone();
                    }
                }

                if let Some(other_type_name) = anno.use_as {
                    ci.borrow_mut().name = other_type_name.clone();
                    // if the translated type already existed, and we can
                    // replace it, just do it (tm).
                    //
                    // We'll still need the translations map for not found
                    // translations and stuff like that.
                    //
                    // This is a linear search, which is crap, but fwiw it's not
                    // too common (just when a type marked as translation is
                    // found).
                    //
                    // NB: We have to also loop through the `name` map to take
                    // declarations in files that haven't been matched into
                    // account (since they won't appear in globals).
                    let mut found_in_globals = false;
                    for v in ctx_.current_module_mut().globals.iter_mut() {
                        match *v {
                            GComp(ref mut other_ci) => {
                                if other_ci.borrow().name == other_type_name {
                                    *other_ci.borrow_mut() = ci.borrow().clone();
                                    found_in_globals = true;
                                }
                            },
                            _ => {},
                        }
                    }

                    for (cursor, v) in ctx_.name.iter_mut() {
                        // We can find ourselves here, and that's no fun at
                        // all.
                        if *cursor == ci.borrow().parser_cursor.unwrap() {
                            continue;
                        }
                        match *v {
                            GComp(ref mut other_ci) |
                            GCompDecl(ref mut other_ci) => {
                                if other_ci.borrow().name == other_type_name {
                                    // We have to preserve template parameter
                                    // names here if we want to survive.
                                    let args = other_ci.borrow().args.clone();
                                    *other_ci.borrow_mut() = ci.borrow().clone();
                                    other_ci.borrow_mut().args = args;
                                }
                            }
                            _ => {}
                        }
                    }

                    if !found_in_globals {
                        ctx_.current_module_mut().translations
                            .insert(other_type_name, GComp(ci));
                    }
                } else {
                    ctx_.current_module_mut().globals.push(GComp(ci));
                }
            });
            CXChildVisit_Continue
        }
        CXCursor_EnumDecl => {
            fwd_decl(ctx, cursor, |ctx_| {
                let decl = decl_name(ctx_, cursor);
                let ei = decl.enuminfo();
                ei.borrow_mut().comment = cursor.raw_comment();
                cursor.visit(|c, _: &Cursor| {
                    let mut ei_ = ei.borrow_mut();
                    visit_enum(c, &mut ei_.items)
                });
                ctx_.current_module_mut().globals.push(GEnum(ei));
            });
            CXChildVisit_Continue
        }
        CXCursor_FunctionDecl => {
            if ctx.options.ignore_functions {
                return CXChildVisit_Continue;
            }

            let linkage = cursor.linkage();
            if linkage != CXLinkage_External && linkage != CXLinkage_UniqueExternal {
                return CXChildVisit_Continue;
            }

            let visibility = cursor.visibility();
            if visibility != CXVisibility_Default {
                return CXChildVisit_Continue;
            }

            if cursor.is_inlined_function() {
                return CXChildVisit_Continue;
            }

            let spelling = cursor.spelling();
            if spelling.len() > 8 &&
               &(spelling)[..8] == "operator" {
                return CXChildVisit_Continue;
            }

            let func = decl_name(ctx, cursor);
            let vi = func.varinfo();
            let mut vi = vi.borrow_mut();

            vi.ty = TFuncPtr(mk_fn_sig(ctx, &cursor.cur_type(), cursor));
            ctx.current_module_mut().globals.push(func);

            CXChildVisit_Continue
        }
        CXCursor_VarDecl => {
            let linkage = cursor.linkage();
            if linkage != CXLinkage_External && linkage != CXLinkage_UniqueExternal {
                return CXChildVisit_Continue;
            }

            let visibility = cursor.visibility();
            if visibility != CXVisibility_Default {
                return CXChildVisit_Continue;
            }
            let val = decl_name(ctx, cursor);
            ctx.current_module_mut().globals.push(val);

            CXChildVisit_Continue
        }
        CXCursor_TypeAliasDecl | CXCursor_TypedefDecl => {
            let anno = Annotations::new(cursor);
            if anno.hide {
                return CXChildVisit_Continue;
            }

            let mut under_ty = cursor.typedef_type();
            if under_ty.kind() == CXType_Unexposed {
                under_ty = under_ty.canonical_type();
            }

            if cursor.spelling() ==
               cursor.typedef_type().declaration().spelling() {
                // XXX: This is a real hack, but in the common idiom of:
                // typedef struct xxx { ... } xxx;
                //
                // The annotation arrives here, so...
                if anno.opaque {
                    ctx.options.opaque_types.push(cursor.spelling());
                }
                return CXChildVisit_Continue;
            }
            let ty = conv_ty(ctx, &under_ty, cursor);
            let typedef = decl_name(ctx, cursor);
            let ti = typedef.typeinfo();
            let mut ti = ti.borrow_mut();
            ti.ty = ty.clone();

            if anno.opaque {
                ti.opaque = true;
            }

            ti.comment = cursor.raw_comment();
            ctx.current_module_mut().globals.push(typedef);

            opaque_ty(ctx, &under_ty);

            CXChildVisit_Continue
        }
        CXCursor_FieldDecl => {
            CXChildVisit_Continue
        }
        CXCursor_Namespace => {
            if !ctx.options.enable_cxx_namespaces {
                return CXChildVisit_Recurse;
            }

            let namespace_name = match ctx.current_translation_unit.tokens(cursor) {
                None => None,
                Some(tokens) => {
                    if tokens.len() <= 1 {
                        None
                    } else {
                        match &*tokens[1].spelling {
                            "{" => None,
                            s => Some(s.to_owned()),
                        }
                    }
                }
            }.unwrap_or_else(|| {
                ctx.anonymous_modules_found += 1;
                format!("__anonymous{}", ctx.anonymous_modules_found)
            });

            // Find an existing namespace children of the current one
            let mod_id = ctx.current_module()
                            .children_ids.iter()
                            .find(|id| ctx.module_map.get(id).unwrap().name == namespace_name)
                            .map(|id| *id);

            let mod_id = match mod_id {
                Some(id) => id,
                None => {
                    let parent_id = ctx.current_module_id;
                    let id = ModuleId::next();
                    ctx.module_map.get_mut(&parent_id).unwrap().children_ids.push(id);
                    ctx.module_map.insert(id, Module::new(namespace_name, Some(parent_id)));
                    id
                }
            };

            let previous_id = ctx.current_module_id;

            ctx.current_module_id = mod_id;
            cursor.visit(|cur, _: &Cursor| visit_top(cur, &mut ctx));
            ctx.current_module_id = previous_id;

            return CXChildVisit_Continue;
        }
        CXCursor_MacroDefinition => {
            let val = parse_int_literal_tokens(cursor, &ctx.current_translation_unit, 1);
            if val.is_none() {
                // Not an integer literal.
                return CXChildVisit_Continue;
            }
            let var = decl_name(ctx, cursor);
            let vi = var.varinfo();
            let mut vi = vi.borrow_mut();
            vi.ty = match val {
                None => TVoid,
                Some(v) if v.abs() > u32::max_value() as i64 => TInt(IULongLong, Layout::new(8, 8)),
                _ => TInt(IUInt, Layout::new(4, 4)),
            };
            vi.is_const = true;
            vi.val = val;
            ctx.current_module_mut().globals.push(var);

            return CXChildVisit_Continue;
        }
        _ => {
            // println!("Not handled cursor: {}", cursor.kind());
            return CXChildVisit_Continue;
        }
    }
}

fn log_err_warn(ctx: &mut ClangParserCtx, msg: &str, is_err: bool) {
    if is_err {
        ctx.err_count += 1;
        ctx.logger.error(msg);
    } else {
        ctx.logger.warn(msg);
    }
}

pub fn parse(options: ClangParserOptions, logger: &Logger) -> Result<ModuleMap, ()> {
    let ix = cx::Index::create(false, true);
    if ix.is_null() {
        logger.error("Clang failed to create index");
        return Err(())
    }

    let unit = TranslationUnit::parse(&ix, "", &options.clang_args, &[], CXTranslationUnit_DetailedPreprocessingRecord);
    if unit.is_null() {
        logger.error("No input files given");
        return Err(())
    }

    let mut ctx = ClangParserCtx {
        options: options,
        name: HashMap::new(),
        builtin_defs: vec!(),
        module_map: ModuleMap::new(),
        current_module_id: ROOT_MODULE_ID,
        current_translation_unit: unit,
        logger: logger,
        err_count: 0,
        anonymous_modules_found: 0,
    };

    ctx.module_map.insert(ROOT_MODULE_ID, Module::new("root".to_owned(), None));

    let diags = ctx.current_translation_unit.diags();
    for d in &diags {
        let msg = d.format(Diagnostic::default_opts());
        let is_err = d.severity() >= CXDiagnostic_Error;
        log_err_warn(&mut ctx, &msg, is_err);
    }

    if ctx.err_count > 0 {
        return Err(())
    }

    let cursor = ctx.current_translation_unit.cursor();

    if ctx.options.emit_ast {
        cursor.visit(|cur, _: &Cursor| ast_dump(cur, 0));
    }

    cursor.visit(|cur, _: &Cursor| visit_top(cur, &mut ctx));

    while !ctx.builtin_defs.is_empty() {
        let c = ctx.builtin_defs.remove(0);
        visit_top(&c.definition(), &mut ctx);
    }

    ctx.current_translation_unit.dispose();
    ix.dispose();

    if ctx.err_count > 0 {
        return Err(())
    }

    Ok(ctx.module_map)
}
