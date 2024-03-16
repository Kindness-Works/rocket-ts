use syn::Type;

/// Extracts the inner parameter type from a boxed Type.
///
/// This function is used to extract the inner parameter type from a boxed Type. It is specifically
/// designed to handle Result<Json<T>> types and return the innermost type T.
///
/// # Arguments
///
/// * `type_box` - A reference to a boxed Type.
///
/// # Returns
///
/// A String representing the inner parameter type extracted from the boxed Type.

pub fn _inner_param(type_box: &Box<Type>) -> String {
    if let Type::Path(type_path) = &**type_box {
        if let Some(segment) = type_path.path.segments.first() {
            if segment.ident == "Result" {
                match &segment.arguments {
                    syn::PathArguments::AngleBracketed(params) => {
                        if let Some(syn::GenericArgument::Type(Type::Path(inner_type))) =
                            params.args.first()
                        {
                            if let Some(inner_segment) = inner_type.path.segments.first() {
                                if inner_segment.ident == "Json" {
                                    if let syn::PathArguments::AngleBracketed(inner_params) =
                                        &inner_segment.arguments
                                    {
                                        if let Some(syn::GenericArgument::Type(Type::Path(
                                            most_inner_type,
                                        ))) = inner_params.args.first()
                                        {
                                            return most_inner_type
                                                .path
                                                .segments
                                                .last()
                                                .unwrap()
                                                .ident
                                                .to_string();
                                        }
                                    }
                                }
                            }
                        }
                    }
                    syn::PathArguments::Parenthesized(_params) => {
                        unimplemented!()
                    }
                    syn::PathArguments::None => return String::from("void"),
                }
            } else if segment.ident == "Json" {
                match &segment.arguments {
                    syn::PathArguments::AngleBracketed(params) => {
                        if let Some(syn::GenericArgument::Type(Type::Path(inner_type))) =
                            params.args.first()
                        {
                            if let Some(inner_segment) = inner_type.path.segments.first() {
                                if let syn::PathArguments::AngleBracketed(inner_params) =
                                    &inner_segment.arguments
                                {
                                    if let Some(syn::GenericArgument::Type(Type::Path(
                                        most_inner_type,
                                    ))) = inner_params.args.first()
                                    {
                                        return most_inner_type
                                            .path
                                            .segments
                                            .last()
                                            .unwrap()
                                            .ident
                                            .to_string();
                                    }
                                }
                            }
                        }
                    }
                    syn::PathArguments::Parenthesized(_params) => {
                        unimplemented!()
                    }
                    syn::PathArguments::None => return String::from("void"),
                }
            } else {
                //println!("segment.ident = {}", segment.ident);
            }
        }
    }

    String::from("any") // Unit types and other return types that don't match the assumption of a single TypeShare'd type.
}