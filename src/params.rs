use crate::ContainerManager;

use web_sys::WebGlRenderingContext;

pub struct KeyDownParams<'a> {

    pub keys: &'a KeyInfo, 
    pub manager: &'a ContainerManager
}

impl<'a> KeyDownParams<'a> {

    pub fn new(keys: &'a KeyInfo, manager: &'a ContainerManager) -> KeyDownParams<'a> {
        KeyDownParams {
            keys,
            manager
        }
    }
}

pub struct KeyUpParams<'a> {

    pub keys: &'a KeyInfo, 
    pub manager: &'a ContainerManager
}

impl<'a> KeyUpParams<'a> {

    pub fn new(keys: &'a KeyInfo, manager: &'a ContainerManager) -> KeyUpParams<'a> {
        KeyUpParams {
            keys,
            manager
        }
    }
}

pub struct MouseClickParams<'a> {

    pub mouse_pos: (f32,f32), 
    pub click: ClickInfo,
    pub manager: &'a ContainerManager
}

impl<'a> MouseClickParams<'a> {

    pub fn new(mouse_pos: (f32,f32), click: ClickInfo, manager: &'a ContainerManager) -> MouseClickParams<'a> {
        MouseClickParams {
            mouse_pos,
            click,
            manager
        }
    }
}

pub struct MouseClickOutParams<'a> {

    pub click: ClickInfo,
    pub manager: &'a ContainerManager
}

impl<'a> MouseClickOutParams<'a> {

    pub fn new(click: ClickInfo, manager: &'a ContainerManager) -> MouseClickOutParams<'a> {
        MouseClickOutParams {
            click,
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
    pub manager: &'a ContainerManager
}

impl<'a> RenderParams<'a> {

    pub fn new(gl: &'a WebGlRenderingContext, manager: &'a ContainerManager) -> RenderParams<'a> {
        RenderParams {
            gl,
            manager
        }
    }
}

pub struct CursorParams<'a> {

    pub manager: &'a ContainerManager
}

impl<'a> CursorParams<'a> {

    pub fn new(manager: &'a ContainerManager) -> CursorParams<'a> {
        CursorParams {
            manager
        }
    }
}

pub struct UpdateParams<'a> {

    pub manager: &'a ContainerManager
}

impl<'a> UpdateParams<'a> {

    pub fn new(manager: &'a ContainerManager) -> UpdateParams<'a> {
        UpdateParams {
            manager
        }
    }
}

#[derive(Clone,Copy)]
pub struct ClickInfo {

    button: i16,
    control_down: bool,
    shift_down: bool,
    alt_down: bool,
    meta_down: bool
}

impl ClickInfo {

    pub fn new(button: i16, control_down: bool, shift_down: bool, alt_down: bool, meta_down: bool) -> ClickInfo {
        ClickInfo {
            button,
            control_down,
            shift_down,
            alt_down,
            meta_down
        }
    }

    pub fn get_button(&self) -> i16 {
        self.button
    }

    pub fn is_control_down(&self) -> bool {
        self.control_down
    }

    pub fn is_shift_down(&self) -> bool {
        self.shift_down
    }

    pub fn is_alt_down(&self) -> bool {
        self.alt_down
    }

    pub fn is_meta_down(&self) -> bool {
        self.meta_down
    }
}

pub struct KeyInfo {

    key: String,
    control_down: bool,
    shift_down: bool,
    alt_down: bool,
    meta_down: bool
}

impl KeyInfo {

    pub fn new(key: String, control_down: bool, shift_down: bool, alt_down: bool, meta_down: bool) -> KeyInfo {
        KeyInfo {
            key,
            control_down,
            shift_down,
            alt_down,
            meta_down
        }
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn is_control_down(&self) -> bool {
        self.control_down
    }

    pub fn is_shift_down(&self) -> bool {
        self.shift_down
    }

    pub fn is_alt_down(&self) -> bool {
        self.alt_down
    }

    pub fn is_meta_down(&self) -> bool {
        self.meta_down
    }
}

pub enum ClipboardData {

    Text(String)
}