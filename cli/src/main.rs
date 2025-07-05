use clap::Parser;
use ffzap_shared::{CmdArgs, Logger, Processor, Progress};
use std::fs;
use std::io::ErrorKind;
use std::process::exit;
use std::sync::Arc;

fn main() {
    let cmd_args = CmdArgs::parse();

    if cmd_args.eta {
        println!("Warning: ETA is a highly experimental feature and prone to absurd estimations. If your encoding process has long pauses in-between each processed file, you WILL experience incredibly inaccurate estimations!");
        println!("This is due to unwanted behaviour in one of ffzap's dependencies and cannot be fixed by ffzap.");
    }

    let paths = load_paths(&cmd_args);
    let progress = Arc::new(Progress::new(paths.len(), cmd_args.eta));
    let logger = Arc::new(Logger::new(Arc::clone(&progress)));
    let processor = Processor::new(Arc::clone(&logger), Arc::clone(&progress));

    processor.process_files(
        paths,
        cmd_args.thread_count,
        cmd_args.ffmpeg_options,
        cmd_args.output,
        cmd_args.overwrite,
        cmd_args.verbose,
        cmd_args.delete,
    );

    let final_output = format!(
        "{} out of {} files have been successful. A detailed log has been written to {}",
        progress.value(),
        progress.len(),
        logger.get_log_path()
    );
    println!("{final_output}");

    let failed_paths = processor.get_failed_paths();
    logger.append_failed_paths_to_log(&std::sync::Mutex::new(failed_paths.clone()).lock().unwrap());
    
    if cmd_args.verbose && !failed_paths.is_empty() {
        println!("\nThe following files were not processed due to the errors above:");
        for path in failed_paths.iter() {
            println!("{path}");
        }
    }
}

fn load_paths(cmd_args: &CmdArgs) -> Vec<String> {
    if let Some(input_file_path) = &cmd_args.file_list {
        match fs::read_to_string(input_file_path) {
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
        cmd_args.input.clone().unwrap()
    }
} 