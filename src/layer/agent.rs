use crate::*;

use std::cell::RefCell;
use std::rc::Rc;

use wasmuri_core::Region;

pub struct ComponentAgent {

    requested_render: bool,
    requested_removal: bool,

    components_to_add: Vec<Rc<RefCell<dyn Component>>>,

    new_container: Option<Rc<RefCell<dyn Container>>>,

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

            new_container: None,

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
    pub fn add_component(&mut self, component: Rc<RefCell<dyn Component>>){
        self.components_to_add.push(component);
        self.has_changes = true;
    }

    pub fn change_container(&mut self, new_container: Rc<RefCell<dyn Container>>){
        self.new_container = Some(new_container);
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
    pub fn get_components_to_add(&mut self) -> &mut Vec<Rc<RefCell<dyn Component>>> {
        &mut self.components_to_add
    }

    /// Checks if the component requested to change the current container
    pub fn requested_container_change(&self) -> bool {
        self.new_container.is_some()
    }

    /// Gets the container that this component requested to become the current container.
    /// This method should only be used after requested_container_change() returned true, or it may panic.
    pub fn get_new_container(&self) -> Rc<RefCell<dyn Container>> {
        Rc::clone(&self.new_container.as_ref().expect("new_container should have been set"))
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
    mouse_pos: Option<(f32,f32)>,
    region: &'a Region
}

impl<'a> BaseAgent<'a> {

    pub fn new(region: &'a Region, manager: &'a ContainerManager, mouse_pos: Option<(f32,f32)>) -> BaseAgent<'a> {
        BaseAgent {
            requested_render: false,
            requested_removal: false,

            components_to_add: Vec::new(),

            region,
            mouse_pos,
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

    /// Check if the mouse is currently hovering over this component and that there is no component in the front
    /// of this component inbetween.
    pub fn is_mouse_over(&self) -> bool {
        self.mouse_pos.is_some() && self.region.is_float_inside(self.mouse_pos.unwrap())
    }

    /// Updates the mouse_pos of this agent to the given value. 
    /// This method should normally only be called from the Layer.
    pub fn update_mouse_pos(&mut self, new_mouse_pos: Option<(f32, f32)>) {
        self.mouse_pos = new_mouse_pos;
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

    pub fn new(region: &'a Region, manager: &'a ContainerManager, mouse_pos: Option<(f32,f32)>) -> ConsumableAgent<'a> {
        ConsumableAgent {
            base_agent: BaseAgent::new(region, manager, mouse_pos),
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