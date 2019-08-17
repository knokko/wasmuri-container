use crate::Component;

use std::cell::RefCell;
use std::rc::Rc;

pub struct ComponentHandle {

    component: Rc<RefCell<Box<dyn Component>>>,

    index: usize

}

impl ComponentHandle {

    pub fn new(component: Box<dyn Component>, index: usize) -> ComponentHandle {
        ComponentHandle {
            component: Rc::new(RefCell::new(component)),
            index
        }
    }
}