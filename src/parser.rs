#![allow(non_uppercase_pattern_statics)]
#![allow(unused_must_use)]

use std::collections::{HashMap, HashSet};
use std::cell::RefCell;
use std::gc::GC;

use il = types;
use types::*;
use cx = clang;
use clang::*;
use clang::ll::*;

use super::Logger;

pub struct ClangParserOptions {
    pub builtin_names: HashSet<String>,
    pub builtins: bool,
    pub match_pat: Vec<String>,
    pub emit_ast: bool,
    pub fail_on_bitfield: bool,
    pub fail_on_unknown_type: bool,
    pub clang_args: Vec<String>,
}

struct ClangParserCtx<'a> {
    options: ClangParserOptions,
    name: HashMap<Cursor, Global>,
    globals: Vec<Global>,
    builtin_defs: Vec<Cursor>,
    logger: &'a Logger,
    err_count: int
}

fn match_pattern(ctx: &mut ClangParserCtx, cursor: &Cursor) -> bool {
    let (file, _, _, _) = cursor.location().location();

    if file.is_null() {
        return ctx.options.builtins;
    }

    if ctx.options.match_pat.is_empty() {
        return true;
    }

    let name = file.name();
    let mut found = false;
    ctx.options.match_pat.iter().advance(|pat| {
        if name.as_slice().contains((*pat).as_slice()) {
            found = true;
        }
        true
    });

    return found;
}

fn decl_name(ctx: &mut ClangParserCtx, cursor: &Cursor) -> Global {
    let mut new_decl = false;
    let decl = {
        *ctx.name.find_or_insert_with(*cursor, |_| {
            new_decl = true;
            let spelling = cursor.spelling();
            let ty = cursor.cur_type();
            let layout = Layout::new(ty.size(), ty.align());

            match cursor.kind() {
                CXCursor_StructDecl => {
                    let ci = box(GC) RefCell::new(CompInfo::new(spelling, true, vec!(), layout));
                    GCompDecl(ci)
                }
                CXCursor_UnionDecl => {
                    let ci = box(GC) RefCell::new(CompInfo::new(spelling, false, vec!(), layout));
                    GCompDecl(ci)
                }
                CXCursor_EnumDecl => {
                    let kind = match cursor.enum_type().kind() {
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
                    };
                    let ei = box(GC) RefCell::new(EnumInfo::new(spelling, kind, vec!(), layout));
                    GEnumDecl(ei)
                }
                CXCursor_TypedefDecl => {
                    let ti = box(GC) RefCell::new(TypeInfo::new(spelling, TVoid));
                    GType(ti)
                }
                CXCursor_VarDecl => {
                    let vi = box(GC) RefCell::new(VarInfo::new(spelling, TVoid));
                    GVar(vi)
                }
                CXCursor_FunctionDecl => {
                    let vi = box(GC) RefCell::new(VarInfo::new(spelling, TVoid));
                    GFunc(vi)
                }
                _ => GOther
            }
        })
    };

    if new_decl {
        if ctx.options.builtin_names.contains(&cursor.spelling()) {
            ctx.builtin_defs.push(*cursor);
        }
    }

    return decl;
}

fn opaque_decl(ctx: &mut ClangParserCtx, decl: &Cursor) {
    let name = decl_name(ctx, decl);
    ctx.globals.push(name);
}

fn fwd_decl(ctx: &mut ClangParserCtx, cursor: &Cursor, f: |ctx: &mut ClangParserCtx|) {
    let def = &cursor.definition();
    if cursor == def {
        f(ctx);
    } else if def.kind() == CXCursor_NoDeclFound ||
              def.kind() == CXCursor_InvalidFile {
        opaque_decl(ctx, cursor);
    }
}

fn conv_ptr_ty(ctx: &mut ClangParserCtx, ty: &cx::Type, cursor: &Cursor, layout: Layout) -> il::Type {
    let is_const = ty.is_const();
    match ty.kind() {
        CXType_Void => {
            return TPtr(box TVoid, is_const, layout)
        }
        CXType_Unexposed |
        CXType_FunctionProto |
        CXType_FunctionNoProto => {
            let ret_ty = ty.ret_type();
            let decl = ty.declaration();
            return if ret_ty.kind() != CXType_Invalid {
                let args_lst = ty.arg_types().iter().map(|arg| {
                    ("".to_string(), conv_ty(ctx, arg, cursor))
                }).collect();
                let ret_ty = box conv_ty(ctx, &ret_ty, cursor);

                TFunc(ret_ty, args_lst, ty.is_variadic())
            } else if decl.kind() != CXCursor_NoDeclFound {
                TPtr(box conv_decl_ty(ctx, &decl), is_const, layout)
            } else {
                TPtr(box TVoid, is_const, layout)
            };
        }
        CXType_Typedef => {
            let decl = ty.declaration();
            let def_ty = decl.typedef_type();
            if def_ty.kind() == CXType_FunctionProto ||
               def_ty.kind() == CXType_FunctionNoProto {
                return TPtr(box conv_ptr_ty(ctx, &def_ty, cursor, layout), is_const, layout);
            } else {
                return TPtr(box conv_ty(ctx, ty, cursor), is_const, layout);
            }
        }
        _ => return TPtr(box conv_ty(ctx, ty, cursor), is_const, layout),
    }
}

