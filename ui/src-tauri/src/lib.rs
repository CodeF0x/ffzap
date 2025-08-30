use std::{sync::Arc, thread};

use ffzap_core::{load_paths, CmdArgs, Logger, Processor, Progress};
use tauri::{AppHandle, Emitter, WindowEvent};

#[tauri::command]
fn start_job(app: AppHandle, options: String) {
    let args = serde_json::from_str::<CmdArgs>(&options).unwrap();

    let app_handle = app.clone();
    let paths = load_paths(&args, &app_handle);
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
        .invoke_handler(tauri::generate_handler![start_job, stop_jobs])
        .on_window_event(|_window, event| {
            // Windows does not automatically tear down spawned child processes when closing the Tauri main window
            // This is relevant if the window is closed while there's still ffmpeg processes running
            #[cfg(target_os = "windows")]
            if let WindowEvent::CloseRequested { .. } = event {
                kill_ffmpeg_processes();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn stop_jobs(app_handle: AppHandle) {
    thread::spawn(move || {
        kill_ffmpeg_processes();
        let _ = app_handle.emit("job-finished", ());
    });
}

fn kill_ffmpeg_processes() {
    use sysinfo::{Process, System};

    let sys = System::new_all();
    let my_pid = std::process::id();

    let to_kill: Vec<&Process> = sys
        .processes()
        .values()
        .filter(|p| {
            p.parent().map(|pp| pp.as_u32()) == Some(my_pid)
                && p.name().to_string_lossy().to_lowercase().contains("ffmpeg")
        })
        .collect();

    for process in to_kill {
        process.kill();
    }
}
