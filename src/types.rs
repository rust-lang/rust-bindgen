use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use syntax::abi;

pub use self::Global::*;
pub use self::Type::*;
pub use self::IKind::*;
pub use self::FKind::*;

/// A representation of a top level entity
#[derive(Clone)]
pub enum Global {
    /// A type definition, like `typedef int my_type;`.
    GType(Rc<RefCell<TypeInfo>>),
    /// A C composed type, like a struct or union.
    GComp(Rc<RefCell<CompInfo>>),
    GCompDecl(Rc<RefCell<CompInfo>>),
    /// A C enum.
    GEnum(Rc<RefCell<EnumInfo>>),
    GEnumDecl(Rc<RefCell<EnumInfo>>),
    /// A C global variable.
    GVar(Rc<RefCell<VarInfo>>),
    /// A function prototype, like `int func();`.
    GFunc(Rc<RefCell<VarInfo>>),
    /// Something else.
    GOther,
}

impl Global {
    pub fn compinfo(&self) -> Rc<RefCell<CompInfo>> {
        match *self {
            GComp(ref i) |
            GCompDecl(ref i) => i.clone(),
            _ => panic!("global_compinfo"),
        }
    }

    pub fn enuminfo(&self) -> Rc<RefCell<EnumInfo>> {
        match *self {
            GEnum(ref i) |
            GEnumDecl(ref i) => i.clone(),
            _ => panic!("global_enuminfo"),
        }
    }

    pub fn typeinfo(&self) -> Rc<RefCell<TypeInfo>> {
        match *self {
            GType(ref i) => i.clone(),
            _ => panic!("global_typeinfo"),
        }
    }

    pub fn varinfo(&self) -> Rc<RefCell<VarInfo>> {
        match *self {
            GVar(ref i) |
            GFunc(ref i) => i.clone(),
            _ => panic!("global_varinfo"),
        }
    }
}

impl fmt::Debug for Global {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GType(ref ti) => ti.borrow().fmt(f),
            GComp(ref ci) |
            GCompDecl(ref ci) => ci.borrow().fmt(f),
            GEnum(ref ei) |
            GEnumDecl(ref ei) => ei.borrow().fmt(f),
            GVar(ref vi) |
            GFunc(ref vi) => vi.borrow().fmt(f),
            GOther => "*".fmt(f),
        }
    }
}

/// A function signature.
#[derive(Clone, PartialEq, Debug)]
pub struct FuncSig {
    /// The return type of the function.
    pub ret_ty: Box<Type>,
    /// The arguments of the function.
    pub args: Vec<(String, Type)>,
    /// Is the function variadic?
    pub is_variadic: bool,
    /// Must it be called in an `unsafe` block?
    pub is_safe: bool,
    /// The ABI of the function
    pub abi: abi::Abi,
}

/// A representation of a C type.
#[derive(Clone, PartialEq, Debug)]
pub enum Type {
    /// The `void` C type.
    TVoid,
    /// A C integer number, like `int` or `char`.
    TInt(IKind, Layout),
    /// A C floating-point number, like `float` or `double`.
    TFloat(FKind, Layout),
    /// A Pointer, the boolean indicating if it is const.
    TPtr(Box<Type>, bool, Layout),
    TArray(Box<Type>, usize, Layout),
    TFuncProto(FuncSig, Layout),
    TFuncPtr(FuncSig, Layout),
    /// A typedef declaration?
    TNamed(Rc<RefCell<TypeInfo>>),
    /// A C composed type, like a struct or an union.
    TComp(Rc<RefCell<CompInfo>>),
    /// A C enum.
    TEnum(Rc<RefCell<EnumInfo>>),
}

impl Type {
    /// Returns the size in bytes of the type.
    pub fn size(&self) -> usize {
        match *self {
            TInt(_, l) |
            TFloat(_, l) |
            TFuncProto(_, l) |
            TFuncPtr(_, l) |
            TPtr(_, _, l) => l.size,
            TArray(_, size, l) => l.size * size,
            TNamed(ref ti) => ti.borrow().layout.size,
            TComp(ref ci) => ci.borrow().layout.size,
            TEnum(ref ei) => ei.borrow().layout.size,
            TVoid => 0,
        }
    }

    /// Returns the alignement of the type.
    #[allow(dead_code)]
    pub fn align(&self) -> usize {
        match *self {
            TInt(_, l) |
            TFloat(_, l) |
            TFuncProto(_, l) |
            TFuncPtr(_, l) |
            TPtr(_, _, l) |
            TArray(_, _, l) => l.align,
            TNamed(ref ti) => ti.borrow().layout.align,
            TComp(ref ci) => ci.borrow().layout.align,
            TEnum(ref ei) => ei.borrow().layout.align,
            TVoid => 0,
        }
    }

    pub fn layout(&self) -> Layout {
        Layout::new(self.size(), self.align())
    }

