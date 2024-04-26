use iced::{button, Button, Element, Sandbox, Settings, Text, text_input, TextInput};
use nfd2::Response;
use std::fs;

#[derive(Default)]
pub struct FileDialog {
    open_button_state: button::State,
    text_input_state: text_input::State,
    text_input_value: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Open,
    TextInputChanged(String),
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
                    Response::Okay(file_path) => {
                        let user_input = fs::read_to_string(&file_path).expect("Unable to read file");
                        self.text_input_value = user_input;
                    },
                    Response::OkayMultiple(file_paths) => println!("File paths: {:?}", file_paths),
                    Response::Cancel => println!("User canceled"),
                }
            },
            // Message::TextInputChanged(_) => todo!()
            Message::TextInputChanged(new_value) => {
                self.text_input_value = new_value;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let open_button = Button::new(&mut self.open_button_state, Text::new("Open File"))
            .on_press(Message::Open);

        let text_input = TextInput::new(
            &mut self.text_input_state,
            "Type something here",
            &self.text_input_value,
            Message::TextInputChanged,
        );

        let content = Text::new(&self.text_input_value)
            .size(30);

        let column = iced::Column::new()
            .push(open_button)
            .push(text_input)
            .push(content);

        column.into()
    }
}

fn main() -> iced::Result {
    // read_text_file::FileDialog::run(iced::Settings::default());
    FileDialog::run(Settings::default())
}