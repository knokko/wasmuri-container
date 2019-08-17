use crate::component::RenderingComponent;
use crate::ContainerManager;
use crate::cursor::Cursor;

use std::cell::RefCell;
use std::rc::Weak;

use super::Region;
use super::agent::RenderAgent;

use wasmuri_events::{
    MouseMoveEvent,
    RenderEvent
};

use web_sys::WebGlRenderingContext;

pub enum RenderTrigger {

    Request,
    MouseInOut,
    MouseMoveInside,
    MouseMove,
    Always
}

struct RenderHandle {
    
    component: Weak<RefCell<Box<dyn RenderingComponent>>>,

    region: Region,

    trigger: RenderTrigger,
    needs_render: bool
}

impl RenderHandle {
    
    fn new(component: Weak<RefCell<Box<dyn RenderingComponent>>>, region: Region, trigger: RenderTrigger) -> RenderHandle {
        RenderHandle {
            component,
            region,
            trigger,

            // Every component should render the first time
            needs_render: true
        }
    }

    fn should_render(&self) -> bool {
        match self.trigger {
            RenderTrigger::Always => true,
            _others => self.needs_render
        }
    }

    fn render<'a>(&'a mut self, gl: &WebGlRenderingContext, event: &RenderEvent, manager: &'a ContainerManager) -> (RenderAgent<'a>,Option<Cursor>) {
        match self.component.upgrade() {
            Some(component_cell) => {
                let mut component_box = component_cell.borrow_mut();
                let mut agent = RenderAgent::new(&self.region, manager);

                let cursor = component_box.render(gl, &mut agent, event);

                // Set this to false after every render
                self.needs_render = false;

                (agent, cursor)
            }, None => {

                // Remove the component from the list
                let agent = RenderAgent::new(&self.region, manager);
                agent.remove_this_component();
                (agent, None)
            }
        }
    }
}

pub struct RenderManager {

    render_components: Vec<RenderHandle>
}

impl RenderManager {

    pub fn new() -> RenderManager {
        RenderManager {
            render_components: Vec::with_capacity(10)
        }
    }

    pub fn render<'a>(&mut self, gl: &WebGlRenderingContext, event: &RenderEvent, manager: &'a ContainerManager) -> Option<Cursor> {
        let mut cursor_result = None;

        let mut components_to_add = Vec::new();

        let mouse_position = manager.get_mouse_position();
        self.render_components.drain_filter(|handle| {
            let render_result = handle.render(gl, event, manager);

            // If the mouse is hovering over the element, that element will determine the cursor
            if handle.region.is_inside(mouse_position) {
                cursor_result = render_result.1;
            }

            let agent_result = render_result.0;
            if agent_result.did_request_render() {
                handle.needs_render = true;
            }

            components_to_add.append(agent_result.get_components_to_add());
            // TODO Remove all components in the components_to_remove

            // Only drain the elements that didn't request removal
            agent_result.did_request_removal()
        });

        // TODO Add all components from components_to_add

        cursor_result
    }

    pub fn on_mouse_move<'a>(&'a mut self, event: &MouseMoveEvent, manager: &'a ContainerManager){

        let old_mouse_pos = manager.get_mouse_position();
        let new_mouse_pos = manager.to_gl_coords((event.mouse_event.offset_x(), event.mouse_event.offset_y()));
        for handle in self.render_components {
            match handle.trigger {
                RenderTrigger::MouseMove => handle.needs_render = true,
                RenderTrigger::MouseMoveInside => {
                    let was_in = handle.region.is_inside(old_mouse_pos);
                    let is_in = handle.region.is_inside(new_mouse_pos);
                    if was_in || is_in {
                        handle.needs_render = true;
                    }
                }, RenderTrigger::MouseInOut => {
                    let was_in = handle.region.is_inside(old_mouse_pos);
                    let is_in = handle.region.is_inside(new_mouse_pos);
                    if was_in != is_in {
                        handle.needs_render = true;
                    }
                }, _other => {}
            };
        }
    }
}