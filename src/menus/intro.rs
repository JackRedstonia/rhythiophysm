use std::time::Duration;

use stacks::framework::widgets::layout::{TimeReport, AB};
use stacks::game::ID;
use stacks::prelude::*;

use skia::gradient_shader::{self, GradientShaderColors};
use skia::utils::parse_path::from_svg;
use skia::{Color4f, ColorSpace, Path, TileMode};

const TEXT: &str = include_str!("../../resources/stacks.svg");

pub struct Intro {
    ab: Wrap<AB<IntroInner>>,
    excl_id: ID,
}

impl Intro {
    pub fn new() -> Wrap<Self> {
        let ab = AB::new(IntroInner::new(), Duration::from_millis(200));
        Wrap::new(Self {
            ab: ab.clone(),
            excl_id: ab.id(),
        })
        .with_child(ab)
    }

    pub fn add_child(&mut self, child: Wrap<impl Widget + 'static>) {
        self.ab.add_child(child);
    }

    pub fn add_child_dyn(&mut self, child: Wrap<dyn Widget>) {
        self.ab.add_child_dyn(child)
    }
}

impl Widget for Intro {
    fn load(&mut self, _state: &mut WidgetState, stack: &mut ResourceStack) {
        self.ab.load(stack);
    }

    fn on_child_add(&mut self, child: &mut Wrap<dyn Widget>) {
        if child.id() != self.excl_id {
            panic!("Cannot add children to Intro - use `inner().add_child`/`inner().add_child_dyn` instead")
        }
    }

    fn size(&mut self, _state: &mut WidgetState) -> (LayoutSize, bool) {
        self.ab.size()
    }

    fn set_size(&mut self, _state: &mut WidgetState, size: Size) {
        self.ab.set_size(size);
    }

    fn draw(&mut self, _state: &mut WidgetState, canvas: &mut Canvas) {
        if !self.ab.inner().is_running() {
            self.ab
                .inner_mut()
                .run(Duration::from_secs_f32(IntroInner::REAL_TIME));
        }
        self.ab.draw(canvas);
    }
}

struct IntroInner {
    progress: scalar,
    size: Size,
    text: Path,
    text_rect: Rect,
}

impl IntroInner {
    const TIME: scalar = 1.2;
    const WAIT_IN: scalar = 0.1;
    const WAIT_OUT: scalar = 0.5;

    const REAL_TIME: scalar = Self::TIME + Self::WAIT_IN + Self::WAIT_OUT;

    const TEXT_HEIGHT: scalar = 50.0;
    const TEXT_PAD: scalar = 100.0;

    const BG_COLORS: [Color4f; 2] = [
        Color4f::new(0.0, 0.2, 0.6, 1.0),
        Color4f::new(0.0, 0.3, 0.5, 1.0),
    ];

    fn new() -> Wrap<Self> {
        let logo = from_svg(TEXT).expect("Failed to parse SVG file for Stacks logo");
        let text_rect = logo.compute_tight_bounds();
        Self {
            progress: 0.0,
            size: Size::default(),
            text: logo,
            text_rect,
        }
        .into()
    }

    fn draw_background(&self, canvas: &mut Canvas) {
        let grc = GradientShaderColors::ColorsInSpace(&Self::BG_COLORS, ColorSpace::new_srgb());
        let gr = gradient_shader::linear(
            (Vector::default(), self.size.bottom_right()),
            grc,
            None,
            TileMode::default(),
            None,
            None,
        );
        canvas.draw_rect(
            Rect::from_size(self.size),
            &Paint::default().with_shader(gr).dither(),
        );
    }

    fn draw_text(&self, te: scalar, canvas: &mut Canvas) {
        let scaling = Self::TEXT_HEIGHT / self.text_rect.height();
        let paint = Paint::new_color4f(1.0, 1.0, 1.0, te).anti_alias();
        canvas.save();
        canvas.translate((
            Self::TEXT_PAD + (1.0 - te) * 20.0,
            (self.size.height - Self::TEXT_HEIGHT) * 0.5,
        ));
        canvas.scale((scaling, scaling));
        canvas.draw_path(&self.text, &paint);
        canvas.restore();
    }

    fn draw_dots(&self, te: scalar, canvas: &mut Canvas) {
        let paint = Paint::new_color4f(1.0, 1.0, 1.0, 0.2).anti_alias();
        let sp = 25.0;
        let offset = Vector::new(sp, sp) * 0.3;
        canvas.save();
        let mut m = Matrix::default();
        m.set_skew((0.02, 0.06), self.size.center());
        canvas.concat(&m);
        for x in -5..=((self.size.width / sp) as i32 + 5) {
            for y in -5..=((self.size.height / sp) as i32 + 5) {
                let p = Vector::new(x as f32, y as f32) * sp + offset;
                let r = ((p.length() / 200.0 - 3.2 - te * 2.0).sin() + 1.0) * 2.0;
                canvas.draw_circle(p, r, &paint);
            }
        }
        canvas.restore();
    }
}

impl Widget for IntroInner {
    fn size(&mut self, _state: &mut WidgetState) -> (LayoutSize, bool) {
        (LayoutSize::ZERO.expand_width().expand_height(), false)
    }

    fn set_size(&mut self, _state: &mut WidgetState, size: Size) {
        self.size = size;
    }

    fn draw(&mut self, _state: &mut WidgetState, canvas: &mut Canvas) {
        let t = self.progress;
        let te = t.ease_out_quart();

        self.draw_background(canvas);
        self.draw_dots(te, canvas);
        self.draw_text(te, canvas);
    }
}

impl TimeReport for IntroInner {
    fn time(&mut self, progress: scalar) {
        let pl = IntroInner::TIME / IntroInner::REAL_TIME;
        let pw = IntroInner::WAIT_IN / IntroInner::REAL_TIME;
        let pt = ((progress - pw).max(0.0) / pl).min(1.0);
        self.progress = pt;
    }
}
