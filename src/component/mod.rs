use crate::container::layer::*;
use crate::ContainerManager;
use crate::cursor::Cursor;

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

pub trait Component {

    fn attach(&mut self, agent: &mut LayerAgent);

    fn key_down(&mut self, _event: &KeyDownEvent, _manager: &ContainerManager) -> bool {
        panic!("The keydown operation is not supported for this component!");
    }

    fn key_up(&mut self, _event: &KeyUpEvent, _manager: &ContainerManager) -> bool {
        panic!("The keyup operation is not supported for this component!");
    }

    fn mouse_click(&mut self, _agent: &mut MouseClickAgent, _event: &MouseClickEvent){
        panic!("The mouseclick operation is not supported for this component!");
    }

    fn mouse_move(&mut self, _agent: &mut MouseMoveAgent, _event: &MouseMoveEvent){
        panic!("The mouseclick operation is not supported for this component!");
    }

    fn mouse_scroll(&mut self, _agent: &mut MouseScrollAgent, _event: &MouseScrollEvent){
        panic!("The mousescroll operation is not supported for this component!");
    }

    fn render(&mut self, _gl: &WebGlRenderingContext, _agent: &mut ComponentAgent, _event: &RenderEvent, manager: &ContainerManager) -> Option<Cursor> {
        panic!("The render operation is not supported for this component!");
    }

    fn get_cursor(&mut self, _agent: &mut ComponentAgent, _event: &RenderEvent, manager: &ContainerManager) -> Option<Cursor> {
        panic!("The get_cursor operation is not supported for this component!");
    }

    fn update(&mut self, _agent: &mut UpdateAgent, _event: &UpdateEvent){
        panic!("The update operation is not supported for this component!");
    }
}