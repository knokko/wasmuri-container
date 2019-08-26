use crate::ContainerManager;

use std::cell::RefCell;
use std::rc::Weak;

use super::{
    ComponentHandle,
    Region,
    OuterHandle
};

use wasmuri_events::{
    KeyDownEvent,
    KeyUpEvent
};

struct KeyListenHandle { 

    component: Weak<RefCell<ComponentHandle>>,

    priority: i8
}

struct HoverListenHandle {

    component: Weak<RefCell<ComponentHandle>>,

    region: Region
}

pub struct KeyListenManager {
    
    hover_down_listeners: Vec<HoverListenHandle>,
    hover_up_listeners: Vec<HoverListenHandle>,

    full_down_listeners: Vec<KeyListenHandle>,
    full_up_listeners: Vec<KeyListenHandle>
}

trait EventProcessor<T> {

    fn process(&self, handle: &mut ComponentHandle, event: &T, manager: &ContainerManager) -> bool;
}

struct KeyDownProcessor {}

impl EventProcessor<KeyDownEvent> for KeyDownProcessor {

    fn process(&self, handle: &mut ComponentHandle, event: &KeyDownEvent, manager: &ContainerManager) -> bool {
        handle.key_down(event, manager)
    }
}

struct KeyUpProcessor {}

impl EventProcessor<KeyUpEvent> for KeyUpProcessor {

    fn process(&self, handle: &mut ComponentHandle, event: &KeyUpEvent, manager: &ContainerManager) -> bool {
        handle.key_up(event, manager)
    }
}

impl KeyListenManager {

    pub fn new() -> KeyListenManager {
        KeyListenManager {
            hover_down_listeners: Vec::new(),
            hover_up_listeners: Vec::new(),

            full_down_listeners: Vec::new(),
            full_up_listeners: Vec::new()
        }
    }

    pub fn can_claim_down(&self, region: Region) -> bool {
        for handle in &self.hover_down_listeners {
            if handle.region.intersects_with(&region) {
                return false;
            }
        }

        true
    }

    pub fn can_claim_up(&self, region: Region) -> bool {
        for handle in &self.hover_up_listeners {
            if handle.region.intersects_with(&region) {
                return false;
            }
        }

        true
    }

    /// Should only be used after can_claim_down confirmed that the given region is available
    pub fn add_region_key_down_listener(&mut self, listener: &OuterHandle, region: Region){
        self.hover_down_listeners.push(HoverListenHandle {
            component: listener.create_weak(),
            region
        });
    }

    /// Should only be used after can_claim_up confirmed that the given region is available
    pub fn add_region_key_up_listener(&mut self, listener: &OuterHandle, region: Region){
        self.hover_up_listeners.push(HoverListenHandle {
            component: listener.create_weak(),
            region
        });
    }

    pub fn add_global_key_down_listener(&mut self, listener: &OuterHandle, priority: i8){
        Self::add_global_key_listener(&mut self.full_down_listeners, listener, priority);
    }

    pub fn add_global_key_up_listener(&mut self, listener: &OuterHandle, priority: i8){
        Self::add_global_key_listener(&mut self.full_up_listeners, listener, priority);
    }

    fn add_global_key_listener(list: &mut Vec<KeyListenHandle>, listener: &OuterHandle, priority: i8){
        let maybe_index = list.binary_search_by(|existing| {

            // Intentionally INVERT the order so that the higher priorities come first
            priority.cmp(&existing.priority)
        });

        let index;
        match maybe_index {
            Ok(the_index) => index = the_index,
            Err(the_index) => index = the_index
        };
        list.insert(index, KeyListenHandle {
            component: listener.create_weak(),
            priority
        });
    }

    pub fn fire_key_down(&mut self, event: &KeyDownEvent, manager: &ContainerManager){
        KeyListenManager::fire(&mut self.hover_down_listeners, &mut self.full_down_listeners, &KeyDownProcessor {}, event, manager);
    }

    pub fn fire_key_up(&mut self, event: &KeyUpEvent, manager: &ContainerManager){
        KeyListenManager::fire(&mut self.hover_up_listeners, &mut self.full_up_listeners, &KeyUpProcessor {}, event, manager);
    }

    fn fire<T>(hover_listeners: &mut Vec<HoverListenHandle>, full_listeners: &mut Vec<KeyListenHandle>, processor: &dyn EventProcessor<T>, event: &T, manager: &ContainerManager){
        let mouse_pos = manager.get_mouse_position();

        // The key listeners with a location have priority over those without bound location
        let mut consumed = false;
        hover_listeners.drain_filter(|handle| {
            match handle.component.upgrade() {
                Some(component_cell) => {
                    if !consumed && handle.region.is_inside(mouse_pos) {
                        let mut component_handle = component_cell.borrow_mut();
                        consumed = processor.process(&mut component_handle, event, manager);
                    }
                    false
                }, None => true
            }
        });

        // If none of the bound key listeners consumed the event, it will be passed to the full key listeners
        if !consumed {
            full_listeners.drain_filter(|handle| {
                match handle.component.upgrade() {
                    Some(component_cell) => {
                        if !consumed {
                            let mut component_handle = component_cell.borrow_mut();
                            consumed = processor.process(&mut component_handle, event, manager);
                        }
                        false
                    }, None => true
                }
            });
        }
    }
}