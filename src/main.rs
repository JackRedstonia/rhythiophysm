mod menus;
mod widgets;

use stacks::framework::{
    widgets::{
        audio::Audio,
        layout::{
            ContainerSize, FullscreenContainer, Margin, MarginContainer,
            SizeFillContainer, VContainer,
        },
        shapes::Rectangle,
        Backgrounded, Font, FontStyle, Fonts, Text, TextLayoutMode,
    },
    Framework,
};
use stacks::prelude::*;

use skia::Paint;

use menus::Intro;

use widgets::Nothing;

fn main() {
    Framework::run("Rhythiophysm", || {
        let logo = Text::new(
            LayoutSize::ZERO,
            Some(TextLayoutMode::OneLine),
            "rhythiophysm",
            Font::Default,
            FontStyle::Bold,
            Some(48.0),
            Paint::new_color4f(1.0, 1.0, 1.0, 1.0).anti_alias(),
        );

        let root = VContainer::new(
            ContainerSize::ZERO.expand_width().expand_height(),
            None,
        )
        .with_child(Nothing::new(LayoutSize::ZERO.expand_height()))
        .with_child(logo)
        .with_child(Nothing::new(LayoutSize::ZERO.expand_height()));

        let root = MarginContainer::new(Margin::all(12.0)).with_child(root);

        let background = Rectangle::new(
            LayoutSize::ZERO.expand_width().expand_height(),
            Paint::new_color4f(0.1, 0.3, 0.6, 1.0),
        );
        let root = Backgrounded::new().with_child(background).with_child(root);

        let mut intro = Intro::new();
        intro.inner_mut().add_child(root);

        // Set up fullscreening and auto-resizing
        let root = SizeFillContainer::new(Some(Size::new(1280.0, 720.0)))
            .with_child(intro);
        let root = FullscreenContainer::new().with_child(root);
        // Set up fonts
        let root = Fonts::new().with_child(root);
        // Set up audio
        let root = Audio::new()?.with_child(root);
        Ok(root)
    })
    .expect("Failed to run game");
}
