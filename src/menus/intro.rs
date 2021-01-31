use std::time::Duration;

use stacks::prelude::*;

use skia::utils::parse_path::from_svg;
use skia::Path;

const STACKS_TEXT: &str = include_str!("../../resources/stacks.svg");

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
    just_changed: bool,
    start_time: Option<Duration>,
    text: Path,
    text_height: scalar,
}

impl<T: Widget> Intro<T> {
    const ANIMATION_DURATION: scalar = 1.0;

    const RING_COUNT: i32 = 7;
    const RING_COUNT_SCALAR: scalar = Self::RING_COUNT as _;

    const DIAMETER: scalar = 50.0;
    const PADDING: scalar = 40.0;
    const STROKE_WIDTH: scalar = 6.5;
    const SWEEP_ANGLE: scalar = 90.0;
    const CIRCLE_SWEEP_ANGLE: scalar = 270.0;

    pub fn new(child: impl Into<Wrap<T>>, size: LayoutSize) -> Self {
        let logo = from_svg(STACKS_TEXT).expect("Failed to parse SVG file for Stacks logo");
        let logo_height = logo.compute_tight_bounds().height();
        Self {
            state: IntroState::Intro,
            child: child.into(),
            layout_size: size,
            size: Size::default(),
            just_changed: false,
            start_time: None,
            text: logo,
            text_height: logo_height,
        }
    }

    fn draw_circles(&self, t: scalar, te: scalar, canvas: &mut Canvas) {
        let stroke_width = Self::STROKE_WIDTH * te;
        let paint = Paint::new_color4f(1.0, 1.0, 1.0, te)
            .stroke()
            .with_stroke_width(stroke_width)
            .anti_alias();
        let center = Vector::new(
            Self::PADDING + Self::DIAMETER / 2.0,
            self.size.height - Self::PADDING - Self::DIAMETER / 2.0,
        );
        let sweep_mult = (t * 1.6).min(1.0).ease_out_quart();
        let sweep = Self::CIRCLE_SWEEP_ANGLE * sweep_mult;

        for i in (0..Self::RING_COUNT).map(|e| e as scalar) {
            let diameter = Self::DIAMETER * (2.0 - te) + stroke_width * i * 4.0;
            let percentage = 1.0 - i / Self::RING_COUNT_SCALAR;
            let paint = paint.clone().with_alpha(percentage.powi(2) * te);
            let start = te * (Self::SWEEP_ANGLE - i * 2.0) * (i + 1.0);
            Self::draw_circle(center, diameter, stroke_width, start, sweep, &paint, canvas);
        }
    }

    fn draw_circle(
        center: Vector,
        diameter: scalar,
        stroke_width: scalar,
        start: scalar,
        sweep: scalar,
        paint: &Paint,
        canvas: &mut Canvas,
    ) {
        let diameter = diameter - stroke_width;
        let oval_center = Vector::new(diameter / 2.0, diameter / 2.0);
        let oval = Rect::from_wh(diameter, diameter).with_offset(center - oval_center);
        canvas.draw_arc(oval, start - 90.0, sweep, false, &paint);
    }

    fn draw_text(&self, te: scalar, canvas: &mut Canvas) {
        let scaling = Self::DIAMETER / self.text_height;
        let paint = Paint::new_color4f(1.0, 1.0, 1.0, te).anti_alias();
        canvas.save();
        canvas.translate((
            Self::PADDING * 2.0 + Self::DIAMETER,
            self.size.height - Self::DIAMETER - Self::PADDING,
        ));
        canvas.scale((scaling, scaling));
        canvas.draw_path(&self.text, &paint);
        canvas.restore();
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
        let b = self.just_changed;
        if b {
            self.just_changed = false;
        }
        if self.state.should_process_child().is_some() {
            let (size, changed) = self.child.size();
            (size, changed || b)
        } else {
            (self.layout_size, b)
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

            let t_prepad = 0.3;
            let t_postpad = 0.5;
            let actual_duration = Self::ANIMATION_DURATION + t_prepad + t_postpad;

            let t = if t >= actual_duration {
                if matches!(self.state, IntroState::Intro) {
                    self.state = IntroState::Transitioning(0.0);
                    self.just_changed = true;
                }
                actual_duration
            } else {
                t
            };

            let t = ((t - t_prepad) / Self::ANIMATION_DURATION).max(0.0).min(1.0);
            let te = t.ease_out_quart();

            self.draw_circles(t, te, canvas);
            self.draw_text(te, canvas);
        }

        if let Some(s) = self.state.should_process_child() {
            let i = canvas.save_layer_alpha(None, (s * 255.0) as _);
            self.child.draw(canvas);
            canvas.restore_to_count(i);
        }
    }
}
