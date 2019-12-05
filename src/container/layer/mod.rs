use crate::*;
use crate::manager::EventResult;

mod handle;

mod update;
mod render;
mod keylistening;
mod mouselistening;

use render::RenderManager;
use update::UpdateManager;
use keylistening::KeyListenManager;
use mouselistening::MouseManager;

use std::cell::RefCell;
use std::rc::Rc;

use wasmuri_core::color::Color;
use wasmuri_core::util::*;

use wasmuri_events::*;

use web_sys::WebGlRenderingContext;

mod agent;

pub use agent::*;
pub use handle::{
    ComponentHandle,
    OuterHandle
};
pub use render::{
    RenderTrigger,
    RenderPhase,
    RenderOpacity,
    RenderAction,
    RenderResult
};

pub struct Layer {
    
    components: Vec<OuterHandle>,

    key_manager: KeyListenManager,
    mouse_manager: MouseManager,
    update_manager: UpdateManager,
    render_manager: RenderManager
}

impl Layer {

    pub fn new(background_color: Option<Color>) -> Layer {
        Layer {
            components: Vec::with_capacity(10),
            render_manager: RenderManager::new(background_color),
            update_manager: UpdateManager::new(),
            key_manager: KeyListenManager::new(),
            mouse_manager: MouseManager::new()
        }
    }

    pub fn on_mouse_move(&mut self, event: &MouseMoveEvent, manager: &ContainerManager) -> EventResult {
        self.mouse_manager.fire_mouse_move(event, manager);
        self.render_manager.on_mouse_move(event, manager);

        self.check_agents()
    }

    pub fn on_mouse_click(&mut self, event: &MouseClickEvent, manager: &ContainerManager) -> EventResult {
        self.mouse_manager.fire_mouse_click(event, manager);

        self.check_agents()
    }

    pub fn on_mouse_scroll(&mut self, event: &MouseScrollEvent, manager: &ContainerManager) -> EventResult{
        self.mouse_manager.fire_mouse_scroll(event, manager);

        self.check_agents()
    }

    pub fn on_key_down(&mut self, event: &KeyDownEvent, manager: &ContainerManager) -> EventResult {
        self.key_manager.fire_key_down(event, manager);

        self.check_agents()
    }

    pub fn on_key_up(&mut self, event: &KeyUpEvent, manager: &ContainerManager) -> EventResult {
        self.key_manager.fire_key_up(event, manager);

        self.check_agents()
    }

    pub fn on_update(&mut self, event: &UpdateEvent, manager: &ContainerManager) -> EventResult {
        self.update_manager.fire_update(event, manager);

        self.check_agents()
    }

    pub fn on_render(&mut self, gl: &WebGlRenderingContext, event: &RenderEvent, manager: &ContainerManager) -> RenderResult {
        let render_result = self.render_manager.render(gl, event, manager);

        self.check_agents().expect_none("A component attempted to replace the current container during a render event");

        render_result
    }

