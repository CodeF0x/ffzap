mod logger;
mod progress;

use crate::logger::Logger;
use crate::progress::Progress;
use clap::Parser;
use std::ffi::OsStr;
use std::fs::create_dir_all;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Parser, Debug, Clone)]
#[command(version, about)]
struct CmdArgs {
    /// the amount of threads you want to utilize. most systems can handle 2. Go higher if you have a powerful computer.
    #[arg(short, long, default_value_t = 2)]
    thread_count: u16,

    /// options you want to pass to ffmpeg. for the output file name, use --output
    #[arg(short, long, allow_hyphen_values = true)]
    ffmpeg_options: String,

    /// the files you want to process.
    #[arg(short, long, num_args = 1..,)]
    input_directory: Vec<String>,

    /// if ffmpeg should overwrite files if they already exist. Default is false
    #[arg(long, default_value_t = false)]
    overwrite: bool,

    /// if verbose logs should be shown while ffzap is running
    #[arg(long, default_value_t = false)]
    verbose: bool,

    /// Specify the output file pattern. Use placeholders to customize file paths:
    ///
    /// {{dir}}  - Original file's directory structure
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

    println!("{:?}", cmd_args.input_directory);

    let progress = Arc::new(Progress::new(cmd_args.input_directory.len()));
    progress.start_stick(500);

    let paths = Arc::new(Mutex::new(cmd_args.input_directory));

    let logger = Arc::new(Logger::new(Arc::clone(&progress)));

    let mut thread_handles = vec![];

    for thread in 0..cmd_args.thread_count {
        let paths = Arc::clone(&paths);
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

                    let split_options = &mut ffmpeg_options.split(' ').collect::<Vec<&str>>();

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
                            progress.inc(1);
                        } else {
                            logger.log_error(
                                format!("Error is: {}", String::from_utf8_lossy(&output.stderr)),
                                thread,
                                verbose,
                            );
                            logger.log_info(
                                "Continuing with next task if there's more to do...".to_string(),
                                thread,
                                verbose,
                            );
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

    println!(
        "{}",
        format!(
            "{} out of {} files have been successful. A detailed log has been written to {}",
            progress.value(),
            progress.len(),
            logger.current_log.display()
        )
    );
}
