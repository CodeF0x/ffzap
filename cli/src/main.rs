use clap::Parser;
use ffzap_shared::{load_paths, CmdArgs, Logger, Processor, Progress};
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

    // IDE may throw an error here that only 7 out of 8 arguments are supplied, but that's just the IDE not understanding
    // that ffzap-shared is used without the ui feature and therefore process_files only accepts 7 arguments.
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
