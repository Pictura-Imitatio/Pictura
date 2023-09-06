use futures::core::mouse;
use iced::window::{self, Level};
use iced_wgpu::{ 
    Renderer,
    Backend,
    wgpu::{ self, Queue, Device,
    Instance, Surface, Backends
    }
};
use log::info;
use winit::{
    event_loop::EventLoop,
    window::Window, 
    dpi::{PhysicalPosition, LogicalPosition, PhysicalSize}, 
    event::ModifiersState
};

use iced_winit::{
    Viewport,
    runtime::{program::State as WInitState, Debug}, 
    Clipboard,
    settings,
    futures, conversion,
};

use crate::gui::theme::Theme;

use super::{App, Message};

pub struct MouseState {
    pub pressed: bool,
    pub was_pressed: bool,
    pub position: LogicalPosition<f64>,
}

pub struct State {
    pub event_loop: EventLoop<()>,
    pub window: Window,
    viewport: Viewport,
    pub cursor_position: Option<PhysicalPosition<f64>>,
    clipboard: Clipboard,
    modifiers: ModifiersState,
    app: App,
    pub _state: WInitState<App>,
    backend: Backends,
    instance: Instance,
    surface: Surface,
    debug: Debug,
    device: Device,
    renderer: Renderer<Theme>,
    queue: Queue,
    mouse_state: MouseState,
}

impl State {
    fn new(
        event_loop: EventLoop<()>,
        window: Window,
        viewport: Viewport,
        cursor_position: Option<PhysicalPosition<f64>>,
        clipboard: Clipboard,
        modifiers: ModifiersState,
        app: App,
        _state: WInitState<App>,
        backend: Backends,
        instance: Instance,
        surface: Surface,
        debug: Debug,
        device: Device,
        renderer: Renderer<Theme>,
        queue: Queue,
        mouse_state: MouseState,
    ) -> State {
        State {
            event_loop,
            window,
            viewport,
            cursor_position,
            clipboard,
            modifiers,
            app,
            _state,
            backend,
            instance,
            surface,
            debug,
            device,
            renderer,
            queue,
            mouse_state
        }
    }

    pub fn release(&self) {
        if self.mouse_pressed() { released = true; }
        self._state.queue_message(Message::OnMouseReleased);
        if released {
            args::capture(( PhysicalPosition::new(pressed_pos.unwrap().x + tl.x,
            pressed_pos.unwrap().y + tl.y), 
                            PhysicalPosition::new(released_pos.unwrap().x + tl.x,
                            released_pos.unwrap().y + tl.y)));
        }

    }

