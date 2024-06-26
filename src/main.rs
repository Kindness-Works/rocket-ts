// Generate a TypeScript interface for Rocket request handlers.
//
// Assumptions:
//  * Handlers return a single TypeShare'd type

use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::Write;
use std::{fs, path::PathBuf};
use syn::{visit::Visit, ReturnType};

use log::debug;
use parser::exclusion_parser::parse_exclusion_file;
use parser::inner_box_type::inner_box_type;
use parser::inner_type_from_path_segment::inner_type_from_path_segment;
use parser::params_as_comma_seperated::params_as_comma_separated_str;
use parser::visitor::Visitor;

mod parser;

/// Provide the inner return type of a request handler that TypeScript callers will care about.
///
/// i.e. Result<Json<Message>> -> "Message"
fn inner_return_type(node: &ReturnType) -> String {
    if let ReturnType::Type(_, type_box) = node {
        if let Some(inner) = inner_box_type(type_box) {
            return inner;
        }
    }

    String::from("any") // Unit types and other return types that don't match the assumption of a single TypeShare'd type.
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
            help = "Input directory or file to parse for interface generation.",
            short = 'i',
            long = "input",
            value_name = "INPUT"
        )]
        input_dir_or_file: PathBuf,

        #[clap(
            help = "Optional output file. STDOUT if not provided.",
            short = 'o',
            long = "output",
            value_name = "OUTPUT"
        )]
        output_file: Option<String>,

        #[clap(
            required = false,
            help = "File listing parameters to exclude (e.g., Request Guards).",
            short = 'e',
            long = "exclude-type",
            value_name = "EXCLUDE"
        )]
        exclude_file: Option<String>,

        #[clap(
            help = "Enable detailed debug output for troubleshooting",
            short = 'v',
            long = "verbose"
        )]
        verbose: bool,
    },
}

/// Main function to parse command-line arguments and generate TypeScript interfaces.
fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    let mut files: Vec<PathBuf> = vec![];

    match args.command {
        Commands::Generate {
            input_dir_or_file,
            output_file,
            exclude_file,
            verbose,
        } => {
            if verbose {
                std::env::set_var("RUST_LOG", "debug");
                env_logger::init();
            }

            if input_dir_or_file.is_file() {
                let file_path = input_dir_or_file;
                if let Some(file_name) = file_path.file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        if file_name_str.ends_with(".rs") {
                            debug!("Loading input file: {}", file_name_str);
                            files.push(file_path);
                        }
                    }
                }
            } else {
                let folder_path = input_dir_or_file;
                let entries = fs::read_dir(folder_path)?;
                for entry in entries {
                    let entry = entry?;
                    let file_path = entry.path();

                    if let Some(file_name) = file_path.file_name() {
                        if let Some(file_name_str) = file_name.to_str() {
                            if file_name_str.ends_with(".rs") {
                                debug!("Loading input file: {}", file_name_str);
                                files.push(file_path);
                            }
                        }
                    }
                }
            }

            let mut exclusion_list = Vec::new();

            if let Some(exclude_file) = exclude_file {
                debug!("Loading exclusion types from {}", &exclude_file);
                match parse_exclusion_file(&exclude_file) {
                    Ok(parsed_list) => {
                        debug!("Excluding types {:?}", parsed_list);

                        exclusion_list = parsed_list;
                    }
                    Err(err) => {
                        eprintln!("Error reading exclusion file: {}  [{}]", &exclude_file, err);
                        std::process::exit(1);
                    }
                }
            }

            let mut ts = r"/*
 * Generated by rocket-ts 0.1.0 🚀 🌎
 */
export interface k7 {"
                .to_string();

            for file_path in files {
                let mut visitor = Visitor { functions: vec![] };
                let contents = fs::read_to_string(&file_path)?;
                let syntax = syn::parse_file(&contents).expect("Unable to parse file");
                visitor.visit_file(&syntax);

                let file_name_os_str = file_path.file_name().expect("Failed to get file name");
                let file_name_str = file_name_os_str.to_str().expect("Failed to convert to str");

                ts.push_str(&format!("\n\t// {file_name_str}\n"));
                for handler in visitor.functions {
                    let params = params_as_comma_separated_str(handler.params, &exclusion_list);
                    let return_type = inner_return_type(&handler.return_type);
                    ts.push_str(&format!("\t// handler \"{}\"\n", handler.path));
                    ts.push_str(&format!(
                        "\t{}: ({}) => {};\n",
                        handler.name, params, return_type
                    ));
                }
            }

            ts.push_str("}\n");

            match output_file {
                Some(output_file) => {
                    let mut out = File::create(&output_file).expect("Could not create file");
                    out.write_all(ts.as_bytes()).expect("Unable to write data");

                    debug!("Exported 🚀 handlers to {}", &output_file);
                }
                None => {
                    println!("{}", ts);
                }
            }
        }
    }

    Ok(())
}
