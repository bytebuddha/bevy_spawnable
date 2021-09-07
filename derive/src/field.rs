use quote::ToTokens;

pub enum Field {
    Component { ident: syn::Ident },
    Bundle { ident: syn::Ident }
}

impl Field {
    pub fn ident(&self) -> &syn::Ident {
        match self {
            Field::Component { ident } => ident,
            Field::Bundle { ident } => ident
        }
    }
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Field::Component { ident } => tokens.extend(quote! {.insert(#ident)}),
            Field::Bundle { ident } => tokens.extend(quote! {.insert_bundle(#ident)})
        }
    }
}
