#![windows_subsystem = "windows"]
// use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Column, Button, TextInput, Container};
use iced::{Alignment, Element, Sandbox, Settings};
use iced::window::icon;
use enigo::{Enigo, Key, KeyboardControllable};
use std::{thread, time::Duration};
use std::sync::{Arc, Mutex};
use image;

struct KeyPressApp {
    key_input: String,
    duration_input: String,
    interval_input: String,
    status_text: String,
    is_running: Arc<Mutex<bool>>,
    receiver: Option<std::sync::mpsc::Receiver<Message>>, // Add this line

}

#[derive(Debug, Clone)]
enum Message {
    KeyInputChanged(String),
    DurationInputChanged(String),
    IntervalInputChanged(String),
    StartPressed,
    StopPressed,
    SimulationEnded,
}

impl Sandbox for KeyPressApp {
    type Message = Message;

    fn new() -> Self {
        Self {
            key_input: String::new(),
            duration_input: String::new(),
            interval_input: String::new(),
            status_text: String::from("Ready"),
            is_running: Arc::new(Mutex::new(true)),
            receiver: None,
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
            },
            Message::StopPressed => {
                *self.is_running.lock().unwrap() = false;
                self.status_text = "Stopped".to_string();
            },
            Message::SimulationEnded => {
                self.status_text = "Simulation Completed".to_string();
            },
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

        let start_button = Button::new("Start").on_press(Message::StartPressed);
        let stop_button = Button::new("Stop")
        .on_press(Message::StopPressed)
        .padding(5);

        Container::new(
            Column::new()
                .push(key_text)
                .push(duration_text)
                .push(interval_text)
                .push(start_button)
                .push(stop_button)
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

            // 使用新线程来运行按键模拟
            let (sender, receiver) = std::sync::mpsc::channel::<Message>();
            let is_running = Arc::new(Mutex::new(true)); // Set is_running to true

            let is_running_clone = Arc::clone(&is_running);
            std::thread::spawn(move || {
                let mut enigo = Enigo::new();
                let start_time = std::time::Instant::now();
    
                while start_time.elapsed().as_secs() < duration {
                    if !*is_running_clone.lock().unwrap() {
                        break; // 检查是否需要提前终止
                    }
    
                    enigo.key_down(Key::Layout(key));
                    enigo.key_up(Key::Layout(key));
                    thread::sleep(Duration::from_millis(interval));
                }
    
                sender.send(Message::SimulationEnded).unwrap(); // 发送模拟结束的消息
            });
            self.is_running = is_running; // Share is_running between threads
            self.receiver = Some(receiver);
    }
}

pub fn run_app() -> iced::Result {
    let img2=image::open("./APP.png");
    let img2_path=match img2 {
        Ok(path)=>path,
        Err(error)=>panic!("error is {}",error),
    };
    let img2_file=img2_path.to_rgba8();
    let ico2=icon::from_rgba(img2_file.to_vec(), 1000, 1000);
    let ico2_file=match ico2{
        Ok(file)=>file,
        Err(error)=>panic!("error is {}",error),
    };

    KeyPressApp::run(Settings {
        window: iced::window::Settings {
            size: (400, 300), // 设置窗口大小为 800x600
            icon: Some(ico2_file),
            ..Default::default()
        },
        ..Default::default()
    })}

pub fn main() {
    match run_app() {
        Ok(_) => println!("Application exited successfully"),
        Err(e) => eprintln!("Application exited with error: {:?}", e),
    }
}