use std::{sync::Arc, thread};

use ffzap_shared::{load_paths, CmdArgs, Logger, Processor, Progress};

#[tauri::command]
fn start_job(options: String) {
    let args = serde_json::from_str::<CmdArgs>(&options).unwrap();
    println!("Encoding has started with options: {:?}", args);

    let paths = load_paths(&args);
    let progress = Arc::new(Progress::new(paths.len(), args.eta));
    let logger = Arc::new(Logger::new(Arc::clone(&progress)));
    let processor = Processor::new(Arc::clone(&logger), Arc::clone(&progress));

    thread::spawn(move || {
        processor.process_files(
            paths,
            args.thread_count,
            args.ffmpeg_options,
            args.output,
            args.overwrite,
            args.verbose,
            args.delete,
        );
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_job])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
