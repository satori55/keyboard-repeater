use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Column, Button, TextInput, Container};
use iced::{Alignment, Element, Sandbox, Settings};
use enigo::{Enigo, Key, KeyboardControllable};
use std::{thread, time::Duration};

struct KeyPressApp {
    key_input: String,
    duration_input: String,
    interval_input: String,
    status_text: String,
}

#[derive(Debug, Clone)]
enum Message {
    KeyInputChanged(String),
    DurationInputChanged(String),
    IntervalInputChanged(String),
    StartPressed,
}

impl Sandbox for KeyPressApp {
    type Message = Message;

    fn new() -> Self {
        Self {
            key_input: String::new(),
            duration_input: String::new(),
            interval_input: String::new(),
            status_text: String::from("Ready"),
        }
    }

    fn title(&self) -> String {
        String::from("Key Press Simulator")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::KeyInputChanged(input) => {
                self.key_input = input;
            },
            Message::DurationInputChanged(input) => {
                self.duration_input = input;
            },
            Message::IntervalInputChanged(input) => {
                self.interval_input = input;
            },
            Message::StartPressed => {
                self.start_key_press_simulation();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let key_text = TextInput::new("Press one key (like:\"a\")", &self.key_input)
            .on_input(Message::KeyInputChanged)
            .padding(5)
            .width(300);

        let duration_text = TextInput::new("Input the duration time ", &self.duration_input)
            .on_input(Message::DurationInputChanged)
            .padding(5)
            .width(300);

        let interval_text = TextInput::new("Input the interval time", &self.interval_input)
            .on_input(Message::IntervalInputChanged)
            .padding(5)
            .width(300);

        let start_button = Button::new("Button").on_press(Message::StartPressed);

        Container::new(
            Column::new()
                .push(key_text)
                .push(duration_text)
                .push(interval_text)
                .push(start_button)
                .spacing(20) // 设置元素之间的间隙为 20 像素
                .align_items(Alignment::Center), // 设置元素的对齐方式为居中对齐
        )
        .into()
    }
}

impl KeyPressApp {
    fn start_key_press_simulation(&mut self) {
        // Validate and parse inputs
        let key = match self.key_input.chars().next() {
            Some(k) => k,
            None => {
                self.status_text = "Invalid key input".to_string();
                return;
            }
        };

        let duration = match self.duration_input.parse::<u64>() {
            Ok(d) => d,
            Err(_) => {
                self.status_text = "Invalid duration input".to_string();
                return;
            }
        };

        let interval = match self.interval_input.parse::<u64>() {
            Ok(i) => i,
            Err(_) => {
                self.status_text = "Invalid interval input".to_string();
                return;
            }
        };

        // Start key press simulation
        self.status_text = "Running".to_string();
        let mut enigo = Enigo::new();
        let start_time = std::time::Instant::now();

        while start_time.elapsed().as_secs() < duration {
            enigo.key_down(Key::Layout(key));
            enigo.key_up(Key::Layout(key));
            thread::sleep(Duration::from_millis(interval));
        }

        self.status_text = "Completed".to_string();
    }
}

pub fn main() -> iced::Result {
    KeyPressApp::run(Settings {
        window: iced::window::Settings {
            size: (400, 300), // 设置窗口大小为 800x600
            ..Default::default()
        },
        ..Default::default()
    })}
