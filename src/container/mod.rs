use crate::{
    EventResult,
    RenderResult,
    ContainerManager
};

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

mod flat;

pub mod layer;

pub use flat::*;

pub trait Container {
    
    fn on_key_down(&mut self, event: &KeyDownEvent, manager: &ContainerManager) -> EventResult;

    fn on_key_up(&mut self, event: &KeyUpEvent, manager: &ContainerManager) -> EventResult;

    fn on_mouse_click(&mut self, event: &MouseClickEvent, manager: &ContainerManager) -> EventResult;

    fn on_mouse_move(&mut self, event: &MouseMoveEvent, manager: &ContainerManager) -> EventResult;

    fn on_mouse_scroll(&mut self, event: &MouseScrollEvent, manager: &ContainerManager) -> EventResult;

    fn on_update(&mut self, event: &UpdateEvent, manager: &ContainerManager) -> EventResult;

    fn render(&mut self, gl: &WebGlRenderingContext, event: &RenderEvent, manager: &ContainerManager) -> RenderResult;

    /// When this method has been called, the Container should re-render everything the next time render is called.
    fn force_render(&mut self, manager: &ContainerManager);
}