pub mod args;
pub mod logger;
pub mod processor;
pub mod progress;

use std::process::exit;
use std::{fs, io::ErrorKind};
#[cfg(feature = "ui")]
use tauri::{AppHandle, Emitter};
pub use args::CmdArgs;
pub use logger::Logger;
pub use processor::Processor;
pub use progress::Progress;

pub fn load_paths(
    cmd_args: &CmdArgs,
    #[cfg(feature = "ui")]
    app_handle: &AppHandle
) -> Vec<String> {
    if let Some(input_file_path) = &cmd_args.file_list {
        match fs::read_to_string(input_file_path) {
            Ok(contents) => contents
                .trim()
                .split('\n')
                .map(|s| s.trim().to_string())
                .collect(),
            Err(err) => {
                // blocks with without tauri code can't be reached from ui code because the file explorer
                // prevents these
                match err.kind() {
                    ErrorKind::NotFound => {
                        eprintln!("No file found at {input_file_path}.");
                        exit(1);
                    }
                    ErrorKind::PermissionDenied => {
                        let error = format!("Permission denied when reading file {input_file_path}.");
                        eprintln!("{}", error);
                        #[cfg(not(feature = "ui"))]
                        exit(1);

                        #[cfg(feature = "ui")]
                        {
                            let _ = app_handle.emit("file-list-error", error);
                            vec![]
                        }
                    }
                    ErrorKind::InvalidData => {
                        let error = format!("The contents of {input_file_path} contain invalid data. Please make sure it is encoded as UTF-8.");
                        eprintln!("{}", error);
                        #[cfg(not(feature = "ui"))]
                        exit(1);

                        #[cfg(feature = "ui")]
                        {
                            let _ = app_handle.emit("file-list-error", error);
                            vec![]
                        }
                    }
                    ErrorKind::IsADirectory => {
                        eprintln!("The path {input_file_path} is a directory.");
                        exit(1);
                    }
                    _ => {
                        let error = format!("An error has occurred reading the file at path {input_file_path}: {:?}.", err);
                        eprintln!("{}", err);
                        #[cfg(not(feature = "ui"))]
                        exit(1);

                        #[cfg(feature = "ui")]
                        {
                            let _ = app_handle.emit("file-list-error", error);
                            vec![]
                        }
                    }
                }
            }
        }
    } else {
        cmd_args.input.clone().unwrap()
    }
}
