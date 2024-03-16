use syn::Type;

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
