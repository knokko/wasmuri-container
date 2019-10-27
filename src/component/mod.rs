use std::cell::RefCell;
use std::rc::Rc;

mod behavior;
pub use behavior::*;

pub trait Component {

    fn create_behaviors(&mut self) -> Vec<Rc<RefCell<dyn ComponentBehavior>>>;
}