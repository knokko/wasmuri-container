use crate::Container;
use crate::cursor::Cursor;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::JsCast;

use wasmuri_events::*;

use wasmuri_text::TextRenderer;

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

    mouse_position: (i32,i32),
    
    current_container: Option<Rc<RefCell<dyn Container>>>,

    text_renderer: RefCell<TextRenderer>
}

impl ContainerManager {

    pub fn start(canvas: HtmlCanvasElement) -> Rc<RefCell<ContainerManager>> {

        let gl = wasmuri_core::get_gl(&canvas);

        let html_canvas = canvas.clone();
        let text_renderer = RefCell::new(TextRenderer::from_canvas(&html_canvas));
        set_event_source(&html_canvas.dyn_into::<HtmlElement>().expect("A canvas should be an HtmlElement"));

        let manager = ContainerManager {
            canvas,
            prev_cursor: None,
            gl,

            // I'm afraid I can't retrieve the mouse position until the mouse moves for the first time
            mouse_position: (0, 0),

            current_container: None,

            text_renderer
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

    /// Gives a reference to the TextRenderer of this ContainerManager, which is inside a RefCell.
    pub fn get_text_renderer(&self) -> &RefCell<TextRenderer> {
        &self.text_renderer
    }

    /// Gives the current mouse position in the OpenGL coordinate system. If a MouseMoveEvent is currently being fired,
    /// this method will return the previous mouse coordinates.
    pub fn get_mouse_position(&self) -> (f32, f32) {
        self.to_gl_coords(self.mouse_position)
    }

    /// Converts the position in pixel coordinates (the offset in pixels between the point and the corner of the canvas) to
    /// OpenGL coordinates.
    pub fn to_gl_coords(&self, pixel_coords: (i32, i32)) -> (f32, f32) {
        let gl_x = 2.0 * (pixel_coords.0 as f32 / self.canvas.width() as f32) - 1.0;
        let gl_y = 1.0 - 2.0 * (pixel_coords.1 as f32 / self.canvas.height() as f32);
        (gl_x, gl_y)
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
                claim_container.on_key_down(event, self)
            }, None => None
        });
    }
}

impl Listener<KeyUpEvent> for ContainerManager {

    fn process(&mut self, event: &KeyUpEvent){
        self.process_result(match &self.current_container {
            Some(current_container) => {

                let mut claim_container = current_container.borrow_mut();
                claim_container.on_key_up(event, self)
            }, None => None
        });
    }
}

impl Listener<MouseClickEvent> for ContainerManager {

    fn process(&mut self, event: &MouseClickEvent){
        self.process_result(match &self.current_container {
            Some(current_container) => {

                let mut claim_container = current_container.borrow_mut();
                claim_container.on_mouse_click(event, self)
            }, None => None
        });
    }
}

impl Listener<MouseMoveEvent> for ContainerManager {

    fn process(&mut self, event: &MouseMoveEvent){

        self.process_result(match &self.current_container {
            Some(current_container) => {

                let mut claim_container = current_container.borrow_mut();
                claim_container.on_mouse_move(event, self)
            }, None => None
        });

        // Unfortunately, offset_x and offset_y are experimental, but there is no alternative that I know of.
        self.mouse_position = (event.mouse_event.offset_x(), event.mouse_event.offset_y());
    }
}

impl Listener<MouseScrollEvent> for ContainerManager {

    fn process(&mut self, event: &MouseScrollEvent){
        self.process_result(match &self.current_container {
            Some(current_container) => {

                let mut claim_container = current_container.borrow_mut();
                claim_container.on_mouse_scroll(event, self)
            }, None => None
        });
    }
}

impl Listener<UpdateEvent> for ContainerManager {

    fn process(&mut self, event: &UpdateEvent){
        self.process_result(match &self.current_container {
            Some(current_container) => {

                let mut claim_container = current_container.borrow_mut();
                claim_container.on_update(event, self)
            }, None => None
        });
    }
}

impl Listener<RenderEvent> for ContainerManager {

    fn process(&mut self, event: &RenderEvent){
        match &self.current_container {
            Some(current_container) => {

                let mut claim_container = current_container.borrow_mut();
                let result = claim_container.render(&self.gl, event, self);

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

// TODO Also create a resize event in wasmuri-events and listen for it