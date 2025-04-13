mod job;
mod logger;
mod progress;

use crate::job::CmdArgs;
use clap::Parser;
use iced::widget::{button, column, text_editor, text_input, Column, checkbox};
use iced_aw::widget::{number_input};
use std::thread;

#[derive(Debug, Clone)]
pub enum Message {
    Start,
    EditInputFiles(String),
    EditFileList(String),
    EditFfmpegOption(text_editor::Action),
    EditOutputPattern(String),
    EditThreadAmount(u16),
    ToggleDelete(bool),
    ToggleOverwrite(bool),
}

struct Gui {
    input_files: String,
    file_list: String,
    output_pattern: String,
    threads: u16,
    ffmpeg_options: text_editor::Content,
    overwrite: bool,
    delete: bool,
    eta: bool,
}

impl Gui {
    pub fn view(&self) -> Column<Message> {
        let mut input_files = text_input("Enter the file paths of the files you want to process, seperated by space",&self.input_files);
        let mut file_list = text_input("Enter path to file list", &self.file_list);
        
        let ffmpeg_options = text_editor(&self.ffmpeg_options)
            .placeholder("Enter the ffmpeg_options you want to process")
            .on_action(Message::EditFfmpegOption);
        
        let output_pattern = text_input("Enter your output pattern", &self.output_pattern)
            .on_input(Message::EditOutputPattern);
        
        let thread_input = number_input(&self.threads, 1..=u16::MAX, Message::EditThreadAmount).on_input(Message::EditThreadAmount);
        let delete = checkbox("Delete source files after job successfully finished", self.delete).on_toggle(Message::ToggleDelete);
        let overwrite = checkbox("Overwrite target if it already exists", self.overwrite).on_toggle(Message::ToggleOverwrite);
        
        if self.input_files.is_empty() {
            file_list = file_list.on_input(Message::EditFileList);
        }
        if self.file_list.is_empty() {
            input_files = input_files.on_input(Message::EditInputFiles);
        }
        
        column![
            input_files,
            file_list,
            ffmpeg_options,
            output_pattern,
            thread_input,
            delete,
            overwrite,
            button("Start").on_press(Message::Start),
        ]
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::EditInputFiles(files) => {
                self.input_files = files;
            }
            Message::EditFileList(list) => {
                self.file_list = list;
            }
            Message::EditFfmpegOption(action) => {
                self.ffmpeg_options.perform(action);
            }
            Message::EditOutputPattern(pattern) => {
                self.output_pattern = pattern;
            }
            Message::EditThreadAmount(amount_of_threads) => {
                self.threads = amount_of_threads;
            }
            Message::ToggleDelete(delete) => {
                self.delete = delete;
            }
            Message::ToggleOverwrite(overwrite) => {
                self.overwrite = overwrite;
            }
            Message::Start => {
                let input = if self.input_files.trim().is_empty() {
                    None
                } else {
                    Some(
                        self.input_files
                            .split(' ')
                            .map(|s| s.trim().to_string())
                            .collect::<Vec<String>>(),
                    )
                };
                let file_list = if self.file_list.trim().is_empty() {
                    None
                } else {
                    Some(self.file_list.clone())
                };
                let ffmpeg_options = if self.ffmpeg_options.text().trim().is_empty() {
                    None
                } else {
                    Some(String::from(self.ffmpeg_options.text().trim()))
                };
                let output_pattern = self.output_pattern.clone();
                let thread_count = self.threads;
                let delete = self.delete;
                let overwrite = self.overwrite;

                // run in new thread to stop the UI from being unresponsive everytime a ffmpeg process finishes
                thread::spawn(move || {
                    job::run_job(CmdArgs {
                        thread_count,
                        ffmpeg_options,
                        input,
                        file_list,
                        overwrite,
                        verbose: true,
                        delete,
                        eta: false,
                        output: Some(output_pattern),
                        gui: false,
                    })
                });
            }
        }
    }
    
    fn default_helper() -> Self {
        Gui {
            input_files: Default::default(),
            file_list: Default::default(),
            output_pattern: Default::default(),
            // we'll overwrite that in the actual default function
            threads: Default::default(),
            ffmpeg_options: Default::default(),
            overwrite: Default::default(),
            delete: Default::default(),
            eta: Default::default(),
        }
    }
}

/**
* I couldn't figure out a nicer way to make iced create an instance of Gui with default values while not defaulting
* to 0 for the thread count
*/
impl Default for Gui {
    fn default() -> Self {
        let mut state = Gui::default_helper();
        state.threads = 2;
        state
    }
}

fn main() {
    let cmd_args = CmdArgs::parse();

    if cmd_args.gui {
        iced::run("ffzap", Gui::update, Gui::view).expect("Could not start GUI");
    }
}
