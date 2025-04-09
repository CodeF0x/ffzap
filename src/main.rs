mod job;
mod logger;
mod progress;

use crate::job::CmdArgs;
use clap::Parser;
use iced::widget::{button, column, text_editor, text_input, Column};
use std::thread;

#[derive(Debug, Clone)]
pub enum Message {
    Start,
    EditInputFiles(text_editor::Action),
    EditFfmpegOption(text_editor::Action),
    EditOutputPattern(String),
}

#[derive(Default)]
struct Gui {
    input_files: text_editor::Content,
    file_list: Option<String>,
    output_pattern: String,
    threads: u16,
    ffmpeg_options: text_editor::Content,
    overwrite: bool,
    delete: bool,
    eta: bool,
}

impl Gui {
    pub fn view(&self) -> Column<Message> {
        let input_files = text_editor(&self.input_files)
            .placeholder(
                "Enter the file paths of the files you want to process, seperated by comma",
            )
            .on_action(Message::EditInputFiles);
        let ffmpeg_options = text_editor(&self.ffmpeg_options)
            .placeholder("Enter the ffmpeg_options you want to process")
            .on_action(Message::EditFfmpegOption);
        let output_pattern = text_input("Enter your output pattern", &self.output_pattern)
            .on_input(Message::EditOutputPattern);

        column![
            input_files,
            ffmpeg_options,
            output_pattern,
            button("Start").on_press(Message::Start),
        ]
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::EditInputFiles(action) => {
                self.input_files.perform(action);
            }
            Message::EditFfmpegOption(action) => {
                self.ffmpeg_options.perform(action);
            }
            Message::EditOutputPattern(output) => {
                self.output_pattern = output;
            }
            Message::Start => {
                let input_file_paths = if self.input_files.text().is_empty() {
                    None
                } else {
                    Some(
                        self.input_files
                            .text()
                            .split(' ')
                            .map(|s| s.trim().to_string())
                            .collect::<Vec<String>>(),
                    )
                };
                let ffmpeg_options = if self.ffmpeg_options.text().trim().is_empty() {
                    None
                } else {
                    Some(String::from(self.ffmpeg_options.text().trim()))
                };
                let output_pattern = self.output_pattern.clone();

                // run in new thread to stop the UI from being unresponsive everytime a ffmpeg process finishes
                thread::spawn(move || {
                    job::run_job(CmdArgs {
                        thread_count: 2,
                        ffmpeg_options,
                        input: input_file_paths,
                        file_list: None,
                        overwrite: true,
                        verbose: true,
                        delete: false,
                        eta: false,
                        output: Some(output_pattern),
                        gui: false,
                    })
                });
            }
        }
    }
}

fn main() {
    let cmd_args = CmdArgs::parse();

    if cmd_args.gui {
        iced::run("ffzap", Gui::update, Gui::view).expect("Could not start GUI");
    }
}
