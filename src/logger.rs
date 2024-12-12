use crate::progress::Progress;
use chrono;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;

pub(crate) struct Logger {
    pub(crate) current_log: PathBuf,
    progress: Arc<Progress>,
}

impl Logger {
    pub(crate) fn new(progress: Arc<Progress>) -> Self {
        let app_name = "ffzap";
        let log_path;

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

        Self::setup_log_dir(&log_path);

        let locale_time = chrono::Local::now().format("%d-%m-%Y@%H:%M:%S");
        let mut current_log = log_path.join(locale_time.to_string());
        current_log.set_extension("log");

        Logger {
            current_log,
            progress,
        }
    }

    pub(crate) fn log_info(&self, line: String, thread: u16, print: bool) {
        let line = format!("[INFO in THREAD {thread}] -- {line}");

        let mut log_file = self.get_log_file();

        writeln!(&mut log_file, "{}", line).unwrap();

        if print {
            self.print(line);
        }
    }

    pub(crate) fn log_error(&self, line: String, thread: u16, print: bool) {
        let line = format!("[ERROR in THREAD {thread} -- {line}");

        let mut log_file = self.get_log_file();

        writeln!(&mut log_file, "{}", line).unwrap();

        if print {
            self.print(line);
        }
    }

    fn setup_log_dir(path: &PathBuf) {
        fs::create_dir_all(path).unwrap();
    }

    fn get_log_file(&self) -> File {
        OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.current_log)
            .unwrap()
    }

    fn print(&self, line: String) {
        self.progress.println(line);
    }
}
