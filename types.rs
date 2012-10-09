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
    IInt,
    IUInt,
    IShort,
    IUShort,
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
    mut ty: @Type
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
                        ty: ty
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
        _ => fail ~"global_compinfo"
    }
}

pure fn global_enuminfo(glob: Global) -> @EnumInfo {
    match glob {
        GEnum(i) => return i,
        GEnumDecl(i) => return i,
        _ => fail ~"global_enuminfo"
    }
}

pure fn global_typeinfo(glob: Global) -> @TypeInfo {
    match glob {
        GType(i) => return i,
        _ => fail ~"global_typeinfo"
    }
}

pure fn global_varinfo(glob: Global) -> @VarInfo {
    match glob {
        GVar(i) => i,
        GFunc(i) => i,
        _ => fail ~"global_varinfo"
    }
}
