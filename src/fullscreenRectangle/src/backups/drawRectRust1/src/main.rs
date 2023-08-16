use iced::{
    canvas::{self, Canvas, Cursor, Frame, Geometry, Path, Text},
    executor, mouse, Application, Color, Command, Element, Length, Point, Rectangle, Settings,
};
use winit::event::{Event, MouseButton, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

// Main application state
#[derive(Debug, Clone, Default)]
struct AppState {
    rectangle: Rectangle,
    drawing: Option<Point>,
}

// Main application
struct MyApp;

impl iced::Application for MyApp {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            MyApp,
            Command::none(), // No initial command
        )
    }

    fn title(&self) -> String {
        String::from("Iced Rectangle Drawing Example")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::MouseInput(mouse_event) => {
                // Handle mouse events
                match mouse_event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        // Start drawing a new rectangle
                        self.state().drawing = Some(mouse_event.position);
                    }
                    mouse::Event::ButtonReleased(mouse::Button::Left) => {
                        // Complete drawing the rectangle
                        if let Some(start) = self.state().drawing.take() {
                            let end = mouse_event.position;
                            let (x, y) = (start.x.min(end.x), start.y.min(end.y));
                            let width = (start.x - end.x).abs();
                            let height = (start.y - end.y).abs();
                            self.state().rectangle = Rectangle { x, y, width, height };
                        }
                    }
                    mouse::Event::CursorMoved { position, .. } => {
                        // Update rectangle while drawing
                        if let Some(start) = self.state().drawing {
                            let end = position;
                            let (x, y) = (start.x.min(end.x), start.y.min(end.y));
                            let width = (start.x - end.x).abs();
                            let height = (start.y - end.y).abs();
                            self.state().rectangle = Rectangle { x, y, width, height };
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        // Create a canvas with a custom renderer
        let canvas = Canvas::new(RectangleRenderer {
            rectangle: self.state().rectangle,
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .style(iced::widget::canvas::Style {
            background: Color::WHITE.into(),
            ..Default::default()
        });

        canvas.into()
    }
}

// Custom renderer for the rectangle
struct RectangleRenderer {
    rectangle: Rectangle,
}

impl<Message> iced::Renderer<Message> for RectangleRenderer {
    fn draw(&self, _bounds: Rectangle, _cursor_position: Point) -> Vec<canvas::Geometry> {
        // Draw the rectangle
        vec![canvas::Geometry::Rectangle {
            bounds: self.rectangle,
            background: Color::TRANSPARENT.into(),
            border_radius: 0.0,
            border_width: 1.0,
            border_color: Color::BLACK.into(),
        }]
    }
}

enum Message {
    MouseInput(mouse::Event),
}

fn main() {
    // Initialize the event loop
    let event_loop = EventLoop::new();

    // Run the application
    MyApp::run(Settings {
        window: iced::window::Settings {
            size: (800, 600),
            ..Default::default()
        },
        flags: (),
        default_font: None,
    });

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::DeviceEvent { event, .. } => {
                // Handle mouse events
                if let Some(mouse_event) = mouse::Event::from_device_event(&event) {
                    MyApp::send_message(Message::MouseInput(mouse_event));
                }
            }
            _ => {}
        }
    });
}
