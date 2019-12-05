use crate::container::layer::*;
use crate::cursor::Cursor;

use std::cell::RefCell;
use std::rc::Weak;

use crate::params::*;

#[derive(Clone)]
pub struct BehaviorRenderResult {

    cursor: Option<Cursor>,
    opacity: RenderOpacity
}

impl BehaviorRenderResult {

    pub fn with_cursor(cursor: Cursor, opacity: RenderOpacity) -> BehaviorRenderResult {
        BehaviorRenderResult {
            cursor: Some(cursor),
            opacity
        }
    }

    pub fn without_cursor(opacity: RenderOpacity) -> BehaviorRenderResult {
        BehaviorRenderResult {
            cursor: None,
            opacity
        }
    }

    pub fn get_cursor(&self) -> Option<Cursor> {
        self.cursor
    }

    pub fn get_opacity(&self) -> RenderOpacity {
        self.opacity
    }
}

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

    fn render(&mut self, _params: &mut RenderParams) -> BehaviorRenderResult {
        BehaviorRenderResult::without_cursor(RenderOpacity::Empty)
    }

    fn get_cursor(&mut self, _params: &mut CursorParams) -> Option<Cursor> {
        None
    }

    fn update(&mut self, _params: &mut UpdateParams){}
}