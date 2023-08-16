use iced::mouse::Cursor;
use iced::widget::canvas::{Geometry, Frame};
use iced::window::{Position, Icon, Level};
use iced::font::{self, Font};
use iced::{mouse, Rectangle, Point, Color};
use iced::{ executor, alignment, Alignment, Command, Length, window, Application, Settings };
use iced::widget:: {checkbox, column, container, text, canvas};
use iced_graphics::geometry::Path;

use self::theme::Theme;
use self::widget::Element;


const ICON_FONT: Font = Font::with_name("icons");

#[derive(Debug, Clone)]
enum Message {
}


fn main() -> iced::Result {
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
            size: (1920u32, 1080u32),
            platform_specific: window::PlatformSpecific::default(),
            ..window::Settings::default()
        },
        antialiasing: true,
        ..Default::default()
    };
    App::run(settings)
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
        Command::none()
    }
    fn view(&self) -> Element<Message> {
        let content = column![];
        container(content)
            .width(Length:: Fill)
            .height(Length:: Fill)
            .center_x()
            .center_y()
            .padding(0)
            .style(theme::Container::Bordered)
            .into()
    }
}

mod widget {
    #![allow(dead_code)]
    use crate::theme::Theme;

    pub type Renderer = iced::Renderer<Theme>;
    pub type Element<'a, Message> = iced::Element<'a, Message, Renderer>;
    pub type Container<'a, Message> = iced::widget::Container<'a, Message, Renderer>;
    pub type Button<'a, Message> = iced::widget::Button<'a, Message, Renderer>;
}


/* 
 * This module defines the theme of the transparent fullscreen window.
 */
mod theme {
    use iced::widget::{button, container, text, row};
    use iced::{application, color, Color};

    #[derive(Debug, Clone, Copy, Default)]
    pub struct Theme;

    impl application::StyleSheet for Theme {
        type Style = ();
        fn appearance(&self, _style: &Self::Style) -> application::Appearance {
            application::Appearance {
                background_color: color!(0x0D, 0x11, 0x17, 0.7),
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










// Custom renderer for the rectangle
struct RectangleRenderer {
    rectangle: Rectangle,
}

impl<Message> canvas::Program<Message> for RectangleRenderer {
    fn draw(&self, bounds: Rectangle, cursor: Cursor) -> Vec<Geometry> {
        let mut frame = Frame::new(bounds.size());

        // Draw the rectangle
        let rectangle_path = Path::rectangle(
            Point::new(self.rectangle.x, self.rectangle.y),
            (self.rectangle.width, self.rectangle.height),
        );

        frame.fill(&rectangle_path, Color::TRANSPARENT);
        frame.stroke(&rectangle_path, Color::BLACK, 1.0);

        frame.into()
    }
}
















/*
mod rectangle {
    use iced::{ Element, Length, Point, Renderer, Theme, mouse, Size };
    use iced::advanced::layout::{self, Layout};
    use iced::advanced::renderer;
    use iced::advanced::widget::{self, Widget};
    use iced::widget::canvas::{ 
        self, Canvas, event::{ self, Event },
        Frame, Geometry, Path, Stroke 
    };

    #[derive(Default)]
    pub struct Rectangle<T = f32> {
        x: T,
        y: T,
        width: T,
        height: T,
    }

    impl Rectangle {
        pub fn new(top_left: Point, size: Size<f32>) -> Self {
            Self { 
                x: top_left.x, 
                y: top_left.y, 
                width: size.width, 
                height: size.height, 
            }
        }

    }
    
    pub fn rectangle(top_left: Point, size: Size<f32>) -> Rectangle {
        Rectangle::new(top_left, size)
    }

    impl<Message, Renderer> Widget<Message, Renderer> for Rectangle
    where
        Renderer: renderer::Renderer,
    {
        fn width(&self) -> Length {
            Length:: Shrink
        }

        fn height(&self) -> Length {
            Length::Shrink
        }

        fn layout(
            &self,
            _renderer: &Renderer,
            _limits: &layout::Limits,
        ) -> layout::Node {
            layout::Node::new(Size::new(self.width, self.height))            
        }
    }
}
*/
