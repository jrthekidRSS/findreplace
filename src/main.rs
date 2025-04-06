//use clap::Parser;
//use std::fs::File;
use std::path::PathBuf;
//use regex::Regex;

fn print_help_message() {
    panic!("Pretend this is the '--help' message.")
}

#[derive(Debug)]
struct Args {
    files: Vec<PathBuf>,
    patterns: Vec<String>,
    replacement: String,
}

impl Args {
    fn parse() -> Self {
        if std::env::args().skip(1).count() < 1 {
            print_help_message();
        }

        let args = std::env::args().skip(1);
        let mut parsed_args = Self {
            files: Vec::new(),
            patterns: Vec::new(),
            replacement: String::new(),
        };
        let mut state = None;

        for arg in args {
            let matched_arg = match arg.as_str() {
                "for" => {
                    state = Some(0 as u8);
                    continue;
                }
                "replace" => {
                    state = Some(1 as u8);
                    continue;
                }
                "with" => {
                    state = Some(2 as u8);
                    continue;
                }
                _ => arg,
            };

            match state {
                Some(0) => {
                    let path = PathBuf::from(matched_arg);
                    if !path.is_file() {
                        panic!("Can't find '{}'! Does it exist? Do you have the proper permissions?", path.display());
                    }
                    push_element(&mut parsed_args.files, path)
                },
                Some(1) => push_element(&mut parsed_args.patterns, matched_arg),
                Some(2) => parsed_args.replacement = matched_arg,
                _ => print_help_message(),
            }
        }
        
        parsed_args
    }
}

fn main() {
    let args = Args::parse();
    //println!("{:?}", args);
    for i in 0..(args.files.len() - 1) {
        for j in 0..(args.patterns.len() - 1) {
            replace_text_in_file(&args.files[i], args.patterns[j].as_str(), args.replacement.as_str()).expect("Failed to access a file!");
        }
    }
}

fn push_element<T: std::iter::Extend<T>>(vector: &mut Vec<T>, element: T) {
    vector.extend([ element ]);
}

fn replace_text_in_file(path: &PathBuf, pattern: &str, replacement: &str) -> Result<(), std::io::Error> {

    let text = std::fs::read_to_string(path).unwrap();

    let text = text.replace(pattern, replacement);
    std::fs::write(path, &text)?;
    Ok(())
}
