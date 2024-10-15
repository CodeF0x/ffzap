use clap::Parser;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
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

fn main() -> io::Result<()> {
    let cmd_args = CmdArgs::parse();

    let paths = Arc::new(Mutex::new(
        read_dir(Path::new(&cmd_args.input_directory))?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<PathBuf>, io::Error>>()?,
    ));

    let mut thread_handles = vec![];

    for _ in 0..cmd_args.thread_count {
        let paths = Arc::clone(&paths);
        let args = cmd_args.clone();

        let handle = thread::spawn(move || loop {
            let path_to_process = {
                let mut queue = paths.lock().unwrap();

                queue.pop()
            };

            match path_to_process {
                Some(path) => {
                    println!("Processing {:?}", path);
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
                        println!("Error!");
                        println!("Error is: {}", String::from_utf8(output.stderr).unwrap());
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

    Ok(())
}
