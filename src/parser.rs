#![allow(non_upper_case_globals)]

use std::collections::{HashMap, HashSet};
use std::collections::hash_map;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use clang_sys::*;

use syntax::abi;

use types as il;
use types::*;
use clang as cx;
use clang::{Cursor, Diagnostic, TranslationUnit, ast_dump};

use super::Logger;

pub struct ClangParserOptions {
    pub builtin_names: HashSet<String>,
    pub builtins: bool,
    pub match_pat: Vec<String>,
    pub emit_ast: bool,
    pub fail_on_unknown_type: bool,
    pub override_enum_ty: Option<il::IKind>,
    pub clang_args: Vec<String>,
}

struct ClangParserCtx<'a> {
    options: ClangParserOptions,
    name: HashMap<Cursor, Global>,
    globals: Vec<Global>,
    builtin_defs: Vec<Cursor>,
    logger: &'a (Logger + 'a),
    err_count: i32,
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

    let mut found = false;
    ctx.options.match_pat.iter().all(|pat| {
        if (&name[..]).contains(pat) {
            found = true;
        }
        true
    });

    found
}

#[cfg_attr(feature = "clippy", allow(match_same_arms))]
fn decl_name(ctx: &mut ClangParserCtx, cursor: &Cursor) -> Global {
    let cursor = cursor.canonical();
    let mut new_decl = false;
    let override_enum_ty = ctx.options.override_enum_ty;
    let decl = match ctx.name.entry(cursor) {
        hash_map::Entry::Occupied(ref e) => e.get().clone(),
        hash_map::Entry::Vacant(e) => {
            new_decl = true;
            let spelling = cursor.spelling();
            let ty = cursor.cur_type();
            let layout = Layout::new(ty.size(), ty.align());

            debug!("type `{}` = {:?}; // {:?}", spelling, ty, layout);

            let glob_decl = match cursor.kind() {
                CXCursorKind::StructDecl => {
                    let ci = Rc::new(RefCell::new(CompInfo::new(spelling,
                                                                CompKind::Struct,
                                                                vec![],
                                                                layout)));
                    GCompDecl(ci)
                }
                CXCursorKind::UnionDecl => {
                    let ci = Rc::new(RefCell::new(CompInfo::new(spelling,
                                                                CompKind::Union,
                                                                vec![],
                                                                layout)));
                    GCompDecl(ci)
                }
                CXCursorKind::EnumDecl => {
                    let kind = match override_enum_ty {
                        Some(t) => t,
                        None => {
                            match cursor.enum_type().kind() {
                                CXTypeKind::SChar | CXTypeKind::Char_S => ISChar,
                                CXTypeKind::UChar | CXTypeKind::Char_U => IUChar,
                                CXTypeKind::UShort => IUShort,
                                CXTypeKind::UInt => IUInt,
                                CXTypeKind::ULong => IULong,
                                CXTypeKind::ULongLong => IULongLong,
                                CXTypeKind::Short => IShort,
                                CXTypeKind::Int => IInt,
                                CXTypeKind::Long => ILong,
                                CXTypeKind::LongLong => ILongLong,
                                _ => IInt,
                            }
                        }
                    };
                    let ei = Rc::new(RefCell::new(EnumInfo::new(spelling, kind, vec![], layout)));
                    GEnumDecl(ei)
                }
                CXCursorKind::TypedefDecl => {
                    let ti = Rc::new(RefCell::new(TypeInfo::new(spelling, TVoid, layout)));
                    GType(ti)
                }
                CXCursorKind::VarDecl => {
                    let vi = Rc::new(RefCell::new(VarInfo::new(spelling, TVoid)));
                    GVar(vi)
                }
                CXCursorKind::FunctionDecl => {
                    let vi = Rc::new(RefCell::new(VarInfo::new(spelling, TVoid)));
                    GFunc(vi)
                }
                _ => GOther,
            };

            e.insert(glob_decl.clone());
            glob_decl
        }
    };

    if new_decl && ctx.options.builtin_names.contains(&cursor.spelling()) {
        ctx.builtin_defs.push(cursor);
    }

    decl
}

