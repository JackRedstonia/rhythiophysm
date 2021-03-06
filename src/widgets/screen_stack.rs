use std::collections::VecDeque;

use stacks::game::ID;
use stacks::prelude::*;

enum ScreenStackMessage {
    Add(Wrap<dyn Widget>),
    Remove(ID),
}

pub struct ScreenStackResource {
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

pub struct ScreenStack {
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
