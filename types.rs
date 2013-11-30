#[deriving(Clone)]
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

impl Global {
    pub fn compinfo(&self) -> @mut CompInfo {
        match *self {
            GComp(i) => return i,
            GCompDecl(i) => return i,
            _ => fail!(~"global_compinfo")
        }
    }

    pub fn enuminfo(&self) -> @mut EnumInfo {
        match *self {
            GEnum(i) => return i,
            GEnumDecl(i) => return i,
            _ => fail!(~"global_enuminfo")
        }
    }

    pub fn typeinfo(&self) -> @mut TypeInfo {
        match *self {
            GType(i) => return i,
            _ => fail!(~"global_typeinfo")
        }
    }

    pub fn varinfo(&self) -> @mut VarInfo {
        match *self {
            GVar(i) => i,
            GFunc(i) => i,
            _ => fail!(~"global_varinfo")
        }
    }
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

#[deriving(Clone, Eq)]
pub enum Type {
    TVoid,
    TInt(IKind, Layout),
    TFloat(FKind, Layout),
    TPtr(~Type, bool, Layout),
    TArray(~Type, uint, Layout),
    TFunc(~Type, ~[(~str, Type)], bool),
    TNamed(@mut TypeInfo),
    TComp(@mut CompInfo),
    TEnum(@mut EnumInfo)
}

impl Type {
    pub fn size(&self) -> uint {
        match *self {
          TInt(_, l) => l.size,
          TFloat(_, l) => l.size,
          TPtr(_, _, l) => l.size,
          TArray(_, _, l) => l.size,
          TNamed(ref t) => t.ty.size(),
          TComp(ref ci) => ci.layout.size,
          TEnum(ref ei) => ei.layout.size,
          TVoid => 0,
          TFunc(..) => 0,
        }
    }

    pub fn align(&self) -> uint {
        match *self {
          TInt(_, l) => l.align,
          TFloat(_, l) => l.align,
          TPtr(_, _, l) => l.align,
          TArray(_, _, l) => l.align,
          TNamed(ref t) => t.ty.align(),
          TComp(ref ci) => ci.layout.align,
          TEnum(ref ei) => ei.layout.align,
          TVoid => 0,
          TFunc(..) => 0,
        }
    }
}

#[deriving(Clone, Eq)]
pub struct Layout {
    size: uint,
    align: uint,
}

impl Layout {
    pub fn new(size: uint, align: uint) -> Layout {
        Layout { size: size, align: align }
    }

    pub fn zero() -> Layout {
        Layout { size: 0, align: 0 }
    }
}

#[deriving(Clone, Eq)]
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

#[deriving(Clone, Eq)]
pub enum FKind {
    FFloat,
    FDouble
}

#[deriving(Clone, Eq)]
pub struct CompInfo {
    cstruct: bool,
    name: ~str,
    fields: ~[FieldInfo],
    layout: Layout,
}

impl CompInfo {
    pub fn new(name: ~str, cstruct: bool, fields: ~[FieldInfo], layout: Layout) -> CompInfo {
        CompInfo {
            cstruct: cstruct,
            name: name,
            fields: fields,
            layout: layout,
        }
    }
}

impl ToStr for CompInfo {
    fn to_str(&self) -> ~str {
        self.name.clone()
    }
}

#[deriving(Clone, Eq)]
pub struct FieldInfo {
    name: ~str,
    ty: Type,
    bit: Option<uint>,
}

impl FieldInfo {
    pub fn new(name: ~str, ty: Type, bit: Option<uint>) -> FieldInfo {
        FieldInfo {
            name: name,
            ty: ty,
            bit: bit,
        }
    }
}

#[deriving(Clone, Eq)]
pub struct EnumInfo {
    name: ~str,
    items: ~[EnumItem],
    kind: IKind,
    layout: Layout,
}

impl EnumInfo {
    pub fn new(name: ~str, kind: IKind, items: ~[EnumItem], layout: Layout) -> EnumInfo {
        EnumInfo {
            name: name,
            items: items,
            kind: kind,
            layout: layout,
        }
    }
}

impl ToStr for EnumInfo {
    fn to_str(&self) -> ~str {
        self.name.clone()
    }
}

#[deriving(Clone, Eq)]
pub struct EnumItem {
    name: ~str,
    val: int
}

impl EnumItem {
    pub fn new(name: ~str, val: int) -> EnumItem {
        EnumItem {
            name: name,
            val: val
        }
    }
}

#[deriving(Clone, Eq)]
pub struct TypeInfo {
    name: ~str,
    ty: Type
}

impl TypeInfo {
    pub fn new(name: ~str, ty: Type) -> TypeInfo {
        TypeInfo {
            name: name,
            ty: ty
        }
    }
}

impl ToStr for TypeInfo {
    fn to_str(&self) -> ~str {
        self.name.clone()
    }
}

#[deriving(Clone)]
pub struct VarInfo {
    name: ~str,
    ty: Type,
    is_const: bool
}

impl VarInfo {
    pub fn new(name: ~str, ty: Type) -> VarInfo {
        VarInfo {
            name: name,
            ty: ty,
            is_const: false
        }
    }
}

impl ToStr for VarInfo {
    fn to_str(&self) -> ~str {
        self.name.clone()
    }
}
