use proc_macro2::TokenStream;
use quote::quote;
use syn::{FnArg, Type};

use crate::inner_type_from_path_segment;
use crate::parser::exclusion_parser::should_exclude_type;

/// Generates a comma-separated string of parameter names and their types, excluding specified types.
///
/// This function takes a vector of `FnArg` items, typically representing the arguments of a function,
/// and generates a comma-separated string of parameter names and their types. It handles various
/// types of function arguments including references, path types, and more. Additionally, it excludes
/// types specified in the provided exclusion list.

pub fn params_as_comma_separated_str(args: Vec<FnArg>, exclusion_list: &[String]) -> String {
    let mut params = Vec::new();

    for arg in args {
        if let FnArg::Typed(syn::PatType { pat, ty, .. }) = arg {
            let pat_ident = match &*pat {
                syn::Pat::Ident(pat_ident) => pat_ident,
                _ => {
                    continue
                },
            };

            let param_name = pat_ident.ident.to_string();

            #[cfg(debug_assertions)]
            println!("Examining param <{param_name}>");

            let param_type = match &*ty {
                Type::Reference(ref type_reference) => {
                    if let Type::Path(ref type_path) = *type_reference.elem {
                        if let Some(syn::PathArguments::AngleBracketed(params)) =
                            type_path.path.segments.last().map(|p| &p.arguments)
                        {
                            if let Some(inner_type) = params.args.first() {
                                let ts: TokenStream = quote! { #inner_type };
                                ts.to_string()
                            } else {
                                continue;
                            }
                        }
                        else {
                            if type_path.path.is_ident("str") {
                                "string".to_string()
                            } else {
                                continue;
                            }                        }
                    } else {
                        continue;
                    }
                }
                Type::Path(type_path) => {
                    let path = &type_path.path;
                    let last = path.segments.iter().last().unwrap();

                    let inner_type = inner_type_from_path_segment(last);
                    if let Some(inner_type) = inner_type {
                        return inner_type;
                    } else {
                        if should_exclude_type(last.ident.to_string(), exclusion_list) {
                            #[cfg(debug_assertions)]
                            println!("Excluding param {param_name}:{}", last.ident.to_string());
                            continue;
                        }

                        // TODO: use [TypeShare](https://github.com/1Password/typeshare) smarts
                        match last.ident.to_string().as_str() {
                            "i32" | "i64" | "u32" | "u64" | "f32" | "f64" | "usize" => "number".to_string(),
                            t => t.to_string(),
                        }
                    }
                }
                _ => {
                    #[cfg(debug_assertions)]
                    println!("Skipping param <{param_name}> due to missing match");
                    continue;
                }
            };
            params.push(format!("{}:{}", param_name, param_type));
        }
        else {
            #[cfg(debug_assertions)]
            println!("Skipping param w/ Receiver type");
        }
    }

    params.join(",")
}
