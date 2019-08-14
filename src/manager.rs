use crate::Container;
use crate::cursor::Cursor;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::JsCast;

use wasmuri_events::*;

use web_sys::{
    HtmlElement,
    HtmlCanvasElement,
    WebGlRenderingContext
};

pub type EventResult = Option<Box<dyn ManagerAction>>;
pub type RenderResult = Cursor;

pub trait ManagerAction {

    fn execute(&mut self, manager: &mut ContainerManager);
}

pub struct ContainerManager {

    canvas: HtmlCanvasElement,
    prev_cursor: Option<Cursor>,
    gl: WebGlRenderingContext,
    
    current_container: Option<Rc<RefCell<dyn Container>>>
}

impl ContainerManager {

    pub fn start(canvas: HtmlCanvasElement) -> Rc<RefCell<ContainerManager>> {

        let gl = canvas.get_context("webgl").expect("Should have get_context method").expect("Should be able to get webgl context").dyn_into::<WebGlRenderingContext>().expect("webgl context should be a WebGlRenderingContext");

        let html_canvas = canvas.clone();
        set_event_source(&html_canvas.dyn_into::<HtmlElement>().expect("A canvas should be an HtmlElement"));

        let manager = ContainerManager {
            canvas,
            prev_cursor: None,
            gl,
            current_container: None
        };

        let manager_cell = Rc::new(RefCell::new(manager));

        start_listen(&manager_cell, &KEY_DOWN_HANDLER);
        start_listen(&manager_cell, &KEY_UP_HANDLER);
        start_listen(&manager_cell, &MOUSE_CLICK_HANDLER);
        start_listen(&manager_cell, &MOUSE_MOVE_HANDLER);
        start_listen(&manager_cell, &MOUSE_SCROLL_HANDLER);
        start_listen(&manager_cell, &UPDATE_HANDLER);
        start_listen(&manager_cell, &RENDER_HANDLER);

        manager_cell
    }

    pub fn set_container_cell(&mut self, new_container: Rc<RefCell<dyn Container>>){
        self.current_container = Some(new_container);
    }

    fn process_result(&mut self, result: EventResult){
        match result {
            Some(mut action) => action.execute(self),
            None => {}
        }
    }
}

fn start_listen<T>(manager_cell: &Rc<RefCell<ContainerManager>>, handler: &Handler<T>) where ContainerManager: Listener<T> {
    let weak_manager = Rc::downgrade(manager_cell);
    handler.add_listener(weak_manager);
}

impl Listener<KeyDownEvent> for ContainerManager {

    fn process(&mut self, event: &KeyDownEvent){
        self.process_result(match &self.current_container {
            Some(current_container) => {

                let mut claim_container = current_container.borrow_mut();
                claim_container.on_key_down(event)
            }, None => None
        });
    }
}

impl Listener<KeyUpEvent> for ContainerManager {

    fn process(&mut self, event: &KeyUpEvent){
        self.process_result(match &self.current_container {
            Some(current_container) => {

                let mut claim_container = current_container.borrow_mut();
                claim_container.on_key_up(event)
            }, None => None
        });
    }
}

impl Listener<MouseClickEvent> for ContainerManager {

    fn process(&mut self, event: &MouseClickEvent){
        self.process_result(match &self.current_container {
            Some(current_container) => {

                let mut claim_container = current_container.borrow_mut();
                claim_container.on_mouse_click(event)
            }, None => None
        });
    }
}

impl Listener<MouseMoveEvent> for ContainerManager {

    fn process(&mut self, event: &MouseMoveEvent){
        self.process_result(match &self.current_container {
            Some(current_container) => {

                let mut claim_container = current_container.borrow_mut();
                claim_container.on_mouse_move(event)
            }, None => None
        });
    }
}

impl Listener<MouseScrollEvent> for ContainerManager {

    fn process(&mut self, event: &MouseScrollEvent){
        self.process_result(match &self.current_container {
            Some(current_container) => {

                let mut claim_container = current_container.borrow_mut();
                claim_container.on_mouse_scroll(event)
            }, None => None
        });
    }
}

impl Listener<UpdateEvent> for ContainerManager {

    fn process(&mut self, _event: &UpdateEvent){
        self.process_result(match &self.current_container {
            Some(current_container) => {

                let mut claim_container = current_container.borrow_mut();
                claim_container.on_update()
            }, None => None
        });
    }
}

impl Listener<RenderEvent> for ContainerManager {

    fn process(&mut self, _event: &RenderEvent){
        match &self.current_container {
            Some(current_container) => {

                let claim_container = current_container.borrow();
                let result = claim_container.render(&self.gl);

                let change_cursor;
                if self.prev_cursor.is_none() {
                    change_cursor = true;
                } else {
                    let prev_cursor = self.prev_cursor.as_ref().unwrap();
                    change_cursor = *prev_cursor == result;
                }

                if change_cursor {
                    let css = self.canvas.style();
                    css.set_property("cursor", &result.to_css_value()).expect("Should be able to set cursor property");
                    self.prev_cursor = Some(result);
                }
            }, None => {}
        }
    }
}