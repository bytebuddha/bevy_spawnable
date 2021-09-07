use quote::ToTokens;

pub struct Child {
    pub ident: syn::Ident
}

impl ToTokens for Child {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = &self.ident;
        tokens.extend(quote! { #ident.spawn(parent.spawn());});
    }
}