fn opaque_decl(ctx: &mut ClangParserCtx, decl: &Cursor) {
    let name = decl_name(ctx, decl);
    ctx.globals.push(name);
}

fn fwd_decl<F: FnOnce(&mut ClangParserCtx) -> ()>(ctx: &mut ClangParserCtx,
                                                  cursor: &Cursor,
                                                  f: F) {
    let def = &cursor.definition();
    if cursor == def {
        f(ctx);
    } else if def.kind() == CXCursorKind::NoDeclFound || def.kind() == CXCursorKind::InvalidFile {
        opaque_decl(ctx, cursor);
    }
}

fn get_abi(cc: CXCallingConv) -> abi::Abi {
    match cc {
        CXCallingConv::Default | CXCallingConv::C => abi::Abi::C,
        CXCallingConv::X86StdCall => abi::Abi::Stdcall,
        CXCallingConv::X86FastCall => abi::Abi::Fastcall,
        CXCallingConv::AAPCS => abi::Abi::Aapcs,
        CXCallingConv::X86_64Win64 => abi::Abi::Win64,
        other => panic!("unsupported calling convention: {:?}", other),
    }
}

fn conv_ptr_ty(ctx: &mut ClangParserCtx,
               ty: &cx::Type,
               cursor: &Cursor,
               layout: Layout)
               -> il::Type {
    let is_const = ty.is_const();
    match ty.kind() {
        CXTypeKind::Unexposed |
        CXTypeKind::FunctionProto |
        CXTypeKind::FunctionNoProto => {
            let ret_ty = ty.ret_type();
            let decl = ty.declaration();
            if ret_ty.kind() != CXTypeKind::Invalid {
                TFuncPtr(mk_fn_sig(ctx, ty, cursor), layout)
            } else if decl.kind() != CXCursorKind::NoDeclFound {
                TPtr(Box::new(conv_decl_ty(ctx, &decl)), ty.is_const(), layout)
            } else if cursor.kind() == CXCursorKind::VarDecl {
                let can_ty = ty.canonical_type();
                conv_ty(ctx, &can_ty, cursor)
            } else {
                TPtr(Box::new(TVoid), ty.is_const(), layout)
            }
        }
        _ => TPtr(Box::new(conv_ty(ctx, ty, cursor)), is_const, layout),
    }
}

fn mk_fn_sig(ctx: &mut ClangParserCtx, ty: &cx::Type, cursor: &Cursor) -> il::FuncSig {
    let args_lst: Vec<(String, il::Type)> = match cursor.kind() {
        CXCursorKind::FunctionDecl => {
            // For CXCursorKind::FunctionDecl, cursor.args() is the reliable way to
            // get parameter names and types.
            cursor.args()
                  .iter()
                  .map(|arg| {
                      let arg_name = arg.spelling();
                      (arg_name, conv_ty(ctx, &arg.cur_type(), arg))
                  })
                  .collect()
        }
        _ => {
            // For non-CXCursorKind::FunctionDecl, visiting the cursor's children is
            // the only reliable way to get parameter names.
            let mut args_lst = vec![];
            cursor.visit(|c: &Cursor, _: &Cursor| {
                if c.kind() == CXCursorKind::ParmDecl {
                    args_lst.push((c.spelling(), conv_ty(ctx, &c.cur_type(), c)));
                }
                CXChildVisitResult::Continue
            });
            args_lst
        }
    };

    let ret_ty = Box::new(conv_ty(ctx, &ty.ret_type(), cursor));
    let abi = get_abi(ty.call_conv());

    // Function is presumed unsafe if it takes a pointer argument.
    let is_unsafe = args_lst.iter().any(|arg| {
        match arg.1 {
            TPtr(_, _, _) => true,
            _ => false,
        }
    });

    il::FuncSig {
        ret_ty: ret_ty,
        args: args_lst,
        is_variadic: ty.is_variadic(),
        is_safe: !is_unsafe,
        abi: abi,
    }
}