    fn check_agents(&mut self) -> EventResult {
        let mut components_to_add = Vec::new();
        let mut new_container = None;
        self.components.drain_filter(|outer_handle| {
            let mut handle = outer_handle.get_rc().borrow_mut();
            let mut agent = handle.get_agent();
            if agent.has_changes() {
                agent.clear_changes();

                if agent.requested_container_change() {
                    if new_container.is_some() {
                        print("Warning: 2 components requested a container change during the same event");
                    }
                    new_container = Some(agent.get_new_container());
                }

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

        new_container
    }

    pub fn force_render(&mut self, manager: &ContainerManager){
        self.render_manager.force_render(manager);
    }

    pub fn add_component(&mut self, component: Rc<RefCell<dyn Component>>) {
        let behaviors = component.borrow_mut().create_behaviors();
        for behavior in &behaviors {
            let mut agent = LayerAgent::new(self);
            behavior.borrow_mut().attach(&mut agent);

            let render_handle = agent.render_handle;

            let key_down_space = agent.key_down_space;
            let key_up_space = agent.key_up_space;
            let key_down_priority = agent.key_down_priority;
            let key_up_priority = agent.key_up_priority;

            let mouse_click_space = agent.mouse_click_space;
            let mouse_click_global = agent.mouse_click_global;
            let mouse_scroll_space = agent.mouse_scroll_space;
            let mouse_scroll_priority = agent.mouse_scroll_priority;
            let mouse_move_space = agent.mouse_move_space;
            let mouse_move_in_out_space = agent.mouse_move_in_out_space;
            let mouse_move_global = agent.mouse_move_global;

            let receive_updates = agent.receive_updates;

            match render_handle {
                Some(render_handle) => {
                    self.render_manager.claim_space(render_handle.0, render_handle.1, render_handle.2, Rc::downgrade(&behavior));
                }, None => {}
            };

            match key_down_space {
                Some(region) => {
                    self.key_manager.add_region_key_down_listener(Rc::downgrade(&behavior), region);
                }, None => {}
            };

            match key_up_space {
                Some(region) => {
                    self.key_manager.add_region_key_up_listener(Rc::downgrade(&behavior), region);
                }, None => {}
            };

            match key_down_priority {
                Some(priority) => {
                    self.key_manager.add_global_key_down_listener(Rc::downgrade(&behavior), priority);
                }, None => {}
            };

            match key_up_priority {
                Some(priority) => {
                    self.key_manager.add_global_key_up_listener(Rc::downgrade(&behavior), priority);
                }, None => {}
            };

            match mouse_click_space {
                Some(space) => {
                    self.mouse_manager.add_click_space_listener(Rc::downgrade(&behavior), space);
                }, None => {}
            };
            
            if mouse_click_global {
                self.mouse_manager.add_full_click_listener(Rc::downgrade(&behavior));
            }

            match mouse_scroll_space {
                Some(space) => {
                    self.mouse_manager.add_scroll_space_listener(Rc::downgrade(&behavior), space);
                }, None => {}
            };

            match mouse_scroll_priority {
                Some(priority) => {
                    self.mouse_manager.add_full_scroll_listener(Rc::downgrade(&behavior), priority);
                }, None => {}
            };

            match mouse_move_space {
                Some(space) => {
                    self.mouse_manager.add_move_space_listener(Rc::downgrade(&behavior), space);
                }, None => {}
            };

            match mouse_move_in_out_space {
                Some(space) => {
                    self.mouse_manager.add_in_out_move_listener(Rc::downgrade(&behavior), space);
                }, None => {}
            };

            if mouse_move_global {
                self.mouse_manager.add_full_move_listener(Rc::downgrade(&behavior));
            }

            if receive_updates {
                self.update_manager.add_listener(Rc::downgrade(&behavior));
            }
        }

        self.components.push(OuterHandle::new(component, behaviors));
    }
}

pub struct LayerAgent<'a> {

    layer: &'a Layer,

    render_handle: Option<(Region,RenderTrigger,RenderPhase)>,

    key_down_space: Option<Region>,
    key_up_space: Option<Region>,

    key_down_priority: Option<i8>,
    key_up_priority: Option<i8>,

    mouse_click_space: Option<Region>,
    mouse_click_global: bool,

    mouse_scroll_space: Option<Region>,
    mouse_scroll_priority: Option<i8>,

    mouse_move_space: Option<Region>,
    mouse_move_in_out_space: Option<Region>,
    mouse_move_global: bool,

    receive_updates: bool
}

impl<'a> LayerAgent<'a> {

    fn new(layer: &'a Layer) -> LayerAgent {
        LayerAgent {
            layer,

            render_handle: None,

            key_down_space: None,
            key_up_space: None,

            key_down_priority: None,
            key_up_priority: None,

            mouse_click_space: None,
            mouse_click_global: false,

            mouse_scroll_space: None,
            mouse_scroll_priority: None,

            mouse_move_space: None,
            mouse_move_in_out_space: None,
            mouse_move_global: false,

            receive_updates: false
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

    pub fn claim_mouse_click_space(&mut self, region: Region) -> Result<(),()> {
        if !self.layer.mouse_manager.can_claim_click_space(region) {
            return Err(());
        }

        self.mouse_click_space = Some(region);
        Ok(())
    }

    pub fn claim_mouse_scroll_space(&mut self, region: Region) -> Result<(),()> {
        if !self.layer.mouse_manager.can_claim_scroll_space(region) {
            return Err(());
        }

        self.mouse_scroll_space = Some(region);
        Ok(())
    }

    pub fn make_mouse_scroll_listener(&mut self, priority: i8) {
        self.mouse_scroll_priority = Some(priority);
    }

    pub fn claim_mouse_move_space(&mut self, region: Region){
        self.mouse_move_space = Some(region);
    }

    pub fn claim_mouse_in_out_space(&mut self, region: Region){
        self.mouse_move_in_out_space = Some(region);
    }

    pub fn make_mouse_move_listener(&mut self){
        self.mouse_move_global = true;
    }

    pub fn make_mouse_click_listener(&mut self){
        self.mouse_click_global = true;
    }

    pub fn make_update_listener(&mut self){
        self.receive_updates = true;
    }
}