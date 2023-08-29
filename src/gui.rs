use iced::window::{ Position, Level };
use iced::widget::{ column, container };
use iced::mouse::{ self, Cursor };
use iced::{ 
    executor, Alignment, Command, Length, Event,
    window, Settings, Subscription, Point, Application, subscription
};

use crate::gui::theme::{ theme::Theme, widget::Element };

use winit::{
    event::{Event as winEvent, ModifiersState, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

mod theme;
mod rectangle;
use rectangle::rectangle as rect;

pub fn run() {
    /*#[cfg(not(target_arch = "wasm32"))]
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = winit::window::WindowBuilder::new().build(&event_loop).unwrap();

    let physical_size = window.inner_size();
    let mut viewport = iced_winit::Viewport::with_physical_size(
        iced::Size::new(physical_size.width, physical_size.height),
        window.scale_factor(),
    );*/
    let settings = iced::Settings {
        window: window::Settings {
            resizable: false,
            decorations: false,
            position: Position::Specific(0i32, 0i32),
            visible: true,
            transparent: true,
            level: Level::AlwaysOnTop,
            icon: None,
            min_size: None,
            max_size: None,
            size: (1920u32,1080u32),
            platform_specific: window::PlatformSpecific::default(),
        },
        ..Default::default()
    };
    App::run(settings);
}
#[derive(Debug, Clone)]
pub enum Message {
    OnMousePressed,
    OnMouseMoved(Point),
    OnMouseReleased,
}

pub struct App {
    width: f32,
    height: f32,
    pressed: bool,
    released: bool,
    cursor_pressed_position: Point,
    cursor_released_position: Point,
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Self::Message>) {
        (
            App {
                width: 0f32,
                height: 0f32,
                pressed: false,
                released: false,
                cursor_pressed_position: Point {x:0.0, y:0.0},
                cursor_released_position: Point {x:0.0, y:0.0},
            }, 
            window::change_mode(iced::window::Mode::Windowed)
        )
    }

    fn title(&self) -> String {
        String::from("Pictura Selection Tool")
    }

    fn update(&mut self, _message: Message) -> Command<Message> {
        match _message {
            Message::OnMousePressed => {
                println!("mouse pressed");
                self.width = 0f32;
                self.height = 0f32;
                self.pressed = true;
                self.released = false;
                Command::none()
            }

            Message::OnMouseMoved(_point) => {
                println!("mouse moved to {:?}", _point);
                if self.pressed && !self.released {
                    self.width = _point.x - self.cursor_pressed_position.x;
                    self.height = _point.y - self.cursor_pressed_position.y;
                    self.cursor_released_position = _point;
                }
                else if !self.released { 
                    self.cursor_pressed_position = _point;
                }
                Command::none()
            }

            Message::OnMouseReleased => {
                println!("mouse released");
                self.pressed = false;
                self.released = true;
                Command::none()
            }

            _ => { Command::none() }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            rect::rectangle(self.cursor_pressed_position.x, self.cursor_pressed_position.y, self.width, self.height),
        ]
        .padding([self.cursor_pressed_position.y, self.cursor_pressed_position.x])
        .spacing(0)
        .align_items(Alignment::Start);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events_with(|event, _status| {
            match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                    Some(Message::OnMousePressed)
                }
                Event::Mouse(mouse::Event::CursorMoved { position }) => {
                    Some(Message::OnMouseMoved(position))
                }
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                    Some(Message::OnMouseReleased)
                }
                _ => None
            }
        })
    }
}

