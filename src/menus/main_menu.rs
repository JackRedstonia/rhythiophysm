use game::InputEvent;
use skia::{scalar, Canvas, Size};
use stacks::prelude::*;

pub struct MainMenu {
    state: MainMenuState,
}

enum MainMenuState {
    MainMenu,
    Transitioning(scalar),
    Child,
}

impl Widget for MainMenu {
    fn load(&mut self, _wrap: &mut WrapState, _stack: &mut ResourceStack) {}

    fn update(&mut self, _wrap: &mut WrapState) {}

    fn input(&mut self, _wrap: &mut WrapState, _event: &InputEvent) -> bool {
        false
    }

    fn hover(&mut self, _wrap: &mut WrapState) {}

    fn hover_lost(&mut self, _wrap: &mut WrapState) {}

    fn size(&mut self, _wrap: &mut WrapState) -> (LayoutSize, bool) {
        (LayoutSize::ZERO, false)
    }

    fn set_size(&mut self, _wrap: &mut WrapState, _size: Size) {}

    fn draw(&mut self, _wrap: &mut WrapState, _canvas: &mut Canvas) {}
}
