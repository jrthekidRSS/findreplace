//use clap::Parser;
//use std::fs::File;
//use regex::Regex;
use std::process;
use findreplace::*;


fn print_help_message() {
    panic!("Pretend this is the '--help' message.")
}


fn main() {
    let args = Args::parse().unwrap_or_else( |err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    for i in 0..args.files.len() {
        for j in 0..args.patterns.len() {
            replace_text_in_file(&args.files[i], args.patterns[j].as_str(), args.replacement.as_str()).expect("Failed to access a file!");
        }
    }
}


