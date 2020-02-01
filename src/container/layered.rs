use crate::*;

use std::cell::RefCell;
use std::rc::Rc;

use wasmuri_events::*;

use web_sys::WebGlRenderingContext;

pub struct LayeredContainer {

    /// The vector containing all layers of the container.
    /// The first layer (layers[0]) will denote the 'background' layer and the last layer will denote the 'front' layer.
    /// 
    /// The background layer will render first and the front layer will render last (so the front layer will draw over the background layer).
    /// The other events (like clicking and pressing keys), will be processed first by the front layer and last by the background layer.
    layers: Vec<Box<dyn Layer>>
}

impl LayeredContainer {

    pub fn new(layers: Vec<Box<dyn Layer>>) -> LayeredContainer {
        LayeredContainer {
            layers
        }
    }

    pub fn celled(layers: Vec<Box<dyn Layer>>) -> Rc<RefCell<LayeredContainer>> {
        Rc::new(RefCell::new(Self::new(layers)))
    }
}

impl std::fmt::Debug for LayeredContainer {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LayeredContainer with {} layers", self.layers.len())
    }
}

impl Container for LayeredContainer {

    fn on_key_down(&mut self, keys: &KeyInfo, manager: &ContainerManager) -> EventResult {
        for layer in &mut self.layers.iter_mut().rev() {
            let key_down_result = layer.on_key_down(keys, manager);
            
            // If a container change was requested, it is guaranteed that the event was consumed
            if key_down_result.requested_container_change() {
                return Some(key_down_result.get_next_container());
            }

            if key_down_result.is_consumed() {
                return None;
            }
        }

        None
    }

    fn on_key_up(&mut self, keys: &KeyInfo, manager: &ContainerManager) -> EventResult {
        for layer in &mut self.layers.iter_mut().rev() {
            let key_up_result = layer.on_key_up(keys, manager);

            // If a container change was requested, it is guaranteed that the event was consumed
            if key_up_result.requested_container_change() {
                return Some(key_up_result.get_next_container());
            }

            if key_up_result.is_consumed() {
                return None;
            }
        }

        None
    }

    fn on_mouse_click(&mut self, click: ClickInfo, manager: &ContainerManager) -> EventResult {

        let mut next_container = None;

        for layer in &mut self.layers {

            // If multiple layers request a container change, layers in the front will get priority
            let click_result = layer.on_mouse_click(click, manager);
            if click_result.is_some() {
                next_container = click_result;
            }
        }

        next_container
    }

    fn on_mouse_move(&mut self, event: &MouseMoveEvent, manager: &ContainerManager) -> EventResult {

        // TODO This sometimes doesn't work properly, requires investigation...
        let mut next_container = None;
        let mut new_mouse_pos = Some(manager.to_gl_coords(event.get_new_position()));

        for layer in &mut self.layers.iter_mut().rev() {
            let current_result = layer.on_mouse_move(new_mouse_pos, manager);
            if current_result.is_consumed() {
                new_mouse_pos = None;
            }
            let requested_container = current_result.as_normal_result();

            // The foreground layers will get priority if multiplie layers request a container change
            if requested_container.is_some() && next_container.is_none() {
                next_container = requested_container;
            }
        }

        next_container
    }

    fn on_mouse_scroll(&mut self, event: &MouseScrollEvent, manager: &ContainerManager) -> EventResult {
        for layer in &mut self.layers.iter_mut().rev() {
            let event_result = layer.on_mouse_scroll(event.mouse_event.delta_y(), manager);

            if event_result.requested_container_change() {
                return Some(event_result.get_next_container());
            }

            if event_result.is_consumed() {
                return None;
            }
        }

        None
    }

    fn on_update(&mut self, manager: &ContainerManager) -> EventResult {
        let mut next_container = None;

        for layer in &mut self.layers.iter_mut().rev() {
            let requested_container = layer.on_update(manager);

            // The foreground layers will get priority if multiplie layers request a container change
            if requested_container.is_some() && next_container.is_none() {
                next_container = requested_container;
            }
        }

        next_container
    }

