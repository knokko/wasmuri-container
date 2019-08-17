use crate::{
    Component,
    ContainerManager
};
use crate::cursor::Cursor;

mod region;
mod handle;

// Internal use only
mod render;

use render::{
    RenderManager,
    RenderTrigger
};

use wasmuri_events::{
    MouseMoveEvent,
    RenderEvent
};

use web_sys::WebGlRenderingContext;

pub mod agent;

pub use region::Region;
pub use handle::ComponentHandle;

pub struct Layer {
    
    components: Vec<ComponentHandle>,

    render_manager: RenderManager
}

impl Layer {

    pub fn new() -> Layer {
        Layer {
            components: Vec::with_capacity(10),
            render_manager: RenderManager::new()
        }
    }

    pub fn add_component(&mut self, mut component: Box<dyn Component>){
        let mut agent = LayerAgent::new(self);
        component.attach(&mut agent);

        let handle = ComponentHandle::new(component, self.components.len());

        match agent.render_handle {
            Some(render_handle) => {
                self.render_manager.claim_space(render_handle.0, render_handle.1, handle.create_weak());
            }, None => {}
        };

        self.components.push(handle);
    }

    pub fn on_mouse_move(&mut self, event: &MouseMoveEvent, manager: &ContainerManager){
        self.render_manager.on_mouse_move(event, manager);
    }

    pub fn on_render(&mut self, gl: &WebGlRenderingContext, event: &RenderEvent, manager: &ContainerManager) -> Option<Cursor> {
        let render_result = self.render_manager.render(gl, event, manager);

        let components_to_add = render_result.1;
        for component in components_to_add {
            self.add_component(component);
        }

        render_result.0
    }
}

pub struct LayerAgent<'a> {

    layer: &'a Layer,

    render_handle: Option<(Region,RenderTrigger)>
}

impl<'a> LayerAgent<'a> {

    fn new(layer: &'a Layer) -> LayerAgent {
        LayerAgent {
            layer,

            render_handle: None
        }
    }

    pub fn claim_render_space(&mut self, region: Region, trigger: RenderTrigger) -> Result<(),()> {

        if !self.layer.render_manager.can_claim(region) {
            return Err(());
        }

        self.render_handle = Some((region, trigger));
        Ok(())
    }
}