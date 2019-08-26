use crate::{
    EventResult,
    RenderResult,
    ContainerManager
};
use crate::cursor::Cursor;

use super::Container;
use super::layer::Layer;

use wasmuri_core::util::print;
use wasmuri_events::{
    KeyDownEvent,
    KeyUpEvent,
    MouseClickEvent,
    MouseMoveEvent,
    MouseScrollEvent,
    RenderEvent,
    UpdateEvent
};

use web_sys::WebGlRenderingContext;

pub struct FlatContainer {

    layer: Layer
}

impl FlatContainer {

    pub fn new(layer: Layer) -> FlatContainer {
        FlatContainer {
            layer
        }
    }
}

// TODO Implement the trait methods properly, this is just for the initial test
impl Container for FlatContainer {

    fn on_key_down(&mut self, event: &KeyDownEvent, manager: &ContainerManager) -> EventResult {
        self.layer.on_key_down(event, manager);
        print("FlatContainer.on_key_down");
        None
    }

    fn on_key_up(&mut self, event: &KeyUpEvent, manager: &ContainerManager) -> EventResult {
        self.layer.on_key_up(event, manager);
        print("FlatContainer.on_key_up");
        None
    }

    fn on_mouse_click(&mut self, _event: &MouseClickEvent, _manager: &ContainerManager) -> EventResult {
        None
    }

    fn on_mouse_move(&mut self, event: &MouseMoveEvent, manager: &ContainerManager) -> EventResult {
        self.layer.on_mouse_move(event, manager);
        None
    }

    fn on_mouse_scroll(&mut self, _event: &MouseScrollEvent, _manager: &ContainerManager) -> EventResult {
        None
    }

    fn on_update(&mut self, _event: &UpdateEvent, _manager: &ContainerManager) -> EventResult {
        None
    }

    fn render(&mut self, gl: &WebGlRenderingContext, event: &RenderEvent, manager: &ContainerManager) -> RenderResult {
        let maybe_cursor = self.layer.on_render(gl, event, manager);
        match maybe_cursor {
            Some(cursor) => cursor,
            None => Cursor::DEFAULT
        }
    }

    fn force_render(&mut self, manager: &ContainerManager){
        self.layer.force_render(manager);
    }
}