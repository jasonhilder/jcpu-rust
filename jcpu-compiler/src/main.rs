mod structures;
mod parser;
mod lexer;

use std::{path::Path, fs};

use crate::parser::*;

fn main() {
    let file_path = std::env::args().nth(1).expect("no file given");
    let output_path = std::env::args().nth(2);

    let fp = Path::new(&file_path).canonicalize();

    match fp {
        Ok(fp) => {
            let outpath = String::from("boot.img");
            if let Some(output) = output_path {
                println!("output: {}", output);
            }

            let jsm = fs::read_to_string(fp).expect("failed to read file.");
            let mut parser = Parser::new(&jsm.to_string());
            parser.parse();
            lexer::lex(parser.tokens, outpath);
        },
        Err(e) => {
           match e.kind() {
               std::io::ErrorKind::NotFound => eprint!("Failed to compile: File not found"),
               _ => eprint!("File path error: {:?}", e.to_string())
           }
        }
    }
}
