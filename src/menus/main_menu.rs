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
    fn load(&mut self, _wrap: &mut WidgetState, _stack: &mut ResourceStack) {}

    fn update(&mut self, _wrap: &mut WidgetState) {}

    fn input(&mut self, _wrap: &mut WidgetState, _event: &InputEvent) -> bool {
        false
    }

    fn hover(&mut self, _wrap: &mut WidgetState) {}

    fn hover_lost(&mut self, _wrap: &mut WidgetState) {}

    fn size(&mut self, _wrap: &mut WidgetState) -> (LayoutSize, bool) {
        (LayoutSize::ZERO, false)
    }

    fn set_size(&mut self, _wrap: &mut WidgetState, _size: Size) {}

    fn draw(&mut self, _wrap: &mut WidgetState, _canvas: &mut Canvas) {}
}
