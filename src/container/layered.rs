use crate::{
    EventResult,
    RenderResult,
    ContainerManager
};
use crate::cursor::Cursor;

use std::cell::RefCell;
use std::rc::Rc;

use super::Container;
use super::layer::Layer;

use wasmuri_events::*;

use web_sys::WebGlRenderingContext;

pub struct LayeredContainer {

    /// The vector containing all layers of the container.
    /// The first layer (layers[0]) will denote the 'background' layer and the last layer will denote the 'front' layer.
    /// 
    /// The background layer will render first and the front layer will render last (so the front layer will draw over the background layer).
    /// The other events (like clicking and pressing keys), will be processed first by the front layer and last by the background layer.
    layers: Vec<Layer>
}

impl LayeredContainer {

    pub fn new(layers: Vec<Layer>) -> LayeredContainer {
        LayeredContainer {
            layers
        }
    }

    pub fn celled(layers: Vec<Layer>) -> Rc<RefCell<LayeredContainer>> {
        Rc::new(RefCell::new(Self::new(layers)))
    }
}

impl std::fmt::Debug for LayeredContainer {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LayeredContainer with {} layers", self.layers.len())
    }
}

impl Container for LayeredContainer {

    fn on_key_down(&mut self, event: &KeyDownEvent, manager: &ContainerManager) -> EventResult {
        let mut next_container = None;

        for layer in &mut self.layers.iter_mut().rev() {
            let requested_container = layer.on_key_down(event, manager);

            // The foreground layers will get priority if multiplie layers request a container change
            if requested_container.is_some() && next_container.is_none() {
                next_container = requested_container;
            }
        }

        next_container
    }

    fn on_key_up(&mut self, event: &KeyUpEvent, manager: &ContainerManager) -> EventResult {
        let mut next_container = None;

        for layer in &mut self.layers.iter_mut().rev() {
            let requested_container = layer.on_key_up(event, manager);

            // The foreground layers will get priority if multiplie layers request a container change
            if requested_container.is_some() && next_container.is_none() {
                next_container = requested_container;
            }
        }

        next_container
    }

    fn on_mouse_click(&mut self, event: &MouseClickEvent, manager: &ContainerManager) -> EventResult {
        let mut next_container = None;

        for layer in &mut self.layers.iter_mut().rev() {
            let requested_container = layer.on_mouse_click(event, manager);

            // The foreground layers will get priority if multiplie layers request a container change
            if requested_container.is_some() && next_container.is_none() {
                next_container = requested_container;
            }
        }

        next_container
    }

    fn on_mouse_move(&mut self, event: &MouseMoveEvent, manager: &ContainerManager) -> EventResult {
        let mut next_container = None;

        for layer in &mut self.layers.iter_mut().rev() {
            let requested_container = layer.on_mouse_move(event, manager);

            // The foreground layers will get priority if multiplie layers request a container change
            if requested_container.is_some() && next_container.is_none() {
                next_container = requested_container;
            }
        }

        next_container
    }

    fn on_mouse_scroll(&mut self, event: &MouseScrollEvent, manager: &ContainerManager) -> EventResult {
        let mut next_container = None;

        for layer in &mut self.layers.iter_mut().rev() {
            let requested_container = layer.on_mouse_scroll(event, manager);

            // The foreground layers will get priority if multiplie layers request a container change
            if requested_container.is_some() && next_container.is_none() {
                next_container = requested_container;
            }
        }

        next_container
    }

    fn on_update(&mut self, event: &UpdateEvent, manager: &ContainerManager) -> EventResult {
        let mut next_container = None;

        for layer in &mut self.layers.iter_mut().rev() {
            let requested_container = layer.on_update(event, manager);

            // The foreground layers will get priority if multiplie layers request a container change
            if requested_container.is_some() && next_container.is_none() {
                next_container = requested_container;
            }
        }

        next_container
    }

    fn render(&mut self, gl: &WebGlRenderingContext, event: &RenderEvent, manager: &ContainerManager) -> RenderResult {

        // First find out which regions are going to be rendered with which opacity
        // TODO Use the predict_render functions of layer

        // TODO Force certain parts of some layers to also render
        let mut maybe_cursor = None;

        for layer in &mut self.layers {
            let requested_cursor = layer.on_render(gl, event, manager).get_cursor();

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