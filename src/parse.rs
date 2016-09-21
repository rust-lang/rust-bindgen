use clang;
use ir::ty::TypeKind;
use ir::item::ItemId;
use ir::context::BindgenContext;

#[derive(Debug)]
pub enum ParseError {
    Recurse,
    Continue,
}

#[derive(Debug)]
pub enum ParseResult<T> {
    AlreadyResolved(ItemId),
    New(T, Option<clang::Cursor>),
}

pub trait ClangSubItemParser : Sized {
    /// The fact that is a reference guarantees it's holded by the context, and
    /// allow returning already existing types.
    fn parse(cursor: clang::Cursor, context: &mut BindgenContext) -> Result<ParseResult<Self>, ParseError>;
}

pub trait ClangItemParser: Sized {
    fn parse(cursor: clang::Cursor,
             parent: Option<ItemId>,
             context: &mut BindgenContext) -> Result<ItemId, ParseError>;
    fn from_ty_or_ref(ty: clang::Type,
                      location: Option<clang::Cursor>,
                      parent_id: Option<ItemId>,
                      context: &mut BindgenContext) -> ItemId;
    fn from_ty_with_id(id: ItemId,
                       ty: &clang::Type,
                       location: Option<clang::Cursor>,
                       parent: Option<ItemId>,
                       ctx: &mut BindgenContext) -> Result<ItemId, ParseError>;
    fn from_ty(ty: &clang::Type,
               location: Option<clang::Cursor>,
               parent: Option<ItemId>,
               ctx: &mut BindgenContext) -> Result<ItemId, ParseError>;
    fn named_type<S>(name: S, default: Option<ItemId>, parent: ItemId,
                     context: &mut BindgenContext) -> ItemId
        where S: Into<String>;
    fn named_type_with_id<S>(id: ItemId, name: S, default: Option<ItemId>,
                             parent: ItemId, context: &mut BindgenContext) -> ItemId
        where S: Into<String>;
    fn builtin_type(kind: TypeKind, is_const: bool, context: &mut BindgenContext) -> ItemId;
}
