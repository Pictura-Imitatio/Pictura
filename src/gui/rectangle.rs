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
            Color::new(0.0, 0.0, 0.0, 0.6),
            );

    }
    fn mouse_interaction(
        &self,
        _state: &widget::Tree,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &iced::Rectangle,
        _renderer: &Renderer
        ) -> iced::mouse::Interaction {
        iced::mouse::Interaction::Crosshair // Change cursor to crosshair on hover
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
