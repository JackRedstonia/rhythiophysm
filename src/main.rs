mod menus;

use stacks::framework::{
    widgets::{
        audio::Audio,
        layout::{FullscreenContainer, SizeFillContainer},
        shapes::Rectangle,
    },
    Framework,
};
use stacks::prelude::*;

use skia::Paint;

use menus::Intro;

fn main() {
    Framework::run("Rhythiophysm", || {
        let root = Rectangle::new(
            LayoutSize::ZERO.expand_width().expand_height(),
            Paint::new_color4f(0.3, 0.3, 0.3, 1.0),
        );
        let mut intro = Intro::new();
        intro.inner_mut().add_child(root);
        // Set up fullscreening and auto-resizing
        let root = SizeFillContainer::new(Some(Size::new(1280.0, 720.0))).with_child(intro);
        let root = FullscreenContainer::new().with_child(root);
        // Set up audio
        let root = Audio::new()?.with_child(root);
        Ok(root)
    })
    .expect("Failed to run game");
}
