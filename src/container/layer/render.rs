use crate::*;
use crate::params::*;

use std::cell::RefCell;
use std::cmp::{
    PartialEq,
    Eq,
    PartialOrd,
    Ord
};
use std::rc::Weak;

use super::Region;

use wasmuri_core::color::Color;
use wasmuri_events::{
    MouseMoveEvent,
    RenderEvent
};

use web_sys::WebGlRenderingContext;

#[derive(Clone,Copy)]
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

#[derive(Clone,Copy,PartialEq,Eq)]
pub enum RenderOpacity {

    /// Each pixel in the render area is fully opaque
    Solid,

    /// Each pixel in the render area is either fully opaque or fully transparent.
    /// Which pixels are opaque and which are transparent only changes when the viewport changes.
    StaticSolidOrNothing,

    /// Each pixel in the render area is either fully opaque or fully transparent.
    /// Which pixels are opaque and which are transparent can change at any time.
    DynamicSolidOrNothing,

    /// Each pixel can have any transparency
    Mixed
}

#[derive(Clone,Copy)]
pub struct RenderAction {

    region: Region,
    opacity: RenderOpacity
}

impl RenderAction {

    pub fn get_region(&self) -> Region {
        self.region
    }

    pub fn get_opacity(&self) -> RenderOpacity {
        self.opacity
    }
}

#[derive(Clone)]
pub struct RenderResult {

    cursor: Option<Cursor>,
    actions: Vec<RenderAction>
}

impl RenderResult {

    pub fn get_cursor(&self) -> Option<Cursor> {
        self.cursor.clone()
    }

    pub fn get_actions(&self) -> &Vec<RenderAction> {
        &self.actions
    }
}

struct RenderHandle {
    
    behavior: Weak<RefCell<dyn ComponentBehavior>>,

    region: Region,

    trigger: RenderTrigger,
    opacity: RenderOpacity,
    phase: RenderPhase
}

impl RenderHandle {
    
