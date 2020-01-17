use crate::*;

use std::cell::RefCell;
use std::rc::*;

pub trait Component {

    fn create_behaviors(&mut self) -> Vec<Rc<RefCell<dyn ComponentBehavior>>>;
}

#[derive(Clone)]
pub struct BehaviorRenderResult {

    cursor: Option<Cursor>
}

impl BehaviorRenderResult {

    pub fn with_cursor(cursor: Cursor) -> BehaviorRenderResult {
        BehaviorRenderResult {
            cursor: Some(cursor)
        }
    }

    pub fn without_cursor() -> BehaviorRenderResult {
        BehaviorRenderResult {
            cursor: None
        }
    }

    pub fn has_cursor(&self) -> bool {
        self.cursor.is_some()
    }

    pub fn get_cursor(&self) -> Option<Cursor> {
        self.cursor.clone()
    }
}

pub trait ComponentBehavior {

    fn attach(&mut self, agent: &mut LayerAgent);

    fn set_agent(&mut self, agent: Weak<RefCell<ComponentAgent>>);

    fn get_agent(&self) -> &Weak<RefCell<ComponentAgent>>;

    /// Returns true if the KeyDownEvent should be consumed: then it will not be passed to other key listeners.
    fn key_down(&mut self, _params: &mut KeyDownParams) -> bool {
        false
    }

    /// Returns true if the KeyUpEvent should be consumed: then it will not be passed to other key listeners.
    fn key_up(&mut self, _params: &mut KeyUpParams) -> bool {
        false
    }

    /// Returns true if the MouseClickEvent should be consumed: then it will not be passed to other mouse click listeners.
    fn mouse_click(&mut self, _params: &mut MouseClickParams) -> bool {
        false
    }

    fn mouse_move(&mut self, _params: &mut MouseMoveParams){}

    /// Returns true if the MouseScrollEvent should be consumed: then it will not be passed to other mouse scroll listeners.
    fn mouse_scroll(&mut self, _params: &mut MouseScrollParams) -> bool {
        false
    }

    fn render(&mut self, _params: &mut RenderParams) -> BehaviorRenderResult {
        BehaviorRenderResult::without_cursor()
    }

    fn get_cursor(&mut self, _params: &mut CursorParams) -> Option<Cursor> {
        None
    }

    fn update(&mut self, _params: &mut UpdateParams){}
}