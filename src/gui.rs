use iced::window::{ Position, Level };
use iced::widget::{ column, container };
use iced::mouse::{ self, Cursor };
use iced::{ 
    executor, Alignment, Length, Event,
    window, Settings, Subscription, Point, Application, subscription
};
use iced_winit::conversion;
use iced_winit::runtime::Debug;
use winit::dpi::{LogicalSize, LogicalPosition, Size, PhysicalPosition, PhysicalSize};

use iced_wgpu::{wgpu, Backend, Renderer, Settings as set};
use iced_wgpu::graphics::Viewport;
use iced_winit::{futures, winit, Clipboard};
use winit::event::{MouseButton, ElementState, KeyboardInput, VirtualKeyCode};
use winit::window::Fullscreen;

use crate::gui::theme::{ theme::Theme, widget::Element };

use winit::{
    event::{Event as winEvent, ModifiersState, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use iced_winit::runtime::{Command, Program};

mod theme;
mod rectangle;
use rectangle::rectangle as rect;

pub fn run() -> (LogicalPosition<f64>, LogicalPosition<f64>, LogicalPosition<f64>) {
    env_logger::init();
    let event_loop = EventLoop::new();
    let win_window = iced_winit::settings::Window {
        resizable: false,
        decorations: false,
        position: iced_winit::Position::Specific(0i32, 0i32),
        visible: true,
        transparent: false,
        level: Level::AlwaysOnTop,
        icon: None,
        min_size: None,
        max_size: None,
        size: (620u32,580u32),
        platform_specific: window::PlatformSpecific::default(),
    };

    let monitor = winit::window::Window::primary_monitor(&winit::window::Window::new(&event_loop).unwrap());
    let mut window = win_window.into_builder(
        "Pictura",
        monitor,
        Some("Pictura".to_string())
        ).build(&event_loop).unwrap();
    window.set_outer_position(LogicalPosition::new(50.0, 50.0));
    //window.set_inner_size(PhysicalSize::new(640.0,480.0));
    let physical_size = window.inner_size();

    let mut viewport = iced_winit::Viewport::with_physical_size(
        iced::Size::new(physical_size.width, physical_size.height),
        window.scale_factor(),
        );
    let mut cursor_position = None;
    let mut clipboard = Clipboard::connect(&window);

    let mut modifiers = ModifiersState::default();

    let default_backend = wgpu::Backends::PRIMARY;

    let backend =
        wgpu::util::backend_bits_from_env().unwrap_or(default_backend);

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: backend,
        ..Default::default()
    });

    let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let (format, (device, queue)) = futures::futures::executor::block_on(async {
            let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: None,
                force_fallback_adapter: false,
            }).await.unwrap();

            let capabilities = surface.get_capabilities(&adapter);
            (capabilities
             .formats
             .iter()
             .copied()
             .find(wgpu::TextureFormat::is_srgb)
             .or_else(|| capabilities.formats.first().copied())
             .expect("Get preferred format"),
             adapter.request_device(&wgpu::DeviceDescriptor::default(), None).await.unwrap())
        });


    surface.configure(
        &device,
        &wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: physical_size.width,
            height: physical_size.height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        },
        );

    let app = App::new(());
    let mut debug = Debug::new();
    let mut renderer = Renderer::new(Backend::new(
            &device,
            &queue,
            iced_wgpu::Settings::default(),
            format,
            ));

    let mut _state = iced_winit::runtime::program::State::new(
        app,
        viewport.logical_size(),
        &mut renderer,
        &mut debug,
        );
    let mut pressed = false;
    let mut pressed_pos = None;
    let mut released_pos = None;
    event_loop.run(move |event, _, control_flow| {
        // You should change this if you want to render continuosly
        *control_flow = ControlFlow::Wait;

        match event {
            winit::event::Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CursorMoved { position, .. } => {
                        let pos: LogicalPosition<f64> = position.to_logical(window.current_monitor().unwrap().scale_factor()); 
                        _state.queue_message(Message::OnMouseMoved(Point { x: pos.x as f32, y: pos.y as f32 }));
                        if pressed { pressed_pos = Some(position); }
                        else { released_pos= Some(position); }
                        cursor_position = Some(position)
                    }
                    WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Q),
                                    ..
                                },
                                ..
                        } => { 
                            *control_flow = ControlFlow::Exit; 
                        }
                    WindowEvent::MouseInput { state, button, .. } => {
                        match button { 
                            MouseButton::Left => {
                                match state {
                                    ElementState::Pressed => {
                                        pressed = true;
                                        _state.queue_message(Message::OnMousePressed);
                                    }
                                    ElementState::Released => {
                                        
                                        _state.queue_message(Message::OnMouseReleased);
                                        *control_flow = ControlFlow::Exit;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
                // Map window event to iced event
                if let Some(event) = iced_winit::conversion::window_event(
                    &event,
                    window.scale_factor(),
                    modifiers,
                    ) {
                    _state.queue_event(event);
                }
                window.request_redraw();
            },
            winEvent::MainEventsCleared => {
                if !_state.is_queue_empty() {
                    // We update iced
                    let _ = _state.update(
                        viewport.logical_size(),
                        cursor_position
                        .map(|p| {
                            conversion::cursor_position(
                                p,
                                viewport.scale_factor(),
                                )
                        })
                        .map(mouse::Cursor::Available)
                        .unwrap_or(mouse::Cursor::Unavailable),
                        &mut renderer,
                        &Theme,
                        &iced_winit::core::renderer::Style {
                            text_color: iced_winit::core::Color::WHITE,
                        },
                        &mut clipboard,
                        &mut debug,
                        );

                    // and request a redraw
                    window.request_redraw();
                }
            }
            winEvent::RedrawRequested(_) => {
                match surface.get_current_texture() {
                    Ok(frame) => {
                        let mut encoder = device.create_command_encoder(
                            &wgpu::CommandEncoderDescriptor { label: None },
                            );

                        let program = _state.program();

                        let view = frame.texture.create_view(
                            &wgpu::TextureViewDescriptor::default(),
                            );
                        // And then iced on top
                        renderer.with_primitives(|backend, primitive| {
                            backend.present(
                                &device,
                                &queue,
                                &mut encoder,
                                None,
                                &view,
                                primitive,
                                &viewport,
                                &debug.overlay(),
                                );
                        });

                        // Then we submit the work
                        queue.submit(Some(encoder.finish()));
                        frame.present();

                        // Update the mouse cursor
                        window.set_cursor_icon(
                            iced_winit::conversion::mouse_interaction(
                                _state.mouse_interaction(),
                                ),
                                );
                    }
                    Err(error) => match error {
                        wgpu::SurfaceError::OutOfMemory => {
                            panic!(
                                "Swapchain error: {error}. \
                                Rendering cannot continue."
                                )
                        }
                        _ => {
                            // Try rendering again next frame.
                            window.request_redraw();
                        }
                    },
                }
            }
            _ => {}
        }
    });
    return (pressed_pos.unwrap().to_logical(window.scale_factor()), 
            released_pos.unwrap().to_logical(window.scale_factor()), 
            window.inner_position().unwrap().to_logical(window.scale_factor()));
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

impl App {
    fn new(_flags: ()) -> App {
            App {
                width: 0f32,
                height: 0f32,
                pressed: false,
                released: false,
                cursor_pressed_position: Point {x:0.0, y:0.0},
                cursor_released_position: Point {x:0.0, y:0.0},
            }
    }
    pub fn get_cursor_pressed_position(&self) -> Point {
        self.cursor_pressed_position
    }
    pub fn get_cursor_released_position(&self) -> Point {
        self.cursor_released_position
    }
    pub fn get_height(&self) -> f32 {
        self.height
    }
    pub fn get_width(&self) -> f32 {
        self.width
    }
}

impl Program for App {
    type Message = Message;
    type Renderer = Renderer<Theme>;

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

    fn view(&self) -> Element<Message, Renderer<Theme>> {
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

}
