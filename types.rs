use std::cell::RefCell;
use std::fmt;
use std::gc::Gc;

#[deriving(Clone)]
pub enum Global {
    GType(Gc<RefCell<TypeInfo>>),
    GComp(Gc<RefCell<CompInfo>>),
    GCompDecl(Gc<RefCell<CompInfo>>),
    GEnum(Gc<RefCell<EnumInfo>>),
    GEnumDecl(Gc<RefCell<EnumInfo>>),
    GVar(Gc<RefCell<VarInfo>>),
    GFunc(Gc<RefCell<VarInfo>>),
    GOther
}

impl Global {
    pub fn compinfo(&self) -> Gc<RefCell<CompInfo>> {
        match *self {
            GComp(i) => return i,
            GCompDecl(i) => return i,
            _ => fail!("global_compinfo".to_string())
        }
    }

    pub fn enuminfo(&self) -> Gc<RefCell<EnumInfo>> {
        match *self {
            GEnum(i) => return i,
            GEnumDecl(i) => return i,
            _ => fail!("global_enuminfo".to_string())
        }
    }

    pub fn typeinfo(&self) -> Gc<RefCell<TypeInfo>> {
        match *self {
            GType(i) => return i,
            _ => fail!("global_typeinfo".to_string())
        }
    }

    pub fn varinfo(&self) -> Gc<RefCell<VarInfo>> {
        match *self {
            GVar(i) => i,
            GFunc(i) => i,
            _ => fail!("global_varinfo".to_string())
        }
    }
}

impl fmt::Show for Global {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GType(ti) => ti.borrow().fmt(f),
            GComp(ci) => ci.borrow().fmt(f),
            GCompDecl(ci) => ci.borrow().fmt(f),
            GEnum(ei) => ei.borrow().fmt(f),
            GEnumDecl(ei) => ei.borrow().fmt(f),
            GVar(vi) => vi.borrow().fmt(f),
            GFunc(vi) => vi.borrow().fmt(f),
            GOther => "*".fmt(f),
        }
    }
}

#[deriving(Clone, PartialEq)]
pub enum Type {
    TVoid,
    TInt(IKind, Layout),
    TFloat(FKind, Layout),
    TPtr(Box<Type>, bool, Layout),
    TArray(Box<Type>, uint, Layout),
    TFunc(Box<Type>, Vec<(String, Type)>, bool),
    TNamed(Gc<RefCell<TypeInfo>>),
    TComp(Gc<RefCell<CompInfo>>),
    TEnum(Gc<RefCell<EnumInfo>>)
}

impl Type {
    pub fn size(&self) -> uint {
        match *self {
            TInt(_, l) => l.size,
            TFloat(_, l) => l.size,
            TPtr(_, _, l) => l.size,
            TArray(_, _, l) => l.size,
            TNamed(ref ti) => ti.borrow().ty.size(),
            TComp(ref ci) => ci.borrow().layout.size,
            TEnum(ref ei) => ei.borrow().layout.size,
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
            TNamed(ref ti) => ti.borrow().ty.align(),
            TComp(ref ci) => ci.borrow().layout.align,
            TEnum(ref ei) => ei.borrow().layout.align,
            TVoid => 0,
            TFunc(..) => 0,
        }
    }
}

#[deriving(Clone, PartialEq)]
pub struct Layout {
    pub size: uint,
    pub align: uint,
}

impl Layout {
    pub fn new(size: uint, align: uint) -> Layout {
        Layout { size: size, align: align }
    }

    pub fn zero() -> Layout {
        Layout { size: 0, align: 0 }
    }
}

#[deriving(Clone, PartialEq)]
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

#[deriving(Clone, PartialEq)]
pub enum FKind {
    FFloat,
    FDouble
}

#[deriving(Clone, PartialEq)]
pub struct CompInfo {
    pub cstruct: bool,
    pub name: String,
    pub fields: Vec<FieldInfo>,
    pub layout: Layout,
}

impl CompInfo {
    pub fn new(name: String, cstruct: bool, fields: Vec<FieldInfo>, layout: Layout) -> CompInfo {
        CompInfo {
            cstruct: cstruct,
            name: name,
            fields: fields,
            layout: layout,
        }
    }
}

impl fmt::Show for CompInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.name.fmt(f)
    }
}

#[deriving(Clone, PartialEq)]
pub struct FieldInfo {
    pub name: String,
    pub ty: Type,
    pub bit: Option<uint>,
}

impl FieldInfo {
    pub fn new(name: String, ty: Type, bit: Option<uint>) -> FieldInfo {
        FieldInfo {
            name: name,
            ty: ty,
            bit: bit,
        }
    }
}

#[deriving(Clone, PartialEq)]
pub struct EnumInfo {
    pub name: String,
    pub items: Vec<EnumItem>,
    pub kind: IKind,
    pub layout: Layout,
}

impl EnumInfo {
    pub fn new(name: String, kind: IKind, items: Vec<EnumItem>, layout: Layout) -> EnumInfo {
        EnumInfo {
            name: name,
            items: items,
            kind: kind,
            layout: layout,
        }
    }
}

impl fmt::Show for EnumInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.name.fmt(f)
    }
}

#[deriving(Clone, PartialEq)]
pub struct EnumItem {
    pub name: String,
    pub val: i64
}

impl EnumItem {
    pub fn new(name: String, val: i64) -> EnumItem {
        EnumItem {
            name: name,
            val: val
        }
    }
}

#[deriving(Clone, PartialEq)]
pub struct TypeInfo {
    pub name: String,
    pub ty: Type
}

impl TypeInfo {
    pub fn new(name: String, ty: Type) -> TypeInfo {
        TypeInfo {
            name: name,
            ty: ty
        }
    }
}

impl fmt::Show for TypeInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.name.fmt(f)
    }
}

#[deriving(Clone)]
pub struct VarInfo {
    pub name: String,
    pub ty: Type,
    pub is_const: bool
}

impl VarInfo {
    pub fn new(name: String, ty: Type) -> VarInfo {
        VarInfo {
            name: name,
            ty: ty,
            is_const: false
        }
    }
}

impl fmt::Show for VarInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.name.fmt(f)
    }
}
