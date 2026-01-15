pub mod args;
pub mod logger;
pub mod processor;
pub mod progress;

pub use args::CmdArgs;
pub use logger::Logger;
pub use processor::Processor;
pub use progress::Progress;
use std::path::Path;
use std::process::exit;
use std::{fs, io::ErrorKind};
#[cfg(feature = "ui")]
use tauri::{AppHandle, Emitter};
use walkdir::WalkDir;

pub fn load_paths(
    cmd_args: &CmdArgs,
    #[cfg(feature = "ui")] app_handle: &AppHandle,
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
                        let error =
                            format!("Permission denied when reading file {input_file_path}.");
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
        let paths = cmd_args.input.clone().unwrap();
        let mut files: Vec<String> = vec![];

        for p in paths {
            let path = Path::new(&p);
            if path.is_file() {
                files.push(p);
            } else if path.is_dir() {
                for entry in WalkDir::new(path)
                    .follow_links(false)
                    .into_iter()
                    .filter_entry(|e| !e.path_is_symlink())
                {
                    match entry {
                        Ok(entry) => {
                            if entry.file_type().is_file() {
                                files.push(entry.path().to_str().unwrap().to_string());
                            }
                        }
                        Err(err) => {
                            let error = format!(
                                "Failed to read directory {}: {}",
                                &err.path().unwrap().display(),
                                err
                            );

                            #[cfg(not(feature = "ui"))]
                            eprintln!("{}", error);

                            #[cfg(feature = "ui")]
                            let _ = app_handle.emit("file-list-error", error);
                        }
                    }
                }
            }
        }

        files
    }
}
