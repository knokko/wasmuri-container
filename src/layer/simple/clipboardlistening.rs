use crate::*;

use std::cell::RefCell;
use std::rc::Weak;

struct Handle {

    behavior: Weak<RefCell<dyn ComponentBehavior>>,

    priority: i8
}

pub struct ClipboardManager {

    copy_listeners: Vec<Handle>,
    paste_listeners: Vec<Handle>,
    cut_listeners: Vec<Handle>
}

impl ClipboardManager {

    pub fn new() -> ClipboardManager {
        ClipboardManager {
            copy_listeners: Vec::new(),
            paste_listeners: Vec::new(),
            cut_listeners: Vec::new()
        }
    }

    fn add_listener(list: &mut Vec<Handle>, behavior: Weak<RefCell<dyn ComponentBehavior>>, priority: i8) {
        let maybe_index = list.binary_search_by(|existing| {

            // Intentionally INVERT the order so that the higher priorities come first
            priority.cmp(&existing.priority)
        });

        let index;
        match maybe_index {
            Ok(the_index) => index = the_index,
            Err(the_index) => index = the_index
        };
        list.insert(index, Handle {
            behavior,
            priority
        });
    }

    pub fn add_copy_listener(&mut self, behavior: Weak<RefCell<dyn ComponentBehavior>>, priority: i8) {
        Self::add_listener(&mut self.copy_listeners, behavior, priority);
    }

    pub fn add_paste_listener(&mut self, behavior: Weak<RefCell<dyn ComponentBehavior>>, priority: i8) {
        Self::add_listener(&mut self.paste_listeners, behavior, priority);
    }

    pub fn add_cut_listener(&mut self, behavior: Weak<RefCell<dyn ComponentBehavior>>, priority: i8) {
        Self::add_listener(&mut self.cut_listeners, behavior, priority);
    }

    pub fn fire_copy_event(&mut self) -> Option<ClipboardData> {
        let mut copied_data = None;
        self.copy_listeners.drain_filter(|handle| {
            match handle.behavior.upgrade() {
                Some(behavior) => {
                    if copied_data.is_none() {
                        let mut borrow_behavior = behavior.borrow_mut();
                        copied_data = borrow_behavior.on_copy();
                    }
                    false
                }, None => true
            }
        });

        copied_data
    }

    pub fn fire_paste_event(&mut self, clipboard: &ClipboardData) -> bool{
        let mut consumed = false;
        self.paste_listeners.drain_filter(|handle| {
            match handle.behavior.upgrade() {
                Some(behavior) => {
                    if !consumed {
                        let mut borrow_behavior = behavior.borrow_mut();
                        consumed = borrow_behavior.on_paste(clipboard);
                    }
                    false
                }, None => true
            }
        });

        consumed
    }

    pub fn fire_cut_event(&mut self) -> Option<ClipboardData> {
        let mut copied_data = None;
        self.cut_listeners.drain_filter(|handle| {
            match handle.behavior.upgrade() {
                Some(behavior) => {
                    if copied_data.is_none() {
                        let mut borrow_behavior = behavior.borrow_mut();
                        copied_data = borrow_behavior.on_cut();
                    }
                    false
                }, None => true
            }
        });

        copied_data
    }
}