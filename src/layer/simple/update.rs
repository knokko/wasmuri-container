use crate::*;

use std::cell::RefCell;
use std::rc::Weak;

use wasmuri_events::UpdateEvent;

pub struct UpdateManager {

    behaviors: Vec<Weak<RefCell<dyn ComponentBehavior>>>
}

impl UpdateManager {

    pub fn new() -> UpdateManager {
        UpdateManager {
            behaviors: Vec::new()
        }
    }

    pub fn add_listener(&mut self, behavior: Weak<RefCell<dyn ComponentBehavior>>){
        self.behaviors.push(behavior);
    }

    pub fn fire_update(&mut self, event: &UpdateEvent, manager: &ContainerManager){
        self.behaviors.drain_filter(|handle| {
            match handle.upgrade() {
                Some(component_cell) => {
                    component_cell.borrow_mut().update(&mut UpdateParams::new(event, manager));
                    false
                }, None => true
            }
        });
    }
}