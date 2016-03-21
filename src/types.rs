use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};

use syntax::abi;

pub use self::Global::*;
pub use self::Type::*;
pub use self::IKind::*;
pub use self::FKind::*;

static NEXT_MODULE_ID: AtomicUsize = ATOMIC_USIZE_INIT;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct ModuleId(usize);
pub static ROOT_MODULE_ID: ModuleId = ModuleId(0);

impl ModuleId {
    pub fn next() -> ModuleId {
        ModuleId(NEXT_MODULE_ID.fetch_add(1, Ordering::SeqCst) + 1)
    }
}

pub type ModuleMap = HashMap<ModuleId, Module>;

#[derive(Clone)]
pub struct Module {
    pub name: String,
    pub globals: Vec<Global>,
    pub parent_id: Option<ModuleId>,
    // Just for convenience
    pub children_ids: Vec<ModuleId>,
}

impl Module {
    pub fn new(name: String, parent_id: Option<ModuleId>) -> Self {
        Module {
            name: name,
            globals: vec![],
            parent_id: parent_id,
            children_ids: vec![],
        }
    }

    #[allow(dead_code)]
    pub fn add_global(&mut self, g: Global) {
        self.globals.push(g)
    }
}

#[derive(Clone)]
pub enum Global {
    GType(Rc<RefCell<TypeInfo>>),
    GComp(Rc<RefCell<CompInfo>>),
    GCompDecl(Rc<RefCell<CompInfo>>),
    GEnum(Rc<RefCell<EnumInfo>>),
    GEnumDecl(Rc<RefCell<EnumInfo>>),
    GVar(Rc<RefCell<VarInfo>>),
    GFunc(Rc<RefCell<VarInfo>>),
    GOther
}

impl Global {
    pub fn compinfo(&self) -> Rc<RefCell<CompInfo>> {
        match *self {
            GComp(ref i)
            | GCompDecl(ref i) => i.clone(),
            _ => panic!("global_compinfo")
        }
    }

    pub fn enuminfo(&self) -> Rc<RefCell<EnumInfo>> {
        match *self {
            GEnum(ref i)
            | GEnumDecl(ref i) => i.clone(),
            _ => panic!("global_enuminfo")
        }
    }

    pub fn typeinfo(&self) -> Rc<RefCell<TypeInfo>> {
        match *self {
            GType(ref i) => i.clone(),
            _ => panic!("global_typeinfo")
        }
    }

    pub fn varinfo(&self) -> Rc<RefCell<VarInfo>> {
        match *self {
            GVar(ref i)
            | GFunc(ref i) => i.clone(),
            _ => panic!("global_varinfo")
        }
    }
}

impl fmt::Debug for Global {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GType(ref ti) => ti.borrow().fmt(f),
            GComp(ref ci)
            | GCompDecl(ref ci) => ci.borrow().fmt(f),
            GEnum(ref ei)
            | GEnumDecl(ref ei) => ei.borrow().fmt(f),
            GVar(ref vi)
            | GFunc(ref vi) => vi.borrow().fmt(f),
            GOther => "*".fmt(f),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct FuncSig {
    pub ret_ty: Box<Type>,
    pub args: Vec<(String, Type)>,
    pub is_variadic: bool,
    pub is_safe: bool,
    pub abi: abi::Abi,
}

#[derive(Clone, PartialEq)]
pub enum Type {
    TVoid,
    TInt(IKind, Layout),
    TFloat(FKind, Layout),
    TPtr(Box<Type>, bool, bool, Layout),
    TArray(Box<Type>, usize, Layout),
    TFuncProto(FuncSig),
    TFuncPtr(FuncSig),
    TNamed(Rc<RefCell<TypeInfo>>),
    TComp(Rc<RefCell<CompInfo>>),
    TEnum(Rc<RefCell<EnumInfo>>)
}

impl Type {
    pub fn size(&self) -> usize {
        match *self {
            TInt(_, l) => l.size,
            TFloat(_, l) => l.size,
            TPtr(_, _, _, l) => l.size,
            TArray(_, _, l) => l.size,
            TNamed(ref ti) => ti.borrow().ty.size(),
            TComp(ref ci) => ci.borrow().layout.size,
            TEnum(ref ei) => ei.borrow().layout.size,
            TVoid => 0,
            TFuncProto(..) => 0,
            TFuncPtr(..) => 0,
        }
    }

    #[allow(dead_code)]
    pub fn align(&self) -> usize {
        match *self {
            TInt(_, l) => l.align,
            TFloat(_, l) => l.align,
            TPtr(_, _, _, l) => l.align,
            TArray(_, _, l) => l.align,
            TNamed(ref ti) => ti.borrow().ty.align(),
            TComp(ref ci) => ci.borrow().layout.align,
            TEnum(ref ei) => ei.borrow().layout.align,
            TVoid
            | TFuncProto(..)
            | TFuncPtr(..) => 0,
        }
    }

