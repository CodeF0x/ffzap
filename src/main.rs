use clap::Parser;
use rand::random;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::{io, thread};

#[derive(Parser, Debug)]
#[command(version, about)]
struct CmdArgs {
    #[arg(short, long, default_value_t = 2)]
    thread_count: u8,

    #[arg(short, long, allow_hyphen_values = true)]
    ffmpeg_options: String,

    #[arg(short, long)]
    source_dir: String,

    #[arg(short, long)]
    output_dir: Option<String>,
}

fn main() -> io::Result<()> {
    let cmd_args = CmdArgs::parse();

    // mtc name
    let paths = Arc::new(Mutex::new(
        read_dir(Path::new(&cmd_args.source_dir))?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<PathBuf>, io::Error>>()?,
    ));

    let mut thread_handles = vec![];

    for _ in 0..cmd_args.thread_count {
        let paths = Arc::clone(&paths);
        let options = cmd_args.ffmpeg_options.clone();

        let handle = thread::spawn(move || loop {
            let path_to_process = {
                let mut queue = paths.lock().unwrap();

                queue.pop()
            };

            match path_to_process {
                Some(path) => {
                    println!("Processing {:?}", path);
                    let output = Command::new("ffmpeg")
                        .args(["-i", path.to_str().unwrap()])
                        .args(options.split(' ').collect::<Vec<&str>>())
                        .arg(format!("{}.mp4", random::<u8>()))
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .output()
                        .unwrap();
                    if !output.stderr.is_empty() {
                        println!("Error!");
                        println!("Error is: {}", String::from_utf8(output.stderr).unwrap());
                    }
                    // let output = Command::new("ffmpeg")
                    //     .args(["--help"])
                    //     .stdout(Stdio::piped())
                    //     .output()
                    //     .unwrap();
                    // println!("{}", String::from_utf8(output.stdout).unwrap());
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