    pub fn request_redraw(&self) {
        match self.surface.get_current_texture() {
            Ok(frame) => {
                let mut encoder = self.device.create_command_encoder(
                    &wgpu::CommandEncoderDescriptor { label: None },
                    );

                let program = self._state.program();

                let view = frame.texture.create_view(
                    &wgpu::TextureViewDescriptor::default(),
                    );
                // And then iced on top
                self.renderer.with_primitives(|backend, primitive| {
                    backend.present(
                        &self.device,
                        &self.queue,
                        &mut encoder,
                        None,
                        &view,
                        primitive,
                        &self.viewport,
                        &self.debug.overlay(),
                        );
                });

                // Then we submit the work
                self.queue.submit(Some(encoder.finish()));
                frame.present();

                // Update the mouse cursor
                self.window.set_cursor_icon(
                    iced_winit::conversion::mouse_interaction(
                        self._state.mouse_interaction(),
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
                    self.window.request_redraw();
                }
            },
        }
    }

    pub fn update_iced(&self) {
        // We update iced
        let _ = self._state.update(
            self.viewport.logical_size(),
            self.cursor_position
            .map(|p| {
                conversion::cursor_position(
                    p,
                    self.viewport.scale_factor(),
                    )
            })
            .map(mouse::Cursor::Available)
            .unwrap_or(mouse::Cursor::Unavailable),
            &mut self.renderer,
            &Theme,
            &iced_winit::core::renderer::Style {
                text_color: iced_winit::core::Color::WHITE,
            },
            &mut self.clipboard,
            &mut self.debug,
            );

        // and request a redraw
        self.window.request_redraw();
    }

    pub fn map_to_iced(&self, event: &winit::event::WindowEvent<'_>) {
        if let Some(event) = iced_winit::conversion::window_event(
            event,
            self.window.scale_factor(),
            self.modifiers,
            ) {
            self._state.queue_event(event);
        }
        self.window.request_redraw();
    }

    // TODO: throw error
    pub fn press(&self) {
        self.mouse_state.pressed = !self.mouse_pressed();
        self.mouse_state.was_pressed = true;
        self.mouse_state.position = self.cursor_position.unwrap().to_logical(self.window.scale_factor());
        self._state.queue_message(Message::OnMousePressed);
    }

    pub fn mouse_pressed(&self) -> bool {
        self.mouse_state.pressed
    }

    pub fn queue_message(&self, message: Message) {
        self._state.queue_message(message);
    }

    fn create_backend() -> Backends {
        let default_backend = Backends::PRIMARY;

        wgpu::util::backend_bits_from_env().unwrap_or(default_backend)
    }

    fn create_viewport(physical_size: &PhysicalSize<u32>, window: &Window) -> Viewport {
        iced_winit::Viewport::with_physical_size(
            iced::Size::new(physical_size.width, physical_size.height),
            window.scale_factor(),
            )}

    fn create_window(event_loop: &EventLoop<()>, 
                     tl: PhysicalPosition<f64>, 
                     br: PhysicalPosition<f64>
                    ) -> Window {
        let win_window = settings::Window {
            resizable: false,
            decorations: false,
            position: iced_winit::Position::Specific(0i32, 0i32),
            visible: true,
            transparent: false,
            level: Level::AlwaysOnTop,
            icon: None,
            min_size: None,
            max_size: None,
            size: ((br.x - tl.x) as u32, (br.y - tl.y) as u32),
            platform_specific: window::PlatformSpecific::default(),
        };

        info!("Window Size: {:?}", win_window.size);
        info!("Window Location {:?}", win_window.position);

        let window = Window::new(&event_loop).unwrap();
        let monitor = window.primary_monitor();
        drop(window);

        let window = win_window.into_builder(
            "Pictura",
            monitor,
            Some("Pictura".to_string())
            ).with_transparent(true)
            //.with_override_redirect(true)
            .build(&event_loop).unwrap();
        window.set_outer_position(PhysicalPosition::new(tl.x,tl.y));
        window
    }


    pub fn setup(tl: PhysicalPosition<f64>, br: PhysicalPosition<f64>) -> State {
        let event_loop = EventLoop::new();
        let window = Self::create_window(&event_loop, tl, br);

        let physical_size = window.inner_size();

        let viewport = Self::create_viewport(&physical_size, &window);
        let mut cursor_position = None;
        let mut clipboard = Clipboard::connect(&window);

        let modifiers = ModifiersState::default();

        let backend = Self::create_backend();

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

        let mut debug = Debug::new();
        let mut renderer = Renderer::new(Backend::new(
                &device,
                &queue,
                iced_wgpu::Settings::default(),
                format,
                ));

        let app = App::new(());
        let mut _state = WInitState::new(
            app,
            viewport.logical_size(),
            &mut renderer,
            &mut debug,
            );

        Self::new(event_loop,
                  window,
                  viewport,
                  cursor_position,
                  clipboard,
                  modifiers,
                  app,
                  _state,
                  backend,
                  instance,
                  surface,
                  debug,
                  device,
                  renderer,
                  queue,
                  MouseState {
                      pressed: false,
                      was_pressed: false,
                      position: LogicalPosition::<f64>::new(0.0,0.0)
                  })
    }
}
