use crate::{Logger, Progress};
use std::ffi::OsStr;
use std::fs::{create_dir_all, remove_file};
use std::io::ErrorKind;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
#[cfg(feature = "ui")]
use tauri::AppHandle;

pub struct Processor {
    logger: Arc<Logger>,
    progress: Arc<Progress>,
    failed_paths: Arc<Mutex<Vec<String>>>,
}

impl Processor {
    pub fn new(logger: Arc<Logger>, progress: Arc<Progress>) -> Self {
        Processor {
            logger,
            progress,
            failed_paths: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn process_files(
        &self,
        paths: Vec<String>,
        thread_count: u16,
        ffmpeg_options: Option<String>,
        output_pattern: String,
        overwrite: bool,
        verbose: bool,
        delete: bool,
        #[cfg(feature = "ui")] app_handle: AppHandle,
    ) {
        let paths = Arc::new(Mutex::new(paths));
        let mut thread_handles = vec![];

        self.progress.start_stick(1000);

        for thread in 0..thread_count {
            let paths = Arc::clone(&paths);
            let failed_paths = Arc::clone(&self.failed_paths);
            let progress = Arc::clone(&self.progress);
            let logger = Arc::clone(&self.logger);
            let ffmpeg_options = ffmpeg_options.clone();
            let output_pattern = output_pattern.clone();
            #[cfg(feature = "ui")]
            let app_handle = app_handle.clone();

            let handle = thread::spawn(move || loop {
                let path_to_process = {
                    let mut queue = paths.lock().unwrap();
                    queue.pop()
                };

                match path_to_process {
                    Some(path) => {
                        let path = Path::new(&path);

                        if !path.is_file() {
                            logger.log_error(
                                format!(
                                    "{} doesn't appear to be a file, ignoring. Continuing with next task if there's more to do...",
                                    path.display()
                                ),
                                thread,
                                verbose,
                            );
                            continue;
                        }

                        logger.log_info(format!("Processing {}", path.display()), thread, verbose);

                        let split_options = match &ffmpeg_options {
                            Some(options) => options.split(' ').collect::<Vec<&str>>(),
                            None => vec![],
                        };

                        let final_file_name = Self::build_output_path(&path, &output_pattern);

                        if Path::new(&final_file_name).exists() && !overwrite {
                            logger.log_error(
                                format!("File {final_file_name} already exists and --overwrite is set to false. Continuing with next task if there is more to do..."),
                                thread,
                                verbose
                            );
                            failed_paths.lock().unwrap().push(final_file_name);
                            continue;
                        }

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

                        let mut command = Command::new("ffmpeg");
                        command.arg("-i").arg(path.to_str().unwrap());
                        command.args(split_options);
                        command.arg(&final_file_name);
                        command.stdout(Stdio::null());
                        command.stderr(Stdio::piped());

                        if overwrite {
                            command.arg("-y");
                        }

                        if let Ok(output) = command.output() {
                            if output.status.success() {
                                logger.log_info(
                                    format!("Success, saving to {final_file_name}"),
                                    thread,
                                    verbose,
                                );

                                if delete {
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
                                #[cfg(feature = "ui")]
                                {
                                    use tauri::Emitter;

                                    let done = progress.value();
                                    let _ = app_handle.emit("progress-update", done);
                                }
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
                                if delete {
                                    logger.log_info(
                                        "Keeping the file due to the error above".to_string(),
                                        thread,
                                        verbose,
                                    )
                                }
                                logger.log_info(
                                    "Continuing with next task if there's more to do..."
                                        .to_string(),
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

        self.progress.finish();
    }

    pub fn get_failed_paths(&self) -> Vec<String> {
        self.failed_paths.lock().unwrap().clone()
    }

    fn build_output_path(path: &Path, output_pattern: &str) -> String {
        let mut final_file_name =
            output_pattern.replace("{{ext}}", path.extension().unwrap().to_str().unwrap());
        final_file_name =
            final_file_name.replace("{{name}}", &path.file_stem().unwrap().to_str().unwrap());
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
        final_file_name
    }
}
