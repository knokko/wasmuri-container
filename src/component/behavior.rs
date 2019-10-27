use crate::container::layer::*;
use crate::cursor::Cursor;

use std::cell::RefCell;
use std::rc::Weak;

use crate::params::*;

pub trait ComponentBehavior {

    fn attach(&mut self, agent: &mut LayerAgent);

    fn set_agent(&mut self, agent: Weak<RefCell<ComponentAgent>>);

    fn get_agent(&self) -> &Weak<RefCell<ComponentAgent>>;

    fn key_down(&mut self, _params: &mut KeyDownParams) -> bool {
        false
    }

    fn key_up(&mut self, _params: &mut KeyUpParams) -> bool {
        false
    }

    fn mouse_click(&mut self, _params: &mut MouseClickParams){}

    fn mouse_move(&mut self, _params: &mut MouseMoveParams){}

    fn mouse_scroll(&mut self, _params: &mut MouseScrollParams) -> bool {
        false
    }

    fn render(&mut self, _params: &mut RenderParams) -> Option<Cursor> {
        None
    }

    fn get_cursor(&mut self, _params: &mut CursorParams) -> Option<Cursor> {
        None
    }

    fn update(&mut self, _params: &mut UpdateParams){}
}