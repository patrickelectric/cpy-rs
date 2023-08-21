extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    bracketed, parse::Parse, parse::ParseStream, parse_macro_input, punctuated::Punctuated,
    Attribute, Ident, ItemFn, Lit, Meta, Result, Token,
};

#[proc_macro_attribute]
pub fn cpy_enum(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemEnum);
    let name = &input.ident;

    let comment = get_comment(&input.attrs);

    let variants: Vec<_> = input.variants.iter().map(|v| &v.ident).collect();
    let expanded = quote! {
        #[doc = #comment]
        #[derive(Clone, Debug)]
        #[repr(C)]
        #[cfg_attr(feature = "python", pyo3::prelude::pyclass)]
        pub enum #name {
            #(#variants),*
        }
    };

    expanded.into()
}

#[proc_macro_attribute]
pub fn cpy_struct(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemStruct);
    let name = &input.ident;

    let comment = get_comment(&input.attrs);

    let fields: Vec<_> = input
        .fields
        .iter()
        .map(|f| {
            let fname = &f.ident;
            let ftype = &f.ty;
            quote! { #fname: #ftype }
        })
        .collect();

    let expanded = quote! {
        #[doc = #comment]
        #[derive(Clone, Debug)]
        #[repr(C)]
        #[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all, set_all))]
        pub struct #name {
            #(#fields),*
        }
    };

    expanded.into()
}

#[proc_macro_attribute]
pub fn cpy_fn(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);

    let (comment_c, comment_py) = get_comments(&input.attrs);
    input.attrs.retain(|attr| {
        !attr.path.is_ident("comment")
            && !attr.path.is_ident("comment_c")
            && !attr.path.is_ident("comment_py")
    });

    let fn_name = &input.sig.ident;
    let inputs = &input.sig.inputs;
    let output = &input.sig.output;
    let block = &input.block;

    let expanded = quote! {
        #[cfg_attr(not(feature = "python"), doc = #comment_c)]
        #[cfg_attr(feature = "python", doc = #comment_py)]
        #[no_mangle]
        #[cfg_attr(feature = "python", pyo3::prelude::pyfunction)]
        pub extern "C" fn #fn_name(#inputs) #output #block
    };

    expanded.into()
}

#[proc_macro_attribute]
pub fn cpy_fn_c(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);

    let comment = get_comment(&input.attrs);
    input.attrs.retain(|attr| !attr.path.is_ident("comment"));

    let mut fn_name = input.sig.ident;
    if fn_name.to_string().ends_with("_c") {
        fn_name = format_ident!("{}", &fn_name.to_string().trim_end_matches("_c"));
    }

    let inputs = &input.sig.inputs;
    let output = &input.sig.output;
    let block = &input.block;

    let expanded = quote! {
        #[doc = #comment]
        #[no_mangle]
        #[cfg(not(feature = "python"))]
        pub extern "C" fn #fn_name(#inputs) #output #block
    };

    expanded.into()
}

#[proc_macro_attribute]
pub fn cpy_fn_py(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);

    let comment = get_comment(&input.attrs);
    input.attrs.retain(|attr| !attr.path.is_ident("comment"));

    let mut fn_name = input.sig.ident;
    if fn_name.to_string().ends_with("_py") {
        fn_name = format_ident!("{}", &fn_name.to_string().trim_end_matches("_py"));
    }
    let inputs = &input.sig.inputs;
    let output = &input.sig.output;
    let block = &input.block;

    let expanded = quote! {
        #[doc = #comment]
        #[cfg(feature = "python")]
        #[pyo3::prelude::pyfunction]
        pub fn #fn_name(#inputs) #output #block
    };

    expanded.into()
}

fn get_comment(attributes: &[Attribute]) -> String {
    for attribute in attributes {
        if let Ok(Meta::NameValue(meta_name_value)) = attribute.parse_meta() {
            if meta_name_value.path.is_ident("comment") {
                if let Lit::Str(lit_str) = meta_name_value.lit {
                    return lit_str.value();
                }
            }
        }
    }
    "No documentation".to_string()
}

fn get_comments(attributes: &[Attribute]) -> (Option<String>, Option<String>) {
    let mut comment_c: Option<String> = None;
    let mut comment_py: Option<String> = None;
    let mut comment: Option<String> = None;

    for attribute in attributes {
        if let Ok(Meta::NameValue(meta_name_value)) = attribute.parse_meta() {
            if let Some(ident) = meta_name_value.path.get_ident() {
                match ident.to_string().as_str() {
                    "comment_c" => {
                        if let Lit::Str(lit_str) = meta_name_value.lit {
                            comment_c = Some(lit_str.value());
                        }
                    }
                    "comment_py" => {
                        if let Lit::Str(lit_str) = meta_name_value.lit {
                            comment_py = Some(lit_str.value());
                        }
                    }
                    "comment" => {
                        if let Lit::Str(lit_str) = meta_name_value.lit {
                            comment = Some(lit_str.value());
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    if let Some(documentation) = comment {
        comment_c = comment_c.or(Some(documentation.to_string()));
        comment_py = comment_py.or(Some(documentation.to_string()));
    } else {
        comment_c = comment_c.or(Some("No documentation".to_string()));
        comment_py = comment_py.or(Some("No documentation".to_string()));
    }

    (comment_c, comment_py)
}

struct CpyModuleInput {
    name: Ident,
    types: Punctuated<Ident, Token![,]>,
    functions: Punctuated<Ident, Token![,]>,
}

impl Parse for CpyModuleInput {
    fn parse(input: ParseStream) -> Result<Self> {
        // Name
        let _: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;

        // Types
        let _: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let types_content;
        bracketed!(types_content in input);
        let types: Punctuated<Ident, Token![,]> = types_content.parse_terminated(Ident::parse)?;
        input.parse::<Token![,]>()?;

        // Functions
        let _: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let functions_content;
        bracketed!(functions_content in input);
        let functions: Punctuated<Ident, Token![,]> =
            functions_content.parse_terminated(Ident::parse)?;

        Ok(CpyModuleInput {
            name,
            types,
            functions,
        })
    }
}

#[proc_macro]
pub fn cpy_module(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as CpyModuleInput);

    let type_additions: Vec<_> = input
        .types
        .iter()
        .map(|item| {
            quote! {
                m.add_class::<#item>()?;
            }
        })
        .collect();

    let function_additions: Vec<_> = input
        .functions
        .iter()
        .map(|item| {
            quote! {
                m.add_function(pyo3::wrap_pyfunction!(#item, m)?)?;
            }
        })
        .collect();

    let module_name = &input.name;

    let expanded = quote! {
        #[cfg(feature = "python")]
        #[pyo3::pymodule]
        fn #module_name(py: pyo3::prelude::Python, m: &pyo3::prelude::PyModule) -> pyo3::prelude::PyResult<()> {
            #(#type_additions)*
            #(#function_additions)*
            Ok(())
        }
    };

    expanded.into()
}
