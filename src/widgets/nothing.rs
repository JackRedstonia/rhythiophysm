use stacks::prelude::*;

pub struct Nothing {
    layout_size: LayoutSize,
}

impl Nothing {
    pub fn new(layout_size: LayoutSize) -> Wrap<Self> {
        Self { layout_size }.into()
    }
}

impl Widget for Nothing {
    fn load(&mut self, _state: &mut WidgetState, _stack: &mut ResourceStack) {}

    fn update(&mut self, _state: &mut WidgetState) {}

    fn size(&mut self, _state: &mut WidgetState) -> (LayoutSize, bool) {
        (self.layout_size, false)
    }
}
