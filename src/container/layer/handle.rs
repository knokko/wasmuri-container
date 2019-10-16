use crate::{
    Component,
    ContainerManager
};

use crate::cursor::Cursor;
use crate::params::*;

use std::cell::*;
use std::rc::*;

use super::ComponentAgent;

use wasmuri_events::{
    KeyDownEvent,
    KeyUpEvent,
    MouseClickEvent,
    MouseMoveEvent,
    MouseScrollEvent,
    UpdateEvent,
    RenderEvent
};
use web_sys::WebGlRenderingContext;

pub struct ComponentHandle {

    component: Rc<RefCell<dyn Component>>,

    agent: Rc<RefCell<ComponentAgent>>,

}

impl ComponentHandle {

    pub fn new(component: Rc<RefCell<dyn Component>>) -> ComponentHandle {
        let agent = Rc::new(RefCell::new(ComponentAgent::new()));
        component.borrow_mut().set_agent(Rc::downgrade(&agent));
        ComponentHandle {
            component,
            agent
        }
    }

    pub fn get_component(&self) -> &Rc<RefCell<dyn Component>> {
        &self.component
    }

    pub fn get_agent(&mut self) -> RefMut<ComponentAgent> {
        self.agent.borrow_mut()
    }

    pub fn render(&mut self, gl: &WebGlRenderingContext, event: &RenderEvent, manager: &ContainerManager) -> Option<Cursor> {
        self.agent.borrow_mut().set_rendering();
        self.component.borrow_mut().render(&mut RenderParams::new(gl, event, manager))
    }

    pub fn update(&mut self, event: &UpdateEvent, manager: &ContainerManager){
        self.component.borrow_mut().update(&mut UpdateParams::new(event, manager));
    }

    pub fn get_cursor(&mut self, event: &RenderEvent, manager: &ContainerManager) -> Option<Cursor> {
        self.component.borrow_mut().get_cursor(&mut CursorParams::new(event, manager))
    }

    pub fn key_down(&mut self, event: &KeyDownEvent, manager: &ContainerManager) -> bool {
        self.component.borrow_mut().key_down(&mut KeyDownParams::new(event, manager))
    }

    pub fn key_up(&mut self, event: &KeyUpEvent, manager: &ContainerManager) -> bool {
        self.component.borrow_mut().key_up(&mut KeyUpParams::new(event, manager))
    }

    pub fn mouse_move(&mut self, event: &MouseMoveEvent, manager: &ContainerManager) {
        self.component.borrow_mut().mouse_move(&mut MouseMoveParams::new(event, manager));
    }

    pub fn mouse_click(&mut self, event: &MouseClickEvent, manager: &ContainerManager) {
        self.component.borrow_mut().mouse_click(&mut MouseClickParams::new(event, manager));
    }

    pub fn mouse_scroll(&mut self, event: &MouseScrollEvent, manager: &ContainerManager) -> bool {
        self.component.borrow_mut().mouse_scroll(&mut MouseScrollParams::new(event, manager))
    }
}

pub struct OuterHandle {

    handle: Rc<RefCell<ComponentHandle>>
}

impl OuterHandle {

    pub fn new(component: Rc<RefCell<dyn Component>>) -> OuterHandle {
        OuterHandle {
            handle: Rc::new(RefCell::new(ComponentHandle::new(component)))
        }
    }

    pub fn create_weak(&self) -> Weak<RefCell<ComponentHandle>> {
        Rc::downgrade(&self.handle)
    }

    pub fn create_strong(&self) -> Rc<RefCell<ComponentHandle>> {
        Rc::clone(&self.handle)
    }

    pub fn get_rc(&self) -> &Rc<RefCell<ComponentHandle>> {
        &self.handle
    }
}