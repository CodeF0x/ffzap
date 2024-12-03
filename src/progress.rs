use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::time::Duration;

pub(crate) struct Progress {
    multi_progress: MultiProgress,
    progress: ProgressBar,
}

impl Progress {
    pub(crate) fn new(length: usize) -> Self {
        let multi = MultiProgress::new();
        let progress = multi.add(ProgressBar::new(length as u64));
        progress.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len}")
                .expect("Failed to set progress style")
                .progress_chars("#>-"),
        );

        Progress {
            multi_progress: multi,
            progress,
        }
    }

    pub(crate) fn inc(&self, amount: u64) {
        self.progress.inc(amount);
    }

    pub(crate) fn start_stick(&self, millis: u32) {
        self.progress.enable_steady_tick(Duration::new(0, millis));
    }

    pub(crate) fn println(&self, message: String) {
        self.multi_progress.println(message).unwrap();
    }

    pub(crate) fn finish(&self) {
        self.progress.finish();
    }
}
