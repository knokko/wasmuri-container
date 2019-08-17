use crate::{
    Component,
    ContainerManager
};

use super::Region;

pub struct BaseAgent<'a> {

    requested_render: bool,
    requested_removal: bool,

    components_to_add: Vec<Box<dyn Component>>,

    manager: &'a ContainerManager,
    region: &'a Region
}

impl<'a> BaseAgent<'a> {

    pub fn new(region: &'a Region, manager: &'a ContainerManager) -> BaseAgent<'a> {
        BaseAgent {
            requested_render: false,
            requested_removal: false,

            components_to_add: Vec::new(),

            region,
            manager
        }
    }

    /// Requests to re-render this component the next frame
    pub fn request_render(&mut self){
        self.requested_render = true;
    }

    /// Removes this component from the layer as soon as possible
    pub fn remove_this_component(&mut self){
        self.requested_removal = true;
    }

    pub fn remove_other_component(&mut self){
        // Create some kind of Key type to index components
    }

    /// Adds the other component as soon as possible
    pub fn add_component(&mut self, component: Box<dyn Component>){
        self.components_to_add.push(component);
    }

    /// Gets a reference to the container manager
    pub fn get_manager(&self) -> &'a ContainerManager {
        &self.manager
    }

    pub fn get_region(&self) -> &Region {
        self.region
    }

    pub fn is_mouse_over(&self) -> bool {
        self.region.is_inside(self.manager.get_mouse_position())
    }

    /// Checks if the request_render() method of this agent has been called
    pub fn did_request_render(&self) -> bool {
        self.requested_render
    }

    /// Checks if the remove_this_component() method of this agent has been called
    pub fn did_request_removal(&self) -> bool {
        self.requested_removal
    }

    /// Gives a mutable reference to the collection of all components passed to this agent by the remove_other_component method
    pub fn get_components_to_add(&mut self) -> &mut Vec<Box<dyn Component>> {
        &mut self.components_to_add
    }
}

pub type RenderAgent<'a> = BaseAgent<'a>;