    fn new(behavior: Weak<RefCell<dyn ComponentBehavior>>, region: Region, trigger: RenderTrigger, opacity: RenderOpacity, phase: RenderPhase) -> RenderHandle {
        RenderHandle {
            behavior,
            region,
            trigger,
            opacity,
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
    pub fn claim_space(&mut self, region: Region, trigger: RenderTrigger, phase: RenderPhase, opacity: RenderOpacity, behavior: Weak<RefCell<dyn ComponentBehavior>>) {

        let maybe_index = self.render_components.binary_search_by(|existing| {
            existing.phase.cmp(&phase)
        });
        let index;
        match maybe_index {
            Ok(the_index) => index = the_index,
            Err(the_index) => index = the_index
        };
        self.render_components.insert(index, RenderHandle::new(behavior, region, trigger, opacity, phase));
    }

    pub fn can_claim(&self, region: Region) -> bool {

        for handle in &self.render_components {
            if handle.region.intersects_with(&region) {
                return false;
            }
        }

        return true;
    }

    pub fn predict_render(&mut self) -> Vec<RenderAction> {

        let mut render_actions = Vec::new();

        if self.render_background {
            match self.background_color {
                Some(color) => {
                    if color.get_alpha() > 0 {
                        let opacity;
                        if color.get_alpha() == u8::max_value() {
                            opacity = RenderOpacity::Solid;
                        } else {
                            opacity = RenderOpacity::Mixed;
                        }
                        render_actions.push(RenderAction {
                            region: Region::entire_viewport(),
                            opacity
                        });
                    }
                }, None => {}
            }
        }

        self.render_components.drain_filter(|handle| {
            match handle.behavior.upgrade() {
                Some(component_cell) => {

                    let component_handle = component_cell.borrow();
                    let agent_cell = component_handle.get_agent().upgrade().expect("Component agent shouldn't have been dropped");
                    let agent = agent_cell.borrow();

                    if agent.did_request_render() {
                        render_actions.push(RenderAction {
                            region: handle.region,
                            opacity: handle.opacity
                        });
                    }
                    false
                }, None => true
            }
        });

        render_actions
    }

    pub fn render<'a>(&mut self, gl: &WebGlRenderingContext, event: &RenderEvent, manager: &'a ContainerManager) -> RenderResult {

        // Draw the background if necessary
        if self.render_background && self.background_color.is_some() {
            let color = self.background_color.as_ref().unwrap();
            gl.clear_color(color.get_red_float(), color.get_green_float(), color.get_blue_float(), color.get_alpha_float());
            gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

            // Once the background has been drawn, don't redraw until we need to do it again
            self.render_background = false;
        }

        let mut cursor_result = None;

        // TODO Maybe just remove render_actions from render()
        // TODO And change BehaviorRenderResult back to normal
        let mut render_actions = Vec::new();

        let mouse_position = manager.get_mouse_position();

        let mut previous_render_phase = RenderPhase::Start;

        self.render_components.drain_filter(|handle| {
            match handle.behavior.upgrade() {
                Some(component_cell) => {
                    let mut component_handle = component_cell.borrow_mut();
                    let agent_holder = component_handle.get_agent().upgrade().expect("Component agent shouldn't have been dropped");
                    let mut agent = agent_holder.borrow_mut();
                    let requested_render = agent.did_request_render();
                    if requested_render {

                        if previous_render_phase != handle.phase {

                            // Currently, the Text phase is the only phase that has built-in support, other phases will have to prepare themselves
                            if handle.phase == RenderPhase::Text {
                                manager.get_text_renderer().borrow_mut().start_rendering();
                            }

                            previous_render_phase = handle.phase;
                        }

                        agent.set_rendering();
                        drop(agent);

                        let local_render_result = component_handle.render(&mut RenderParams::new(gl, event, manager));
                        let local_cursor = local_render_result.get_cursor();
                        if handle.region.is_float_inside(mouse_position) {
                            cursor_result = local_cursor;
                        }
                    } else {

                        if handle.region.is_float_inside(mouse_position) {
                            cursor_result = component_handle.get_cursor(&mut CursorParams::new(event, manager));
                        }
                    }
                    false
                }, None => true
            }
        });

        RenderResult {
            cursor: cursor_result,
            actions: render_actions
        }
    }

    /// Ensures that all components will render during the next call to render()
    pub fn force_full_render(&mut self){
        self.render_background = true;
        for handle in &self.render_components {
            match handle.behavior.upgrade() {
                Some(cell) => {
                    cell.borrow_mut().get_agent().upgrade().expect("Component agent shouldn't have been dropped").borrow_mut().request_render();
                }, None => {}
            };
        }
    }

    /// Ensures that all components that are (partially) in the given region will render during the next call to render()
    pub fn force_partial_render(&mut self, region: Region){
        // TODO Finish this method!
    }

    pub fn on_mouse_move<'a>(&'a mut self, event: &MouseMoveEvent, manager: &'a ContainerManager){

        let old_mouse_pos = manager.get_mouse_position();
        let new_mouse_pos = manager.to_gl_coords((event.mouse_event.offset_x(), event.mouse_event.offset_y()));
        for handle in &mut self.render_components {
            let needs_render = match &mut handle.trigger {
                RenderTrigger::MouseMove => true,
                RenderTrigger::MouseMoveInside => {
                    let was_in = handle.region.is_float_inside(old_mouse_pos);
                    let is_in = handle.region.is_float_inside(new_mouse_pos);
                    was_in || is_in
                }, RenderTrigger::MouseInOut => {
                    let was_in = handle.region.is_float_inside(old_mouse_pos);
                    let is_in = handle.region.is_float_inside(new_mouse_pos);
                    was_in != is_in
                }, _other => false
            };

            if needs_render {
                match handle.behavior.upgrade() {
                    Some(component_cell) => {
                        let the_component = component_cell.borrow_mut();
                        the_component.get_agent().upgrade().expect("Component agent shouldn't have been dropped").borrow_mut().request_render();
                    }, None => {
                        // If the component happens to be dropped, it will be removed from the vec during the next frame
                    }
                };
            }
        }
    }
}