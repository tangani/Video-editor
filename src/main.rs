use iced::{button, Button, Element, Sandbox, Settings, Text};
use nfd2::Response;

#[derive(Default)]
pub struct FileDialog {
    open_button_state: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    Open,
}

impl Sandbox for FileDialog {
    type Message = Message;

    fn new() -> FileDialog {
        FileDialog::default()
    }

    fn title(&self) -> String {
        String::from("File Dialog")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Open => {
                let result = nfd2::open_file_dialog(None, None).unwrap();
                match result {
                    Response::Okay(file_path) => println!("File path: {:?}", file_path),
                    Response::OkayMultiple(file_paths) => println!("File paths: {:?}", file_paths),
                    Response::Cancel => println!("User canceled"),
                }
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Button::new(&mut self.open_button_state, Text::new("Open File"))
            .on_press(Message::Open)
            .into()
    }
}

fn main() -> iced::Result {
    FileDialog::run(Settings::default())
}