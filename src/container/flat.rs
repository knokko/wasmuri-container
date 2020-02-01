use crate::*;

use std::cell::RefCell;
use std::rc::Rc;

use wasmuri_events::*;

use web_sys::WebGlRenderingContext;

pub struct FlatContainer {

    layer: Box<dyn Layer>
}

impl FlatContainer {

    pub fn new(layer: Box<dyn Layer>) -> FlatContainer {
        FlatContainer {
            layer
        }
    }

    pub fn celled(layer: Box<dyn Layer>) -> Rc<RefCell<FlatContainer>> {
        Rc::new(RefCell::new(Self::new(layer)))
    }
}

impl std::fmt::Debug for FlatContainer {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FlatContainer")
    }
}

impl Container for FlatContainer {

    fn on_key_down(&mut self, keys: &KeyInfo, manager: &ContainerManager) -> EventResult {
        self.layer.on_key_down(keys, manager).as_normal_result()
    }

    fn on_key_up(&mut self, keys: &KeyInfo, manager: &ContainerManager) -> EventResult {
        self.layer.on_key_up(keys, manager).as_normal_result()
    }

    fn on_mouse_click(&mut self, click: ClickInfo, manager: &ContainerManager) -> EventResult {
        self.layer.on_mouse_click(click, manager)
    }

    fn on_mouse_move(&mut self, event: &MouseMoveEvent, manager: &ContainerManager) -> EventResult {
        self.layer.on_mouse_move(Some(manager.to_gl_coords(event.get_new_position())), manager).as_normal_result()
    }

    fn on_mouse_scroll(&mut self, event: &MouseScrollEvent, manager: &ContainerManager) -> EventResult {
        self.layer.on_mouse_scroll(event.mouse_event.delta_y(), manager).as_normal_result()
    }

    fn on_update(&mut self, manager: &ContainerManager) -> EventResult {
        self.layer.on_update(manager)
    }

    fn render(&mut self, gl: &WebGlRenderingContext, manager: &ContainerManager) -> ContainerRenderResult {
        let maybe_cursor = self.layer.on_render(gl, manager).get_cursor();
        match maybe_cursor {
            Some(cursor) => cursor,
            None => Cursor::DEFAULT
        }
    }

    fn force_render(&mut self){
        self.layer.force_render();
    }
}