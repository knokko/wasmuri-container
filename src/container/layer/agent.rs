use crate::{
    Component,
    ContainerManager
};

use super::Region;

pub struct ComponentAgent {

    requested_render: bool,
    requested_removal: bool,

    components_to_add: Vec<Box<dyn Component>>,

    has_changes: bool
}

impl ComponentAgent {

    pub fn new() -> ComponentAgent {
        ComponentAgent {

            // Every (renderable) component should be rendered its first frame
            // The render manager will make sure only the components that can actually render will be rendered
            requested_render: true,
            requested_removal: false,

            components_to_add: Vec::new(),

            has_changes: false
        }
    }

    /// Requests to re-render this component the next frame
    pub fn request_render(&mut self){
        self.requested_render = true;
    }

    /// Removes this component from the layer as soon as possible
    pub fn remove_this_component(&mut self){
        self.requested_removal = true;
        self.has_changes = true;
    }

    pub fn remove_other_component(&mut self){
        // Create some kind of Key type to index components
        self.has_changes = true;
    }

    /// Adds the other component as soon as possible
    pub fn add_component(&mut self, component: Box<dyn Component>){
        self.components_to_add.push(component);
        self.has_changes = true;
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

    /// Sets the needs_render flag of this agent to false. 
    /// Should only be used by the component handle right before calling the render method of the component.
    pub(super) fn set_rendering(&mut self){
        self.requested_render = false;
    }

    pub(super) fn clear_changes(&mut self){
        self.has_changes = false;
    }

    pub(super) fn has_changes(&self) -> bool {
        self.has_changes
    }
}

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

pub struct ConsumableAgent<'a> {

    base_agent: BaseAgent<'a>,

    consumed: bool
}

impl<'a> ConsumableAgent<'a> {

    pub fn new(region: &'a Region, manager: &'a ContainerManager) -> ConsumableAgent<'a> {
        ConsumableAgent {
            base_agent: BaseAgent::new(region, manager),
            consumed: false
        }
    }

    pub fn get_base_agent(&'a mut self) -> &BaseAgent<'a> {
        &self.base_agent
    }

    pub fn consume(&mut self){
        self.consumed = true;
    }

    pub fn is_consumed(&self) -> bool {
        self.consumed
    }
}

pub type KeyDownAgent<'a> = ConsumableAgent<'a>;

pub type KeyUpAgent<'a> = ConsumableAgent<'a>;

pub type MouseClickAgent<'a> = BaseAgent<'a>;

pub type MouseMoveAgent<'a> = BaseAgent<'a>;

pub type MouseScrollAgent<'a> = ConsumableAgent<'a>;

pub type RenderAgent<'a> = BaseAgent<'a>;

pub type UpdateAgent<'a> = BaseAgent<'a>;