//! Attribute macro to define enum by differences of variants with useful accessors
//!
//! This is a small Rust library provides one attribute macro `#[diff_enum::common_fields]` to help defining
//! `enum` variants by their differences. It is useful when you need to handle data which are almost the
//! same, but different partially.
//!
//! By the attribute macro, common fields among all variants and different fields for each variant can
//! be defined separately. Common fields are defined once. Additionally accessor methods for common fields
//! are automatically defined.
//!
//! For example,
//!
//! ```rust
//! extern crate diff_enum;
//! use diff_enum::common_fields;
//!
//! #[common_fields {
//!     user: String,
//!     name: String,
//!     stars: u32,
//!     issues: u32,
//! }]
//! #[derive(Debug)]
//! enum RemoteRepo {
//!     GitHub {
//!         language: String,
//!         pull_requests: u32,
//!     },
//!     GitLab {
//!         merge_requests: u32,
//!     },
//! }
//! # let repo = RemoteRepo::GitHub {
//! #     user: "rust-lang".to_string(),
//! #     name: "rust".to_string(),
//! #     language: "rust".to_string(),
//! #     issues: 4536,
//! #     pull_requests: 129,
//! #     stars: 33679,
//! # };
//!
//! # println!("User: {}", repo.user());
//! ```
//!
//! is expanded to
//!
//! ```rust,ignore
//! #[derive(Debug)]
//! enum RemoteRepo {
//!     GitHub {
//!         language: String,
//!         pull_requests: u32,
//!         user: String,
//!         name: String,
//!         stars: u32,
//!         issues: u32,
//!     },
//!     GitLab {
//!         merge_requests: u32,
//!         user: String,
//!         name: String,
//!         stars: u32,
//!         issues: u32,
//!     },
//! }
//! ```
//!
//! Additionally, accessor functions are defined for common fields. For example,
//!
//! ```rust
//! # extern crate diff_enum;
//! # use diff_enum::common_fields;
//!
//! # #[common_fields {
//! #     user: String,
//! #     name: String,
//! #     stars: u32,
//! #     issues: u32,
//! # }]
//! # #[derive(Debug)]
//! # enum RemoteRepo {
//! #     GitHub {
//! #         language: String,
//! #         pull_requests: u32,
//! #     },
//! #     GitLab {
//! #         merge_requests: u32,
//! #     },
//! # }
//! let repo = RemoteRepo::GitHub {
//!     user: "rust-lang".to_string(),
//!     name: "rust".to_string(),
//!     language: "rust".to_string(),
//!     issues: 4536,
//!     pull_requests: 129,
//!     stars: 33679,
//! };
//!
//! println!("User: {}", repo.user());
//! ```
//!
//!
//!
//! ## Alternative
//!
//! Without this crate, it's typical to separate the data into a struct with common fields and a enum
//! variants for differences.
//!
//! For above `RemoteRepo` example,
//!
//! ```rust,ignore
//! enum RemoteRepoKind {
//!     GitHub {
//!         language: String,
//!         pull_requests: u32,
//!     },
//!     GitLab {
//!         merge_requests: u32,
//!     },
//! }
//! struct RemoteRepo {
//!     user: String,
//!     name: String,
//!     stars: u32,
//!     issues: u32,
//!     kind: RemoteRepoKind,
//! }
//! ```
//!
//! This solution has problems as follows:
//!
//! - Fields are split into 2 parts for the reason of Rust enum. Essentially number of issues and number
//!   of pull requests are both properties of a GitHub repository. As natural data structure they should
//!   be in the same flat struct.
//! - Naming the inner enum is difficult. Here I used 'Kind' to separate parts. But is it appropriate?
//!   'Kind' is too generic name with weak meaning. The weak name comes from awkwardness of the data
//!   structure.
//!
//! ## Usage
//!
//! At first, please load the crate.
//!
//! ```rust,irgnore
//! extern crate diff_enum;
//! use diff_enum::common_fields;
//! ```
//!
//! And use `#[common_fields]` attribute macro for your enum definitions.
//!
//! ```ignore
//! #[common_fields {
//!     common fields here...
//! }]
//! enum ...
//! ```
//!
//! or fully qualified name if you like
//!
//! ```ignore
//! #[diff_enum::common_fields {
//!     common fields here...
//! }]
//! enum ...
//! ```
//!
//! Any attributes and comments can be put to the common fields as normal `enum` fields.
//!
//! Accessor methods corresponding to common fields are defined. It is a useful helper to access common
//! fields without using pattern match.
//!
//! For example,
//!
//! ```rust,ignore
//! #[common_fields { i: i32 }]
//! enum E { A, B{ b: bool } }
//! ```
//!
//! Generates an accessor method for `i` as follows:
//!
//! ```rust,ignore
//! impl E {
//!     fn i(&self) -> &i32 {
//!         match self {
//!             E::A{ref i, ..} => i,
//!             E::B{ref i, ..} => i,
//!         }
//!     }
//! }
//! ```
//!
//! ## Errors
//!
//! The attribute macro causes compilation errors in the following cases.
//!
//! - When no common field is put
//! - When fields in attribute argument is not form of `field: type`
//! - When `#[common_fields {...}]` is set to other than `enum` definitions
//! - When tuple style enum variant is used in `enum` definition

extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_attribute;
use quote::quote;
use syn::{Data, DeriveInput, Fields, FieldsNamed, Ident};

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
            Fields::Named(ref mut f) => {
                for shared_field in shared.named.iter() {
                    f.named.push(shared_field.clone());
                }
            }
            Fields::Unnamed(_) => panic!(
                "#[diff_enum::common_fields] cannot mix named fields with unnamed fields at enum variant {}",
                variant.ident.to_string()
            ),
            Fields::Unit => {
                variant.fields = Fields::Named(shared.clone());
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
