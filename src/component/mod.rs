use crate::container::layer::LayerAgent;
use crate::container::layer::agent::RenderAgent;

use crate::cursor::Cursor;

use wasmuri_events::RenderEvent;

use web_sys::WebGlRenderingContext;

pub trait Component {

    fn attach(&mut self, agent: &mut LayerAgent);

    fn render(&mut self, _gl: &WebGlRenderingContext, _agent: &mut RenderAgent, _event: &RenderEvent) -> Option<Cursor> {
        panic!("The render operation is not supported for this component!");
    }
}