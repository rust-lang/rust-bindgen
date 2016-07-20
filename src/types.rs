use std::cell::Cell;
use hacks::refcell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};

use syntax::abi;

pub use self::Global::*;
pub use self::Type::*;
pub use self::IKind::*;
pub use self::FKind::*;
use clang::Cursor;

use parser::{Annotations, Accessor};

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
    /// Types that must be substituted in this module,
    /// in the form original_name -> substituted_type
    pub translations: HashMap<String, Global>,
}

impl Module {
    pub fn new(name: String, parent_id: Option<ModuleId>) -> Self {
        Module {
            name: name,
            globals: vec![],
            parent_id: parent_id,
            children_ids: vec![],
            translations: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn add_global(&mut self, g: Global) {
        self.globals.push(g)
    }
}

#[derive(Clone, PartialEq)]
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
    // XXX prevent this dumb to_owned()... didn't want to deal with the borrowed lifetime
    pub fn name(&self) -> String {
        match *self {
            GType(ref info) => info.borrow().name.to_owned(),
            GComp(ref info)
            | GCompDecl(ref info) => info.borrow().name.to_owned(),
            GEnum(ref info)
            | GEnumDecl(ref info) => info.borrow().name.to_owned(),
            GVar(ref info)
            | GFunc(ref info) => info.borrow().name.to_owned(),
            GOther => "".to_owned(),
        }
    }

    pub fn layout(&self) -> Option<Layout> {
        Some(match *self {
            GType(ref info) => info.borrow().layout,
            GComp(ref info)
            | GCompDecl(ref info) => info.borrow().layout,
            GEnum(ref info)
            | GEnumDecl(ref info) => info.borrow().layout,
            GVar(_)
            | GFunc(_)
            | GOther => return None,
        })
    }

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

    pub fn to_type(self) -> Type {
        match self {
            GType(ti) => TNamed(ti),
            GComp(ci)
            | GCompDecl(ci) => TComp(ci),
            GEnum(ei)
            | GEnumDecl(ei) => TEnum(ei),
            GVar(_)
            | GFunc(_)
            | GOther => TVoid,
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

#[derive(Debug, Clone, PartialEq)]
pub struct FuncSig {
    pub ret_ty: Box<Type>,
    pub args: Vec<(String, Type)>,
    pub is_variadic: bool,
    pub is_safe: bool,
    pub abi: abi::Abi,
}

#[derive(Clone, PartialEq, Debug)]
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
    #[allow(dead_code)]
    pub fn name(&self) -> Option<String> {
        match *self {
            TNamed(ref info) => Some(info.borrow().name.clone()),
            TComp(ref info)  => Some(info.borrow().name.clone()),
            TEnum(ref info)  => Some(info.borrow().name.clone()),
            TArray(ref t, _, _) => t.name(),
            TPtr(ref t, _, _, _) => t.name(),
            _ => None
        }
    }

    pub fn signature_contains_type(&self, other: &Type) -> bool {
        self == other || match *self {
            TPtr(ref t, _, _, _) => t.signature_contains_type(other),
            TArray(ref t, _, _) => t.signature_contains_type(other),
            TComp(ref info) => info.borrow().signature_contains_type(other),
            _ => false,
        }
    }

    // XXX Add this info to enums?
    pub fn was_unnamed(&self) -> bool {
        match *self {
            TComp(ref ci) => ci.borrow().was_unnamed,
            TArray(ref t, _, _) => t.was_unnamed(),
            TPtr(ref t, _, _, _) => t.was_unnamed(),
            _ => false,
        }
    }

    pub fn get_outermost_composite(&self) -> Option<Rc<RefCell<CompInfo>>> {
        match *self {
            TComp(ref ci) => Some(ci.clone()),
            TArray(ref t, _, _) => t.get_outermost_composite(),
            TPtr(ref t, _, _, _) => t.get_outermost_composite(),
            _ => None,
        }
    }

    pub fn size(&self) -> usize {
        self.layout().map(|l| l.size).unwrap_or(0)
    }

    #[allow(dead_code)]
    pub fn align(&self) -> usize {
        self.layout().map(|l| l.align).unwrap_or(0)
    }

    pub fn layout(&self) -> Option<Layout> {
        Some(match *self {
            TInt(_, l) => l.clone(),
            TFloat(_, l) => l.clone(),
            TPtr(_, _, _, l) => l.clone(),
            TArray(_, _, l) => l.clone(),
            TComp(ref ci) => ci.borrow().layout.clone(),
            TEnum(ref ei) => ei.borrow().layout.clone(),
            // Test first with the underlying type layout, else with the reported one
            // This fixes a weird bug in SM when it can't find layout for uint32_t
            TNamed(ref ti) => ti.borrow().ty.layout().unwrap_or(ti.borrow().layout.clone()),
            TVoid |
            TFuncProto(..) |
            TFuncPtr(..) => return None,
        })
    }

    pub fn can_derive_debug(&self) -> bool {
        !self.is_opaque() && match *self {
            TArray(ref t, size, _) => size <= 32 && t.can_derive_debug(),
            TNamed(ref ti) => ti.borrow().ty.can_derive_debug(),
            TComp(ref comp) => comp.borrow().can_derive_debug(),
            _ => true,
        }
    }

    // For some reason, deriving copies of an array of a type that is not known to be copy
    // is a compile error. e.g.:
    //
    // #[derive(Copy)]
    // struct A<T> {
    //     member: T,
    // }
    //
    // is fine, while:
    //
    // #[derive(Copy)]
    // struct A<T> {
    //     member: [T; 1],
    // }
    //
    // is an error.
    //
    // That's the point of the existance of can_derive_copy_in_array().
    pub fn can_derive_copy_in_array(&self) -> bool {
        match *self {
            TVoid => false,
            TNamed(ref ti) => ti.borrow().ty.can_derive_copy_in_array(),
            TArray(ref t, _, _) => t.can_derive_copy_in_array(),
            ref t => t.can_derive_copy(),
        }
    }

    pub fn can_derive_copy(&self) -> bool {
        !self.is_opaque() && match *self {
            TArray(ref t, _, _) => t.can_derive_copy_in_array(),
            TNamed(ref ti) => ti.borrow().ty.can_derive_copy(),
            TComp(ref comp) => comp.borrow().can_derive_copy(),
            _ => true,
        }
    }

    pub fn is_opaque(&self) -> bool {
        match *self {
            TArray(ref t, _, _) => t.is_opaque(),
            TPtr(ref t, _, _, _) => t.is_opaque(),
            TNamed(ref ti) => ti.borrow().opaque || ti.borrow().ty.is_opaque(),
            TComp(ref ci) => ci.borrow().is_opaque(),
            _ => false,
        }
    }

    #[allow(dead_code)]
    pub fn is_union_like(&self) -> bool {
        match *self {
            TArray(ref t, _, _) => t.is_union_like(),
            TPtr(ref t, _, _, _) => t.is_union_like(),
            TNamed(ref ti) => ti.borrow().ty.is_union_like(),
            TComp(ref ci) => ci.borrow().kind == CompKind::Union,
            _ => false,
        }
    }

    // If a type is opaque we conservatively
    // assume it has destructor
    pub fn has_destructor(&self) -> bool {
        self.is_opaque() || match *self {
            TArray(ref t, _, _) => t.has_destructor(),
            TNamed(ref ti) => ti.borrow().ty.has_destructor(),
            TComp(ref ci) => ci.borrow().has_destructor(),
            _ => false,
        }
    }

    pub fn is_translatable(&self) -> bool {
        match *self {
            TVoid => false,
            TArray(ref t, _, _) => t.is_translatable(),
            TComp(ref ci) => ci.borrow().is_translatable(),
            // NB: TNamed explicitely ommited here
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

#[derive(Debug, Copy, Clone, PartialEq)]
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FKind {
    FFloat,
    FDouble
}

#[derive(Clone, PartialEq, Debug)]
pub enum CompMember {
    Field(FieldInfo),
    Comp(Rc<RefCell<CompInfo>>),
    #[allow(dead_code)]
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
    pub has_nonempty_base: bool,
    pub hide: bool,
    pub parser_cursor: Option<Cursor>,
    /// If this struct should be replaced by an opaque blob.
    ///
    /// This is useful if for some reason we can't generate
    /// the correct layout.
    pub opaque: bool,
    pub base_members: usize,
    pub layout: Layout,
    /// If this struct is explicitely marked as non-copiable.
    pub no_copy: bool,
    /// Typedef'd types names, that we'll resolve early to avoid name conflicts
    pub typedefs: Vec<String>,
    /// If this type has a template parameter which is not a type (e.g.: a size_t)
    pub has_non_type_template_params: bool,
    /// If this type was unnamed when parsed
    pub was_unnamed: bool,
    /// Set of static vars declared inside this class.
    pub vars: Vec<Global>,
    /// Used to detect if we've run in a can_derive_debug cycle while cycling
    /// around the template arguments.
    detect_derive_debug_cycle: Cell<bool>,
    /// Used to detect if we've run in a has_destructor cycle while cycling
    /// around the template arguments.
    detect_has_destructor_cycle: Cell<bool>,

    /// Annotations on the decl
    pub anno: Annotations,
}

static mut UNNAMED_COUNTER: u32 = 0;

fn unnamed_name(name: String, filename: &String) -> String {
    if name.is_empty() {
        let n = unsafe { UNNAMED_COUNTER += 1; UNNAMED_COUNTER };
        format!("{}_unnamed_{}", filename, n)
    } else {
        name
    }
}

impl CompInfo {
    pub fn new(name: String,
               module_id: ModuleId,
               filename: String,
               comment: String,
               kind: CompKind,
               members: Vec<CompMember>,
               layout: Layout,
               anno: Annotations) -> CompInfo {
        let was_unnamed = name.is_empty();
        CompInfo {
            kind: kind,
            module_id: module_id,
            name: unnamed_name(name, &filename),
            filename: filename,
            comment: comment,
            members: members,
            args: vec![],
            methods: vec![],
            vmethods: vec![],
            ref_template: None,
            has_vtable: false,
            has_destructor: false,
            has_nonempty_base: false,
            hide: false,
            parser_cursor: None,
            opaque: false,
            no_copy: false,
            base_members: 0,
            layout: layout,
            typedefs: vec![],
            vars: vec![],
            has_non_type_template_params: false,
            was_unnamed: was_unnamed,
            detect_derive_debug_cycle: Cell::new(false),
            detect_has_destructor_cycle: Cell::new(false),
            anno: anno,
        }
    }

    // Return the module id or the class declaration module id.
    pub fn module_id(&self) -> ModuleId {
        self.ref_template.as_ref().and_then(|t| if let TComp(ref ci) = *t {
            Some(ci.borrow().module_id)
        } else {
            None
        }).unwrap_or(self.module_id)
    }

    pub fn can_derive_debug(&self) -> bool {
        if self.hide || self.is_opaque() {
            return false;
        }

        if self.detect_derive_debug_cycle.get() {
            println!("Derive debug cycle detected: {}!", self.name);
            return true;
        }

        match self.kind {
            CompKind::Union => {
                let size_divisor = if self.layout.align == 0 { 1 } else { self.layout.align };
                if self.layout.size / size_divisor > 32 {
                    return false;
                }

                true
            }
            CompKind::Struct => {
                self.detect_derive_debug_cycle.set(true);

                let can_derive_debug = self.args.iter().all(|ty| ty.can_derive_debug()) &&
                    self.members.iter()
                        .all(|member| match *member {
                            CompMember::Field(ref f) |
                            CompMember::CompField(_, ref f) => f.ty.can_derive_debug(),
                            _ => true,
                        });
                self.detect_derive_debug_cycle.set(false);

                can_derive_debug
            }
        }
    }

    pub fn is_opaque(&self) -> bool {
        if let Some(ref template) = self.ref_template {
            if template.is_opaque() {
                return true;
            }
        }
        self.opaque
    }

    pub fn has_destructor(&self) -> bool {
        if self.detect_has_destructor_cycle.get() {
            warn!("Cycle detected looking for destructors: {}!", self.name);
            // Assume no destructor, since we don't have an explicit one.
            return false;
        }

        self.detect_has_destructor_cycle.set(true);

        let has_destructor = self.has_destructor || match self.kind {
            CompKind::Union => false,
            CompKind::Struct => {
                // NB: We can't rely on a type with type parameters
                // not having destructor.
                //
                // This is unfortunate, but...
                self.ref_template.as_ref().map_or(false, |t| t.has_destructor()) ||
                self.args.iter().any(|t| t.has_destructor()) ||
                self.members.iter().enumerate().any(|(index, m)| match *m {
                    CompMember::Field(ref f) |
                    CompMember::CompField(_, ref f) => {
                        // Base members may not be resolved yet
                        if index < self.base_members {
                            f.ty.has_destructor()
                        } else {
                            f.ty.has_destructor() || !f.ty.is_translatable()
                        }
                    },
                    _ => false,
                })
            }
        };

        self.detect_has_destructor_cycle.set(false);

        has_destructor
    }

    pub fn can_derive_copy(&self) -> bool {
        if self.no_copy {
            return false;
        }
        match self.kind {
            CompKind::Union => true,
            CompKind::Struct => {
                if self.has_destructor() {
                    return false;
                }

                // With template args, use a safe subset of the types,
                // since copyability depends on the types itself.
                self.ref_template.as_ref().map_or(true, |t| t.can_derive_copy()) &&
                self.members.iter().all(|m| match *m {
                    CompMember::Field(ref f) |
                    CompMember::CompField(_, ref f) => f.ty.can_derive_copy(),
                    _ => true,
                })
            }
        }
    }

    pub fn is_translatable(&self) -> bool {
        match self.kind {
            CompKind::Union => true,
            CompKind::Struct => {
                self.args.iter().all(|t| t != &TVoid) && !self.has_non_type_template_params
            }
        }
    }

    pub fn signature_contains_type(&self, other: &Type) -> bool {
        self.args.iter().any(|t| t.signature_contains_type(other))
    }
}

impl fmt::Debug for CompInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CompInfo({}, ref: {:?}, args: {:?}, members: {:?}", self.name, self.ref_template, self.args, self.members)
    }
}

#[derive(Clone, PartialEq)]
pub struct FieldInfo {
    pub name: String,
    pub ty: Type,
    pub comment: String,
    pub bitfields: Option<Vec<(String, u32)>>,
    /// If the C++ field is marked as `mutable`
    pub mutable: bool,
    /// True when field or enclosing struct
    /// has a `<div rust-bindgen private>` annotation
    pub private: bool,
    /// Set by the `<div rust-bindgen accessor="..">`
    /// annotation on a field or enclosing struct
    pub accessor: Accessor,
}

impl FieldInfo {
    pub fn new(name: String,
               ty: Type,
               comment: String,
               bitfields: Option<Vec<(String, u32)>>,
               mutable: bool) -> FieldInfo {
        FieldInfo {
            name: name,
            ty: ty,
            comment: comment,
            bitfields: bitfields,
            mutable: mutable,
            private: false,
            accessor: Accessor::None,
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
    // TODO: Is this really useful?
    // You can just make opaque the underlying type
    pub opaque: bool,
}

impl TypeInfo {
    pub fn new(name: String, module_id: ModuleId, ty: Type, layout: Layout) -> TypeInfo {
        TypeInfo {
            name: name,
            module_id: module_id,
            comment: String::new(),
            ty: ty,
            layout: layout,
            opaque: false,
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