fn conv_decl_ty(ctx: &mut ClangParserCtx, cursor: &Cursor) -> il::Type {
    match cursor.kind() {
        CXCursorKind::StructDecl | CXCursorKind::UnionDecl => {
            let decl = decl_name(ctx, cursor);
            let ci = decl.compinfo();
            TComp(ci)
        }
        CXCursorKind::EnumDecl => {
            let decl = decl_name(ctx, cursor);
            let ei = decl.enuminfo();
            TEnum(ei)
        }
        CXCursorKind::TypedefDecl => {
            let decl = decl_name(ctx, cursor);
            let ti = decl.typeinfo();
            TNamed(ti)
        }
        _ => TVoid,
    }
}

fn conv_ty(ctx: &mut ClangParserCtx, ty: &cx::Type, cursor: &Cursor) -> il::Type {
    debug!("conv_ty: ty=`{:?}` sp=`{}` loc=`{}` size=`{}` align=`{}`",
           ty.kind(),
           cursor.spelling(),
           cursor.location(),
           ty.size(),
           ty.align());

    let layout = Layout::new(ty.size(), ty.align());

    match ty.kind() {
        CXTypeKind::Void | CXTypeKind::Invalid => TVoid,
        CXTypeKind::Bool => TInt(IBool, layout),
        CXTypeKind::SChar |
        CXTypeKind::Char_S => TInt(ISChar, layout),
        CXTypeKind::UChar |
        CXTypeKind::Char_U => TInt(IUChar, layout),
        CXTypeKind::UShort => TInt(IUShort, layout),
        CXTypeKind::UInt => TInt(IUInt, layout),
        CXTypeKind::ULong => TInt(IULong, layout),
        CXTypeKind::ULongLong => TInt(IULongLong, layout),
        CXTypeKind::Short => TInt(IShort, layout),
        CXTypeKind::Int => TInt(IInt, layout),
        CXTypeKind::Long => TInt(ILong, layout),
        CXTypeKind::LongLong => TInt(ILongLong, layout),
        CXTypeKind::Float => TFloat(FFloat, layout),
        CXTypeKind::Double | CXTypeKind::LongDouble => TFloat(FDouble, layout),
        CXTypeKind::Pointer => conv_ptr_ty(ctx, &ty.pointee_type(), cursor, layout),
        CXTypeKind::VariableArray => unreachable!(),
        CXTypeKind::DependentSizedArray | CXTypeKind::IncompleteArray => {
            TArray(Box::new(conv_ty(ctx, &ty.elem_type(), cursor)), 0, layout)
        }
        CXTypeKind::FunctionProto | CXTypeKind::FunctionNoProto => {
            TFuncProto(mk_fn_sig(ctx, ty, cursor), layout)
        }
        CXTypeKind::Record |
        CXTypeKind::Typedef |
        CXTypeKind::Unexposed |
        CXTypeKind::Enum => conv_decl_ty(ctx, &ty.declaration()),
        CXTypeKind::ConstantArray => {
            TArray(Box::new(conv_ty(ctx, &ty.elem_type(), cursor)),
                   ty.array_size(),
                   layout)
        }
        _ => {
            let fail = ctx.options.fail_on_unknown_type;
            log_err_warn(ctx,
                         &format!("unsupported type `{:?}` ({})",
                                  ty.kind(),
                                  cursor.location())[..],
                         fail);
            TVoid
        }
    }
}

fn opaque_ty(ctx: &mut ClangParserCtx, ty: &cx::Type) {
    if ty.kind() == CXTypeKind::Record || ty.kind() == CXTypeKind::Enum {
        let decl = ty.declaration();
        let def = decl.definition();
        if def.kind() == CXCursorKind::NoDeclFound || def.kind() == CXCursorKind::InvalidFile {
            opaque_decl(ctx, &decl);
        }
    }
}

