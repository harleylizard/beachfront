use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Ident};

#[proc_macro_derive(Enum)]
pub fn derive_enum(item: TokenStream) -> TokenStream {
    derive_enum_inner(item.into()).into()
}

fn derive_enum_inner(item: TokenStream2) -> TokenStream2 {
    let input: DeriveInput = syn::parse2(item).expect("input should be valid");

    let Data::Enum(data_enum) = input.data else {
        panic!("`Enum` can only be applied to enums.")
    };

    if data_enum.variants.is_empty() {
        panic!(
            "{} must have at least one variant so that `Enum` can be applied to it.",
            input.ident
        );
    }

    let mut next = 0;
    let mut discriminants = Vec::with_capacity(data_enum.variants.len());

    for variant in data_enum.variants {
        if !variant.fields.is_empty() {
            panic!(
                "{} must not have any variants with fields so that `Enum` can be applied to it.",
                input.ident
            );
        } else if variant.discriminant.is_some() {
            panic!(
                "{} must not have any manually assigned discriminants so that `Enum` can be applied to it.",
                input.ident
            )
        } else {
            discriminants.push(next);
            next += 1;
        }
    }

    let repr_type = input
        .attrs
        .iter()
        .find(|v| v.path().is_ident("repr"))
        .map(extract_type_from_repr)
        .expect("enum {} should be annotated with #[repr(..)]");

    let min = discriminants.first().unwrap();
    let max = discriminants.last().unwrap();
    let ident = input.ident;

    quote! {
        const _: () = {
            use discriminant::Enum;
            use discriminant::FromDiscriminant;

            unsafe impl Enum for #ident {
                type Discriminant = #repr_type;

                const MIN: <Self as Enum>::Discriminant = #min as <Self as Enum>::Discriminant;
                const MAX: <Self as Enum>::Discriminant = #max as <Self as Enum>::Discriminant;
            }

            unsafe impl FromDiscriminant for #ident {
                fn from_discriminant(d: <Self as Enum>::Discriminant) -> ::core::option::Option<Self> {
                    unsafe {
                        ::core::mem::transmute(d)
                    }
                }

                fn next(self) -> Option<Self> {
                    <Self as FromDiscriminant>::from_discriminant(self as <Self as Enum>::Discriminant + (1 as <Self as Enum>::Discriminant))
                }

                fn previous(self) -> Option<Self> {
                    <Self as FromDiscriminant>::from_discriminant(self as <Self as Enum>::Discriminant - (1 as <Self as Enum>::Discriminant))
                }
            }
        };
    }
}

fn extract_type_from_repr(attr: &Attribute) -> Ident {
    syn::parse2::<Ident>(attr.meta.require_list().unwrap().tokens.clone()).unwrap()
}
