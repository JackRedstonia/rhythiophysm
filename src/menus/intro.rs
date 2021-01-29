use stacks::prelude::*;
use skia::{scalar, Size, Canvas};
use game::{InputEvent, State};

enum IntroState {
    Intro,
    Transitioning(scalar),
    Child,
}

impl IntroState {
    fn should_process_self(&self) -> Option<scalar> {
        match self {
            IntroState::Intro => Some(1.0),
            IntroState::Transitioning(s) => Some(1.0 - *s),
            IntroState::Child => None,
        }
    }

    fn should_process_child(&self) -> Option<scalar> {
        match self {
            IntroState::Intro => None,
            IntroState::Transitioning(s) => Some(*s),
            IntroState::Child => Some(1.0),
        }
    }
}

pub struct Intro<T: Widget> {
    state: IntroState,
    child: Wrap<T>,
    size: LayoutSize,
}

impl<T: Widget> Intro<T> {
    pub fn new(child: impl Into<Wrap<T>>, size: LayoutSize) -> Self {
        Self {
            state: IntroState::Intro,
            child: child.into(),
            size,
        }
    }
}

impl<T: Widget> Widget for Intro<T> {
    fn load(&mut self, wrap: &mut WrapState, stack: &mut ResourceStack) {
        self.child.load(stack);
    }

    fn update(&mut self, wrap: &mut WrapState) {
        if self.state.should_process_child().is_some() {
            self.child.update();
        }
    }

    fn input(&mut self, wrap: &mut WrapState, event: &InputEvent) -> bool {
        let child = self.state.should_process_child().is_some() && self.child.input(event);
        let myself = self.state.should_process_self().is_some() && {
            // TODO: this is just a placeholder to trigger transition to child
            if let InputEvent::KeyDown(Keycode::Semicolon) = event {
                self.state = IntroState::Transitioning(0.0);
                true
            } else {
                false
            }
        };

        myself || child
    }

    fn size(&mut self, wrap: &mut WrapState) -> (LayoutSize, bool) {
        let child_changed = self.state.should_process_child().is_some() && self.child.size().1;
        let changed = child_changed || false;
        (self.size, changed)
    }

    fn set_size(&mut self, wrap: &mut WrapState, size: Size) {
        self.child.set_size(size);
    }

    fn draw(&mut self, wrap: &mut WrapState, canvas: &mut Canvas) {
        match &mut self.state {
            IntroState::Transitioning(alpha) => {
                let factor = State::last_update_time_draw().as_secs_f32();
                *alpha += factor * 6.0;
                if *alpha >= 1.0 {
                    self.state = IntroState::Child;
                }
            }
            _ => {}
        }

        if let Some(s) = self.state.should_process_child() {
            let i = canvas.save_layer_alpha(None, (s * 255.0) as _);
            self.child.draw(canvas);
            canvas.restore_to_count(i);
        }

        if let Some(s) = self.state.should_process_self() {
            let i = canvas.save_layer_alpha(None, (s * 255.0) as _);
            {
                // TODO: draw intro here
            }
            canvas.restore_to_count(i);
        }
    }
}