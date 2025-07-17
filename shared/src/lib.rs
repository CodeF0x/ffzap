pub mod args;
pub mod logger;
pub mod processor;
pub mod progress;

use std::process::exit;
use std::{fs, io::ErrorKind};

pub use args::CmdArgs;
pub use logger::Logger;
pub use processor::Processor;
pub use progress::Progress;

pub fn load_paths(cmd_args: &CmdArgs) -> Vec<String> {
    if let Some(input_file_path) = &cmd_args.file_list {
        match fs::read_to_string(input_file_path) {
            Ok(contents) => contents
                .trim()
                .split('\n')
                .map(|s| s.trim().to_string())
                .collect(),
            Err(err) => {
                match err.kind() {
                    ErrorKind::NotFound => {
                        eprintln!("No file found at {input_file_path}.");
                        exit(1);
                    }
                    ErrorKind::PermissionDenied => {
                        eprintln!("Permission denied when reading file {input_file_path}.");
                        exit(1);
                    }
                    ErrorKind::InvalidData => {
                        eprintln!("The contents of {input_file_path} contain invalid data. Please make sure it is encoded as UTF-8.");
                        exit(1);
                    }
                    ErrorKind::IsADirectory => {
                        eprintln!("The path {input_file_path} is a directory.");
                        exit(1);
                    }
                    _ => {
                        eprintln!("An error has occurred reading the file at path {input_file_path}: {:?}.", err);
                        exit(1);
                    }
                }
            }
        }
    } else {
        cmd_args.input.clone().unwrap()
    }
}
