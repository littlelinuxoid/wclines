extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DataEnum, parse_macro_input};

/// NOTE: This derive macro is not very general since i wrote specifically for wclines, but i can see
/// its slightly refactored version in a separate library for general_purpose enum handling.
/// This macro creates two important for wlcines things:
/// 1. It creates functionality to match enum variants, returning string values.
///#EXAMPLES
/// ```
/// #[derive(Matcher)]
/// enum Format {
///     Txt,
///     Rust,
///     Js,
///     Fortran,
///     Other,
/// }
/// fn main() {
///     let a = Format::Rust;
///     assert_eq!("Rust", a.match_to_str()),
/// }
///```
/// But it is not just stringify'ing enum variants, although it is by default
/// For defining match_to_str() function's custom behaviour, this macro provides output
/// attribute, that changes a string to which the enum variant will be matched.
///
/// ```
/// #[derive(Matcher)]
/// enum Format {
///  #[output("Hello World!")]
///  Rust,
///  //same enum from previous example
/// }
/// fn main() {
///     let a = Format::Txt,
///     assert_eq!("Hello World!", a.match_to_str())
/// }
/// ```
/// 2. The second purpose of this macro is essentially a reverse of the first.
/// It creates an implementation of ```std::str::FromStr``` trait for the enum, making it possible
/// to call ```parse::<EnumName>()``` on a string literal provides ```file_format```
/// attribute for defining custom behaviour:
/// ```
/// #[derive(Matcher)]
/// enum Format {
/// //same enum from previous example
///     #[file_format("I Love React")]
///     JavaScript,
///     C,
///
/// }
/// fn main() {
///     let a = "c".parse::<Format>().unwrap();
///     // by default, the lowercase version of enum member's name gets matched to it, making it easy to
///     // parse most languages' names without defining custom behaviour, since for most languages
///     // their name is the same as their file extension.
///     assert_eq!(Format::Rust,a);
///     let b = "I Love React".parse::<Format>().unwrap();
///     assert_eq!(Format::JavaScript, b)
/// }
///
/// ```
///
///
#[proc_macro_derive(Matcher, attributes(file_format, output, error_attr))]
pub fn matcher_derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::DeriveInput);
    let name = input.ident;
    let variants = if let syn::Data::Enum(DataEnum { variants, .. }) = input.data {
        variants
    } else {
        panic!("Enum-only macro!")
    };

    //maps enum variants to strings
    let match_arms_to_str = variants.iter().map(|variant| {
        let var_name = &variant.ident;
        if var_name == "ErrorOccured" {
            return quote! {
                #name::ErrorOccured {message,file} => {
                    Box::leak(format!("[ERROR] file {file}: {message}").into_boxed_str())
                },
            };
        }
        let parsed_with_attr = variant.attrs.iter().find_map(|attribute| {
            if attribute.path().is_ident("output") {
                attribute
                    .parse_args::<syn::LitStr>()
                    .ok()
                    .map(|lit_str| lit_str.value())
            } else {
                None
            }
        });
        let var_str = parsed_with_attr.unwrap_or_else(|| var_name.to_string());
        quote! {
             #name::#var_name => #var_str,
        }
    });
    //maps strings to enum variants
    let match_arms_rev = variants.iter().filter_map(|variant| {
        let var_name = &variant.ident;
        if var_name == "ErrorOccured" {
            None
        } else if var_name == "Other" {
            Some(quote! { _ => Ok(#name::#var_name), })
        } else {
            let parsed_with_attr = variant.attrs.iter().find_map(|attribute| {
                if attribute.path().is_ident("file_format") {
                    attribute
                        .parse_args::<syn::LitStr>()
                        .ok()
                        .map(|lit_str| lit_str.value())
                } else {
                    None
                }
            });
            let var_str = parsed_with_attr.unwrap_or_else(|| var_name.to_string().to_lowercase());

            Some(quote! {
                #var_str => Ok(#name::#var_name),
            })
        }
    });

    quote! {
        impl #name {
            pub fn match_to_str(&self) -> &'static str {
                match self {
                    #(#match_arms_to_str)*
                }
            }

        }

        impl std::str::FromStr for #name {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s{
                    #(#match_arms_rev)*
                }
            }
        }
    }
    .into()
}
