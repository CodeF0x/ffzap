use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::time::Duration;

pub(crate) struct Progress {
    multi_progress: MultiProgress,
    progress: ProgressBar,
}

impl Progress {
    pub(crate) fn new(length: usize, eta: bool) -> Self {
        let multi = MultiProgress::new();
        let progress = multi.add(ProgressBar::new(length as u64));
        let progress_bar_template = if eta { "{spinner:.green} [{elapsed_precise} - ETA: {eta_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)" } else { "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)" };
        progress.set_style(
            ProgressStyle::default_bar()
                .template(progress_bar_template)
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
        self.progress.abandon();
    }

    pub(crate) fn len(&self) -> u64 {
        self.progress.length().unwrap()
    }

    pub(crate) fn value(&self) -> u64 {
        self.progress.position()
    }
}