    /// Whether the type contains a field can't be derived
    pub fn can_auto_derive(&self) -> bool {
        match *self {
            TArray(_, size, _) => size <= 32,
            TComp(ref comp) => {
                comp.borrow()
                    .members
                    .iter()
                    .all(|member| {
                        match *member {
                            CompMember::Field(ref f) |
                            CompMember::CompField(_, ref f) => f.ty.can_auto_derive(),
                            _ => true,
                        }
                    })
            }
            _ => true,
        }
    }
}

/// Describes the layout of an element
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Layout {
    /// The size in bytes of the element.
    pub size: usize,
    pub align: usize,
    /// See `#[repr(C, Packed)]`.
    pub packed: bool,
}

impl Layout {
    pub fn new(size: usize, align: usize) -> Layout {
        Layout {
            size: size,
            align: align,
            packed: false,
        }
    }
}

impl Default for Layout {
    fn default() -> Layout {
        Layout {
            size: 0,
            align: 0,
            packed: false,
        }
    }
}

/// A representation of a C integer kind.
///
/// For example: bool -> `IBool`, unsigned long -> `IULong`.
#[derive(Copy, Clone, PartialEq, Debug)]
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
    IULongLong,
    IWChar,
}

impl IKind {
    pub fn is_signed(self) -> bool {
        match self {
            IBool | IUChar | IWChar | IUShort | IUInt | IULong | IULongLong => false,
            ISChar | IShort | IInt | ILong | ILongLong => true,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FKind {
    FFloat,
    FDouble,
}

#[derive(Clone, PartialEq, Debug)]
pub enum CompMember {
    Field(FieldInfo),
    Comp(Rc<RefCell<CompInfo>>),
    Enum(Rc<RefCell<EnumInfo>>),
    CompField(Rc<RefCell<CompInfo>>, FieldInfo),
    EnumField(Rc<RefCell<EnumInfo>>, FieldInfo),
}

impl CompMember {
    pub fn name(&self) -> String {
        match *self {
            CompMember::Comp(ref rc_c) => rc_c.borrow().name.clone(),
            CompMember::Enum(ref rc_e) => rc_e.borrow().name.clone(),
            CompMember::Field(ref f) |
            CompMember::EnumField(_, ref f) |
            CompMember::CompField(_, ref f) => f.name.clone(),
        }
    }

    pub fn layout(&self) -> Layout {
        match *self {
            CompMember::Comp(ref rc_c) => rc_c.borrow().layout,
            CompMember::Enum(ref rc_e) => rc_e.borrow().layout,
            CompMember::Field(ref f) |
            CompMember::EnumField(_, ref f) |
            CompMember::CompField(_, ref f) => f.ty.layout(),
        }
    }
}

/// Is the composed element a struct or an union?
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CompKind {
    Struct,
    Union,
}

/// Represents a composed element declaration, like a struct or an union.
#[derive(Clone, PartialEq)]
pub struct CompInfo {
    pub kind: CompKind,
    pub name: String,
    pub members: Vec<CompMember>,
    pub layout: Layout,
}

impl CompInfo {
    pub fn new(name: String, kind: CompKind, members: Vec<CompMember>, layout: Layout) -> CompInfo {
        CompInfo {
            kind: kind,
            name: name,
            members: members,
            layout: layout,
        }
    }
}

impl fmt::Debug for CompInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.name.fmt(f)
    }
}

/// A field in a struct or a union.
#[derive(Clone, PartialEq, Debug)]
pub struct FieldInfo {
    pub name: String,
    pub ty: Type,
    pub bitfields: Option<Vec<(String, u32)>>,
}

impl FieldInfo {
    pub fn new(name: String, ty: Type, bitfields: Option<Vec<(String, u32)>>) -> FieldInfo {
        FieldInfo {
            name: name,
            ty: ty,
            bitfields: bitfields,
        }
    }
}

/// A C enum.
#[derive(Clone, PartialEq)]
pub struct EnumInfo {
    pub name: String,
    pub items: Vec<EnumItem>,
    /// The underlining representation of the enum.
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

impl fmt::Debug for EnumInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.name.fmt(f)
    }
}

/// A variant in a C enum.
#[derive(Clone, PartialEq, Debug)]
pub struct EnumItem {
    pub name: String,
    pub val: i64,
}

impl EnumItem {
    pub fn new(name: String, val: i64) -> EnumItem {
        EnumItem {
            name: name,
            val: val,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct TypeInfo {
    pub name: String,
    pub ty: Type,
    pub layout: Layout,
}

impl TypeInfo {
    pub fn new(name: String, ty: Type, layout: Layout) -> TypeInfo {
        TypeInfo {
            name: name,
            ty: ty,
            layout: layout,
        }
    }
}

impl fmt::Debug for TypeInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {:?}", self.name, self.ty)
    }
}

/// A C variable declaration.
#[derive(Clone)]
pub struct VarInfo {
    pub name: String,
    pub ty: Type,
    // TODO: support non-integer constants
    pub val: Option<i64>,
    /// Is the variable constant?
    pub is_const: bool,
}

impl VarInfo {
    pub fn new(name: String, ty: Type) -> VarInfo {
        VarInfo {
            name: name,
            ty: ty,
            val: None,
            is_const: false,
        }
    }
}

impl fmt::Debug for VarInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.name.fmt(f)
    }
}
