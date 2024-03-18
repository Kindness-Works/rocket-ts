use syn::Type;

/// Extracts the innermost type name from a Box containing a syn::Type.
///
/// This function takes a reference to a Box containing a syn::Type and returns the innermost
/// type name as a String if it matches certain patterns. It traverses through nested types,
/// looking for specific patterns such as Result<Json<T>> or Json<T> to extract the inner type.
///
/// # Arguments
///
/// * `val` - A reference to a Box containing a syn::Type.
///
/// # Returns
///
/// An Option<String>:
/// - Some(String): If the innermost type is successfully extracted, it returns the type name as a String.
/// - None: If the provided type does not match any expected pattern.

pub fn inner_box_type(val: &Box<syn::Type>) -> Option<String> {
    if let Type::Path(type_path) = &**val {
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
                                            return Some(
                                                most_inner_type
                                                    .path
                                                    .segments
                                                    .last()
                                                    .unwrap()
                                                    .ident
                                                    .to_string(),
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                    syn::PathArguments::Parenthesized(_params) => {
                        unimplemented!()
                    }
                    syn::PathArguments::None => return Some(String::from("void")),
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
                                        return Some(
                                            most_inner_type
                                                .path
                                                .segments
                                                .last()
                                                .unwrap()
                                                .ident
                                                .to_string(),
                                        );
                                    }
                                }
                            }
                        }
                    }
                    syn::PathArguments::Parenthesized(_params) => {
                        unimplemented!()
                    }
                    syn::PathArguments::None => return Some(String::from("void")),
                }
            } else {
                match &segment.arguments {
                    syn::PathArguments::AngleBracketed(params) => {
                        if let Some(syn::GenericArgument::Type(Type::Path(inner_type))) =
                            params.args.first()
                        {
                            if let Some(inner_segment) = inner_type.path.segments.first() {
                                if inner_segment.ident == "Vec" {
                                    if let syn::PathArguments::AngleBracketed(inner_params) =
                                        &inner_segment.arguments
                                    {
                                        if let Some(syn::GenericArgument::Type(Type::Path(
                                            most_inner_type,
                                        ))) = inner_params.args.first()
                                        {
                                            let most_vec_inner_type = format!(
                                                "{}{}",
                                                most_inner_type.path.segments.last().unwrap().ident,
                                                "[]"
                                            );
                                            return Some(most_vec_inner_type);
                                        }
                                    }
                                } else {
                                    return Some(
                                        inner_type.path.segments.last().unwrap().ident.to_string(),
                                    );
                                }
                            }
                        }
                    }
                    syn::PathArguments::Parenthesized(_params) => {
                        unimplemented!()
                    }
                    syn::PathArguments::None => return Some(String::from("void")),
                }
            }
        }
    }

    None
}
