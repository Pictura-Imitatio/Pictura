use iced::{ Point };
use winit::dpi::{ LogicalPosition, PhysicalPosition };
use log::info;

use iced_winit::winit;
use winit::event::{MouseButton, ElementState, KeyboardInput, VirtualKeyCode};

use crate::gui::theme::Theme;

use winit::{
    event::{Event as winEvent, WindowEvent},
    event_loop::ControlFlow,
};

mod theme;
mod app;
pub mod rectangle;
pub use rectangle as rect;

use self::app::state::State;


pub fn run(tl: PhysicalPosition<f64>, br: PhysicalPosition<f64>) {
    let app_state = State::setup(tl, br);
    app_state.event_loop.run(move |event, _, control_flow| {
        // You should change this if you want to render continuosly
        *control_flow = ControlFlow::Wait;
        match event {
            winit::event::Event::WindowEvent { event, .. } => { match event {
                WindowEvent::CursorMoved { position, .. } => {
                    let scale_factor = app_state.window.current_monitor().unwrap().scale_factor();
                    let pos: LogicalPosition<f64> = position.to_logical(scale_factor); 
                    app_state.queue_message(app::Message::OnMouseMoved(Point { x: pos.x as f32, y: pos.y as f32 }));
                    app_state.cursor_position = Some(position)
                }
                WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
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
                    match (button, state) {
                        (MouseButton::Left,
                        ElementState::Pressed) => {
                            app_state.press();
                        }
                        (MouseButton::Left,
                        ElementState::Released) => {
                            app_state.release();
                            *control_flow = ControlFlow::Exit; 
                        }
                        (MouseButton::Right, _) => {
                            *control_flow = ControlFlow::Exit;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
            // Map window event to iced event
            app_state.map_to_iced(&event);
            },
            winEvent::MainEventsCleared => {
                if !app_state._state.is_queue_empty() {
                    app_state.update_iced();
                }
            }
            winEvent::RedrawRequested(_) => {
                app_state.request_redraw()
            }
            _ => {}
        }
    });

}
