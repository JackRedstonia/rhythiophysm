use stacks::prelude::*;
use skia::{scalar, Size, Canvas};
use game::{InputEvent};

pub struct MainMenu {
    state: MainMenuState,
}

enum MainMenuState {
    MainMenu,
    Transitioning(scalar),
    Child,
}

impl Widget for MainMenu {
    fn load(&mut self, wrap: &mut WrapState, stack: &mut ResourceStack) {}

    fn update(&mut self, wrap: &mut WrapState) {}

    fn input(&mut self, wrap: &mut WrapState, event: &InputEvent) -> bool {
        false
    }

    fn hover(&mut self, wrap: &mut WrapState) {}

    fn hover_lost(&mut self, wrap: &mut WrapState) {}

    fn size(&mut self, wrap: &mut WrapState) -> (LayoutSize, bool) {
        (LayoutSize::ZERO, false)
    }

    fn set_size(&mut self, wrap: &mut WrapState, size: Size) {}

    fn draw(&mut self, wrap: &mut WrapState, canvas: &mut Canvas) {}
}