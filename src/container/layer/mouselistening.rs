use std::cell::RefCell;
use std::rc::Weak;

use crate::ContainerManager;

use super::{
    ComponentHandle,
    OuterHandle,
    Region
};

use wasmuri_events::{
    MouseClickEvent,
    MouseMoveEvent,
    MouseScrollEvent
};

struct RegionHandle {

    component: Weak<RefCell<ComponentHandle>>,

    region: Region
}

struct FullHandle {

    component: Weak<RefCell<ComponentHandle>>,

    priority: i8
}

type AreaClickHandle = RegionHandle;

type FullClickHandle = Weak<RefCell<ComponentHandle>>;

type AreaScrollHandle = RegionHandle;

type FullScrollHandle = FullHandle;

type AreaMoveHandle = RegionHandle;

type FullMoveHandle = Weak<RefCell<ComponentHandle>>;

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
    pub fn add_scroll_space_listener(&mut self, listener: &OuterHandle, region: Region){
        self.area_scroll_listeners.push(AreaScrollHandle {
            component: listener.create_weak(),
            region
        });
    }

    pub fn add_move_space_listener(&mut self, listener: &OuterHandle, region: Region){
        self.area_move_listeners.push(AreaMoveHandle {
            component: listener.create_weak(),
            region
        });
    }

    /// Should only be used after can_claim_click_space confirmed that this is allowed
    pub fn add_click_space_listener(&mut self, listener: &OuterHandle, region: Region){
        self.area_click_listeners.push(AreaClickHandle {
            component: listener.create_weak(),
            region
        });
    }

    pub fn add_full_click_listener(&mut self, listener: &OuterHandle){
        self.full_click_listeners.push(listener.create_weak());
    }

    pub fn add_full_scroll_listener(&mut self, listener: &OuterHandle, priority: i8){
        Self::add_full_listener(&mut self.full_scroll_listeners, listener, priority);
    }

    pub fn add_full_move_listener(&mut self, listener: &OuterHandle){
        self.full_move_listeners.push(listener.create_weak());
    }

    pub fn add_in_out_move_listener(&mut self, listener: &OuterHandle, region: Region){
        self.in_out_move_listeners.push(InOutMoveHandle {
            component: listener.create_weak(),
            region
        });
    }

    fn add_full_listener(list: &mut Vec<FullHandle>, listener: &OuterHandle, priority: i8){
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
            component: listener.create_weak(),
            priority
        });
    }

    pub fn fire_mouse_move(&mut self, event: &MouseMoveEvent, manager: &ContainerManager){

        let prev_mouse_pos = manager.get_mouse_position();
        let pixel_next_mouse_pos = (event.mouse_event.offset_x(), event.mouse_event.offset_y());
        let next_mouse_pos = manager.to_gl_coords(pixel_next_mouse_pos);

        self.in_out_move_listeners.drain_filter(|handle| {
            match handle.component.upgrade() {
                Some(component_cell) => {
                    if handle.region.is_inside(prev_mouse_pos) != handle.region.is_inside(next_mouse_pos) {
                        component_cell.borrow_mut().mouse_move(event, manager);
                    }
                    false
                }, None => true
            }
        });

        self.area_move_listeners.drain_filter(|handle| {
            match handle.component.upgrade() {
                Some(component_cell) => {
                    if handle.region.is_inside(prev_mouse_pos) || handle.region.is_inside(next_mouse_pos) {
                        component_cell.borrow_mut().mouse_move(event, manager);
                    }
                    false
                }, None => true
            }
        });

        self.full_move_listeners.drain_filter(|handle| {
            match handle.upgrade() {
                Some(component_cell) => {
                    component_cell.borrow_mut().mouse_move(event, manager);
                    false
                }, None => true
            }
        });
    }

    pub fn fire_mouse_click(&mut self, event: &MouseClickEvent, manager: &ContainerManager){

        let mouse_pos = manager.get_mouse_position();

        self.area_click_listeners.drain_filter(|handle| {
            match handle.component.upgrade() {
                Some(component_cell) => {
                    if handle.region.is_inside(mouse_pos) {
                        component_cell.borrow_mut().mouse_click(event, manager);
                    }
                    false
                }, None => true
            }
        });

        self.full_click_listeners.drain_filter(|handle| {
            match handle.upgrade() {
                Some(component_cell) => {
                    component_cell.borrow_mut().mouse_click(event, manager);
                    false
                }, None => true
            }
        });
    }

    pub fn fire_mouse_scroll(&mut self, event: &MouseScrollEvent, manager: &ContainerManager){

        let mouse_pos = manager.get_mouse_position();

        let mut consumed = false;

        self.area_scroll_listeners.drain_filter(|handle| {
            match handle.component.upgrade() {
                Some(component_cell) => {
                    if !consumed && handle.region.is_inside(mouse_pos){
                        consumed = component_cell.borrow_mut().mouse_scroll(event, manager);
                    }
                    false
                }, None => true
            }
        });

        if !consumed {
            self.full_scroll_listeners.drain_filter(|handle| {
                match handle.component.upgrade() {
                    Some(component_cell) => {
                        if !consumed {
                            consumed = component_cell.borrow_mut().mouse_scroll(event, manager);
                        }
                        false
                    }, None => true
                }
            });
        }
    }
}