    fn render(&mut self, gl: &WebGlRenderingContext, manager: &ContainerManager) -> ContainerRenderResult {

        // First find out which regions are going to be rendered with which opacity initially
        let mut rerender_actions = Vec::with_capacity(self.layers.len());
        for layer in &mut self.layers {
            rerender_actions.push(layer.predict_render());
        }

        // These are needed to keep track of the progress of propagating renders between layers
        let mut rerender_indices = vec![0; self.layers.len()];

        /*
        The next lines of code are to propagate render actions between layers.
        For each region in some layer that has to be re-rendered, the following things need to happen:
        1. That same region also has to be re-rendered in all layers that are in front of that layer.
        2. If the render action in the region is partially transparent, that region must also be re-rendered in all layers behind that layer.

        The interesting part is that, in order to re-render a certain region in a certain layer, all components intersecting with that region
        need to be re-rendered, but it is very well possible that some components can lay partially outside that region. When that is the case,
        step 1 and 2 also need to be executed for the region of that component as well.
        */
        let mut current_layer_index = 0;
        while current_layer_index < self.layers.len() {
            let all_actions_of_layer = &rerender_actions[current_layer_index];
            let current_action_index = rerender_indices[current_layer_index];

            let num_actions_of_layer = all_actions_of_layer.len();

            // Only do stuff if there are actually unprocessed Region's
            if current_action_index != num_actions_of_layer {
                let actions_to_process = &all_actions_of_layer[current_action_index..all_actions_of_layer.len()];

                // Obtain the regions of the actions to process
                let mut regions_to_process_front = Vec::with_capacity(actions_to_process.len());
                let mut regions_to_process_back = Vec::with_capacity(actions_to_process.len());
                for action in actions_to_process {
                    regions_to_process_front.push(action.get_region());

                    let opacity = action.get_opacity();

                    // If the opacity of the render action is fully solid or static solid, no need to re-render the stuff behind it
                    if opacity != RenderOpacity::Solid && opacity != RenderOpacity::StaticSolidOrNothing {
                        regions_to_process_back.push(action.get_region());
                    }
                }

                // Force the layers in front of the current layer to re-render those regions as well
                for front_layer_index in current_layer_index + 1 .. self.layers.len() {
                    let front_layer = &mut self.layers[front_layer_index];
                    let mut new_actions_to_process = front_layer.force_partial_render(&regions_to_process_front);

                    // Append all new render actions for the front layer
                    rerender_actions[front_layer_index].append(&mut new_actions_to_process);
                }

                // Force the layers behind the current layer to re-render the regions behind the transparent render actions
                if current_layer_index > 0 {
                    for back_layer_index in (0 .. current_layer_index).rev() {
                        
                        let back_layer = &mut self.layers[back_layer_index];
                        let mut new_actions_to_process = back_layer.force_partial_render(&regions_to_process_back);
                        let num_new_actions = new_actions_to_process.len();

                        // Append all new render actions for the back layer
                        rerender_actions[back_layer_index].append(&mut new_actions_to_process);

                        // TODO For performance, remove the regions in regions_to_process_back that are fully covered by a solid render action
                        // in new_actions_to_process

                        // The new render actions of this back layer will also have to be processed...
                        if num_new_actions > 0 {
                            current_layer_index = back_layer_index;
                        }
                    }
                }

                // Mark the regions we just processed as 'completed'
                rerender_indices[current_layer_index] = num_actions_of_layer;
            }

            current_layer_index += 1;
        }

        // Now that all layers know exactly which components to render, the real render can finally begin
        let mut maybe_cursor = None;

        for layer in &mut self.layers {
            let requested_cursor = layer.on_render(gl, manager).get_cursor();

            if maybe_cursor.is_none() && requested_cursor.is_some() {
                maybe_cursor = requested_cursor;
            }
        }
        
        match maybe_cursor {
            Some(cursor) => cursor,
            None => Cursor::DEFAULT
        }
    }

    fn force_render(&mut self){
        for layer in &mut self.layers {
            layer.force_render();
        }
    }
}