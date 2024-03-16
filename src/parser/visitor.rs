use syn::{
    visit::{self, Visit},
    Attribute, FnArg, ItemFn, ReturnType,
};

pub struct RocketReqHandler {
    pub name: String,
    pub path: String,
    pub params: Vec<FnArg>,
    pub return_type: ReturnType,
}

pub struct Visitor {
    pub functions: Vec<RocketReqHandler>,
}

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
