enum Global {
    GType(@TypeInfo),
    GComp(@CompInfo),
    GCompDecl(@CompInfo),
    GEnum(@EnumInfo),
    GEnumDecl(@EnumInfo),
    GVar(@VarInfo),
    GFunc(@VarInfo),
    GOther
}

enum Type {
    TVoid,
    TInt(IKind),
    TFloat(FKind),
    TPtr(@Type),
    TArray(@Type, uint),
    TFunc(@Type, ~[(~str, @Type)], bool),
    TNamed(@TypeInfo),
    TComp(@CompInfo),
    TEnum(@EnumInfo)
}

enum IKind {
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

enum FKind {
    FFloat,
    FDouble
}

struct CompInfo {
    mut cstruct: bool,
    mut name: ~str,
    mut fields: ~[@FieldInfo]
}

struct FieldInfo {
    mut comp: @CompInfo,
    mut name: ~str,
    mut ty: @Type,
    mut bit: Option<uint>
}

struct EnumInfo {
    mut name: ~str,
    mut items: ~[@EnumItem],
    mut kind: IKind
}

struct EnumItem {
    mut host: @EnumInfo,
    mut name: ~str,
    mut val: int
}

struct TypeInfo {
    mut name: ~str,
    mut ty: @Type
}

struct VarInfo {
    mut name: ~str,
    mut ty: @Type
}

pure fn mk_compinfo(name: ~str, cstruct: bool) -> @CompInfo {
    return @CompInfo { cstruct: cstruct,
                       name: name,
                       fields: ~[]
                     };
}

pure fn mk_fieldinfo(name: ~str, ty: @Type, comp: @CompInfo) -> @FieldInfo {
    return @FieldInfo { comp: comp,
                        name: name,
                        ty: ty,
                        bit: None
                      };
}

pure fn mk_enuminfo(name: ~str, kind: IKind) -> @EnumInfo {
    return @EnumInfo { name: name,
                       items: ~[],
                       kind: kind
                     };
}

pure fn mk_enumitem(name: ~str, val: int, host: @EnumInfo) -> @EnumItem {
    return @EnumItem { host: host,
                       name: name,
                       val: val
                     };
}

pure fn mk_typeinfo(name: ~str, ty: @Type) -> @TypeInfo {
    return @TypeInfo { name: name,
                       ty: ty
                     };
}

pure fn mk_varinfo(name: ~str, ty: @Type) -> @VarInfo {
    return @VarInfo { name: name,
                      ty: ty
                    };
}

pure fn global_compinfo(glob: Global) -> @CompInfo {
    match glob {
        GComp(i) => return i,
        GCompDecl(i) => return i,
        _ => fail!(~"global_compinfo")
    }
}

pure fn global_enuminfo(glob: Global) -> @EnumInfo {
    match glob {
        GEnum(i) => return i,
        GEnumDecl(i) => return i,
        _ => fail!(~"global_enuminfo")
    }
}

pure fn global_typeinfo(glob: Global) -> @TypeInfo {
    match glob {
        GType(i) => return i,
        _ => fail!(~"global_typeinfo")
    }
}

pure fn global_varinfo(glob: Global) -> @VarInfo {
    match glob {
        GVar(i) => i,
        GFunc(i) => i,
        _ => fail!(~"global_varinfo")
    }
}

#[cfg(target_arch="x86_64")]
pure fn type_align(ty: @Type) -> uint {
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
        TPtr(_) => 8,
        TArray(t, _) => type_align(t),
        TNamed(t) => type_align(t.ty),
        TComp(ci) => {
            do vec::foldl(0, ci.fields) |a, t| {
                uint::max(a, type_align(t.ty))
            }
        },
        TEnum(_) => 4,
        _ => fail!(~"ty_align: unhandled type")
    };
}

#[cfg(target_arch="x86")]
pure fn type_align(ty: @Type) -> uint {
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
        TPtr(_) => 4,
        TArray(t, _) => type_align(t),
        TNamed(t) => type_align(t.ty),
        TComp(ci) => {
            do vec::foldl(0, ci.fields) |a, t| {
                uint::max(a, type_align(t.ty))
            }
        },
        TEnum(_) => 4,
        _ => fail!(~"ty_align: unhandled type")
    };
}

#[cfg(target_arch="x86_64")]
pure fn type_size(ty: @Type) -> uint {
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
        TPtr(_) => 8,
        TArray(t, s) => type_size(t) * s,
        TNamed(t) => type_size(t.ty),
        TComp(ci) => if ci.cstruct {
            let size = do vec::foldl(0, ci.fields) |s, t| {
                align(s, t.ty) + type_size(t.ty)
            };
            align(size, ty)
        } else {
            let size = do vec::foldl(0, ci.fields) |s, t| {
                uint::max(s, type_size(t.ty))
            };
            align(size, ty)
        },
        TEnum(_) => 4,
        _ => fail!(~"ty_size: unhandled type")
    };
}

#[cfg(target_arch="x86")]
pure fn type_size(ty: @Type) -> uint {
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
        TPtr(_) => 4,
        TArray(t, s) => type_size(t) * s,
        TNamed(t) => type_size(t.ty),
        TComp(ci) => if ci.cstruct {
            let size = do vec::foldl(0, ci.fields) |s, t| {
                align(s, t.ty) + type_size(t.ty)
            };
            align(size, ty)
        } else {
            let size = do vec::foldl(0, ci.fields) |s, t| {
                uint::max(s, type_size(t.ty))
            };
            align(size, ty)
        },
        TEnum(_) => 4,
        _ => fail!(~"ty_size: unhandled type")
    };
}

pure fn align(off: uint, ty: @Type) -> uint {
    let a = type_align(ty);
    return (off + a - 1u) / a * a;
}
