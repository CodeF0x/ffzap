mod logger;
mod progress;

use crate::logger::Logger;
use crate::progress::Progress;
use clap::Parser;
use std::ffi::OsStr;
use std::fs::{create_dir_all, remove_file};
use std::io::ErrorKind;
use std::path::Path;
use std::process::{exit, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::{fs, thread};

#[derive(Parser, Debug, Clone)]
#[command(version, about)]
struct CmdArgs {
    /// The amount of threads you want to utilize. most systems can handle 2. Go higher if you have a powerful computer. Default is 2. Can't be lower than 1
    #[arg(short, long, default_value_t = 2, value_parser = clap::value_parser!(u16).range(1..))]
    thread_count: u16,

    /// Options you want to pass to ffmpeg. For the output file name, use --output
    #[arg(short, long, allow_hyphen_values = true)]
    ffmpeg_options: Option<String>,

    /// The files you want to process.
    #[arg(short, long, num_args = 1.., required_unless_present = "file_list", conflicts_with = "file_list")]
    input: Option<Vec<String>>,

    /// Path to a file containing paths to process. One path per line
    #[arg(long, required_unless_present = "input", conflicts_with = "input")]
    file_list: Option<String>,

    /// If ffmpeg should overwrite files if they already exist. Default is false
    #[arg(long, default_value_t = false)]
    overwrite: bool,

    /// If verbose logs should be shown while ffzap is running
    #[arg(long, default_value_t = false)]
    verbose: bool,

    /// Delete the source file after it was successfully processed. If the process fails, the file is kept.
    #[arg(long, default_value_t = false)]
    delete: bool,

    /// Displays the current eta in the progressbar
    #[arg(long, default_value_t = false)]
    eta: bool,

    /// Specify the output file pattern. Use placeholders to customize file paths:
    ///
    /// {{dir}}  - Entire specified file path, e.g. ./path/to/file.txt -> ?./path/to/
    ///
    /// {{name}} - Original file's name (without extension)
    ///
    /// {{ext}}  - Original file's extension
    ///
    /// Example: /destination/{{dir}}/{{name}}_transcoded.{{ext}}
    ///
    /// Outputs the file in /destination, mirroring the original structure and keeping both the file extension and name, while adding _transcoded to the name.
    #[arg(short, long)]
    output: String,
    // {{ext}} -> extension, {{name}} filename without extension, {{dir}} -> directory structure from starting point to file, {{parent}} -> parent directory of starting point
}

fn main() {
    let cmd_args = CmdArgs::parse();

    if cmd_args.eta {
        println!("Warning: ETA is a highly experimental feature and prone to absurd estimations. If your encoding process has long pauses in-between each processed file, you WILL experience incredibly inaccurate estimations!");
        println!("This is due to unwanted behaviour in one of ffzap's dependencies and cannot be fixed by ffzap.");
    }

    let paths: Vec<String>;
    if let Some(input_file_path) = cmd_args.file_list {
        paths = match fs::read_to_string(&input_file_path) {
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
        paths = cmd_args.input.unwrap();
    }

    let paths = Arc::new(Mutex::new(paths));
    let failed_paths: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    let progress = Arc::new(Progress::new(paths.lock().unwrap().len(), cmd_args.eta));
    let logger = Arc::new(Logger::new(Arc::clone(&progress)));
    let mut thread_handles = vec![];

    progress.start_stick(1000);

    for thread in 0..cmd_args.thread_count {
        let paths = Arc::clone(&paths);
        let failed_paths = Arc::clone(&failed_paths);
        let progress = Arc::clone(&progress);
        let logger = Arc::clone(&logger);
        let verbose = cmd_args.verbose;
        let ffmpeg_options = cmd_args.ffmpeg_options.clone();
        let output = cmd_args.output.clone();

        let handle = thread::spawn(move || loop {
            let path_to_process = {
                let mut queue = paths.lock().unwrap();
                queue.pop()
            };

            match path_to_process {
                Some(path) => {
                    let path = Path::new(&path);

                    if !path.is_file() {
                        logger.log_error(format!(
                            "{} doesn't appear to be a file, ignoring. Continuing with next task if there's more to do...",
                            path.display()
                        ), thread, verbose);
                        continue;
                    }

                    logger.log_info(format!("Processing {}", path.display()), thread, verbose);

                    let split_options = match &ffmpeg_options {
                        Some(options) => options.split(' ').collect::<Vec<&str>>(),
                        None => vec![],
                    };

                    let mut final_file_name =
                        output.replace("{{ext}}", path.extension().unwrap().to_str().unwrap());
                    final_file_name = final_file_name
                        .replace("{{name}}", &path.file_stem().unwrap().to_str().unwrap());
                    final_file_name = final_file_name.replace(
                        "{{dir}}",
                        &path.parent().unwrap_or(Path::new("")).to_str().unwrap(),
                    );
                    final_file_name = final_file_name.replace(
                        "{{parent}}",
                        &path
                            .parent()
                            .unwrap_or(Path::new(""))
                            .file_name()
                            .unwrap_or(OsStr::new(""))
                            .to_str()
                            .unwrap_or(""),
                    );
                    let final_path_parent = Path::new(&final_file_name).parent().unwrap();

                    if !final_path_parent.exists() {
                        match create_dir_all(final_path_parent) {
                            Ok(_) => {}
                            Err(err) => {
                                logger.log_error(
                                    format!(
                                        "Could not create directory structure for file {}",
                                        final_file_name
                                    ),
                                    thread,
                                    verbose,
                                );
                                logger.log_error(format!("{}", err), thread, verbose);
                            }
                        }
                    }

                    let overwrite = match cmd_args.overwrite {
                        true => "-y",
                        false => "-n",
                    };

                    if let Ok(output) = Command::new("ffmpeg")
                        .args(["-i", path.to_str().unwrap()])
                        .args(split_options)
                        .arg(&final_file_name)
                        .arg(overwrite)
                        .stdout(Stdio::null())
                        .stderr(Stdio::piped())
                        .output()
                    {
                        if output.status.success() {
                            logger.log_info(
                                format!("Success, saving to {final_file_name}"),
                                thread,
                                verbose,
                            );

                            if cmd_args.delete {
                                match remove_file(path) {
                                    Ok(_) => logger.log_info(
                                        format!("Removed {}", path.display()),
                                        thread,
                                        verbose,
                                    ),
                                    Err(err) => match err.kind() {
                                        ErrorKind::PermissionDenied => logger.log_error(
                                            format!("Permission denied when trying to delete file {}", path.display()),
                                            thread,
                                            verbose,
                                        ),
                                        _ => logger.log_error(
                                            format!("An unknown error occurred when trying to delete file {}", path.display()),
                                            thread,
                                            verbose
                                        )
                                    },
                                }
                            }

                            progress.inc(1);
                        } else {
                            logger.log_error(
                                format!(
                                    "Error processing file {}. Error is: {}",
                                    path.display(),
                                    String::from_utf8_lossy(&output.stderr)
                                ),
                                thread,
                                verbose,
                            );
                            if cmd_args.delete {
                                logger.log_info(
                                    "Keeping the file due to the error above".to_string(),
                                    thread,
                                    verbose,
                                )
                            }
                            logger.log_info(
                                "Continuing with next task if there's more to do...".to_string(),
                                thread,
                                verbose,
                            );

                            failed_paths
                                .lock()
                                .unwrap()
                                .push(path.display().to_string());
                        }
                    } else {
                        eprintln!("[THREAD {thread}] -- There was an error running ffmpeg. Please check if it's correctly installed and working as intended.");
                    }
                }
                None => {
                    break;
                }
            }
        });

        thread_handles.push(handle);
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }

    progress.finish();

    let final_output = format!(
        "{} out of {} files have been successful. A detailed log has been written to {}",
        progress.value(),
        progress.len(),
        logger.get_log_path()
    );
    println!("{final_output}");

    let failed_paths = failed_paths.lock().unwrap();

    logger.append_failed_paths_to_log(&failed_paths);
    if cmd_args.verbose && failed_paths.len() > 0 {
        println!("\nThe following files were not processed due to the errors above:");
        for path in failed_paths.iter() {
            println!("{path}");
        }
    }
}
