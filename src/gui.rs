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
                WindowEvent::MouseInput { state, button, .. } => { if event == MouseButton::Left { 
                    ElementState::Pressed => {
                        app_state.press();
                    }
                    ElementState::Released => {
                        if pressed { released = true; }
                        _state.queue_message(Message::OnMouseReleased);
                        *control_flow = ControlFlow::Exit; 
                        if released {
                            args::capture(( PhysicalPosition::new(pressed_pos.unwrap().x + tl.x,
                            pressed_pos.unwrap().y + tl.y), 
                                            PhysicalPosition::new(released_pos.unwrap().x + tl.x,
                                            released_pos.unwrap().y + tl.y)));
                        }
                    }
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
