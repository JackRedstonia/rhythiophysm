mod menus;
mod widgets;

use stacks::framework::widgets::audio::Audio;
use stacks::framework::widgets::layout::{
    FullscreenContainer, SizeFillContainer,
};
use stacks::framework::widgets::Fonts;
use stacks::framework::Framework;
use stacks::prelude::*;

use menus::{Intro, MainMenu};
use widgets::ScreenStack;

fn main() {
    Framework::run("Rhythiophysm", || {
        let main_menu = MainMenu::new();

        let mut screens = ScreenStack::new();
        screens.inner_mut().add_screen(main_menu);

        let mut intro = Intro::new();
        intro.inner_mut().add_child(screens);

        // Set up fullscreening and auto-resizing
        let root = SizeFillContainer::new(Some(Size::new(1920.0, 1080.0)))
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
