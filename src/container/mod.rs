use crate::*;

use wasmuri_events::*;

use web_sys::WebGlRenderingContext;

mod flat;
mod layered;

pub use flat::*;
pub use layered::*;

pub trait Container : std::fmt::Debug {
    
    fn on_key_down(&mut self, event: &KeyDownEvent, manager: &ContainerManager) -> EventResult;

    fn on_key_up(&mut self, event: &KeyUpEvent, manager: &ContainerManager) -> EventResult;

    fn on_mouse_click(&mut self, event: &MouseClickEvent, manager: &ContainerManager) -> EventResult;

    fn on_mouse_move(&mut self, event: &MouseMoveEvent, manager: &ContainerManager) -> EventResult;

    fn on_mouse_scroll(&mut self, event: &MouseScrollEvent, manager: &ContainerManager) -> EventResult;

    fn on_update(&mut self, event: &UpdateEvent, manager: &ContainerManager) -> EventResult;

    fn render(&mut self, gl: &WebGlRenderingContext, event: &RenderEvent, manager: &ContainerManager) -> ContainerRenderResult;

    /// When this method has been called, the Container should re-render everything the next time render is called.
    fn force_render(&mut self);
}