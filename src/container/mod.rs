use crate::{
    EventResult,
    RenderResult
};

use wasmuri_events::{
    KeyDownEvent,
    KeyUpEvent,
    MouseClickEvent,
    MouseMoveEvent,
    MouseScrollEvent
};

use web_sys::WebGlRenderingContext;

mod flat;

pub mod layer;

pub use flat::*;

pub trait Container {
    
    fn on_key_down(&mut self, event: &KeyDownEvent) -> EventResult;

    fn on_key_up(&mut self, event: &KeyUpEvent) -> EventResult;

    fn on_mouse_click(&mut self, event: &MouseClickEvent) -> EventResult;

    fn on_mouse_move(&mut self, event: &MouseMoveEvent) -> EventResult;

    fn on_mouse_scroll(&mut self, event: &MouseScrollEvent) -> EventResult;

    fn on_update(&mut self) -> EventResult;

    fn render(&self, gl: &WebGlRenderingContext) -> RenderResult;
}