use crate::ContainerManager;

use std::cell::RefCell;
use std::rc::Weak;

use super::{
    ComponentHandle,
    OuterHandle
};

use wasmuri_events::UpdateEvent;

pub struct UpdateManager {

    components: Vec<Weak<RefCell<ComponentHandle>>>
}

impl UpdateManager {

    pub fn new() -> UpdateManager {
        UpdateManager {
            components: Vec::new()
        }
    }

    pub fn add_listener(&mut self, listener: &OuterHandle){
        self.components.push(listener.create_weak());
    }

    pub fn fire_update(&mut self, event: &UpdateEvent, manager: &ContainerManager){
        self.components.drain_filter(|handle| {
            match handle.upgrade() {
                Some(component_cell) => {
                    component_cell.borrow_mut().update(event, manager);
                    false
                }, None => true
            }
        });
    }
}