/// Recursively visits a cursor that represents a composite (struct or union)
/// type and fills members with `CompMember` instances representing the fields and
/// nested composites that make up the visited composite.
fn visit_composite(cursor: &Cursor,
                   parent: &Cursor,
                   ctx: &mut ClangParserCtx,
                   compinfo: &mut CompInfo)
                   -> CXChildVisitResult {
    fn is_bitfield_continuation(field: &il::FieldInfo, ty: &il::Type, width: u32) -> bool {
        match (&field.bitfields, ty) {
            (&Some(ref bitfields), &il::TInt(_, layout)) if *ty == field.ty => {
                bitfields.iter().map(|&(_, w)| w).fold(0u32, |acc, w| acc + w) + width <=
                (layout.size * 8) as u32
            }
            _ => false,
        }
    }

    fn inner_composite(mut ty: &il::Type) -> Option<&Rc<RefCell<CompInfo>>> {
        loop {
            match *ty {
                TComp(ref comp_ty) => return Some(comp_ty),
                TPtr(ref ptr_ty, _, _) => ty = &**ptr_ty,
                TArray(ref array_ty, _, _) => ty = &**array_ty,
                _ => return None,
            }
        }
    }

    fn inner_enumeration(mut ty: &il::Type) -> Option<&Rc<RefCell<EnumInfo>>> {
        loop {
            match *ty {
                TEnum(ref enum_ty) => return Some(enum_ty),
                TPtr(ref ptr_ty, _, _) => ty = &**ptr_ty,
                TArray(ref array_ty, _, _) => ty = &**array_ty,
                _ => return None,
            }
        }
    }

    let members = &mut compinfo.members;

    match cursor.kind() {
        CXCursorKind::FieldDecl => {
            let ty = conv_ty(ctx, &cursor.cur_type(), cursor);

            let (name, bitfields) = match (cursor.bit_width(), members.last_mut()) {
                // The field is a continuation of an exising bitfield
                (Some(width), Some(&mut il::CompMember::Field(ref mut field)))
                    if is_bitfield_continuation(field, &ty, width) => {

                    if let Some(ref mut bitfields) = field.bitfields {
                        bitfields.push((cursor.spelling(), width));
                    } else {
                        unreachable!()
                    }
                    return CXChildVisitResult::Continue;
                }
                // The field is the start of a new bitfield
                (Some(width), _) => {
                    // Bitfields containing enums are not supported by the c standard
                    // https://stackoverflow.com/questions/11983231/is-it-safe-to-use-an-enum-in-a-bit-field
                    match ty {
                        il::TInt(_, _) => (),
                        _ => {
                            let msg = format!("Enums in bitfields are not supported ({}.{}).",
                                              cursor.spelling(),
                                              parent.spelling());
                            ctx.logger.warn(&msg[..]);
                        }
                    }
                    ("".to_owned(), Some(vec![(cursor.spelling(), width)]))
                }
                // The field is not a bitfield
                (None, _) => (cursor.spelling(), None),
            };

            // The Clang C api does not fully expose composite and enumeration
            // fields, but it does expose them in a way that can be detected.
            // When the current field kind is TComp/TEnum, TPtr or TArray and
            // the previous member is a composite/enumeration type - the same
            // type as this field - then this is a composite or enumeration
            // field. e.g.:
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
            //     struct foo {
            //         enum {
            //             OPTION_1,
            //             OPTION_2,
            //             OPTION_3
            //         } bar;
            //     };
            //

            let is_composite = match (inner_composite(&ty), members.last()) {
                (Some(ty_compinfo), Some(&CompMember::Comp(ref c))) => {
                    c.borrow().deref() as *const _ == ty_compinfo.borrow().deref() as *const _
                }
                _ => false,
            };

            let is_enumeration = match (inner_enumeration(&ty), members.last()) {
                (Some(ty_enuminfo), Some(&CompMember::Enum(ref e))) => {
                    e.borrow().deref() as *const _ == ty_enuminfo.borrow().deref() as *const _
                }
                _ => false,
            };

            let field = FieldInfo::new(name, ty.clone(), bitfields);
            if is_composite {
                if let Some(CompMember::Comp(c)) = members.pop() {
                    members.push(CompMember::CompField(c, field));
                } else {
                    unreachable!(); // Checks in is_composite make this unreachable.
                }
            } else if is_enumeration {
                if let Some(CompMember::Enum(e)) = members.pop() {
                    members.push(CompMember::EnumField(e, field));
                } else {
                    unreachable!(); // Checks in is_enumeration make this unreachable.
                }
            } else {
                members.push(CompMember::Field(field));
            }
        }
        CXCursorKind::StructDecl | CXCursorKind::UnionDecl => {
            fwd_decl(ctx, cursor, |ctx_| {
                // If the struct is anonymous (i.e. declared here) then it
                // cannot be used elsewhere and so does not need to be added
                // to globals otherwise it will be declared later and a global.
                let decl = decl_name(ctx_, cursor);
                let ci = decl.compinfo();
                cursor.visit(|c, p| {
                    let mut ci_ = ci.borrow_mut();
                    visit_composite(c, p, ctx_, &mut ci_)
                });
                members.push(CompMember::Comp(decl.compinfo()));
            });
        }
        CXCursorKind::EnumDecl => {
            fwd_decl(ctx, cursor, |ctx_| {
                // If the enum is anonymous (i.e. declared here) then it
                // cannot be used elsewhere and so does not need to be added
                // to globals otherwise it will be declared later and a global.
                let decl = decl_name(ctx_, cursor);
                let ci = decl.enuminfo();
                cursor.visit(|c, _| {
                    let mut ci_ = ci.borrow_mut();
                    visit_enum(c, &mut ci_.items)
                });
                members.push(CompMember::Enum(decl.enuminfo()));
            });
        }
        CXCursorKind::PackedAttr => {
            compinfo.layout.packed = true;
        }
        CXCursorKind::UnexposedAttr => {
            // skip unknown attributes
        }
        _ => {
            // XXX: Some kind of warning would be nice, but this produces far
            //      too many.
            log_err_warn(ctx,
                         &format!("unhandled composite member `{}` (kind {:?}) in `{}` ({})",
                                  cursor.spelling(),
                                  cursor.kind(),
                                  parent.spelling(),
                                  cursor.location())[..],
                         false);
        }
    }
    CXChildVisitResult::Continue
}

