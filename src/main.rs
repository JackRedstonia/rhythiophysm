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

        let main_menu = VContainer::new(
            ContainerSize::ZERO.expand_width().expand_height(),
            None,
        )
        .with_child(Nothing::new(LayoutSize::ZERO.expand_height()))
        .with_child(logo)
        .with_child(Nothing::new(LayoutSize::ZERO.expand_height()));

        let main_menu = MarginContainer::new(Margin::all(96.0)).with_child(main_menu);

        let background = Rectangle::new(
            LayoutSize::ZERO.expand_width().expand_height(),
            Paint::new_color4f(0.1, 0.3, 0.6, 1.0),
        );
        let main_menu = Backgrounded::new().with_child(background).with_child(main_menu);

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

use stacks::game::ID;
use std::collections::VecDeque;

enum ScreenStackMessage {
    Add(Wrap<dyn Widget>),
    Remove(ID),
}

struct ScreenStackResource {
    incoming_messages: VecDeque<ScreenStackMessage>,
}

impl ScreenStackResource {
    pub fn add_screen<T: Widget + 'static>(&mut self, screen: Wrap<T>) {
        self.incoming_messages
            .push_back(ScreenStackMessage::Add(screen.to_dyn()));
    }

    pub fn remove_screen(&mut self, screen_id: ID) {
        self.incoming_messages
            .push_back(ScreenStackMessage::Remove(screen_id));
    }
}

struct ScreenStack {
    resource: ResourceHoster<ScreenStackResource>,
    just_switched: bool,
}

impl ScreenStack {
    pub fn new() -> Wrap<Self> {
        let resource = ResourceHoster::new(ScreenStackResource {
            incoming_messages: VecDeque::new(),
        });
        Self {
            resource,
            just_switched: false,
        }
        .into()
    }

    pub fn add_screen<T: Widget + 'static>(&mut self, screen: Wrap<T>) {
        self.resource.access_mut().add_screen(screen);
    }

    pub fn remove_screen(&mut self, screen_id: ID) {
        self.resource.access_mut().remove_screen(screen_id);
    }
}

impl Widget for ScreenStack {
    fn load(&mut self, state: &mut WidgetState, stack: &mut ResourceStack) {
        stack.push(self.resource.new_user());
        for i in state.children() {
            i.load(stack);
        }
        stack.pop::<ResourceUser<ScreenStackResource>>();
    }

    fn update(&mut self, state: &mut WidgetState) {
        let mut r = self.resource.access_mut();
        while let Some(i) = r.incoming_messages.pop_front() {
            match i {
                ScreenStackMessage::Add(w) => {
                    state.add_child_dyn(w);
                }
                ScreenStackMessage::Remove(_id) => {
                    unimplemented!();
                }
            }
        }

        for i in state.children() {
            i.update();
        }
    }

    fn input(&mut self, state: &mut WidgetState, event: &InputEvent) -> bool {
        state
            .children()
            .rev()
            .next()
            .map(|top| top.input(event))
            .unwrap_or_default()
    }

    fn size(&mut self, state: &mut WidgetState) -> (LayoutSize, bool) {
        let top_size = state
            .children()
            .rev()
            .next()
            .map(|top| top.size())
            .unwrap_or_default();
        (top_size.0, top_size.1 || self.just_switched)
    }

    fn set_size(&mut self, state: &mut WidgetState, size: Size) {
        // TODO: should set size of from and to
        if let Some(top) = state.children().rev().next() {
            top.set_size(size);
        }
    }

    fn draw(&mut self, state: &mut WidgetState, canvas: &mut Canvas) {
        // TODO: should draw from and to
        if let Some(top) = state.children().rev().next() {
            top.draw(canvas);
        }
    }
}
