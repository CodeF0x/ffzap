pub mod args;
pub mod logger;
pub mod processor;
pub mod progress;

use std::process::exit;
use std::{fs, io::ErrorKind};
use std::path::Path;
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
        let paths = cmd_args.input.clone().unwrap();
        let mut files: Vec<String> = vec![];

        for p in paths {
            let path = Path::new(&p);
            if path.is_file() {
                files.push(p);
            } else if path.is_dir() {
                let mut stack = vec![path.to_path_buf()];
                while let Some(dir) = stack.pop() {
                    match fs::read_dir(&dir) {
                        Ok(entries) => {
                            for entry_res in entries {
                                if let Ok(entry) = entry_res {
                                    let entry_path = entry.path();
                                    if entry_path.is_dir() {
                                        stack.push(entry_path);
                                    } else if entry_path.is_file() {
                                        if let Some(s) = entry_path.to_str() {
                                            files.push(s.to_string());
                                        }
                                    }
                                }
                            }
                        }
                        Err(err) => {
                            let error = format!("Failed to read directory {}: {}", dir.display(), err);
                            eprintln!("{}", error);

                            #[cfg(feature = "ui")]
                            {
                                let _ = app_handle.emit("file-list-error", error);
                            }

                            continue;
                        }
                    }
                }
            } else {
                let error = format!("The path {} is neither a file nor a directory.", p);
                eprintln!("{}", error);

                #[cfg(feature = "ui")]
                {
                    let _ = app_handle.emit("file-list-error", error);
                }
            }
        }

        files
    }
}
