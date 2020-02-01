use crate::*;

mod agent;
mod handle;
mod render;
mod simple;

use std::cell::RefCell;
use std::rc::Rc;

use wasmuri_core::*;

use wasmuri_events::*;

use web_sys::WebGlRenderingContext;

pub use agent::*;
pub use handle::*;
pub use render::*;
pub use simple::*;



pub trait Layer {

    /// If the event is consumed, the remaining layers will get passed a new_pos of None
    fn on_mouse_move(&mut self, new_pos: Option<(f32, f32)>, manager: &ContainerManager) -> ConsumableEventResult;

    fn on_mouse_click(&mut self, click: ClickInfo, manager: &ContainerManager) -> EventResult;

    fn on_mouse_scroll(&mut self, delta: f64, manager: &ContainerManager) -> ConsumableEventResult;

    fn on_key_down(&mut self, event: &KeyDownEvent, manager: &ContainerManager) -> ConsumableEventResult;

    fn on_key_up(&mut self, event: &KeyUpEvent, manager: &ContainerManager) -> ConsumableEventResult;

    fn on_update(&mut self, manager: &ContainerManager) -> EventResult;

    fn predict_render(&mut self) -> Vec<PlannedRenderAction>;

    fn force_partial_render(&mut self, regions: &[Region]) -> Vec<PlannedRenderAction>;

    fn on_render(&mut self, gl: &WebGlRenderingContext, manager: &ContainerManager) -> RenderResult;

    /// Ensures that all components in this layer will render during the next call to on_render()
    fn force_render(&mut self);

    fn add_component(&mut self, component: Rc<RefCell<dyn Component>>);
}

pub trait LayerAgent {

    fn claim_render_space(&mut self, region: Region, trigger: RenderTrigger, opacity: RenderOpacity, phase: RenderPhase) -> Result<(),()>;

    fn claim_key_down_space(&mut self, region: Region) -> Result<(),()>;

    fn claim_key_up_space(&mut self, region: Region) -> Result<(),()>;

    fn claim_key_listen_space(&mut self, region: Region) -> Result<(),()>;

    fn make_key_down_listener(&mut self, priority: i8);

    fn make_key_up_listener(&mut self, priority: i8);

    fn make_key_listener(&mut self, priority: i8);

    fn claim_mouse_click_space(&mut self, region: Region) -> Result<(),()>;

    fn claim_mouse_scroll_space(&mut self, region: Region) -> Result<(),()>;

    fn make_mouse_scroll_listener(&mut self, priority: i8);

    fn claim_mouse_move_space(&mut self, region: Region);

    fn claim_mouse_in_out_space(&mut self, region: Region);

    fn make_mouse_move_listener(&mut self);

    fn make_mouse_click_listener(&mut self);

    fn make_update_listener(&mut self);
}