use std::time::Duration;

use stacks::prelude::*;

use stacks::framework::widgets::layout::{TimeReport, AB};

use skia::utils::parse_path::from_svg;
use skia::Path;

const STACKS_TEXT: &str = include_str!("../../resources/stacks.svg");

pub struct Intro<T: Widget> {
    ab: Wrap<AB<IntroInner, T>>,
}

impl<T: Widget> Intro<T> {
    pub fn new(inner: impl Into<Wrap<T>>, size: LayoutSize) -> Self {
        Self {
            ab: AB::new(IntroInner::new(size), inner, Duration::from_millis(200)).wrap(),
        }
    }
}

impl<T: Widget> Widget for Intro<T> {
    fn load(&mut self, _wrap: &mut WidgetState, stack: &mut ResourceStack) {
        self.ab.load(stack);
    }

    fn update(&mut self, _wrap: &mut WidgetState) {
        self.ab.update();
    }

    fn input(&mut self, _wrap: &mut WidgetState, event: &InputEvent) -> bool {
        self.ab.input(event)
    }

    fn size(&mut self, _wrap: &mut WidgetState) -> (LayoutSize, bool) {
        self.ab.size()
    }

    fn set_size(&mut self, _wrap: &mut WidgetState, size: Size) {
        self.ab.set_size(size);
    }

    fn draw(&mut self, _wrap: &mut WidgetState, canvas: &mut Canvas) {
        if !self.ab.inner().is_running() {
            self.ab
                .inner()
                .run(Duration::from_secs_f32(IntroInner::ACTUAL_DURATION));
        }
        self.ab.draw(canvas);
    }
}

struct IntroInner {
    progress: scalar,
    layout_size: LayoutSize,
    size: Size,
    text: Path,
    text_height: scalar,
}

impl IntroInner {
    const ANIMATION_DURATION: scalar = 1.0;
    const PREPAD: scalar = 0.3;
    const POSTPAD: scalar = 0.5;

    const ACTUAL_DURATION: scalar = Self::ANIMATION_DURATION + Self::PREPAD + Self::POSTPAD;
    const ANIMATION_PERCENTAGE: scalar = Self::ANIMATION_DURATION / Self::ACTUAL_DURATION;
    const PREPAD_PERCENTAGE: scalar = Self::PREPAD / Self::ACTUAL_DURATION;

    const RING_COUNT: i32 = 7;
    const RING_COUNT_SCALAR: scalar = Self::RING_COUNT as _;

    const DIAMETER: scalar = 50.0;
    const PADDING: scalar = 40.0;
    const STROKE_WIDTH: scalar = 6.5;
    const SWEEP_ANGLE: scalar = 90.0;
    const CIRCLE_SWEEP_ANGLE: scalar = 270.0;

    fn new(size: LayoutSize) -> Self {
        let logo = from_svg(STACKS_TEXT).expect("Failed to parse SVG file for Stacks logo");
        let logo_height = logo.compute_tight_bounds().height();
        Self {
            progress: 0.0,
            layout_size: size,
            size: Size::default(),
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

impl Widget for IntroInner {
    fn size(&mut self, _wrap: &mut WidgetState) -> (LayoutSize, bool) {
        (self.layout_size, false)
    }

    fn set_size(&mut self, _wrap: &mut WidgetState, size: Size) {
        self.size = size;
    }

    fn draw(&mut self, _wrap: &mut WidgetState, canvas: &mut Canvas) {
        let t = self.progress;
        let te = t.ease_out_quart();

        self.draw_circles(t, te, canvas);
        self.draw_text(te, canvas);
    }
}

impl TimeReport for IntroInner {
    fn time(&mut self, progress: scalar) {
        let pt =
            ((progress - Self::PREPAD_PERCENTAGE).max(0.0) / Self::ANIMATION_PERCENTAGE).min(1.0);
        self.progress = pt;
    }
}
