use crate::ContainerManager;

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

    pub event: &'a KeyDownEvent, 
    pub manager: &'a ContainerManager
}

impl<'a> KeyDownParams<'a> {

    pub fn new(event: &'a KeyDownEvent, manager: &'a ContainerManager) -> KeyDownParams<'a> {
        KeyDownParams {
            event,
            manager
        }
    }
}

pub struct KeyUpParams<'a> {

    pub event: &'a KeyUpEvent, 
    pub manager: &'a ContainerManager
}

impl<'a> KeyUpParams<'a> {

    pub fn new(event: &'a KeyUpEvent, manager: &'a ContainerManager) -> KeyUpParams<'a> {
        KeyUpParams {
            event,
            manager
        }
    }
}

pub struct MouseClickParams<'a> {

    pub event: &'a MouseClickEvent, 
    pub manager: &'a ContainerManager
}

impl<'a> MouseClickParams<'a> {

    pub fn new(event: &'a MouseClickEvent, manager: &'a ContainerManager) -> MouseClickParams<'a> {
        MouseClickParams {
            event,
            manager
        }
    }
}

pub struct MouseMoveParams<'a> {

    pub event: &'a MouseMoveEvent, 
    pub manager: &'a ContainerManager
}

impl<'a> MouseMoveParams<'a> {

    pub fn new(event: &'a MouseMoveEvent, manager: &'a ContainerManager) -> MouseMoveParams<'a> {
        MouseMoveParams {
            event,
            manager
        }
    }
}

pub struct MouseScrollParams<'a> {

    pub event: &'a MouseScrollEvent, 
    pub manager: &'a ContainerManager
}

impl<'a> MouseScrollParams<'a> {

    pub fn new(event: &'a MouseScrollEvent, manager: &'a ContainerManager) -> MouseScrollParams<'a> {
        MouseScrollParams {
            event,
            manager
        }
    }
}

pub struct RenderParams<'a> {

    pub gl: &'a WebGlRenderingContext, 
    pub event: &'a RenderEvent, 
    pub manager: &'a ContainerManager
}

impl<'a> RenderParams<'a> {

    pub fn new(gl: &'a WebGlRenderingContext, event: &'a RenderEvent, manager: &'a ContainerManager) -> RenderParams<'a> {
        RenderParams {
            gl,
            event,
            manager
        }
    }
}

pub struct CursorParams<'a> {

    pub event: &'a RenderEvent, 
    pub manager: &'a ContainerManager
}

impl<'a> CursorParams<'a> {

    pub fn new(event: &'a RenderEvent, manager: &'a ContainerManager) -> CursorParams<'a> {
        CursorParams {
            event,
            manager
        }
    }
}

pub struct UpdateParams<'a> {

    pub event: &'a UpdateEvent, 
    pub manager: &'a ContainerManager
}

impl<'a> UpdateParams<'a> {

    pub fn new(event: &'a UpdateEvent, manager: &'a ContainerManager) -> UpdateParams<'a> {
        UpdateParams {
            event,
            manager
        }
    }
}