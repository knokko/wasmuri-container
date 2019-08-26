use crate::{
    Component,
    ContainerManager
};

use crate::cursor::Cursor;

use std::cell::RefCell;
use std::rc::{
    Rc,
    Weak
};

use super::ComponentAgent;

use wasmuri_events::{
    KeyDownEvent,
    KeyUpEvent,
    RenderEvent
};
use web_sys::WebGlRenderingContext;

pub struct ComponentHandle {

    component: Box<dyn Component>,

    agent: ComponentAgent,

}

impl ComponentHandle {

    pub fn new(component: Box<dyn Component>) -> ComponentHandle {
        ComponentHandle {
            component,
            agent: ComponentAgent::new()
        }
    }

    pub fn get_component(&self) -> &Box<dyn Component> {
        &self.component
    }

    pub fn get_agent(&mut self) -> &mut ComponentAgent {
        &mut self.agent
    }

    pub fn render(&mut self, gl: &WebGlRenderingContext, event: &RenderEvent, manager: &ContainerManager) -> Option<Cursor> {
        self.agent.set_rendering();
        self.component.render(gl, &mut self.agent, event, manager)
    }

    pub fn get_cursor(&mut self, event: &RenderEvent, manager: &ContainerManager) -> Option<Cursor> {
        self.component.get_cursor(&mut self.agent, event, manager)
    }

    pub fn key_down(&mut self, event: &KeyDownEvent, manager: &ContainerManager) -> bool {
        self.component.key_down(&mut self.agent, event, manager)
    }

    pub fn key_up(&mut self, event: &KeyUpEvent, manager: &ContainerManager) -> bool {
        self.component.key_up(&mut self.agent, event, manager)
    }
}

pub struct OuterHandle {

    handle: Rc<RefCell<ComponentHandle>>,

    /// Planned for future use
    _index: usize
}

impl OuterHandle {

    pub fn new(component: Box<dyn Component>, index: usize) -> OuterHandle {
        OuterHandle {
            handle: Rc::new(RefCell::new(ComponentHandle::new(component))),
            _index: index
        }
    }

    pub fn create_weak(&self) -> Weak<RefCell<ComponentHandle>> {
        Rc::downgrade(&self.handle)
    }

    pub fn get_rc(&self) -> &Rc<RefCell<ComponentHandle>> {
        &self.handle
    }
}