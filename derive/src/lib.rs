#![feature(proc_macro_diagnostic)]
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use syn::spanned::Spanned;
use proc_macro::TokenStream;

#[proc_macro_derive(Spawnable, attributes(child, bundle))]
pub fn derive_spawnable(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as syn::DeriveInput);
    let crate_path = quote! {::bevy_spawnable};
    let ident = &derive_input.ident;
    let generics = &derive_input.generics;
    let mut children = vec![];
    let mut components = vec![];
    let mut bundles = vec![];
    let mut fields = vec![];
    match derive_input.data {
        syn::Data::Struct(data) => {
            for field in &data.fields {
                let _ident = &field.ident;
                fields.push(field.ident.clone());
                let mut has_attribute = false;
                for attr in &field.attrs {
                    if let Some(attr_ident) = attr.path.get_ident() {
                        let _type = &field.ty;
                        if attr_ident == "bundle" {
                           bundles.push(quote! { #_ident });
                           has_attribute = true;
                        } else if attr_ident == "child" {
                            children.push(quote! { #_ident.spawn(parent.spawn()); });
                            has_attribute = true;
                        }
                    }
                }
                if !has_attribute {
                    components.push(quote! { #_ident });
                }
            }
        },
        _ =>  derive_input
                .span()
                .unwrap()
                .error("Spawnable can only be derived from structs")
                .emit()
    }
    let childs = if children.is_empty() {
        quote! {}
    } else {
        quote! {.with_children(|parent| { #(#children)*})}
    };
    let output = quote! {
        impl #crate_path::Spawnable for #ident #generics {
            fn spawn(self, mut commands: ::bevy::ecs::world::EntityMut) {
                let #ident { #(#fields,)* } = self;
                commands
                    #(.insert(#components))*
                    #(.insert_bundle(#bundles))*
                    #childs;
            }
        }
    };
    output.into()
}
