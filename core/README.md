This is the shared core for both the ffzap [cli](https://crates.io/crates/ffzap) and ffzap [https://github.com/CodeF0x/ffzap/blob/tauri/ui/README.md]. It handles the actual file processing with ffmpeg.

It's available on crates.io as [ffzap_core](https://crates.io/crates/ffzap_core) and can be compiled with or without tauri support (for sending events to the ui crate).

### Features

- ui (compiles tauri specific code)
- default (leaves out tauri specific code)

### Tauri Events

- `log-update-info` (sends a single log line to the ui; payload: string)
- `log-update-error` (sends a single log line to the ui; payload: string)
- `progress-update` (sends the current progress bar state; payload: u64)
- `general-ffmpeg-error` (sends an error that something went wrong running a ffmpeg process; payload: string)

### Usage

Consider this minimal example without the ui feature:

```rust
use ffzap_core::{CmdArgs, Processor, Logger, Progress};
use std::sync::Arc;

fn main() {
    // Create processor arguments (usually they come form the terminal or some GUI)
    let input = Some(vec!["input1.mp4".to_string(), "input2.mp4".to_string()]);
    let cmd_args: CmdArgs = CmdArgs {
        thread_count: 2,
        ffmpeg_options: Some("-c:v libx264 -c:a aac".to_string()),
        input,
        file_list: None,
        overwrite: false,
        verbose: true,
        delete: false,
        eta: false,
        output: "output/{{name}}_processed.{{ext}}".to_string(),
    };

    // Create progress tracker
    let progress: Arc<Progress> = Arc::new(Progress::new(input.unwrap().len(), cmd_args.eta));

    // Create logger (without UI features)
    let logger: Arc<Logger> = Arc::new(Logger::new(progress.clone()));

    // Create processor
    let processor: Processor = Processor::new(logger.clone(), progress.clone());

    // Process the files
    processor.process_files(
        paths,
        cmd_args.thread_count,
        cmd_args.ffmpeg_options,
        cmd_args.output,
        cmd_args.overwrite,
        cmd_args.verbose,
        cmd_args.delete,
    );

    // Get results
    let successful_files: u64 = progress.value();
    let total_files: u64 = progress.len();
    let log_path: std::path::Display = logger.get_log_path();

    println!("{} out of {} files have been successful. A detailed log has been written to {}",
             successful_files, total_files, log_path);

    // Handle failed paths
    let failed_paths: Vec<String> = processor.get_failed_paths();
    logger.append_failed_paths_to_log(&std::sync::Mutex::new(failed_paths.clone()).lock().unwrap());

    if cmd_args.verbose && !failed_paths.is_empty() {
        println!("\nThe following files were not processed due to the errors above:");
        for path in failed_paths.iter() {
            println!("{path}");
        }
    }
}
```

If this is run in a terminal context, a progress bar is automatically shown and updated and verbose output is shown if `verbose` is set to `true`.
