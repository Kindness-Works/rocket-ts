// Generate a TypeScript interface for Rocket request handlers.
//
// Assumptions:
//  * Handlers return a single TypeShare'd type

use clap::{Parser, Subcommand};
use proc_macro2::TokenStream;
use quote::quote;
use std::fs::File;
use std::io::Write;
use std::{fs, path::PathBuf};
use syn::{
    visit::{self, Visit},
    Attribute, FnArg, ItemFn, ReturnType, Type,
};
struct RocketReqHandler {
    pub name: String,
    pub path: String,
    pub params: Vec<FnArg>,
    pub return_type: ReturnType,
}

struct Visitor {
    pub functions: Vec<RocketReqHandler>,
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

fn handle_get_attr(attr: &Attribute) -> String {
    let path = attr.tokens.to_string();
    let path = path.trim_matches(|c| c == '(' || c == ')');
    let parts: Vec<_> = path.split(',').collect();

    let mut route = parts[0];
    route = route.trim_matches(|c| c == '"' || c == ' ');
    route.to_string()
}

fn inner_box_type(val: &Box<syn::Type>) -> Option<String> {
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

fn inner_type_from_path_segment(segment: &syn::PathSegment) -> Option<String> {
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
        // println!("segment.ident =  Json");

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
    } else {
        // println!("segment.ident = {}", segment.ident);
    }

    // println!("inner_type_from_path_segment ending w/ None");

    None
}

// Provide the inner return type of a request handler that TypeScript callers will care about.
//
// i.e. Result<Json<Message>> -> "Message"
fn inner_return_type(node: &ReturnType) -> String {
    if let ReturnType::Type(_, type_box) = node {
        match inner_box_type(type_box) {
            Some(inner) => return inner,
            None => {}
        }
    }

    String::from("any") // Unit types and other return types that don't match the assumption of a single TypeShare'd type.
}

fn _inner_param(type_box: &Box<Type>) -> String {
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

fn params_as_comma_separated_str(args: Vec<FnArg>) -> String {
    let mut params = Vec::new();

    for arg in args {
        if let FnArg::Typed(syn::PatType { pat, ty, .. }) = arg {
            let pat_ident = match &*pat {
                syn::Pat::Ident(pat_ident) => pat_ident,
                _ => continue,
            };

            let param_name = pat_ident.ident.to_string();

            let param_type = match &*ty {
                Type::Reference(ref type_reference) => {
                    if let Type::Path(ref type_path) = *type_reference.elem {
                        if let Some(syn::PathArguments::AngleBracketed(params)) =
                            type_path.path.segments.last().map(|p| &p.arguments)
                        {
                            if let Some(inner_type) = params.args.first() {
                                let ts: TokenStream = quote! { #inner_type };
                                //println!("{}", ts);
                                ts.to_string()
                            } else {
                                //println!("Skipping param {param_name} due to else #1");

                                continue;
                            }
                        } else {
                            //println!("Skipping param {param_name} due to else #2");

                            continue;
                        }
                    } else {
                        //println!("Skipping param {param_name} due to else #3");

                        continue;
                    }
                }
                Type::Path(type_path) => {
                    let path = &type_path.path;
                    let last = path.segments.iter().last().unwrap();

                    //println!("last={}", last.ident);

                    let inner_type = inner_type_from_path_segment(last);
                    if let Some(inner_type) = inner_type {
                        return inner_type;
                    } else {
                        last.ident.to_string()
                    }
                }
                _ => {
                    //println!("Skipping param {param_name} due to missing match");
                    continue;
                }
            };
            params.push(format!("{}:{}", param_name, param_type));
        }
    }

    params.join(",")
}

#[derive(Debug, Parser)]
#[command(name = "rts")]
#[command(bin_name = "rocket-ts")]
#[command(about = "A blazing fast type generator for typescript from rocket backend 🦀", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command()]
    Generate {
        #[clap(
            required = true,
            help = "Input directory Or Input file",
            default_value = "thread.rs",
            short = 'i',
            long = "input"
        )]
        input_dir: PathBuf,

        #[clap(
            required = true,
            help = "Output file",
            default_value = "k7.ts",
            short = 'o',
            long = "output"
        )]
        output_dir: String,
    },
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    let mut files: Vec<PathBuf> = vec![];

    match args.command {
        Commands::Generate {
            input_dir,
            output_dir,
        } => {
            if input_dir.is_file() {
                let file_path = input_dir;
                if let Some(file_name) = file_path.file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        if file_name_str.ends_with(".rs") {
                            println!("Loading the Input File : {}", file_name_str);
                            files.push(file_path);
                        }
                    }
                }
            } else {
                let folder_path = input_dir;
                let entries = fs::read_dir(folder_path)?;
                for entry in entries {
                    let entry = entry?;
                    let file_path = entry.path();

                    if let Some(file_name) = file_path.file_name() {
                        if let Some(file_name_str) = file_name.to_str() {
                            if file_name_str.ends_with(".rs") {
                                println!("Loading the Input File : {}", file_name_str);
                                files.push(file_path);
                            }
                        }
                    }
                }
            }

            println!("Loading the Output File : {}", &output_dir);

            let mut ts = r#"/*
* Generated by rocket-ts 0.1.0 🚀 🌎
*/
   export interface k7 {
   "#
            .to_string();

            for file_path in files {
                let mut visitor = Visitor { functions: vec![] };
                let contents = fs::read_to_string(&file_path)?;
                let syntax = syn::parse_file(&contents).expect("Unable to parse file");
                visitor.visit_file(&syntax);

                let file_name_os_str = file_path.file_name().expect("Failed to get file name");
                let file_name_str = file_name_os_str.to_str().expect("Failed to convert to str");

                ts.push_str(&format!("    // {file_name_str}\n"));
                for handler in visitor.functions {
                    let params = params_as_comma_separated_str(handler.params);
                    let return_type = inner_return_type(&handler.return_type);
                    ts.push_str(&format!("    // Route: \"{}\"\n", handler.path));
                    ts.push_str(&format!(
                        "    {}: ({}) => {};\n",
                        handler.name, params, return_type
                    ));
                }
                ts.push('\n')
            }

            ts.push_str("}\n");

            let mut out = File::create(&output_dir).expect("Could not create file");
            out.write_all(ts.as_bytes()).expect("Unable to write data");

            println!("Exported 🚀 handlers to {}", &output_dir);
        }
    }

    Ok(())
}
