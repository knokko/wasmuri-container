use crate::container::layer::*;
use crate::cursor::Cursor;

use std::cell::RefCell;
use std::rc::Weak;

use wasmuri_core::util::print;

use crate::params::*;

pub trait Component {

    fn attach(&mut self, _agent: &mut LayerAgent);

    fn set_agent(&mut self, _agent: Weak<RefCell<ComponentAgent>>);

    fn key_down(&mut self, _params: &mut KeyDownParams) -> bool {
        print("The keydown operation is not supported for this component!");
        false
    }

    fn key_up(&mut self, _params: &mut KeyUpParams) -> bool {
        print("The keyup operation is not supported for this component!");
        false
    }

    fn mouse_click(&mut self, _params: &mut MouseClickParams){
        print("The mouseclick operation is not supported for this component!");
    }

    fn mouse_move(&mut self, _params: &mut MouseMoveParams){
        print("The mouseclick operation is not supported for this component!");
    }

    fn mouse_scroll(&mut self, _params: &mut MouseScrollParams) -> bool {
        print("The mousescroll operation is not supported for this component!");
        false
    }

    fn render(&mut self, _params: &mut RenderParams) -> Option<Cursor> {
        print("The render operation is not supported for this component!");
        None
    }

    fn get_cursor(&mut self, _params: &mut CursorParams) -> Option<Cursor> {
        print("The get_cursor operation is not supported for this component!");
        None
    }

    fn update(&mut self, _params: &mut UpdateParams){
        print("The update operation is not supported for this component!");
    }
}