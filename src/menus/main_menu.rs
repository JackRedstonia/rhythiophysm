use super::super::widgets::Nothing;

use stacks::framework::widgets::layout::{
    ContainerSize, Margin, MarginContainer, VContainer,
};
use stacks::framework::widgets::shapes::Rectangle;
use stacks::framework::widgets::{
    Backgrounded, Font, FontStyle, Text, TextLayoutMode,
};
use stacks::prelude::*;

pub struct MainMenu {}

impl MainMenu {
    pub fn new() -> Wrap<Self> {
        let logo = Text::new(
            LayoutSize::ZERO,
            Some(TextLayoutMode::OneLine),
            "rhythiophysm",
            Font::Default,
            FontStyle::Bold,
            Some(48.0),
            Paint::new_color4f(1.0, 1.0, 1.0, 1.0).anti_alias(),
        );

        let main = VContainer::new(
            ContainerSize::ZERO.expand_width().expand_height(),
            None,
        )
        .with_child(Nothing::new(LayoutSize::ZERO.expand_height()))
        .with_child(logo)
        .with_child(Nothing::new(LayoutSize::ZERO.expand_height()));

        let main = MarginContainer::new(Margin::all(96.0)).with_child(main);

        let background = Rectangle::new(
            LayoutSize::ZERO.expand_width().expand_height(),
            Paint::new_color4f(0.1, 0.3, 0.6, 1.0),
        );
        let main = Backgrounded::new().with_child(background).with_child(main);

        Self {}.wrap().with_child(main)
    }
}

impl Widget for MainMenu {
    fn size(&mut self, state: &mut WidgetState) -> (LayoutSize, bool) {
        state.child().unwrap().size()
    }

    fn set_size(&mut self, state: &mut WidgetState, size: Size) {
        state.child().unwrap().set_size(size);
    }

    fn draw(&mut self, state: &mut WidgetState, canvas: &mut Canvas) {
        state.child().unwrap().draw(canvas);
    }
}