fn conv_decl_ty(ctx: &mut ClangParserCtx, cursor: &Cursor) -> il::Type {
    return match cursor.kind() {
        CXCursor_StructDecl => {
            let decl = decl_name(ctx, cursor);
            let ci = decl.compinfo();
            TComp(ci)
        }
        CXCursor_UnionDecl => {
            let decl = decl_name(ctx, cursor);
            let ci = decl.compinfo();
            TComp(ci)
        }
        CXCursor_EnumDecl => {
            let decl = decl_name(ctx, cursor);
            let ei = decl.enuminfo();
            TEnum(ei)
        }
        CXCursor_TypedefDecl => {
            let decl = decl_name(ctx, cursor);
            let ti = decl.typeinfo();
            TNamed(ti)
        }
        _ => TVoid
    };
}

fn conv_ty(ctx: &mut ClangParserCtx, ty: &cx::Type, cursor: &Cursor) -> il::Type {
    debug!("conv_ty: ty=`{}`", type_to_str(ty.kind()));
    let layout = Layout::new(ty.size(), ty.align());
    return match ty.kind() {
        CXType_Void | CXType_Invalid => TVoid,
        CXType_Bool => TInt(IBool, layout),
        CXType_SChar |
        CXType_Char_S => TInt(ISChar, layout),
        CXType_UChar |
        CXType_Char_U => TInt(IUChar, layout),
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
        CXType_Pointer => conv_ptr_ty(ctx, &ty.pointee_type(), cursor, layout),
        CXType_VariableArray | CXType_DependentSizedArray | CXType_IncompleteArray => {
            conv_ptr_ty(ctx, &ty.elem_type(), cursor, layout)
        }
        CXType_Record |
        CXType_Typedef  |
        CXType_Unexposed |
        CXType_Enum => conv_decl_ty(ctx, &ty.declaration()),
        CXType_ConstantArray => TArray(box conv_ty(ctx, &ty.elem_type(), cursor), ty.array_size(), layout),
        _ => {
            let fail = ctx.options.fail_on_unknown_type;
            log_err_warn(ctx,
                format!("unsupported type `{}` ({})",
                    type_to_str(ty.kind()), cursor.location()
                ).as_slice(),
                fail
            );
            TVoid
        },
    };
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

fn visit_struct(cursor: &Cursor,
                parent: &Cursor,
                ctx: &mut ClangParserCtx,
                fields: &mut Vec<FieldInfo>) -> Enum_CXVisitorResult {
    if cursor.kind() == CXCursor_FieldDecl {
        let ty = conv_ty(ctx, &cursor.cur_type(), cursor);
        let name = cursor.spelling();
        let bit = cursor.bit_width();
        // If we encounter a bitfield, and fail_on_bitfield is set, throw an
        // error and exit entirely.
        if bit != None {
            let fail = ctx.options.fail_on_bitfield;
            log_err_warn(ctx,
                format!("unsupported bitfield `{}` in struct `{}` ({})",
                    name.as_slice(), parent.spelling().as_slice(), cursor.location()
                ).as_slice(),
                fail
            );
        }
        let field = FieldInfo::new(name, ty, bit);
        fields.push(field);
    }
    return CXChildVisit_Continue;
}

fn visit_union(cursor: &Cursor,
               ctx: &mut ClangParserCtx,
               fields: &mut Vec<FieldInfo>) -> Enum_CXVisitorResult {
    if cursor.kind() == CXCursor_FieldDecl {
        let ty = conv_ty(ctx, &cursor.cur_type(), cursor);
        let name = cursor.spelling();
        let field = FieldInfo::new(name, ty, None);
        fields.push(field);
    }
    return CXChildVisit_Continue;
}

fn visit_enum(cursor: &Cursor,
              items: &mut Vec<EnumItem>) -> Enum_CXVisitorResult {
    if cursor.kind() == CXCursor_EnumConstantDecl {
        let name = cursor.spelling();
        let val = cursor.enum_val();
        let item = EnumItem::new(name, val);
        items.push(item);
    }
    return CXChildVisit_Continue;
}

fn visit_top<'r>(cur: &'r Cursor,
                 parent: &'r Cursor,
                 ctx: &mut ClangParserCtx) -> Enum_CXVisitorResult {
    let cursor = if ctx.options.builtin_names.contains(&parent.spelling()) {
        parent
    } else {
        cur
    };

    if !match_pattern(ctx, cursor) {
        return CXChildVisit_Continue;
    }

    match cursor.kind() {
        CXCursor_StructDecl => {
          fwd_decl(ctx, cursor, |ctx_| {
              let decl = decl_name(ctx_, cursor);
              let ci = decl.compinfo();
              cursor.visit(|c, p| {
                  let mut ci_ = ci.borrow_mut();
                  visit_struct(c, p, ctx_, &mut ci_.fields)
              });
              ctx_.globals.push(GComp(ci));
          });
          return if cur.kind() == CXCursor_FieldDecl {
              CXChildVisit_Break
          } else {
              CXChildVisit_Recurse
          };
      }
      CXCursor_UnionDecl => {
          fwd_decl(ctx, cursor, |ctx_| {
              let decl = decl_name(ctx_, cursor);
              let ci = decl.compinfo();
              cursor.visit(|c, _| {
                  let mut ci_ = ci.borrow_mut();
                  visit_union(c, ctx_, &mut ci_.fields)
              });
              ctx_.globals.push(GComp(ci));
          });
          return CXChildVisit_Recurse;
      }
      CXCursor_EnumDecl => {
          fwd_decl(ctx, cursor, |ctx_| {
              let decl = decl_name(ctx_, cursor);
              let ei = decl.enuminfo();
              cursor.visit(|c, _| {
                  let mut ei_ = ei.borrow_mut();
                  visit_enum(c, &mut ei_.items)
              });
              ctx_.globals.push(GEnum(ei));
          });
          return CXChildVisit_Continue;
      }
      CXCursor_FunctionDecl => {
          let linkage = cursor.linkage();
          if linkage != CXLinkage_External && linkage != CXLinkage_UniqueExternal {
              return CXChildVisit_Continue;
          }

          let args_lst: Vec<(String, il::Type)> = cursor.args().iter().map(|arg| {
              let arg_name = arg.spelling();
              (arg_name, conv_ty(ctx, &arg.cur_type(), cursor))
          }).collect();

          let ty = cursor.cur_type();
          let ret_ty = box conv_ty(ctx, &cursor.ret_type(), cursor);

          let func = decl_name(ctx, cursor);
          let vi = func.varinfo();
          let mut vi = vi.borrow_mut();
          vi.ty = TFunc(ret_ty.clone(), args_lst.clone(), ty.is_variadic());
          ctx.globals.push(func);

          return CXChildVisit_Continue;
      }
      CXCursor_VarDecl => {
          let linkage = cursor.linkage();
          if linkage != CXLinkage_External && linkage != CXLinkage_UniqueExternal {
              return CXChildVisit_Continue;
          }

          let ty = conv_ty(ctx, &cursor.cur_type(), cursor);
          let var = decl_name(ctx, cursor);
          let vi = var.varinfo();
          let mut vi = vi.borrow_mut();
          vi.ty = ty.clone();
          vi.is_const = cursor.cur_type().is_const();
          ctx.globals.push(var);

          return CXChildVisit_Continue;
      }
      CXCursor_TypedefDecl => {
          let mut under_ty = cursor.typedef_type();
          if under_ty.kind() == CXType_Unexposed {
              under_ty = under_ty.canonical_type();
          }

          let ty = conv_ty(ctx, &under_ty, cursor);
          let typedef = decl_name(ctx, cursor);
          let ti = typedef.typeinfo();
          let mut ti = ti.borrow_mut();
          ti.ty = ty.clone();
          ctx.globals.push(typedef);

          opaque_ty(ctx, &under_ty);

          return CXChildVisit_Continue;
      }
      CXCursor_FieldDecl => {
          return CXChildVisit_Continue;
      }
      _ => return CXChildVisit_Recurse,
    }
}

