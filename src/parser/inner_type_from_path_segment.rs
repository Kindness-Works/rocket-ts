use syn::Type;

/// Extracts the inner type from a syn::PathSegment.
///
/// This function is used to extract the inner type from a syn::PathSegment, specifically handling
/// Result<Json<T>> and Json<T> types. It returns the innermost type as a String.
///
/// # Arguments
///
/// * `segment` - A reference to a syn::PathSegment.
///
/// # Returns
///
/// An Option containing the innermost type as a String if found, otherwise None.

pub fn inner_type_from_path_segment(segment: &syn::PathSegment) -> Option<String> {
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
                            if let Some(syn::GenericArgument::Type(Type::Path(most_inner_type))) =
                                inner_params.args.first()
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
                        } else {
                            return Some(inner_segment.ident.to_string());
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

    // There is no inner type. E.g. String, AgentService, etc.

    None
}