fn visit_enum(cursor: &Cursor, items: &mut Vec<EnumItem>) -> CXChildVisitResult {
    if cursor.kind() == CXCursorKind::EnumConstantDecl {
        let name = cursor.spelling();
        let val = cursor.enum_val();
        let item = EnumItem::new(name, val);
        items.push(item);
    }
    CXChildVisitResult::Continue
}

fn visit_literal(cursor: &Cursor, unit: &TranslationUnit) -> Option<i64> {
    if cursor.kind() == CXCursorKind::IntegerLiteral {
        match unit.tokens(cursor) {
            None => None,
            Some(tokens) => {
                if tokens.is_empty() || tokens[0].kind != CXTokenKind::Literal {
                    None
                } else {
                    let s = &tokens[0].spelling;
                    let parsed = {
                        // TODO: try to preserve hex literals?
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
    } else {
        None
    }
}

fn visit_top(cursor: &Cursor,
             ctx: &mut ClangParserCtx,
             unit: &TranslationUnit)
             -> CXChildVisitResult {
    if !match_pattern(ctx, cursor) {
        return CXChildVisitResult::Continue;
    }

    match cursor.kind() {
        CXCursorKind::UnexposedDecl => CXChildVisitResult::Recurse,
        CXCursorKind::StructDecl | CXCursorKind::UnionDecl => {
            fwd_decl(ctx, cursor, |ctx_| {
                let decl = decl_name(ctx_, cursor);
                let ci = decl.compinfo();
                cursor.visit(|c, p| {
                    let mut ci_ = ci.borrow_mut();
                    visit_composite(c, p, ctx_, &mut ci_)
                });
                ctx_.globals.push(GComp(ci));
            });
            CXChildVisitResult::Continue
        }
        CXCursorKind::EnumDecl => {
            fwd_decl(ctx, cursor, |ctx_| {
                let decl = decl_name(ctx_, cursor);
                let ei = decl.enuminfo();
                cursor.visit(|c, _: &Cursor| {
                    let mut ei_ = ei.borrow_mut();
                    visit_enum(c, &mut ei_.items)
                });
                ctx_.globals.push(GEnum(ei));
            });
            CXChildVisitResult::Continue
        }
        CXCursorKind::FunctionDecl => {
            let linkage = cursor.linkage();
            if linkage != CXLinkageKind::External && linkage != CXLinkageKind::UniqueExternal {
                return CXChildVisitResult::Continue;
            }

            let func = decl_name(ctx, cursor);
            let vi = func.varinfo();
            let mut vi = vi.borrow_mut();

            let ty = cursor.cur_type();
            let layout = Layout::new(ty.size(), ty.align());

            vi.ty = TFuncPtr(mk_fn_sig(ctx, &ty, cursor), layout);
            ctx.globals.push(func);

            CXChildVisitResult::Continue
        }
        CXCursorKind::VarDecl => {
            let linkage = cursor.linkage();
            if linkage != CXLinkageKind::External && linkage != CXLinkageKind::UniqueExternal {
                return CXChildVisitResult::Continue;
            }

            let ty = conv_ty(ctx, &cursor.cur_type(), cursor);
            let var = decl_name(ctx, cursor);
            let vi = var.varinfo();
            let mut vi = vi.borrow_mut();
            vi.ty = ty.clone();
            vi.is_const = cursor.cur_type().is_const();
            cursor.visit(|c, _: &Cursor| {
                vi.val = visit_literal(c, unit);
                CXChildVisitResult::Continue
            });
            ctx.globals.push(var);

            CXChildVisitResult::Continue
        }
        CXCursorKind::TypedefDecl => {
            let mut under_ty = cursor.typedef_type();
            if under_ty.kind() == CXTypeKind::Unexposed {
                under_ty = under_ty.canonical_type();
            }

            let ty = conv_ty(ctx, &under_ty, cursor);
            let typedef = decl_name(ctx, cursor);
            let ti = typedef.typeinfo();
            let mut ti = ti.borrow_mut();
            ti.ty = ty.clone();
            ctx.globals.push(typedef);

            opaque_ty(ctx, &under_ty);

            CXChildVisitResult::Continue
        }
        CXCursorKind::FieldDecl => CXChildVisitResult::Continue,
        _ => CXChildVisitResult::Continue,
    }
}

fn log_err_warn(ctx: &mut ClangParserCtx, msg: &str, is_err: bool) {
    if is_err {
        ctx.err_count += 1;
        ctx.logger.error(msg)
    } else {
        ctx.logger.warn(msg)
    }
}

pub fn parse(options: ClangParserOptions, logger: &Logger) -> Result<Vec<Global>, ()> {
    let mut ctx = ClangParserCtx {
        options: options,
        name: HashMap::new(),
        builtin_defs: vec![],
        globals: vec![],
        logger: logger,
        err_count: 0,
    };

    let ix = cx::Index::create(false, true);
    if ix.is_null() {
        ctx.logger.error("Clang failed to create index");
        return Err(());
    }

    let flags = CXTranslationUnit_Flags::empty();
    let unit = TranslationUnit::parse(&ix, "", &ctx.options.clang_args[..], &[], flags);
    if unit.is_null() {
        ctx.logger.error("No input files given");
        return Err(());
    }

    let diags = unit.diags();
    for d in &diags {
        let msg = d.format(Diagnostic::default_opts());
        let is_err = d.severity() >= CXDiagnosticSeverity::Error;
        log_err_warn(&mut ctx, &msg[..], is_err);
    }

    if ctx.err_count > 0 {
        return Err(());
    }

    let cursor = unit.cursor();

    if ctx.options.emit_ast {
        cursor.visit(|cur, _: &Cursor| ast_dump(cur, 0));
    }

    cursor.visit(|cur, _: &Cursor| visit_top(cur, &mut ctx, &unit));

    while !ctx.builtin_defs.is_empty() {
        let c = ctx.builtin_defs.remove(0);
        visit_top(&c.definition(), &mut ctx, &unit);
    }

    unit.dispose();
    ix.dispose();

    if ctx.err_count > 0 {
        return Err(());
    }

    Ok(ctx.globals)
}
