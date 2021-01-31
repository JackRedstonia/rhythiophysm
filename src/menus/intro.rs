use std::time::Duration;

use stacks::prelude::*;

use skia::utils::parse_path::from_svg;
use skia::Path;

const STACKS_LOGO: &str = include_str!("../../resources/stacks.svg");

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
    logo: Path,
    logo_height: scalar,
}

impl<T: Widget> Intro<T> {
    const ANIMATION_DURATION: scalar = 1.0;

    pub fn new(child: impl Into<Wrap<T>>, size: LayoutSize) -> Self {
        let logo = from_svg(STACKS_LOGO).expect("Failed to parse SVG file for Stacks logo");
        let logo_height = logo.compute_tight_bounds().height();
        Self {
            state: IntroState::Intro,
            child: child.into(),
            layout_size: size,
            size: Size::default(),
            start_time: None,
            logo,
            logo_height,
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

            let opacity = (t * 1.4).min(1.0);

            let diameter = 50.0;
            let padding = 40.0;
            let stroke_width = 6.5;
            let t_sweep = 50.0;

            let paint = Paint::new_color4f(1.0, 1.0, 1.0, opacity).anti_alias();

            let half_width = stroke_width / 2.0;
            let oval = Rect {
                left: padding + half_width,
                top: self.size.height - diameter - padding + half_width,
                right: padding + diameter - half_width,
                bottom: self.size.height - padding - half_width,
            };
            let start = t * t_sweep;
            let sweep = 360.0 - start * 2.0;
            canvas.draw_arc(
                oval,
                start - 90.0,
                sweep,
                false,
                &paint.clone().stroke().with_stroke_width(stroke_width),
            );

            let scaling = diameter / self.logo_height;
            canvas.save();
            canvas.translate((
                padding * 2.0 + diameter,
                self.size.height - diameter - padding,
            ));
            canvas.scale((scaling, scaling));
            canvas.draw_path(&self.logo, &paint);
            canvas.restore();
        }

        if let Some(s) = self.state.should_process_child() {
            let i = canvas.save_layer_alpha(None, (s * 255.0) as _);
            self.child.draw(canvas);
            canvas.restore_to_count(i);
        }
    }
}
