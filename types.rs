pub enum Global {
    GType(@mut TypeInfo),
    GComp(@mut CompInfo),
    GCompDecl(@mut CompInfo),
    GEnum(@mut EnumInfo),
    GEnumDecl(@mut EnumInfo),
    GVar(@mut VarInfo),
    GFunc(@mut VarInfo),
    GOther
}

pub enum Type {
    TVoid,
    TInt(IKind),
    TFloat(FKind),
    TPtr(@Type, bool),
    TArray(@Type, uint),
    TFunc(@Type, ~[(~str, @Type)], bool),
    TNamed(@mut TypeInfo),
    TComp(@mut CompInfo),
    TEnum(@mut EnumInfo)
}

pub enum IKind {
    IBool,
    ISChar,
    IUChar,
    IShort,
    IUShort,
    IInt,
    IUInt,
    ILong,
    IULong,
    ILongLong,
    IULongLong
}

pub enum FKind {
    FFloat,
    FDouble
}

pub struct CompInfo {
    cstruct: bool,
    name: ~str,
    fields: ~[@FieldInfo]
}

pub struct FieldInfo {
    name: ~str,
    ty: @Type,
    bit: Option<uint>
}

pub struct EnumInfo {
    name: ~str,
    items: ~[@EnumItem],
    kind: IKind
}

pub struct EnumItem {
    name: ~str,
    val: int
}

pub struct TypeInfo {
    name: ~str,
    ty: @Type
}

pub struct VarInfo {
    name: ~str,
    ty: @Type
}

impl ToStr for Global {
    fn to_str(&self) -> ~str {
      match *self {
        GType(ti) => ti.to_str(),
        GComp(ci) => ci.to_str(),
        GCompDecl(ci) => ci.to_str(),
        GEnum(ei) => ei.to_str(),
        GEnumDecl(ei) => ei.to_str(),
        GVar(vi) => vi.to_str(),
        GFunc(vi) => vi.to_str(),
        GOther => ~"*"
      }
    }
}

impl ToStr for CompInfo {
    fn to_str(&self) -> ~str {
        copy self.name
    }
}

impl ToStr for EnumInfo {
    fn to_str(&self) -> ~str {
        copy self.name
    }
}

impl ToStr for TypeInfo {
    fn to_str(&self) -> ~str {
        copy self.name
    }
}

impl ToStr for VarInfo {
    fn to_str(&self) -> ~str {
        copy self.name
    }
}

pub fn mk_compinfo(name: ~str, cstruct: bool, fields: ~[@FieldInfo]) -> @mut CompInfo {
    return @mut CompInfo { cstruct: cstruct,
                           name: name,
                           fields: fields
                         };
}

pub fn mk_fieldinfo(name: ~str, ty: @Type, bit: Option<uint>) -> @FieldInfo {
    return @FieldInfo { name: name,
                        ty: ty,
                        bit: bit
                      };
}

pub fn mk_enuminfo(name: ~str, kind: IKind, items: ~[@EnumItem]) -> @mut EnumInfo {
    return @mut EnumInfo { name: name,
                           items: items,
                           kind: kind
                         };
}

pub fn mk_enumitem(name: ~str, val: int) -> @EnumItem {
    return @EnumItem { name: name,
                       val: val
                     };
}

pub fn mk_typeinfo(name: ~str, ty: @Type) -> @mut TypeInfo {
    return @mut TypeInfo { name: name,
                           ty: ty
                         };
}

pub fn mk_varinfo(name: ~str, ty: @Type) -> @mut VarInfo {
    return @mut VarInfo { name: name,
                          ty: ty
                        };
}

pub fn global_compinfo(glob: Global) -> @mut CompInfo {
    match glob {
        GComp(i) => return i,
        GCompDecl(i) => return i,
        _ => fail!(~"global_compinfo")
    }
}

pub fn global_enuminfo(glob: Global) -> @mut EnumInfo {
    match glob {
        GEnum(i) => return i,
        GEnumDecl(i) => return i,
        _ => fail!(~"global_enuminfo")
    }
}

pub fn global_typeinfo(glob: Global) -> @mut TypeInfo {
    match glob {
        GType(i) => return i,
        _ => fail!(~"global_typeinfo")
    }
}

