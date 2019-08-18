use crate::{
    Component,
    ContainerManager
};
use crate::cursor::Cursor;

use std::cell::RefCell;
use std::rc::Weak;

use super::Region;
use super::agent::RenderAgent;

use wasmuri_core::util::color::Color;
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
    
    component: Weak<RefCell<Box<dyn Component>>>,

    region: Region,

    trigger: RenderTrigger,
    needs_render: bool
}

impl RenderHandle {
    
    fn new(component: Weak<RefCell<Box<dyn Component>>>, region: Region, trigger: RenderTrigger) -> RenderHandle {
        RenderHandle {
            component,
            region,
            trigger,

            // Every component should render the first time
            needs_render: true
        }
    }

    fn should_render(&self) -> bool {
        match &self.trigger {
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
                let mut agent = RenderAgent::new(&self.region, manager);
                agent.remove_this_component();
                (agent, None)
            }
        }
    }
}

pub struct RenderManager {

    render_components: Vec<RenderHandle>,

    background_color: Option<Color>,
    render_background: bool
}

impl RenderManager {

    pub fn new(background_color: Option<Color>) -> RenderManager {
        RenderManager {
            render_components: Vec::with_capacity(10),
            background_color,
            render_background: true
        }
    }

    /// Should only be called after can_claim confirms that the region can be claimed!
    pub fn claim_space(&mut self, region: Region, trigger: RenderTrigger, component: Weak<RefCell<Box<dyn Component>>>) {

        self.render_components.push(RenderHandle::new(component, region, trigger));
    }

    pub fn can_claim(&self, region: Region) -> bool {

        for handle in &self.render_components {
            if handle.region.intersects_with(&region) {
                return false;
            }
        }

        return true;
    }

    pub fn render<'a>(&mut self, gl: &WebGlRenderingContext, event: &RenderEvent, manager: &'a ContainerManager) -> (Option<Cursor>,Vec<Box<dyn Component>>) {

        // Draw the background if necessary
        if self.render_background && self.background_color.is_some() {
            let color = self.background_color.as_ref().unwrap();
            gl.clear_color(color.get_red_float(), color.get_green_float(), color.get_blue_float(), color.get_alpha_float());
            gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

            // Once the background has been drawn, don't redraw until we need to do it again
            self.render_background = false;
        }

        let mut cursor_result = None;

        let mut components_to_add = Vec::new();

        let mouse_position = manager.get_mouse_position();
        self.render_components.drain_filter(|handle| {
            if handle.should_render() {
                let render_result = handle.render(gl, event, manager);
                let mut result_agent = render_result.0;
                let result_cursor = render_result.1;

                let requested_removal = result_agent.did_request_removal();

                components_to_add.append(result_agent.get_components_to_add());

                if result_agent.did_request_render() {
                    handle.needs_render = true;
                }

                // If the mouse is hovering over the element, that element will determine the cursor
                if handle.region.is_inside(mouse_position) {
                    cursor_result = result_cursor;
                }
                // TODO Remove all components in the components_to_remove

                // Only drain the elements that didn't request removal
                requested_removal
            } else {
                false
            }
        });

        (cursor_result, components_to_add)
    }

    pub fn on_mouse_move<'a>(&'a mut self, event: &MouseMoveEvent, manager: &'a ContainerManager){

        let old_mouse_pos = manager.get_mouse_position();
        let new_mouse_pos = manager.to_gl_coords((event.mouse_event.offset_x(), event.mouse_event.offset_y()));
        for handle in &mut self.render_components {
            match &mut handle.trigger {
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