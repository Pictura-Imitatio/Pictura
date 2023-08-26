use iced::window::{Position, Icon, Level};
use iced::font::{self, Font};
use iced::{ executor, alignment, Alignment, Command, Length, window, Application, Settings };
use iced::widget:: {checkbox, column, container, text};
use iced::mouse::{self, Event};

use self::theme::Theme;
use self::widget::Element;


const ICON_FONT: Font = Font::with_name("icons");

#[derive(Debug, Clone)]
enum Message {
    MouseInput
}


pub fn run() {
    let settings = Settings {
        window: window::Settings {
            resizable: false,
            decorations: false,
            position: Position::Specific((0i32), (0i32)),
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

struct App;

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Self::Message>) {
        (App, window::change_mode(iced::window::Mode::Fullscreen))
    }
    fn title(&self) -> String {
        String::from("Hi")
    }
    fn update(&mut self, message: Message) -> Command<Message> {
        println!("fdafdas");
        match message {
            /*Message::MouseInput(Event::ButtonPressed(mouse::Button::Left)) => {
                println!("fhdjlahfdja");
                Command::none()
            }
            _ => {
                println!("fhdjlhfdlashflawhdjsfklhlashdfas");
                Command::none()
            }*/
            Message::MouseInput => {
                match message {
                    _ => { 
                        println!("fhdjsla");
                        Command::none() 
                    }
                }
            }
        }
    }
    fn view(&self) -> Element<Message> {
        let content = column![];
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(0)
            .style(theme::Container::Bordered)
            .into()    
    }
}

mod widget {
    #![allow(dead_code)]
    use crate::args::gui::theme::Theme;
    pub type Renderer = iced::Renderer<Theme>;
    pub type Element<'a, Message> = iced::Element<'a, Message, Renderer>;
    pub type Container<'a, Message> = iced::widget::Container<'a, Message, Renderer>;
    pub type Button<'a, Message> = iced::widget::Button<'a, Message, Renderer>;
}

mod theme {
    use iced::widget::{button, container, text, row};
    use iced::{application, color, Color};

    #[derive(Debug, Clone, Copy, Default)]
    pub struct Theme;

    impl application::StyleSheet for Theme {
        type Style = ();
        fn appearance(&self, _style: &Self::Style) -> application::Appearance {
            application::Appearance {
                background_color: color!(0x28, 0x28, 0x28, 0.7),
                text_color: color!(0xeb, 0xdb, 0xb2),
            }
        }
    }
    #[derive(Debug, Clone, Copy, Default)]
    pub enum Container {
        #[default]
        Default,
        Bordered,
    }
    impl container::StyleSheet for Theme {
        type Style = Container;

        fn appearance(&self, style: &Self::Style) -> container::Appearance {
            match style {
                Container::Default => container::Appearance::default(),
                Container::Bordered => container::Appearance {
                    border_color: color!(0x45, 0x85, 0x88),
                    ..Default::default()
                },
            }
        }
    }
}
