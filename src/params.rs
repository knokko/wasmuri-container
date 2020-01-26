use crate::ContainerManager;

use wasmuri_events::{
    KeyDownEvent,
    KeyUpEvent,
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

    pub mouse_pos: (f32,f32), 
    pub button: i16,
    pub manager: &'a ContainerManager
}

impl<'a> MouseClickParams<'a> {

    pub fn new(mouse_pos: (f32,f32), button: i16, manager: &'a ContainerManager) -> MouseClickParams<'a> {
        MouseClickParams {
            mouse_pos,
            button,
            manager
        }
    }
}

pub struct MouseClickOutParams<'a> {

    pub button: i16,
    pub manager: &'a ContainerManager
}

impl<'a> MouseClickOutParams<'a> {

    pub fn new(button: i16, manager: &'a ContainerManager) -> MouseClickOutParams<'a> {
        MouseClickOutParams {
            button,
            manager
        }
    }
}

pub type MouseClickAnyParams<'a> = MouseClickOutParams<'a>;

pub struct MouseMoveParams<'a> {

    pub old_mouse_pos: Option<(f32,f32)>,
    pub new_mouse_pos: Option<(f32,f32)>,
    pub manager: &'a ContainerManager
}

impl<'a> MouseMoveParams<'a> {

    pub fn new(old_mouse_pos: Option<(f32,f32)>, new_mouse_pos: Option<(f32,f32)>, manager: &'a ContainerManager) -> MouseMoveParams<'a> {
        MouseMoveParams {
            old_mouse_pos,
            new_mouse_pos,
            manager
        }
    }
}

pub struct MouseScrollParams<'a> {

    pub mouse_pos: Option<(f32,f32)>, 
    pub delta: f64,
    pub manager: &'a ContainerManager
}

impl<'a> MouseScrollParams<'a> {

    pub fn new(mouse_pos: Option<(f32,f32)>, delta: f64, manager: &'a ContainerManager) -> MouseScrollParams<'a> {
        MouseScrollParams {
            mouse_pos,
            delta,
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