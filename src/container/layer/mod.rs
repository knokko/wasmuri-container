use crate::{
    Component,
    ContainerManager
};
use crate::cursor::Cursor;

mod region;
mod handle;

mod render;
mod keylistening;

use render::RenderManager;
use keylistening::KeyListenManager;

use wasmuri_core::util::print;
use wasmuri_events::{
    MouseMoveEvent,
    KeyDownEvent,
    KeyUpEvent,
    RenderEvent
};
use wasmuri_core::util::color::Color;

use web_sys::WebGlRenderingContext;

mod agent;

pub use agent::*;
pub use region::Region;
pub use handle::{
    ComponentHandle,
    OuterHandle
};
pub use render::{
    RenderTrigger,
    RenderPhase
};

pub struct Layer {
    
    components: Vec<OuterHandle>,

    key_manager: KeyListenManager,
    render_manager: RenderManager
}

impl Layer {

    pub fn new(background_color: Option<Color>) -> Layer {
        Layer {
            components: Vec::with_capacity(10),
            render_manager: RenderManager::new(background_color),
            key_manager: KeyListenManager::new()
        }
    }

    pub fn on_mouse_move(&mut self, event: &MouseMoveEvent, manager: &ContainerManager){
        self.render_manager.on_mouse_move(event, manager);

        self.check_agents();
    }

    pub fn on_key_down(&mut self, event: &KeyDownEvent, manager: &ContainerManager){
        self.key_manager.fire_key_down(event, manager);
        print("layer.on_key_down");

        self.check_agents();
    }

    pub fn on_key_up(&mut self, event: &KeyUpEvent, manager: &ContainerManager){
        self.key_manager.fire_key_up(event, manager);
        print("layer.on_key_up");

        self.check_agents();
    }

    pub fn on_render(&mut self, gl: &WebGlRenderingContext, event: &RenderEvent, manager: &ContainerManager) -> Option<Cursor> {
        let render_result = self.render_manager.render(gl, event, manager);

        self.check_agents();

        render_result
    }

    fn check_agents(&mut self){
        let mut components_to_add = Vec::new();
        self.components.drain_filter(|outer_handle| {
            let mut handle = outer_handle.get_rc().borrow_mut();
            let agent = handle.get_agent();
            if agent.has_changes() {
                agent.clear_changes();

                if agent.did_request_removal() {
                    return true;
                }

                // The did_request_render will be handled in the render manager of this lay

                components_to_add.append(agent.get_components_to_add());

                // TODO Add support for removing components
            }

            false
        });

        for component in components_to_add {
            self.add_component(component);
        }
    }

    pub fn force_render(&mut self, manager: &ContainerManager){
        self.render_manager.force_render(manager);
    }

    pub fn add_component(&mut self, mut component: Box<dyn Component>){
        let mut agent = LayerAgent::new(self);
        component.attach(&mut agent);

        let render_handle = agent.render_handle;
        let key_down_space = agent.key_down_space;
        let key_up_space = agent.key_up_space;
        let key_down_priority = agent.key_down_priority;
        let key_up_priority = agent.key_up_priority;

        let handle = OuterHandle::new(component, self.components.len());

        match render_handle {
            Some(render_handle) => {
                self.render_manager.claim_space(render_handle.0, render_handle.1, render_handle.2, &handle);
            }, None => {}
        };

        match key_down_space {
            Some(region) => {
                self.key_manager.add_region_key_down_listener(&handle, region);
            }, None => {}
        };

        match key_up_space {
            Some(region) => {
                self.key_manager.add_region_key_up_listener(&handle, region);
            }, None => {}
        };

        match key_down_priority {
            Some(priority) => {
                self.key_manager.add_global_key_down_listener(&handle, priority);
            }, None => {}
        };

        match key_up_priority {
            Some(priority) => {
                self.key_manager.add_global_key_up_listener(&handle, priority);
            }, None => {}
        };

        self.components.push(handle);
    }
}

pub struct LayerAgent<'a> {

    layer: &'a Layer,

    render_handle: Option<(Region,RenderTrigger,RenderPhase)>,

    key_down_space: Option<Region>,
    key_up_space: Option<Region>,

    key_down_priority: Option<i8>,
    key_up_priority: Option<i8>
}

impl<'a> LayerAgent<'a> {

    fn new(layer: &'a Layer) -> LayerAgent {
        LayerAgent {
            layer,

            render_handle: None,
            key_down_space: None,
            key_up_space: None,
            key_down_priority: None,
            key_up_priority: None
        }
    }

    pub fn claim_render_space(&mut self, region: Region, trigger: RenderTrigger, phase: RenderPhase) -> Result<(),()> {

        if !self.layer.render_manager.can_claim(region) {
            return Err(());
        }

        self.render_handle = Some((region, trigger, phase));
        Ok(())
    }

    pub fn claim_key_down_space(&mut self, region: Region) -> Result<(),()> {

        if !self.layer.key_manager.can_claim_down(region) {
            return Err(());
        }

        self.key_down_space = Some(region);
        Ok(())
    }

    pub fn claim_key_up_space(&mut self, region: Region) -> Result<(),()> {

        if !self.layer.key_manager.can_claim_up(region) {
            return Err(());
        }

        self.key_up_space = Some(region);
        Ok(())
    }

    pub fn claim_key_listen_space(&mut self, region: Region) -> Result<(),()> {
        if !self.layer.key_manager.can_claim_down(region) && !self.layer.key_manager.can_claim_up(region) {
            return Err(());
        }

        self.key_down_space = Some(region);
        self.key_up_space = Some(region);
        Ok(())
    }

    pub fn make_key_down_listener(&mut self, priority: i8){
        self.key_down_priority = Some(priority);
    }

    pub fn make_key_up_listener(&mut self, priority: i8){
        self.key_up_priority = Some(priority);
    }

    pub fn make_key_listener(&mut self, priority: i8){
        self.key_down_priority = Some(priority);
        self.key_up_priority = Some(priority);
    }
}