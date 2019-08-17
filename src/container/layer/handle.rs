use crate::Component;

use std::cell::RefCell;
use std::rc::{
    Rc,
    Weak
};

pub struct ComponentHandle {

    component: Rc<RefCell<Box<dyn Component>>>,

    /// Planned to be used later for modifying and removal of other components
    _index: usize

}

impl ComponentHandle {

    pub fn new(component: Box<dyn Component>, index: usize) -> ComponentHandle {
        ComponentHandle {
            component: Rc::new(RefCell::new(component)),
            _index: index
        }
    }

    pub fn create_weak(&self) -> Weak<RefCell<Box<dyn Component>>> {
        Rc::downgrade(&self.component)
    }
}