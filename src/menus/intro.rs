use std::time::Duration;

use stacks::prelude::*;

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
    layout_size: LayoutSize,
    size: Size,
    start_time: Option<Duration>,
}

impl<T: Widget> Intro<T> {
    const ANIMATION_DURATION: scalar = 1.0;

    pub fn new(child: impl Into<Wrap<T>>, size: LayoutSize) -> Self {
        Self {
            state: IntroState::Intro,
            child: child.into(),
            layout_size: size,
            size: Size::default(),
            start_time: None,
        }
    }
}

impl<T: Widget> Widget for Intro<T> {
    fn load(&mut self, _wrap: &mut WidgetState, stack: &mut ResourceStack) {
        self.child.load(stack);
    }

    fn update(&mut self, _wrap: &mut WidgetState) {
        if self.state.should_process_child().is_some() {
            self.child.update();
        }
    }

    fn input(&mut self, _wrap: &mut WidgetState, event: &InputEvent) -> bool {
        self.state.should_process_child().is_some() && self.child.input(event)
    }

    fn size(&mut self, _wrap: &mut WidgetState) -> (LayoutSize, bool) {
        if self.state.should_process_child().is_some() {
            self.child.size()
        } else {
            (self.layout_size, false)
        }
    }

    fn set_size(&mut self, _wrap: &mut WidgetState, size: Size) {
        self.size = size;
        self.child.set_size(size);
    }

    fn draw(&mut self, _wrap: &mut WidgetState, canvas: &mut Canvas) {
        match &mut self.state {
            IntroState::Transitioning(alpha) => {
                let factor = State::last_update_time_draw().as_secs_f32();
                *alpha += factor * 6.0;
                if *alpha >= 1.0 {
                    self.state = IntroState::Child;
                }
            }
            IntroState::Child => {
                self.child.draw(canvas);
                return;
            }
            _ => {}
        }

        if let Some(_s) = self.state.should_process_self() {
            let t = match self.start_time {
                Some(s) => (State::elapsed_draw() - s).as_secs_f32(),
                None => {
                    self.start_time = Some(State::elapsed_draw());
                    0.0
                }
            };

            let backoff = 0.3;

            let t = if t >= Self::ANIMATION_DURATION + backoff {
                if matches!(self.state, IntroState::Intro) {
                    self.state = IntroState::Transitioning(0.0);
                }
                Self::ANIMATION_DURATION + backoff
            } else {
                t
            };

            let t = (t - backoff).max(0.0) / Self::ANIMATION_DURATION;
            let t = 1.0 - (1.0 - t).powi(4);

            let diameter = 50.0;
            let padding = 40.0;
            let stroke_width = 8.0;
            let t_sweep = 50.0;

            let oval = Rect {
                left: padding,
                top: self.size.height - diameter - padding,
                right: padding + diameter,
                bottom: self.size.height - padding,
            };
            let opacity = (t * 1.4).min(1.0);
            let paint = Paint::new_color4f(1.0, 1.0, 1.0, opacity)
                .stroke()
                .with_stroke_width(stroke_width)
                .anti_alias();
            let start = t * t_sweep;
            let sweep = 360.0 - start * 2.0;
            canvas.draw_arc(oval, start - 90.0, sweep, false, &paint);
        }

        if let Some(s) = self.state.should_process_child() {
            let i = canvas.save_layer_alpha(None, (s * 255.0) as _);
            self.child.draw(canvas);
            canvas.restore_to_count(i);
        }
    }
}
