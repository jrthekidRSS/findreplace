use std::path::PathBuf;

#[derive(Debug)]
pub struct Args {
    pub files: Vec<PathBuf>,
    pub patterns: Vec<String>,
    pub replacement: String,
}

impl Args {
    pub fn parse() -> Result<Self, &'static str> {
        if std::env::args().skip(1).count() < 1 {
            return Err("Not enough args were given. Use 'findreplace --help' for usage.")
        }

        let args = std::env::args().skip(1);
        let mut parsed_args = Self {
            files: Vec::new(),
            patterns: Vec::new(),
            replacement: String::new(),
        };

        let mut state = None;
        let mut error_happened = false;

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
                        eprintln!("Error: Can't find the file '{}'.", path.display());
                        error_happened = true;
                        continue
                    }
                    push_element(&mut parsed_args.files, path)
                },
                Some(1) => push_element(&mut parsed_args.patterns, matched_arg),
                Some(2) => parsed_args.replacement = matched_arg,
                _ => panic!("I have no idea what just happened"),
            }
        }
        
        if error_happened {
            Err("Unable to parse args. Check error messages above for more details.")
        } else {
            Ok(parsed_args)
        }
    }
}

pub fn replace_text_in_file(path: &PathBuf, pattern: &str, replacement: &str) -> Result<(), std::io::Error> {
    let text = std::fs::read_to_string(path).unwrap();

    let text = text.replace(pattern, replacement);
    std::fs::write(path, &text)?;
    Ok(())
}

fn push_element<T: std::iter::Extend<T>>(vector: &mut Vec<T>, element: T) {
    vector.extend([ element ]);
}
