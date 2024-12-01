use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::ffi::OsStr;
use std::fs::create_dir_all;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Parser, Debug, Clone)]
#[command(version, about)]
struct CmdArgs {
    /// the amount of threads you want to utilize. most systems can handle 2. go higher if you have a powerful computer.
    #[arg(short, long, default_value_t = 2)]
    thread_count: u8,

    /// options you want to pass to ffmpeg. for the output file name, use --output
    #[arg(short, long, allow_hyphen_values = true)]
    ffmpeg_options: String,

    /// the files you want to process.
    #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
    input_directory: Vec<String>,

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

    let progress = ProgressBar::new(cmd_args.input_directory.len() as u64);
    let should_update = Arc::new(Mutex::new(false));
    progress.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len}")
            .expect("Failed to set progress style")
            .progress_chars("#>-"),
    );
    let paths = Arc::new(Mutex::new(cmd_args.input_directory));

    let mut thread_handles = vec![];

    for thread in 0..cmd_args.thread_count {
        let paths = Arc::clone(&paths);
        let should_update = Arc::clone(&should_update);
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
                        eprintln!(
                            "[THREAD {thread}] -- {} doesn't appear to be a file, ignoring. Continuing with next task if there's more to do...",
                            path.to_str().unwrap()
                        );
                        continue;
                    }

                    println!("[THREAD {thread}] -- Processing {}", path.display());
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
                                eprintln!(
                                    "[THREAD {thread}] -- Could not create directory structure for file {}",
                                    final_file_name
                                );
                                eprintln!("{}", err)
                            }
                        }
                    }

                    if let Ok(output) = Command::new("ffmpeg")
                        .args(["-i", path.to_str().unwrap()])
                        .args(split_options)
                        .arg(&final_file_name)
                        .stdout(Stdio::null())
                        .stderr(Stdio::piped())
                        .output()
                    {
                        if output.status.success() {
                            println!("[THREAD {thread}] -- Success, saving to {final_file_name}");
                        } else {
                            eprintln!("[THREAD {thread}] -- Error!");
                            eprintln!(
                                "[THREAD {thread}] -- Error is: {}",
                                String::from_utf8_lossy(&output.stderr)
                            );
                            eprintln!("[THREAD {thread}] -- Continuing with next task if there's more to do...");
                        }
                        let mut update = should_update.lock().unwrap();
                        *update = true;
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

    let _ = thread::spawn(move || {
        while !progress.is_finished() {
            let mut update = should_update.lock().unwrap();
            if *update {
                progress.inc(1);
                *update = false;
            }
            thread::sleep(Duration::new(0, 250));
        }
        progress.finish();
    });

    for handle in thread_handles {
        handle.join().unwrap();
    }
}
