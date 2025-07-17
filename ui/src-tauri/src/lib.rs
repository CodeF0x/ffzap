use ffzap_shared::CmdArgs;

#[tauri::command]
fn start_job(options: String) {
    let args = serde_json::from_str::<CmdArgs>(&options);
    println!("Encoding has started with options: {:?}", args.unwrap());
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
