use crate::*;

use std::cell::RefCell;
use std::rc::*;

use web_sys::*;

use wasmuri_core::*;

struct RenderHandle {
    
    behavior: Weak<RefCell<dyn ComponentBehavior>>,

    region: Region,

    trigger: RenderTrigger,
    opacity: RenderOpacity,
    phase: RenderPhase,

    prev_render_actions: Vec<PassedRenderAction>
}

impl RenderHandle {
    
    fn new(behavior: Weak<RefCell<dyn ComponentBehavior>>, region: Region, trigger: RenderTrigger, opacity: RenderOpacity, phase: RenderPhase) -> RenderHandle {
        RenderHandle {
            behavior,
            region,
            trigger,
            opacity,
            phase,
            prev_render_actions: Vec::with_capacity(1)
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
            if handle.region.intersects_with(region) {
                return false;
            }
        }

        return true;
    }

    pub fn predict_render(&mut self) -> Vec<PlannedRenderAction> {

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
                        render_actions.push(PlannedRenderAction::new(Region::entire_viewport(), opacity));
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
                        render_actions.push(PlannedRenderAction::new(handle.region, handle.opacity));
                    }
                    false
                }, None => true
            }
        });

        render_actions
    }

    pub fn render<'a>(&mut self, gl: &WebGlRenderingContext, manager: &'a ContainerManager, 
            mouse_position: Option<(f32,f32)>) -> (RenderResult, Vec<PassedRenderAction>) {

        let mut render_actions = Vec::new();

        // Draw the background if necessary
        if self.render_background && self.background_color.is_some() {
            let color = self.background_color.as_ref().unwrap();
            gl.clear_color(color.get_red_float(), color.get_green_float(), color.get_blue_float(), color.get_alpha_float());
            gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

            // Once the background has been drawn, don't redraw until we need to do it again
            self.render_background = false;
            
        }

        let has_background = self.background_color.is_some();

        // If we have a background, then we will always render the entire viewport
        if has_background {
            render_actions.push(PassedRenderAction::new(Region::entire_viewport()));
        }

        let mut cursor_result = None;

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

                            // TODO Handle render phase switching per container rather than per layer

                            // Currently, the Text phase is the only phase that has built-in support, other phases will have to prepare themselves
                            if handle.phase == RenderPhase::Text {
                                manager.get_text_renderer().borrow_mut().start_rendering();
                            }

                            previous_render_phase = handle.phase;
                        }

                        agent.set_rendering();
                        drop(agent);

                        let mut local_render_result = component_handle.render(&mut RenderParams::new(gl, manager));
                        let mut local_render_actions = local_render_result.get_render_actions();

                        // If we have a background, we will render the entire viewport anyway, so adding a part of the viewport to it is useless
                        if !has_background {
                            handle.prev_render_actions = local_render_actions.clone();
                            render_actions.append(&mut local_render_actions);
                        }

                        let local_cursor = local_render_result.get_cursor();
                        if mouse_position.is_some() && handle.region.is_float_inside(mouse_position.unwrap()) {
                            cursor_result = local_cursor;
                        }
                    } else {

                        if mouse_position.is_some() && handle.region.is_float_inside(mouse_position.unwrap()) {
                            cursor_result = component_handle.get_cursor(&mut CursorParams::new(manager));
                        }

                        if !has_background {
                            for prev_render_action in &handle.prev_render_actions {
                                render_actions.push(*prev_render_action);
                            }
                        }
                    }
                    false
                }, None => true
            }
        });

        (RenderResult::new(cursor_result), render_actions)
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

    /// Ensures that all components that are (partially) in any of the given regions will render during the next call to render()
    /// Returns a Vec containing all RenderAction's that will be done during the next render() call due to this method call
    pub fn force_partial_render(&mut self, regions: &[Region]) -> Vec<PlannedRenderAction> {

        // If this layer has a background color, this vector will contain all regions that will need to be re-rendered by fully solid components
        let mut solid_rerender_regions = Vec::new();

        let mut caused_render_actions = Vec::new();

        for handle in &self.render_components {
            match handle.behavior.upgrade() {
                Some(cell) => {
                    for region in regions {
                        if handle.region.intersects_with(*region) {

                            // We need this info to determine whether or not the background needs to be re-rendered
                            if self.background_color.is_some() && handle.opacity == RenderOpacity::Solid {
                                solid_rerender_regions.push(handle.region);
                            }

                            let agent_cell = cell.borrow_mut().get_agent().upgrade().expect("Component agent shouldn't have been dropped");
                            let mut agent = agent_cell.borrow_mut();

                            if !agent.did_request_render() {
                                agent.request_render();
                                caused_render_actions.push(PlannedRenderAction::new(handle.region, handle.opacity));
                            }
                            break;
                        }
                    }
                }, None => {}
            }
        }

        // If we have a background color and the regions are not entirely covered by solid components, we need to re-render the background
        if self.background_color.is_some() {
            for region in regions {
                if !region.get_uncovered_regions(&solid_rerender_regions).is_empty() {

                    // Since we need to re-render the background, we also need to re-render all other components
                    self.force_full_render();
                    break;
                }
            }
        }

        // Finally return the render actions that were caused by this method call
        caused_render_actions
    }

    pub fn on_mouse_move<'a>(&'a mut self, old_mouse_pos: Option<(f32,f32)>, new_mouse_pos: Option<(f32,f32)>){
        for handle in &mut self.render_components {
            let needs_render = match &mut handle.trigger {
                RenderTrigger::MouseMove => true,
                RenderTrigger::MouseMoveInside => {
                    let was_in = old_mouse_pos.is_some() && handle.region.is_float_inside(old_mouse_pos.unwrap());
                    let is_in = new_mouse_pos.is_some() && handle.region.is_float_inside(new_mouse_pos.unwrap());
                    was_in || is_in
                }, RenderTrigger::MouseInOut => {
                    let was_in = old_mouse_pos.is_some() && handle.region.is_float_inside(old_mouse_pos.unwrap());
                    let is_in = new_mouse_pos.is_some() && handle.region.is_float_inside(new_mouse_pos.unwrap());
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