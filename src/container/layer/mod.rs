use crate::Component;

use wasmuri_events::{
    Listener,
    RenderEvent
};

mod region;
mod handle;

// Internal use only
mod render;

pub mod agent;

pub use region::Region;
pub use handle::ComponentHandle;

pub struct Layer {
    
    components: Vec<ComponentHandle>
}

impl Layer {

    pub fn add_component(&mut self, component: Box<dyn Component>){
        let mut agent = LayerAgent {
            layer: self
        };
        component.attach(&mut agent);
        let handle = ComponentHandle::new(component, self.components.len());
        self.components.push(handle);
    }

    fn claim_render_space(&mut self, region: Region, renderer: Box<dyn Listener<RenderEvent>>){
        //
    }
}

pub struct LayerAgent<'a> {

    layer: &'a mut Layer
}

impl<'a> LayerAgent<'a> {

    pub fn claim_render_space(&mut self, region: Region, renderer: Box<dyn Listener<RenderEvent>>){
        self.layer.claim_render_space(region, renderer);
    }
}