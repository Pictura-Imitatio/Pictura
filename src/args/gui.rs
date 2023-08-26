use iced::window::{Position,Level};
use iced::font::{self, Font};
use iced:: { 
    executor, alignment, Alignment, Command, Length, 
    window, Settings, Subscription, Point, Application
};
use iced::widget:: {column, container};
// use iced::mouse::{self, Event};
use iced::mouse;
use iced::subscription;
use iced::Event;
use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Text};
use crate::args::image_proc;

use self::theme::Theme;
use self::widget::Element;
use iced::Rectangle;

const ICON_FONT: Font = Font::with_name("icons");



#[derive(Debug, Clone)]
enum Message {
    OnMousePressed,
    OnMouseMoved(Point),
    OnMouseReleased,
}


pub fn run() {
    let settings = Settings {
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

#[derive(Debug, Clone, Default)]
struct AppState {
    rectnagle: Rectangle,
    drawing: Option<Point>
}

struct App{
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    pressed: bool,
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Self::Message>) {
        (
            App {
                x: 0f32,
                y: 0f32,
                width: 10f32,
                height: 10f32,
                pressed: false
            }, 
            window::change_mode(iced::window::Mode::Fullscreen)
        )
    }
    fn title(&self) -> String {
        String::from("Hi")
    }
    fn update(&mut self, _message: Message) -> Command<Message>{
        match _message {

            Message::OnMousePressed => {
                println!("mouse pressed");
                self.x = 0f32;
                self.y = 0f32;
                self.pressed = true;
                Command::none()
            }

            Message::OnMouseMoved(_point) => {
                println!("mouse moved to {:?}", _point);
                if self.pressed {
                    self.width = _point.x;
                    self.height = _point.y;
                }
                Command::none()
            }

            Message::OnMouseReleased => {
                println!("mouse released");
                self.pressed = false;
                Command::none()
            }

            _ => {Command::none()}
        }
    }
    fn view(&self) -> Element<Message> {
        /*let content = column![];
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(0)
            .style(theme::Container::Bordered)
            .into() */
        let content = column![
            rectangle::rectangle(self.x, self.y, self.width, self.height),
        ]
        .padding(0)
        .spacing(0)
        .align_items(Alignment::Center);

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

mod rectangle {
    use iced::advanced::layout::{self, Layout};
    use iced::advanced::renderer;
    use iced::advanced::widget::{self, Widget};
    use iced::mouse;
    use iced::{Color, Element, Length, Size};
    
    pub struct Rectangle {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    }

    impl Rectangle {
        pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
            Self { 
                x,
                y,
                width,
                height,
            }
        }
    }

    pub fn rectangle(x: f32, y: f32, width: f32, height: f32) -> Rectangle {
        Rectangle::new(x, y, width, height)
    }

    impl<Message, Renderer> Widget<Message, Renderer> for Rectangle 
        where
            Renderer: renderer::Renderer,
        {
            fn width(&self) -> Length {
                Length::Shrink
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
            fn draw(
                &self,
                _state: &widget::Tree,
                renderer: &mut Renderer,
                _theme: &Renderer::Theme,
                _style: &renderer::Style,
                layout: Layout<'_>,
                _cursor: mouse::Cursor,
                _viewport: &iced::Rectangle,
                ) {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: layout.bounds(),
                        border_radius: 2.0.into(),
                        border_width: 0.0,
                        border_color: Color::TRANSPARENT,
                    },
                    Color::WHITE,
                );
            }
        }
    impl<'a, Message, Renderer> From<Rectangle> for Element<'a, Message, Renderer>
    where
        Renderer: renderer::Renderer,
    {
        fn from(rectangle: Rectangle) -> Self {
            Self::new(rectangle)
        }
    }

}

/*mod rectangle {
  use iced::mouse;
  use iced::widget::canvas::event::{self, Event};
  use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke};
  use iced::{Element, Length, Point, Renderer, Theme};

  #[derive(Default)]
  pub struct State {
  cache: canvas::Cache,
  }

  impl State {
  pub fn view<'a>(&'a self, rectangle: &'a iced::Rectangle) -> Element<'a, Rectangle> {
  let canvas = Canvas::new(Rectangle {
  state: self,
  Rectangle: rectangle,
  });
  canvas.width(Length.Fill);
  canvas.height(Length.Fill);
  canvas.into()
  }


  }
  struct Rectangle<'a> {
  state: &'a State,
  Rectangle: &'a iced::Rectangle,
  }

  impl<'a> canvas::Program<iced::Rectangle> for Rectangle<'a> {
  type State = Option<Pending>;
  fn update(
  &self,
  state: &mut Self::State,
  event: Event,
  bounds: Rectangle,
  cursor: mouse::Cursor,
  ) -> (event::Status, Option<iced::Rectangle>) {
  let cursor_position =
  if let Some(position) = cursor.position_in(bounds) {
  position
  } else {
  return (event::Status::Ignored, None);
  };
  match event {
  Event::Mouse(mouse_event) => {
  let message = match mouse_event {
  mouse::Event::ButtonPressed(mouse::Button::Left) => {
  match *state 
  }
  }
  }
  }*/
