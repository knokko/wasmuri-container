use std::cell::RefCell;
use std::rc::Weak;

use crate::*;

use wasmuri_core::*;

struct FullHandle {

    behavior: Weak<RefCell<dyn ComponentBehavior>>,

    priority: i8
}

pub struct MouseManager {

    area_click_listeners: WeakMetaVec<dyn ComponentBehavior, Region>,
    full_click_listeners: WeakVec<dyn ComponentBehavior>,

    area_scroll_listeners: WeakMetaVec<dyn ComponentBehavior, Region>,
    full_scroll_listeners: Vec<FullHandle>,

    area_move_listeners: WeakMetaVec<dyn ComponentBehavior, Region>,
    full_move_listeners: WeakVec<dyn ComponentBehavior>,
    in_out_move_listeners: WeakMetaVec<dyn ComponentBehavior, Region>
}

impl MouseManager {

    pub fn new() -> MouseManager {
        MouseManager {
            area_click_listeners: WeakMetaVec::new(),
            full_click_listeners: WeakVec::new(),

            area_scroll_listeners: WeakMetaVec::new(),
            full_scroll_listeners: Vec::new(),

            area_move_listeners: WeakMetaVec::new(),
            full_move_listeners: WeakVec::new(),
            in_out_move_listeners: WeakMetaVec::new()
        }
    }

    pub fn can_claim_click_space(&self, region: Region) -> bool {
        for handle in &self.area_click_listeners.vec {
            if handle.metadata.intersects_with(region) {
                return false;
            }
        }

        true
    }

    pub fn can_claim_scroll_space(&self, region: Region) -> bool {
        for handle in &self.area_scroll_listeners.vec {
            if handle.metadata.intersects_with(region) {
                return false;
            }
        }

        true
    }

    /// Should only be used after can_claim_scroll_space confirmed that this is allowed
    pub fn add_scroll_space_listener(&mut self, behavior: Weak<RefCell<dyn ComponentBehavior>>, region: Region){
        self.area_scroll_listeners.push(behavior, region);
    }

    pub fn add_move_space_listener(&mut self, behavior: Weak<RefCell<dyn ComponentBehavior>>, region: Region){
        self.area_move_listeners.push(behavior, region);
    }

    /// Should only be used after can_claim_click_space confirmed that this is allowed
    pub fn add_click_space_listener(&mut self, behavior: Weak<RefCell<dyn ComponentBehavior>>, region: Region){
        self.area_click_listeners.push(behavior, region);
    }

    pub fn add_full_click_listener(&mut self, behavior: Weak<RefCell<dyn ComponentBehavior>>){
        self.full_click_listeners.push(behavior);
    }

    pub fn add_full_scroll_listener(&mut self, behavior: Weak<RefCell<dyn ComponentBehavior>>, priority: i8){
        Self::add_full_listener(&mut self.full_scroll_listeners, behavior, priority);
    }

    pub fn add_full_move_listener(&mut self, behavior: Weak<RefCell<dyn ComponentBehavior>>){
        self.full_move_listeners.push(behavior);
    }

    pub fn add_in_out_move_listener(&mut self, behavior: Weak<RefCell<dyn ComponentBehavior>>, region: Region){
        self.in_out_move_listeners.push(behavior, region);
    }

    fn add_full_listener(list: &mut Vec<FullHandle>, behavior: Weak<RefCell<dyn ComponentBehavior>>, priority: i8){
        let maybe_index = list.binary_search_by(|existing| {

            // Intentionally INVERT the order so that the higher priorities come first
            priority.cmp(&existing.priority)
        });

        let index;
        match maybe_index {
            Ok(the_index) => index = the_index,
            Err(the_index) => index = the_index
        };
        list.insert(index, FullHandle {
            behavior,
            priority
        });
    }

    fn mouse_inside(region: Region, mouse_pos: Option<(f32, f32)>) -> bool {
        mouse_pos.is_some() && region.is_float_inside(mouse_pos.unwrap())
    }

    pub fn fire_mouse_move(&mut self, prev_mouse_pos: Option<(f32,f32)>, next_mouse_pos: Option<(f32, f32)>, manager: &ContainerManager) {
        self.in_out_move_listeners.for_each_mut(|behavior, region| {
            if Self::mouse_inside(*region, prev_mouse_pos) != Self::mouse_inside(*region, next_mouse_pos) {
                behavior.mouse_move(&mut MouseMoveParams::new(prev_mouse_pos, next_mouse_pos, manager));
            }
        });

        self.area_move_listeners.for_each_mut(|behavior, region| {
            if Self::mouse_inside(*region, prev_mouse_pos) || Self::mouse_inside(*region, next_mouse_pos) {
                behavior.mouse_move(&mut MouseMoveParams::new(prev_mouse_pos, next_mouse_pos, manager));
            }
        });

        self.full_move_listeners.for_each_mut(|behavior| {
            behavior.mouse_move(&mut MouseMoveParams::new(prev_mouse_pos, next_mouse_pos, manager));
        });
    }

    pub fn fire_mouse_click(&mut self, manager: &ContainerManager, mouse_pos: (f32,f32), click: ClickInfo) {
        self.area_click_listeners.for_each_mut(|behavior, region| {
            if region.is_float_inside(mouse_pos) {
                behavior.mouse_click_inside(&mut MouseClickParams::new(mouse_pos, click, manager));
            } else {
                behavior.mouse_click_outside(&mut MouseClickOutParams::new(click, manager));
            }
        });

        self.full_click_listeners.for_each_mut(|behavior| {
            behavior.mouse_click_anywhere(&mut MouseClickAnyParams::new(click, manager));
        });
    }

    pub fn fire_mouse_click_outside(&mut self, manager: &ContainerManager, click: ClickInfo) {
        self.area_click_listeners.for_each_mut(|behavior, _region| {
            behavior.mouse_click_outside(&mut MouseClickOutParams::new(click, manager));
        });

        self.full_click_listeners.for_each_mut(|behavior| {
            behavior.mouse_click_anywhere(&mut MouseClickOutParams::new(click, manager));
        });
    }

    pub fn fire_mouse_scroll(&mut self, manager: &ContainerManager, mouse_pos: Option<(f32,f32)>, delta: f64) -> bool {

        let mut consumed = false;

        if mouse_pos.is_some() {
            self.area_scroll_listeners.for_each_mut(|behavior, region| {
                if !consumed && region.is_float_inside(mouse_pos.unwrap()){
                    consumed = behavior.mouse_scroll(&mut MouseScrollParams::new(mouse_pos, delta, manager));
                }
            });
        }

        if !consumed {
            self.full_scroll_listeners.drain_filter(|handle| {
                match handle.behavior.upgrade() {
                    Some(component_cell) => {
                        if !consumed {
                            consumed = component_cell.borrow_mut().mouse_scroll(&mut MouseScrollParams::new(mouse_pos, delta, manager));
                        }
                        false
                    }, None => true
                }
            });
        }

        consumed
    }
}