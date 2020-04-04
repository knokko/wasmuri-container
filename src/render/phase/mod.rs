use crate::*;

use std::fmt::{Debug, Error, Formatter};

use web_sys::WebGlRenderingContext as GL;

mod id;
mod store;

pub use id::*;
pub use store::*;

pub trait RenderPhase {

    fn get_priority(&self) -> RenderPriority;

    fn start(&self, gl: GL, context: RenderContext);

    fn stop(&self, gl: GL, context: RenderContext);
}

impl Debug for dyn RenderPhase {

    // No extensive debugging for this
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        write!(formatter, "RenderPhase")
    }
}