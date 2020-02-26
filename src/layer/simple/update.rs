use crate::*;

use std::cell::RefCell;
use std::rc::Weak;

use wasmuri_core::WeakVec;

pub struct UpdateManager {

    behaviors: WeakVec<dyn ComponentBehavior>
}

impl UpdateManager {

    pub fn new() -> UpdateManager {
        UpdateManager {
            behaviors: WeakVec::new()
        }
    }

    pub fn add_listener(&mut self, behavior: Weak<RefCell<dyn ComponentBehavior>>){
        self.behaviors.push(behavior);
    }

    pub fn fire_update(&mut self, manager: &ContainerManager){
        self.behaviors.for_each_mut(|behavior| {
            behavior.update(&mut UpdateParams::new(manager));
            false
        });
    }
}