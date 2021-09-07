use quote::ToTokens;
use syn::spanned::Spanned;

use super::field::Field;
use super::child::Child;

pub struct SpawnableDerive {
    ident: syn::Ident,
    generics: syn::Generics,
    fields: Vec<Field>,
    children: Vec<Child>
}

impl SpawnableDerive {

    fn get_field_names(&self) -> impl Iterator<Item = &syn::Ident> {
        self.fields.iter().map(|x|x.ident()).chain(self.children.iter().map(|x|&x.ident))
    }
}

impl From<syn::DeriveInput> for SpawnableDerive {
    fn from(derive_input: syn::DeriveInput) -> SpawnableDerive {
        let mut children = vec![];
        let mut fields = vec![];
        match derive_input.data {
            syn::Data::Struct(data) => {
                for field in &data.fields {
                    let ident = &field.ident;
                    let mut has_attribute = false;
                    for attr in &field.attrs {
                        if let Some(attr_ident) = attr.path.get_ident() {
                            let _type = &field.ty;
                            if attr_ident == "bundle" {
                               fields.push(Field::Bundle { ident: field.ident.as_ref().unwrap().clone() });
                               has_attribute = true;
                            } else if attr_ident == "child" {
                                children.push(Child { ident: ident.as_ref().unwrap().clone() });
                                has_attribute = true;
                            }
                        }
                    }
                    if !has_attribute {
                        fields.push(Field::Component { ident: ident.as_ref().unwrap().clone() });
                    }
                }
            },
            _ =>  derive_input
                    .span()
                    .unwrap()
                    .error("Spawnable can only be derived from structs")
                    .emit()
        }
        SpawnableDerive {
            children,
            fields,
            ident: derive_input.ident,
            generics: derive_input.generics,
        }
    }
}

impl ToTokens for SpawnableDerive {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = &self.ident;
        let generics = &self.generics;
        let field_names = self.get_field_names();
        let fields = &self.fields;
        let children = &self.children;
        let childs = if children.is_empty() {
            quote!{}
        } else {
            quote!{ .with_children(|parent|{#(#children)*}) }
        };
        tokens.extend(quote! {
            impl ::bevy_spawnable::Spawnable for #ident #generics {
                fn spawn(self, mut commands: ::bevy::ecs::world::EntityMut) {
                    let #ident { #(#field_names,)* } = self;
                    commands
                        #(#fields)*
                        #childs;
                }
            }
        });
    }
}
