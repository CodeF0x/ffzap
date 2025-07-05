use crate::progress::Progress;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Display, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard};

pub struct Logger {
    progress: Arc<Progress>,
    log_file: Arc<Mutex<File>>,
    log_path: PathBuf,
}

impl Logger {
    pub fn new(progress: Arc<Progress>) -> Self {
        let path_file_tuple = Self::setup_log_dir_and_create_log_file();

        let log_path = path_file_tuple.0;
        let log_file = path_file_tuple.1;

        Logger {
            log_path,
            log_file,
            progress,
        }
    }

    pub fn log_info(&self, line: String, thread: u16, print: bool) {
        let line = format!("[INFO in THREAD {thread}] -- {line}\n");

        self.write_to_log(&line);

        if print {
            self.print(line);
        }
    }

    pub fn log_error(&self, line: String, thread: u16, print: bool) {
        let line = format!("[ERROR in THREAD {thread} -- {line}\n");

        self.write_to_log(&line);

        if print {
            self.print(line);
        }
    }

    pub fn append_failed_paths_to_log(&self, paths: &MutexGuard<Vec<String>>) {
        if paths.len() == 0 {
            return;
        }

        let static_line = "\nThe following files were not processed due to the errors above:";

        let paths_lines = paths.join("\n");

        let to_write = format!("{}\n{}", static_line, paths_lines);

        self.write_to_log(&to_write);
    }

    pub fn get_log_path(&self) -> Display {
        self.log_path.display()
    }

    fn setup_log_dir_and_create_log_file() -> (PathBuf, Arc<Mutex<File>>) {
        let log_path;
        let app_name = "ffzap";

        #[cfg(target_os = "windows")]
        {
            log_path = dirs::data_local_dir()
                .unwrap_or(dirs::home_dir().unwrap().join("AppData/Local"))
                .join(app_name)
                .join("logs")
        }

        #[cfg(target_os = "macos")]
        {
            log_path = dirs::home_dir()
                .unwrap_or(PathBuf::from("/Users/Shared"))
                .join("Library/Logs")
                .join(app_name)
        }

        #[cfg(target_os = "linux")]
        {
            log_path = dirs::cache_dir()
                .unwrap_or(dirs::home_dir().unwrap().join(".cache"))
                .join(app_name)
                .join("logs")
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            log_path = dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(app_name)
                .join("logs")
        }

        fs::create_dir_all(&log_path).unwrap();

        let locale_time = chrono::Local::now().format("%d-%m-%YT%H-%M-%S");
        let mut current_log = log_path.join(locale_time.to_string());
        current_log.set_extension("log");

        (
            current_log.clone(),
            Arc::new(Mutex::new(File::create(&current_log).unwrap())),
        )
    }

    fn write_to_log(&self, line: &str) {
        self.log_file
            .lock()
            .unwrap()
            .write_all(line.as_bytes())
            .unwrap();
    }

    fn print(&self, line: String) {
        self.progress.println(line);
    }
} 