    #[allow(dead_code)]
    pub fn can_derive_debug(&self) -> bool {
        match *self {
            TArray(_, size, _) => size <= 32,
            TComp(ref comp) => {
                comp.borrow()
                    .members
                    .iter()
                    .all(|member| match *member {
                        CompMember::Field(ref f) |
                        CompMember::CompField(_, ref f) => f.ty.can_derive_debug(),
                        _ => true,
                    })
            }
            _ => true,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Layout {
    pub size: usize,
    pub align: usize,
    pub packed: bool,
}

impl Layout {
    pub fn new(size: usize, align: usize) -> Layout {
        Layout { size: size, align: align, packed: false }
    }

    pub fn zero() -> Layout {
        Layout { size: 0, align: 0, packed: false }
    }
}

#[derive(Copy, Clone, PartialEq)]
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

impl IKind {
    #[allow(dead_code)]
    pub fn is_signed(self) -> bool {
        match self {
            IBool => false,
            ISChar => true,
            IUChar => false,
            IShort => true,
            IUShort => false,
            IInt => true,
            IUInt => false,
            ILong => true,
            IULong => false,
            ILongLong => true,
            IULongLong => false,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum FKind {
    FFloat,
    FDouble
}

#[derive(Clone, PartialEq)]
pub enum CompMember {
    Field(FieldInfo),
    Comp(Rc<RefCell<CompInfo>>),
    CompField(Rc<RefCell<CompInfo>>, FieldInfo),
    Enum(Rc<RefCell<EnumInfo>>),
}

#[derive(Copy, Clone, PartialEq)]
pub enum CompKind {
    Struct,
    Union,
}

#[derive(Clone, PartialEq)]
pub struct CompInfo {
    pub kind: CompKind,
    pub name: String,
    pub module_id: ModuleId,
    pub filename: String,
    pub comment: String,
    pub members: Vec<CompMember>,
    pub args: Vec<Type>,
    pub methods: Vec<VarInfo>,
    pub vmethods: Vec<VarInfo>,
    pub ref_template: Option<Type>,
    pub has_vtable: bool,
    pub has_destructor: bool,
    pub hide: bool,
    pub base_members: usize,
    pub layout: Layout,
    /// Typedef'd types names, that we'll resolve early to avoid name conflicts
    pub typedefs: Vec<String>,
    /// If this type has a template parameter which is not a type (e.g.: a size_t)
    pub has_non_type_template_params: bool,
}

static mut UNNAMED_COUNTER: u32 = 0;

fn unnamed_name(name: String, filename: &String) -> String {
    return if name.is_empty() {
        let n = unsafe { UNNAMED_COUNTER += 1; UNNAMED_COUNTER };
        format!("{}_unnamed_{}", filename, n)
    } else {
        name
    };
}

impl CompInfo {
    pub fn new(name: String, module_id: ModuleId, filename: String, comment: String, kind: CompKind, members: Vec<CompMember>, layout: Layout) -> CompInfo {
        CompInfo {
            kind: kind,
            module_id: module_id,
            name: unnamed_name(name, &filename),
            filename: filename,
            comment: comment,
            members: members,
            args: vec!(),
            methods: vec!(),
            vmethods: vec!(),
            ref_template: None,
            has_vtable: false,
            has_destructor: false,
            hide: false,
            base_members: 0,
            layout: layout,
            typedefs: vec!(),
            has_non_type_template_params: false,
        }
    }
}

impl fmt::Debug for CompInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.name.fmt(f)
    }
}

#[derive(Clone, PartialEq)]
pub struct FieldInfo {
    pub name: String,
    pub ty: Type,
    pub comment: String,
    pub bitfields: Option<Vec<(String, u32)>>,
}

impl FieldInfo {
    pub fn new(name: String, ty: Type, comment: String, bitfields: Option<Vec<(String, u32)>>) -> FieldInfo {
        FieldInfo {
            name: name,
            ty: ty,
            comment: comment,
            bitfields: bitfields,
        }
    }
}

impl fmt::Debug for FieldInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.name.fmt(f)
    }
}

#[derive(Clone, PartialEq)]
pub struct EnumInfo {
    pub name: String,
    pub module_id: ModuleId,
    pub comment: String,
    pub filename: String,
    pub items: Vec<EnumItem>,
    pub kind: IKind,
    pub layout: Layout,
}

impl EnumInfo {
    pub fn new(name: String, module_id: ModuleId, filename: String, kind: IKind, items: Vec<EnumItem>, layout: Layout) -> EnumInfo {
        EnumInfo {
            name: unnamed_name(name, &filename),
            module_id: module_id,
            comment: String::new(),
            filename: filename,
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

#[derive(Clone, PartialEq)]
pub struct EnumItem {
    pub name: String,
    pub comment: String,
    pub val: i64
}

impl EnumItem {
    pub fn new(name: String, comment: String, val: i64) -> EnumItem {
        EnumItem {
            name: name,
            comment: comment,
            val: val
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct TypeInfo {
    pub name: String,
    pub module_id: ModuleId,
    pub comment: String,
    pub ty: Type,
    pub layout: Layout,
}

impl TypeInfo {
    pub fn new(name: String, module_id: ModuleId, ty: Type, layout: Layout) -> TypeInfo {
        TypeInfo {
            name: name,
            module_id: module_id,
            comment: String::new(),
            ty: ty,
            layout: layout,
        }
    }
}

impl fmt::Debug for TypeInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.name.fmt(f)
    }
}

#[derive(Clone, PartialEq)]
pub struct VarInfo {
    pub name: String,
    pub mangled: String,
    pub comment: String,
    pub ty: Type,
    //TODO: support non-integer constants
    pub val: Option<i64>,
    pub is_const: bool,
    pub is_static: bool,
}

impl VarInfo {
    pub fn new(name: String, mangled: String, comment: String, ty: Type) -> VarInfo {
        let mangled = if name == mangled {
            String::new()
        } else {
            mangled
        };
        VarInfo {
            name: name,
            mangled: mangled,
            comment: comment,
            ty: ty,
            val: None,
            is_const: false,
            is_static: false,
        }
    }
}

impl fmt::Debug for VarInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.name.fmt(f)
    }
}
