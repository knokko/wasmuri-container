use crate::*;

use std::cell::RefCell;
use std::rc::*;

pub trait Component {

    fn create_behaviors(&mut self) -> Vec<Rc<RefCell<dyn ComponentBehavior>>>;
}

#[derive(Clone)]
pub struct BehaviorRenderResult {

    cursor: Option<Cursor>,

    actions: Vec<PassedRenderAction>
}

impl BehaviorRenderResult {

    pub fn with_cursor(cursor: Cursor, actions: Vec<PassedRenderAction>) -> BehaviorRenderResult {
        BehaviorRenderResult {
            cursor: Some(cursor),
            actions
        }
    }

    pub fn without_cursor(actions: Vec<PassedRenderAction>) -> BehaviorRenderResult {
        BehaviorRenderResult {
            cursor: None,
            actions
        }
    }

    pub fn has_cursor(&self) -> bool {
        self.cursor.is_some()
    }

    pub fn get_cursor(&self) -> Option<Cursor> {
        self.cursor.clone()
    }

    pub fn get_render_actions(&mut self) -> &mut Vec<PassedRenderAction> {
        &mut self.actions
    }
}

pub trait ComponentBehavior {

    fn attach(&mut self, agent: &mut dyn LayerAgent);

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

    /// Called when the user clicked on this component
    fn mouse_click_inside(&mut self, _params: &mut MouseClickParams) {}

    /// Called when a mouse click occurred, but the click wasn't on this component
    fn mouse_click_outside(&mut self, _params: &mut MouseClickOutParams) {}

    fn mouse_click_anywhere(&mut self, _params: &mut MouseClickAnyParams) {}

    /// Called when a mouse click occurred, no matter where
    fn mouse_move(&mut self, _params: &mut MouseMoveParams){}

    /// Returns true if the MouseScrollEvent should be consumed: then it will not be passed to other mouse scroll listeners.
    fn mouse_scroll(&mut self, _params: &mut MouseScrollParams) -> bool {
        false
    }

    fn render(&mut self, _params: &mut RenderParams) -> BehaviorRenderResult {
        BehaviorRenderResult::without_cursor(Vec::with_capacity(0))
    }

    fn get_cursor(&mut self, _params: &mut CursorParams) -> Option<Cursor> {
        None
    }

    fn update(&mut self, _params: &mut UpdateParams){}
}