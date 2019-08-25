use crate::{
    Component,
    ContainerManager
};
use crate::cursor::Cursor;

use std::cell::RefCell;
use std::cmp::{
    PartialEq,
    Eq,
    PartialOrd,
    Ord
};
use std::rc::Weak;

use super::{
    ComponentHandle,
    OuterHandle,
    Region
};

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

#[derive(PartialEq,Eq,PartialOrd,Ord,Clone,Copy)]
pub enum RenderPhase {

    Start,
    Text,
    End
}

struct RenderHandle {
    
    component: Weak<RefCell<ComponentHandle>>,

    region: Region,

    trigger: RenderTrigger,
    phase: RenderPhase
}

impl RenderHandle {
    
    fn new(component: &OuterHandle, region: Region, trigger: RenderTrigger, phase: RenderPhase) -> RenderHandle {
        RenderHandle {
            component: component.create_weak(),
            region,
            trigger,
            phase,
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
    pub fn claim_space(&mut self, region: Region, trigger: RenderTrigger, phase: RenderPhase, component: &OuterHandle) {

        let maybe_index = self.render_components.binary_search_by(|other: &RenderHandle| {
            other.phase.cmp(&phase)
        });
        let index;
        match maybe_index {
            Ok(the_index) => index = the_index,
            Err(the_index) => index = the_index
        };
        self.render_components.insert(index, RenderHandle::new(component, region, trigger, phase));
    }

    pub fn can_claim(&self, region: Region) -> bool {

        for handle in &self.render_components {
            if handle.region.intersects_with(&region) {
                return false;
            }
        }

        return true;
    }

    pub fn render<'a>(&mut self, gl: &WebGlRenderingContext, event: &RenderEvent, manager: &'a ContainerManager) -> Option<Cursor> {

        // Draw the background if necessary
        if self.render_background && self.background_color.is_some() {
            let color = self.background_color.as_ref().unwrap();
            gl.clear_color(color.get_red_float(), color.get_green_float(), color.get_blue_float(), color.get_alpha_float());
            gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

            // Once the background has been drawn, don't redraw until we need to do it again
            self.render_background = false;
        }

        let mut cursor_result = None;

        let mouse_position = manager.get_mouse_position();

        let mut previous_render_phase = RenderPhase::Start;

        self.render_components.drain_filter(|handle| {
            match handle.component.upgrade() {
                Some(component_cell) => {
                    let mut component_handle = component_cell.borrow_mut();
                    let requested_render = component_handle.get_agent().did_request_render();
                    if requested_render {

                        if previous_render_phase != handle.phase {

                            // Currently, the Text phase is the only phase that has built-in support, other phases will have to prepare themselves
                            if handle.phase == RenderPhase::Text {
                                manager.get_text_renderer().borrow_mut().start_rendering();
                            }

                            previous_render_phase = handle.phase;
                        }

                        let local_cursor = component_handle.render(gl, event, manager);
                        if handle.region.is_inside(mouse_position) {
                            cursor_result = local_cursor;
                        }
                        
                        false
                    } else {

                        if handle.region.is_inside(mouse_position) {
                            cursor_result = component_handle.get_cursor(event, manager);
                        }

                        false
                    }
                }, None => true
            }
        });

        cursor_result
    }

    pub fn force_render(&mut self, _manager: &ContainerManager){
        self.render_background = true;
    }

    pub fn on_mouse_move<'a>(&'a mut self, event: &MouseMoveEvent, manager: &'a ContainerManager){

        let old_mouse_pos = manager.get_mouse_position();
        let new_mouse_pos = manager.to_gl_coords((event.mouse_event.offset_x(), event.mouse_event.offset_y()));
        for handle in &mut self.render_components {
            let needs_render = match &mut handle.trigger {
                RenderTrigger::MouseMove => true,
                RenderTrigger::MouseMoveInside => {
                    let was_in = handle.region.is_inside(old_mouse_pos);
                    let is_in = handle.region.is_inside(new_mouse_pos);
                    was_in || is_in
                }, RenderTrigger::MouseInOut => {
                    let was_in = handle.region.is_inside(old_mouse_pos);
                    let is_in = handle.region.is_inside(new_mouse_pos);
                    was_in != is_in
                }, _other => false
            };

            if needs_render {
                match handle.component.upgrade() {
                    Some(component_cell) => {
                        let mut the_component = component_cell.borrow_mut();
                        the_component.get_agent().request_render();
                    }, None => {
                        // If the component happens to be dropped, it will be removed from the vec during the next frame
                    }
                };
            }
        }
    }
}