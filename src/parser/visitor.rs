use syn::{
    visit::{self, Visit},
    Attribute, FnArg, ItemFn, ReturnType,
};

/// Represents a Rocket request handler.
///
/// This struct holds information about a Rocket request handler, including its name,
/// path, parameters, and return type.
pub struct RocketReqHandler {
    /// The name of the request handler function.
    pub name: String,
    /// The path associated with the request handler.
    pub path: String,
    /// The parameters of the request handler function.
    pub params: Vec<FnArg>,
    /// The return type of the request handler function.
    pub return_type: ReturnType,
}

/// Visitor for traversing Rust syntax tree and extracting Rocket request handlers.
///
/// This struct implements the Visit trait to traverse the syntax tree and extract
/// information about Rocket request handlers.
pub struct Visitor {
    /// Vector to store extracted Rocket request handlers.
    pub functions: Vec<RocketReqHandler>,
}

/// Extracts the path attribute value from a syn::Attribute.
///
/// This function takes a syn::Attribute representing a Rocket attribute like #[get("/path")]
/// and extracts the path value from it.
///
/// # Arguments
///
/// * `attr` - A reference to a syn::Attribute representing a Rocket attribute.
///
/// # Returns
///
/// A String containing the path value extracted from the attribute.
pub fn handle_get_attr(attr: &Attribute) -> String {
    let path = attr.tokens.to_string();
    let path = path.trim_matches(|c| c == '(' || c == ')');
    let parts: Vec<_> = path.split(',').collect();

    let mut route = parts[0];
    route = route.trim_matches(|c| c == '"' || c == ' ');
    route.to_string()
}

impl<'ast> Visit<'ast> for Visitor {
    fn visit_item_fn(&mut self, item_fn: &'ast ItemFn) {
        for attr in &item_fn.attrs {
            if let Some(first_segment) = attr.path.segments.first() {
                if first_segment.ident == "get" || first_segment.ident == "post" {
                    let function_name = item_fn.sig.ident.to_string();

                    let path = handle_get_attr(attr);

                    let mut params: Vec<FnArg> = vec![];
                    for input in &item_fn.sig.inputs {
                        params.push((*input).clone());
                    }

                    let return_type = item_fn.sig.output.clone();

                    let req_handler = RocketReqHandler {
                        name: function_name,
                        path,
                        params,
                        return_type,
                    };
                    self.functions.push(req_handler);
                }
            }
        }

        visit::visit_item_fn(self, item_fn);
    }
}
