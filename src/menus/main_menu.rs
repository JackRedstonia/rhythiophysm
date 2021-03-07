use super::super::widgets::Nothing;

use stacks::framework::widgets::layout::{
    ContainerSize, Margin, MarginContainer, VContainerDyn,
};
use stacks::framework::widgets::shapes::Rectangle;
use stacks::framework::widgets::{
    Backgrounded, Font, FontStyle, Text, TextLayoutMode,
};
use stacks::prelude::*;

pub struct MainMenu {
    child: Wrap<Backgrounded<Rectangle, MarginContainer<VContainerDyn>>>,
}

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

        let mut main = VContainerDyn::new(
            ContainerSize::ZERO.expand_width().expand_height(),
            None,
        );
        main.inner_mut()
            .add_child(Nothing::new(LayoutSize::ZERO.expand_height()).to_dyn())
            .add_child(logo.to_dyn())
            .add_child(Nothing::new(LayoutSize::ZERO.expand_height()).to_dyn());

        let main = MarginContainer::new(main, Margin::all(96.0));

        let background = Rectangle::new(
            LayoutSize::ZERO.expand_width().expand_height(),
            Paint::new_color4f(0.1, 0.3, 0.6, 1.0),
        );
        let child = Backgrounded::new(background, main, false);

        Self { child }.into()
    }
}

impl Widget for MainMenu {
    fn load(&mut self, _state: &mut WidgetState, stack: &mut ResourceStack) {
        self.child.load(stack);
    }

    fn update(&mut self, _state: &mut WidgetState) {
        self.child.update();
    }

    fn input(&mut self, _state: &mut WidgetState, event: &InputEvent) -> bool {
        self.child.input(event)
    }

    fn size(&mut self, _state: &mut WidgetState) -> (LayoutSize, bool) {
        self.child.size()
    }

    fn set_size(&mut self, _state: &mut WidgetState, size: Size) {
        self.child.set_size(size);
    }

    fn draw(&mut self, _state: &mut WidgetState, canvas: &mut Canvas) {
        self.child.draw(canvas);
    }
}