pub fn global_varinfo(glob: Global) -> @mut VarInfo {
    match glob {
        GVar(i) => i,
        GFunc(i) => i,
        _ => fail!(~"global_varinfo")
    }
}

#[cfg(target_arch="x86_64")]
pub fn type_align(ty: @Type) -> uint {
    return match *ty {
        TInt(k) => match k {
            IBool | ISChar | IUChar => 1,
            IShort | IUShort => 2,
            IInt | IUInt => 4,
            ILong | IULong => 8,
            ILongLong | IULongLong => 8
        },
        TFloat(k) => match k {
            FFloat => 4,
            FDouble => 8
        },
        TPtr(*) => 8,
        TArray(t, _) => type_align(t),
        TNamed(t) => type_align(t.ty),
        TComp(ci) => {
            let fs = copy ci.fields;
            do fs.foldl(0) |a, t| {
                uint::max(*a, type_align(t.ty))
            }
        },
        TEnum(_) => 4,
        TVoid => 0,
        TFunc(_, _, _) => 0
    };
}

#[cfg(target_arch="x86")]
pub fn type_align(ty: @Type) -> uint {
    return match *ty {
        TInt(k) => match k {
            IBool | ISChar | IUChar => 1,
            IShort | IUShort => 2,
            IInt | IUInt => 4,
            ILong | IULong => 4,
            ILongLong | IULongLong => 8
        },
        TFloat(k) => match k {
            FFloat => 4,
            FDouble => 8
        },
        TPtr(*) => 4,
        TArray(t, _) => type_align(t),
        TNamed(t) => type_align(t.ty),
        TComp(ci) => {
            let fs = copy ci.fields;
            do fs.foldl(0) |a, t| {
                uint::max(*a, type_align(t.ty))
            }
        },
        TEnum(_) => 4,
        TVoid => 0,
        TFunc(_, _, _) => 0
    };
}

#[cfg(target_arch="x86_64")]
pub fn type_size(ty: @Type) -> uint {
    return match *ty {
        TInt(k) => match k {
            IBool | ISChar | IUChar => 1,
            IShort | IUShort => 2,
            IInt | IUInt => 4,
            ILong | IULong => 8,
            ILongLong | IULongLong => 8
        },
        TFloat(k) => match k {
            FFloat => 4,
            FDouble => 8
        },
        TPtr(*) => 8,
        TArray(t, s) => type_size(t) * s,
        TNamed(t) => type_size(t.ty),
        TComp(ci) => if ci.cstruct {
            let fs = copy ci.fields;
            let size = do fs.foldl(0) |s, t| {
                align(*s, t.ty) + type_size(t.ty)
            };
            align(size, ty)
        } else {
            let fs = copy ci.fields;
            let size = do fs.foldl(0) |s, t| {
                uint::max(*s, type_size(t.ty))
            };
            align(size, ty)
        },
        TEnum(_) => 4,
        TVoid => 0,
        TFunc(_, _, _) => 0
    };
}

#[cfg(target_arch="x86")]
pub fn type_size(ty: @Type) -> uint {
    return match *ty {
        TInt(k) => match k {
            IBool | ISChar | IUChar => 1,
            IShort | IUShort => 2,
            IInt | IUInt => 4,
            ILong | IULong => 4,
            ILongLong | IULongLong => 8
        },
        TFloat(k) => match k {
            FFloat => 4,
            FDouble => 8
        },
        TPtr(*) => 4,
        TArray(t, s) => type_size(t) * s,
        TNamed(t) => type_size(t.ty),
        TComp(ci) => if ci.cstruct {
            let fs = copy ci.fields;
            let size = do fs.foldl(0) |s, t| {
                align(*s, t.ty) + type_size(t.ty)
            };
            align(size, ty)
        } else {
            let fs = copy ci.fields;
            let size = do fs.foldl(0) |s, t| {
                uint::max(*s, type_size(t.ty))
            };
            align(size, ty)
        },
        TEnum(_) => 4,
        TVoid => 0,
        TFunc(_, _, _) => 0
    };
}

pub fn align(off: uint, ty: @Type) -> uint {
    let a = type_align(ty);
    if a == 0 {
        return 0;
    } else {
        return (off + a - 1u) / a * a;
    }
}
