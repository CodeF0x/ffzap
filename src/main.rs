mod job;
mod logger;
mod progress;

use crate::job::CmdArgs;
use clap::Parser;
use iced::widget::{button, column, Column};
use std::thread;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Start,
}

#[derive(Default)]
struct Gui {}

impl Gui {
    pub fn view(&self) -> Column<Message> {
        column![
            "Press button to start encoding job",
            button("Start").on_press(Message::Start),
        ]
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Start => {
                // run in new thread to stop the UI from being unresponsive everytime a ffmpeg process finishes
                thread::spawn(move || {
                    job::run_job(CmdArgs {
                        thread_count: 1,
                        ffmpeg_options: None,
                        input: Some(vec![String::from("vids/crusty.mp4")]),
                        file_list: None,
                        overwrite: true,
                        verbose: true,
                        delete: false,
                        eta: false,
                        output: Some(String::from("test/gui-test.mp3")),
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
