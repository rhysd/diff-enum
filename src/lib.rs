//! Attribute macro to define enum by differences of variants with useful accessors
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_attribute;
use quote::quote;
use syn::{Data, DeriveInput, FieldsNamed, Ident};

#[proc_macro_attribute]
pub fn common_fields(attr: TokenStream, item: TokenStream) -> TokenStream {
    let shared: FieldsNamed = parse_shared_fields(attr);
    if shared.named.is_empty() {
        panic!("No shared field is set to #[diff_enum::common_fields]");
    }

    let input: DeriveInput = match syn::parse(item) {
        Ok(parsed) => parsed,
        Err(err) => panic!(
            "#[diff_enum::common_fields] only can be set at enum definition: {}",
            err
        ),
    };

    let impl_accessors = generate_accessors(&shared, &input, input.ident.clone());
    let expanded_enum = expand_shared_fields(&shared, input);
    let tokens = quote! {
        #expanded_enum
        #impl_accessors
    };

    tokens.into()
}

fn parse_shared_fields(attr: TokenStream) -> FieldsNamed {
    use proc_macro::{Delimiter, Group, TokenTree};
    let braced = TokenStream::from(TokenTree::Group(Group::new(Delimiter::Brace, attr)));
    match syn::parse(braced) {
        Ok(fields) => fields,
        Err(err) => panic!(
            "Cannot parse fields in attributes at #[diff_enum::common_fields]: {}",
            err
        ),
    }
}

fn expand_shared_fields(shared: &FieldsNamed, mut input: DeriveInput) -> TokenStream2 {
    let mut enum_ = match input.data {
        Data::Enum(e) => e,
        _ => panic!("#[diff_enum::common_fields] can be set at only enum"),
    };

    for variant in enum_.variants.iter_mut() {
        match variant.fields {
            syn::Fields::Named(ref mut f) => {
                for shared_field in shared.named.iter() {
                    f.named.push(shared_field.clone());
                }
            }
            syn::Fields::Unnamed(_) => panic!(
                "#[diff_enum::common_fields] cannot mix named fields with unnamed fields at enum variant {}",
                variant.ident.to_string()
            ),
            syn::Fields::Unit => {
                variant.fields = syn::Fields::Named(shared.clone());
            }
        }
    }

    input.data = Data::Enum(enum_);
    quote!(#input)
}

fn generate_accessors(shared: &FieldsNamed, input: &DeriveInput, enum_name: Ident) -> TokenStream2 {
    let variants = match input.data {
        Data::Enum(ref e) => &e.variants,
        _ => panic!("#[diff_enum::common_fields] can be set at only enum"),
    };

    let accessors = shared.named.iter().map(|field| {
        let field_name = &field.ident;
        let ty = &field.ty;
        let arms = variants.iter().map(|variant| {
            let ident = &variant.ident;
            quote! {
                #enum_name::#ident{ref #field_name, ..} => #field_name,
            }
        });
        quote! {
            #[inline]
            #[allow(dead_code)]
            pub fn #field_name (&self) -> &#ty {
                match self {
                    #( #arms )*
                }
            }
        }
    });

    quote! {
        impl #enum_name {
            #( #accessors )*
        }
    }
}
