use std::{sync::Arc, thread};

use ffzap_shared::{load_paths, CmdArgs, Logger, Processor, Progress};
use tauri::{AppHandle, Emitter};

#[tauri::command]
fn start_job(app: AppHandle, options: String) {
    let args = serde_json::from_str::<CmdArgs>(&options).unwrap();

    let app_handle = app.clone();
    let paths = load_paths(&args);
    let progress = Arc::new(Progress::new(paths.len(), args.eta));
    let logger = Arc::new(Logger::new(Arc::clone(&progress), app_handle.clone()));
    let processor = Processor::new(Arc::clone(&logger), Arc::clone(&progress));

    let _ = app_handle.emit("update-total-file-count", paths.len());

    thread::spawn(move || {
        processor.process_files(
            paths,
            args.thread_count,
            args.ffmpeg_options,
            args.output,
            args.overwrite,
            args.verbose,
            args.delete,
            app_handle.clone(),
        );

        let _ = app_handle.emit(
            "job-finished",
            (
                logger.get_log_path().to_string(),
                progress.value(),
                processor.get_failed_paths(),
            ),
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
