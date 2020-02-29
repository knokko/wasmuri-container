use crate::*;

use std::cell::RefCell;
use std::rc::Weak;

use wasmuri_core::*;

pub struct ClipboardManager {

    copy_listeners: WeakMetaVec<dyn ComponentBehavior, i8>,
    paste_listeners: WeakMetaVec<dyn ComponentBehavior, i8>,
    cut_listeners: WeakMetaVec<dyn ComponentBehavior, i8>
}

// TODO Use WeakMetaVec in the other managers as well

impl ClipboardManager {

    pub fn new() -> ClipboardManager {
        ClipboardManager {
            copy_listeners: WeakMetaVec::new(),
            paste_listeners: WeakMetaVec::new(),
            cut_listeners: WeakMetaVec::new()
        }
    }

    fn add_meta_listener(list: &mut WeakMetaVec<dyn ComponentBehavior, i8>, behavior: Weak<RefCell<dyn ComponentBehavior>>, priority: i8) {
        let maybe_index = list.vec.binary_search_by(|existing| {

            // Intentionally INVERT the order so that the higher priorities come first
            priority.cmp(&existing.metadata)
        });

        let index;
        match maybe_index {
            Ok(the_index) => index = the_index,
            Err(the_index) => index = the_index
        };
        list.vec.insert(index, WeakMetaHandle {
            weak_cell: behavior,
            metadata: priority
        });
    }

    pub fn add_copy_listener(&mut self, behavior: Weak<RefCell<dyn ComponentBehavior>>, priority: i8) {
        Self::add_meta_listener(&mut self.copy_listeners, behavior, priority);
    }

    pub fn add_paste_listener(&mut self, behavior: Weak<RefCell<dyn ComponentBehavior>>, priority: i8) {
        Self::add_meta_listener(&mut self.paste_listeners, behavior, priority);
    }

    pub fn add_cut_listener(&mut self, behavior: Weak<RefCell<dyn ComponentBehavior>>, priority: i8) {
        Self::add_meta_listener(&mut self.cut_listeners, behavior, priority);
    }

    pub fn fire_copy_event(&mut self) -> Option<ClipboardData> {
        let mut copied_data = None;

        self.copy_listeners.for_each_mut(|behavior, _prio| {
            if copied_data.is_none() {
                copied_data = behavior.on_copy();
            }
        });

        copied_data
    }

    pub fn fire_paste_event(&mut self, clipboard: &ClipboardData) -> bool{
        let mut consumed = false;

        self.paste_listeners.for_each_mut(|behavior, _prio| {
            if !consumed {
                consumed = behavior.on_paste(clipboard);
            }
        });

        consumed
    }

    pub fn fire_cut_event(&mut self) -> Option<ClipboardData> {
        let mut copied_data = None;

        self.cut_listeners.for_each_mut(|behavior, _prio| {
            if copied_data.is_none() {
                copied_data = behavior.on_cut();
            }
        });

        copied_data
    }
}