use crate::container::layer::LayerAgent;
use crate::container::layer::agent::RenderAgent;

use crate::cursor::Cursor;

use wasmuri_events::RenderEvent;

use web_sys::WebGlRenderingContext;

pub trait Component {

    fn attach(&mut self, agent: &mut LayerAgent);
}

pub trait RenderingComponent {

    fn render(&mut self, gl: &WebGlRenderingContext, agent: &mut RenderAgent, event: &RenderEvent) -> Option<Cursor>;
}