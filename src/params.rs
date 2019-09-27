use crate::container::layer::*;
use crate::ContainerManager;
use crate::cursor::Cursor;

use wasmuri_events::{
    KeyDownEvent,
    KeyUpEvent,
    MouseClickEvent,
    MouseMoveEvent,
    MouseScrollEvent,
    RenderEvent,
    UpdateEvent
};

use web_sys::WebGlRenderingContext;

pub struct KeyDownParams<'a> {

    pub agent: &'a mut ComponentAgent, 
    pub event: &'a KeyDownEvent, 
    pub manager: &'a ContainerManager
}

impl<'a> KeyDownParams<'a> {

    pub fn new(agent: &'a mut ComponentAgent, event: &'a KeyDownEvent, manager: &'a ContainerManager) -> KeyDownParams<'a> {
        KeyDownParams {
            agent,
            event,
            manager
        }
    }
}

pub struct KeyUpParams<'a> {

    pub agent: &'a mut ComponentAgent, 
    pub event: &'a KeyUpEvent, 
    pub manager: &'a ContainerManager
}

impl<'a> KeyUpParams<'a> {

    pub fn new(agent: &'a mut ComponentAgent, event: &'a KeyUpEvent, manager: &'a ContainerManager) -> KeyUpParams<'a> {
        KeyUpParams {
            agent,
            event,
            manager
        }
    }
}

pub struct MouseClickParams<'a> {

    pub agent: &'a mut ComponentAgent, 
    pub event: &'a MouseClickEvent, 
    pub manager: &'a ContainerManager
}

impl<'a> MouseClickParams<'a> {

    pub fn new(agent: &'a mut ComponentAgent, event: &'a MouseClickEvent, manager: &'a ContainerManager) -> MouseClickParams<'a> {
        MouseClickParams {
            agent,
            event,
            manager
        }
    }
}

pub struct MouseMoveParams<'a> {

    pub agent: &'a mut ComponentAgent, 
    pub event: &'a MouseMoveEvent, 
    pub manager: &'a ContainerManager
}

impl<'a> MouseMoveParams<'a> {

    pub fn new(agent: &'a mut ComponentAgent, event: &'a MouseMoveEvent, manager: &'a ContainerManager) -> MouseMoveParams<'a> {
        MouseMoveParams {
            agent,
            event,
            manager
        }
    }
}

pub struct MouseScrollParams<'a> {

    pub agent: &'a mut ComponentAgent, 
    pub event: &'a MouseScrollEvent, 
    pub manager: &'a ContainerManager
}

impl<'a> MouseScrollParams<'a> {

    pub fn new(agent: &'a mut ComponentAgent, event: &'a MouseScrollEvent, manager: &'a ContainerManager) -> MouseScrollParams<'a> {
        MouseScrollParams {
            agent,
            event,
            manager
        }
    }
}

pub struct RenderParams<'a> {

    pub gl: &'a WebGlRenderingContext, 
    pub agent: &'a mut ComponentAgent, 
    pub event: &'a RenderEvent, 
    pub manager: &'a ContainerManager
}

impl<'a> RenderParams<'a> {

    pub fn new(gl: &'a WebGlRenderingContext, agent: &'a mut ComponentAgent, event: &'a RenderEvent, manager: &'a ContainerManager) -> RenderParams<'a> {
        RenderParams {
            gl,
            agent,
            event,
            manager
        }
    }
}

pub struct CursorParams<'a> {

    pub agent: &'a mut ComponentAgent, 
    pub event: &'a RenderEvent, 
    pub manager: &'a ContainerManager
}

impl<'a> CursorParams<'a> {

    pub fn new(agent: &'a mut ComponentAgent, event: &'a RenderEvent, manager: &'a ContainerManager) -> CursorParams<'a> {
        CursorParams {
            agent,
            event,
            manager
        }
    }
}

pub struct UpdateParams<'a> {

    pub agent: &'a mut ComponentAgent, 
    pub event: &'a UpdateEvent, 
    pub manager: &'a ContainerManager
}

impl<'a> UpdateParams<'a> {

    pub fn new(agent: &'a mut ComponentAgent, event: &'a UpdateEvent, manager: &'a ContainerManager) -> UpdateParams<'a> {
        UpdateParams {
            agent,
            event,
            manager
        }
    }
}