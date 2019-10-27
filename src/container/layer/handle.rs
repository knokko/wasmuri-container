use crate::*;

use std::cell::*;
use std::rc::*;

use super::ComponentAgent;

pub struct ComponentHandle {

    component: Rc<RefCell<dyn Component>>,

    // Currently, the only purpose of this field is to prevent the behaviors from being dropped while the component is alive
    _behaviors: Vec<Rc<RefCell<dyn ComponentBehavior>>>,

    agent: Rc<RefCell<ComponentAgent>>
}

impl ComponentHandle {

    pub fn new(component: Rc<RefCell<dyn Component>>, behaviors: Vec<Rc<RefCell<dyn ComponentBehavior>>>) -> ComponentHandle {
        let agent = Rc::new(RefCell::new(ComponentAgent::new()));
        for behavior in &behaviors {
            behavior.borrow_mut().set_agent(Rc::downgrade(&agent));
        }

        ComponentHandle {
            component,
            _behaviors: behaviors,
            agent
        }
    }

    pub fn get_component(&self) -> &Rc<RefCell<dyn Component>> {
        &self.component
    }

    pub fn get_agent(&mut self) -> RefMut<ComponentAgent> {
        self.agent.borrow_mut()
    }
}

pub struct OuterHandle {

    handle: Rc<RefCell<ComponentHandle>>
}

impl OuterHandle {

    pub fn new(component: Rc<RefCell<dyn Component>>, helpers: Vec<Rc<RefCell<dyn ComponentBehavior>>>) -> OuterHandle {
        OuterHandle {
            handle: Rc::new(RefCell::new(ComponentHandle::new(component, helpers)))
        }
    }

    pub fn create_weak(&self) -> Weak<RefCell<ComponentHandle>> {
        Rc::downgrade(&self.handle)
    }

    pub fn create_strong(&self) -> Rc<RefCell<ComponentHandle>> {
        Rc::clone(&self.handle)
    }

    pub fn get_rc(&self) -> &Rc<RefCell<ComponentHandle>> {
        &self.handle
    }
}