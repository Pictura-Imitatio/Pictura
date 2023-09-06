pub mod widget {
    #![allow(dead_code)]
    use crate::selection_tool::theme::Theme;
    pub type Renderer = iced_wgpu::Renderer<Theme>;
    pub type Element<'a, Message, Renderer> = iced_winit::core::Element<'a, Message, Renderer>;
    pub type Container<'a, Message> = iced::widget::Container<'a, Message, Renderer>;
    pub type Button<'a, Message> = iced::widget::Button<'a, Message, Renderer>;
    pub type Cursor = iced::mouse::Cursor;
}

use iced::widget::container;
use iced::{ application, color };

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