fn log_err_warn(ctx: &mut ClangParserCtx, msg: &str, is_err: bool) {
    match is_err {
        true => {
            ctx.err_count += 1;
            ctx.logger.error(msg)
        },
        false => ctx.logger.warn(msg)
    }
}

pub fn parse(options: ClangParserOptions, logger: &Logger) -> Result<Vec<Global>, ()> {
    let mut ctx = ClangParserCtx {
        options: options,
        name: HashMap::new(),
        builtin_defs: vec!(),
        globals: vec!(),
        logger: logger,
        err_count: 0
    };

    let ix = cx::Index::create(false, true);
    if ix.is_null() {
        ctx.logger.error("Clang failed to create index");
        return Err(())
    }

    let unit = TranslationUnit::parse(&ix, "", ctx.options.clang_args.as_slice(), [], 0);
    if unit.is_null() {
        ctx.logger.error("No input files given");
        return Err(())
    }

    let diags = unit.diags();
    for d in diags.iter() {
        let msg = d.format(Diagnostic::default_opts());
        let is_err = d.severity() >= CXDiagnostic_Error;
        log_err_warn(&mut ctx, msg.as_slice(), is_err);
    }

    if ctx.err_count > 0 {
        return Err(())
    }

    let cursor = unit.cursor();

    if ctx.options.emit_ast {
        cursor.visit(|cur, _| ast_dump(cur, 0));
    }     

    cursor.visit(|cur, parent| visit_top(cur, parent, &mut ctx));

    while !ctx.builtin_defs.is_empty() {
        let c = ctx.builtin_defs.shift().unwrap();
        c.visit(|cur, parent| visit_top(cur, parent, &mut ctx));
    }

    unit.dispose();
    ix.dispose();

    if ctx.err_count > 0 {
        return Err(())
    }

    Ok(ctx.globals)
}
