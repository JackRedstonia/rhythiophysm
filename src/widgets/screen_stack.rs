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

impl<'a> ScreenStackResource {
    pub fn add_screen<T: Widget + 'static>(&mut self, screen: Wrap<T>) {
        self.incoming_messages
            .push_back(ScreenStackMessage::Add(screen.to_dyn()));
    }

    pub fn add_screen_dyn(&mut self, screen: Wrap<dyn Widget>) {
        self.incoming_messages
            .push_back(ScreenStackMessage::Add(screen));
    }

    pub fn remove_screen(&mut self, screen_id: ID) {
        self.incoming_messages
            .push_back(ScreenStackMessage::Remove(screen_id));
    }
}

pub struct ScreenStack {
    resource: ResourceHoster<ScreenStackResource>,
    screens: Vec<Wrap<dyn Widget>>,
    just_switched: bool,
}

impl ScreenStack {
    pub fn new() -> Wrap<Self> {
        let resource = ResourceHoster::new(ScreenStackResource {
            incoming_messages: VecDeque::new(),
        });
        // `FrameworkState::request_load();` isn't needed here, as there are no
        // children to be loaded just yet.
        Self {
            resource,
            screens: vec![],
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
    fn load(&mut self, _state: &mut WidgetState, stack: &mut ResourceStack) {
        stack.push(self.resource.new_user());
        for child in &mut self.screens {
            child.load(stack);
        }
        stack.pop::<ResourceUser<ScreenStackResource>>();
    }

    fn update(&mut self, _state: &mut WidgetState) {
        let mut r = self.resource.access_mut();
        while let Some(i) = r.incoming_messages.pop_front() {
            match i {
                ScreenStackMessage::Add(w) => {
                    self.screens.push(w);
                    self.just_switched = true;
                    FrameworkState::request_load();
                }
                ScreenStackMessage::Remove(id) => {
                    let screen = self
                        .screens
                        .pop()
                        .expect("Attempted to remove non-existent screen");
                    if screen.id() != id {
                        panic!("Inconsistent screen removal behaviour");
                    }
                    self.just_switched = true;
                }
            }
        }

        for child in &mut self.screens {
            child.update();
        }
    }

    fn input(&mut self, _state: &mut WidgetState, event: &InputEvent) -> bool {
        self.screens
            .last_mut()
            .map_or(false, |top| top.input(event))
    }

    fn size(&mut self, _state: &mut WidgetState) -> (LayoutSize, bool) {
        let top_size = self
            .screens
            .last_mut()
            .map(|top| top.size())
            .unwrap_or_default();
        let js = self.just_switched;
        if js {
            self.just_switched = false;
        }
        (top_size.0, top_size.1 || js)
    }

    fn set_size(&mut self, _state: &mut WidgetState, size: Size) {
        // TODO: should set size of from and to
        if let Some(top) = self.screens.last_mut() {
            top.set_size(size);
        }
    }

    fn draw(&mut self, _state: &mut WidgetState, canvas: &mut Canvas) {
        // TODO: should draw from and to
        if let Some(top) = self.screens.last_mut() {
            top.draw(canvas);
        }
    }
}
