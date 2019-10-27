use std::cell::RefCell;
use std::rc::Weak;

use crate::*;
use crate::params::*;

use super::Region;

use wasmuri_events::{
    MouseClickEvent,
    MouseMoveEvent,
    MouseScrollEvent
};

struct RegionHandle {

    behavior: Weak<RefCell<dyn ComponentBehavior>>,

    region: Region
}

struct FullHandle {

    behavior: Weak<RefCell<dyn ComponentBehavior>>,

    priority: i8
}

type AreaClickHandle = RegionHandle;

type FullClickHandle = Weak<RefCell<dyn ComponentBehavior>>;

type AreaScrollHandle = RegionHandle;

type FullScrollHandle = FullHandle;

type AreaMoveHandle = RegionHandle;

type FullMoveHandle = Weak<RefCell<dyn ComponentBehavior>>;

type InOutMoveHandle = RegionHandle;

pub struct MouseManager {

    area_click_listeners: Vec<AreaClickHandle>,
    full_click_listeners: Vec<FullClickHandle>,

    area_scroll_listeners: Vec<AreaScrollHandle>,
    full_scroll_listeners: Vec<FullScrollHandle>,

    area_move_listeners: Vec<AreaMoveHandle>,
    full_move_listeners: Vec<FullMoveHandle>,
    in_out_move_listeners: Vec<InOutMoveHandle>
}

impl MouseManager {

    pub fn new() -> MouseManager {
        MouseManager {
            area_click_listeners: Vec::new(),
            full_click_listeners: Vec::new(),

            area_scroll_listeners: Vec::new(),
            full_scroll_listeners: Vec::new(),

            area_move_listeners: Vec::new(),
            full_move_listeners: Vec::new(),
            in_out_move_listeners: Vec::new()
        }
    }

    pub fn can_claim_click_space(&self, region: Region) -> bool {
        for handle in &self.area_click_listeners {
            if handle.region.intersects_with(&region) {
                return false;
            }
        }

        true
    }

    pub fn can_claim_scroll_space(&self, region: Region) -> bool {
        for handle in &self.area_scroll_listeners {
            if handle.region.intersects_with(&region) {
                return false;
            }
        }

        true
    }

    /// Should only be used after can_claim_scroll_space confirmed that this is allowed
    pub fn add_scroll_space_listener(&mut self, behavior: Weak<RefCell<dyn ComponentBehavior>>, region: Region){
        self.area_scroll_listeners.push(AreaScrollHandle {
            behavior,
            region
        });
    }

    pub fn add_move_space_listener(&mut self, behavior: Weak<RefCell<dyn ComponentBehavior>>, region: Region){
        self.area_move_listeners.push(AreaMoveHandle {
            behavior,
            region
        });
    }

    /// Should only be used after can_claim_click_space confirmed that this is allowed
    pub fn add_click_space_listener(&mut self, behavior: Weak<RefCell<dyn ComponentBehavior>>, region: Region){
        self.area_click_listeners.push(AreaClickHandle {
            behavior,
            region
        });
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
        self.in_out_move_listeners.push(InOutMoveHandle {
            behavior,
            region
        });
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

    pub fn fire_mouse_move(&mut self, event: &MouseMoveEvent, manager: &ContainerManager){

        let prev_mouse_pos = manager.get_mouse_position();
        let pixel_next_mouse_pos = (event.mouse_event.offset_x(), event.mouse_event.offset_y());
        let next_mouse_pos = manager.to_gl_coords(pixel_next_mouse_pos);

        self.in_out_move_listeners.drain_filter(|handle| {
            match handle.behavior.upgrade() {
                Some(component_cell) => {
                    if handle.region.is_inside(prev_mouse_pos) != handle.region.is_inside(next_mouse_pos) {
                        component_cell.borrow_mut().mouse_move(&mut MouseMoveParams::new(event, manager));
                    }
                    false
                }, None => true
            }
        });

        self.area_move_listeners.drain_filter(|handle| {
            match handle.behavior.upgrade() {
                Some(component_cell) => {
                    if handle.region.is_inside(prev_mouse_pos) || handle.region.is_inside(next_mouse_pos) {
                        component_cell.borrow_mut().mouse_move(&mut MouseMoveParams::new(event, manager));
                    }
                    false
                }, None => true
            }
        });

        self.full_move_listeners.drain_filter(|handle| {
            match handle.upgrade() {
                Some(component_cell) => {
                    component_cell.borrow_mut().mouse_move(&mut MouseMoveParams::new(event, manager));
                    false
                }, None => true
            }
        });
    }

    pub fn fire_mouse_click(&mut self, event: &MouseClickEvent, manager: &ContainerManager){

        let mouse_pos = manager.get_mouse_position();

        self.area_click_listeners.drain_filter(|handle| {
            match handle.behavior.upgrade() {
                Some(component_cell) => {
                    if handle.region.is_inside(mouse_pos) {
                        component_cell.borrow_mut().mouse_click(&mut MouseClickParams::new(event, manager));
                    }
                    false
                }, None => true
            }
        });

        self.full_click_listeners.drain_filter(|handle| {
            match handle.upgrade() {
                Some(component_cell) => {
                    component_cell.borrow_mut().mouse_click(&mut MouseClickParams::new(event, manager));
                    false
                }, None => true
            }
        });
    }

    pub fn fire_mouse_scroll(&mut self, event: &MouseScrollEvent, manager: &ContainerManager){

        let mouse_pos = manager.get_mouse_position();

        let mut consumed = false;

        self.area_scroll_listeners.drain_filter(|handle| {
            match handle.behavior.upgrade() {
                Some(component_cell) => {
                    if !consumed && handle.region.is_inside(mouse_pos){
                        consumed = component_cell.borrow_mut().mouse_scroll(&mut MouseScrollParams::new(event, manager));
                    }
                    false
                }, None => true
            }
        });

        if !consumed {
            self.full_scroll_listeners.drain_filter(|handle| {
                match handle.behavior.upgrade() {
                    Some(component_cell) => {
                        if !consumed {
                            consumed = component_cell.borrow_mut().mouse_scroll(&mut MouseScrollParams::new(event, manager));
                        }
                        false
                    }, None => true
                }
            });
        }
    }
}