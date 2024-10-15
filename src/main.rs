use clap::Parser;
use glob::glob;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::{io, thread};

#[derive(Parser, Debug, Clone)]
#[command(version, about)]
struct CmdArgs {
    #[arg(short, long, default_value_t = 2)]
    thread_count: u8,

    #[arg(short, long, allow_hyphen_values = true)]
    ffmpeg_options: String,

    #[arg(short, long)]
    input_directory: String,

    #[arg(short, long)]
    output_directory: Option<String>,

    #[arg(short, long)]
    name_scheme: Option<String>,
}

fn main() {
    let cmd_args = CmdArgs::parse();

    let paths = Arc::new(Mutex::new(match glob(&cmd_args.input_directory) {
        Ok(paths) => paths.filter_map(Result::ok).collect::<Vec<PathBuf>>(),
        Err(err) => {
            eprintln!("{}", err.msg);
            std::process::exit(1);
        }
    }));

    let mut thread_handles = vec![];

    for _ in 0..cmd_args.thread_count {
        let paths: Arc<Mutex<Vec<PathBuf>>> = Arc::clone(&paths);
        let args = cmd_args.clone();

        let handle = thread::spawn(move || loop {
            let path_to_process = {
                let mut queue = paths.lock().unwrap();

                queue.pop()
            };

            match path_to_process {
                Some(path) => {
                    println!("Processing {}", path.display());
                    let split_options = &mut args.ffmpeg_options.split(' ').collect::<Vec<&str>>();
                    let file_extension_from_options = split_options[split_options.len() - 1]
                        .split('.')
                        .last()
                        .unwrap();
                    let mut file_name = format!(
                        "{}_transcoded.{}",
                        path.file_stem().unwrap().to_str().unwrap(),
                        file_extension_from_options
                    );
                    // remove both file stem and file format to just get the options
                    split_options.pop();

                    if let Some(_scheme) = &args.name_scheme {
                        file_name = String::from("");
                    }

                    let output = Command::new("ffmpeg")
                        .args(["-i", path.to_str().unwrap()])
                        .args(split_options)
                        .arg(file_name)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .output()
                        .unwrap();
                    if !output.status.success() {
                        eprintln!("Error!");
                        eprintln!("Error is: {}", String::from_utf8_lossy(&output.stderr));
                        break;
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
}
