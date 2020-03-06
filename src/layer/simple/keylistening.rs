use crate::*;

use std::cell::RefCell;
use std::rc::Weak;

use wasmuri_core::*;

pub struct KeyListenManager {
    
    hover_down_listeners: WeakMetaVec<dyn ComponentBehavior, Region>,
    hover_up_listeners: WeakMetaVec<dyn ComponentBehavior, Region>,

    full_down_listeners: WeakMetaVec<dyn ComponentBehavior, i8>,
    full_up_listeners: WeakMetaVec<dyn ComponentBehavior, i8>
}

impl KeyListenManager {

    pub fn new() -> KeyListenManager {
        KeyListenManager {
            hover_down_listeners: WeakMetaVec::new(),
            hover_up_listeners: WeakMetaVec::new(),

            full_down_listeners: WeakMetaVec::new(),
            full_up_listeners: WeakMetaVec::new()
        }
    }

    pub fn can_claim_down(&self, region: Region) -> bool {
        for handle in &self.hover_down_listeners.vec {
            if handle.metadata.intersects_with(region) {
                return false;
            }
        }

        true
    }

    pub fn can_claim_up(&self, region: Region) -> bool {
        for handle in &self.hover_up_listeners.vec {
            if handle.metadata.intersects_with(region) {
                return false;
            }
        }

        true
    }

    /// Should only be used after can_claim_down confirmed that the given region is available
    pub fn add_region_key_down_listener(&mut self, behavior: Weak<RefCell<dyn ComponentBehavior>>, region: Region){
        self.hover_down_listeners.push(behavior, region);
    }

    /// Should only be used after can_claim_up confirmed that the given region is available
    pub fn add_region_key_up_listener(&mut self, behavior: Weak<RefCell<dyn ComponentBehavior>>, region: Region){
        self.hover_up_listeners.push(behavior, region);
    }

    pub fn add_global_key_down_listener(&mut self, behavior: Weak<RefCell<dyn ComponentBehavior>>, priority: i8){
        Self::add_global_key_listener(&mut self.full_down_listeners, behavior, priority);
    }

    pub fn add_global_key_up_listener(&mut self, behavior: Weak<RefCell<dyn ComponentBehavior>>, priority: i8){
        Self::add_global_key_listener(&mut self.full_up_listeners, behavior, priority);
    }

    fn add_global_key_listener(list: &mut WeakMetaVec<dyn ComponentBehavior, i8>, behavior: Weak<RefCell<dyn ComponentBehavior>>, priority: i8){
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

    pub fn fire_key_down(&mut self, keys: &KeyInfo, manager: &ContainerManager, mouse_pos: Option<(f32, f32)>) -> bool {
        KeyListenManager::fire(&mut self.hover_down_listeners, &mut self.full_down_listeners, |behavior, manager| {
            behavior.key_down(&mut KeyDownParams::new(keys, manager))
        }, manager, mouse_pos)
    }

    pub fn fire_key_up(&mut self, keys: &KeyInfo, manager: &ContainerManager, mouse_pos: Option<(f32, f32)>) -> bool {
        KeyListenManager::fire(&mut self.hover_up_listeners, &mut self.full_up_listeners, |behavior, manager| {
            behavior.key_up(&mut KeyUpParams::new(keys, manager))
        }, manager, mouse_pos)
    }

    fn fire<F: FnMut(&mut dyn ComponentBehavior, &ContainerManager) -> bool>(
            hover_listeners: &mut WeakMetaVec<dyn ComponentBehavior, Region>, full_listeners: &mut WeakMetaVec<dyn ComponentBehavior, i8>, 
            mut processor: F, manager: &ContainerManager, mouse_pos: Option<(f32, f32)>) -> bool {

        // The key listeners with a location have priority over those without bound location
        let mut consumed = false;

        if mouse_pos.is_some() {
            hover_listeners.for_each_mut(|behavior, region| {
                if !consumed && region.is_float_inside(mouse_pos.unwrap()) {
                    consumed = processor(behavior, manager);
                } 
            });
        }

        // If none of the bound key listeners consumed the event, it will be passed to the full key listeners
        if !consumed {
            full_listeners.for_each_mut(|behavior, _priority| {
                if !consumed {
                    consumed = processor(behavior, manager);
                }
            });
        }

        consumed
    }
}