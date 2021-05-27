TODO:

meld soll eine datei in /tmp benutzen, damit es keine race condition gibt


default constructor









arg_item.try_to_rust_ty


impl TryToRustTy for Item {
    type Extra = ();

    fn try_to_rust_ty(
        &self,
        ctx: &BindgenContext,
        _: &(),
    ) -> error::Result<proc_macro2::TokenStream> {
        self.kind().expect_type().try_to_rust_ty(ctx, self)
    }
}