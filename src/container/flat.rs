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

    fn on_key_down(&mut self, event: &KeyDownEvent, manager: &ContainerManager) -> EventResult {
        self.layer.on_key_down(event, manager).as_normal_result()
    }

    fn on_key_up(&mut self, event: &KeyUpEvent, manager: &ContainerManager) -> EventResult {
        self.layer.on_key_up(event, manager).as_normal_result()
    }

    fn on_mouse_click(&mut self, event: &MouseClickEvent, manager: &ContainerManager) -> EventResult {
        self.layer.on_mouse_click(event, manager).as_normal_result()
    }

    fn on_mouse_move(&mut self, event: &MouseMoveEvent, manager: &ContainerManager) -> EventResult {
        self.layer.on_mouse_move(event, manager)
    }

    fn on_mouse_scroll(&mut self, event: &MouseScrollEvent, manager: &ContainerManager) -> EventResult {
        self.layer.on_mouse_scroll(event, manager).as_normal_result()
    }

    fn on_update(&mut self, event: &UpdateEvent, manager: &ContainerManager) -> EventResult {
        self.layer.on_update(event, manager)
    }

    fn render(&mut self, gl: &WebGlRenderingContext, event: &RenderEvent, manager: &ContainerManager) -> ContainerRenderResult {
        let maybe_cursor = self.layer.on_render(gl, event, manager).get_cursor();
        match maybe_cursor {
            Some(cursor) => cursor,
            None => Cursor::DEFAULT
        }
    }

    fn force_render(&mut self){
        self.layer.force_render();
    }
}