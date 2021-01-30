mod menus;

use stacks::framework::{
    widgets::{
        audio::Audio,
        layout::{FullscreenContainer, SizeFillContainer},
        shapes::Rect,
    },
    Framework,
};
use stacks::prelude::*;

use skia::Paint;

use menus::Intro;

fn main() {
    Framework::run("Rhythiophysm", || {
        let root = Rect::new(
            LayoutSize::min(1280.0, 720.0)
                .expand_width()
                .expand_height(),
            Paint::new_color4f(0.3, 0.3, 0.3, 1.0),
        );
        let root = Intro::new(
            root,
            LayoutSize::min(1280.0, 720.0)
                .expand_width()
                .expand_height(),
        );
        // Set up fullscreening and auto-resizing
        let root = FullscreenContainer::new(SizeFillContainer::new(root));
        // Set up audio
        let root = Audio::new(root)?;
        Ok(root)
    })
    .expect("Failed to run game");
}
