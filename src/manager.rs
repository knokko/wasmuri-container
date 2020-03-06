use crate::*;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::JsCast;

use wasmuri_core::print;
use wasmuri_events::*;

use wasmuri_text::TextRenderer;

use web_sys::{
    HtmlElement,
    HtmlCanvasElement,
    WebGlRenderingContext
};

pub type EventResult = Option<Rc<RefCell<dyn Container>>>;

pub struct ConsumableEventResult {

    result: EventResult,
    consumed: bool
}

impl ConsumableEventResult {

    pub fn change_container(new_container: Rc<RefCell<dyn Container>>) -> ConsumableEventResult {
        ConsumableEventResult {
            result: Some(new_container),
            consumed: true
        }
    }

    pub fn dont_consume() -> ConsumableEventResult {
        Self::consume(false)
    }

    pub fn do_consume() -> ConsumableEventResult {
        Self::consume(true)
    }

    pub fn consume(consumed: bool) -> ConsumableEventResult {
        ConsumableEventResult {
            result: None,
            consumed
        }
    }

    pub fn requested_container_change(&self) -> bool {
        self.result.is_some()
    }

    pub fn is_consumed(&self) -> bool {
        self.consumed
    }

    /// Only use this after requested_container_change() returned true. 
    /// If that's not the case, this method will panic.
    pub fn get_next_container(&self) -> Rc<RefCell<dyn Container>> {
        Rc::clone(self.result.as_ref().unwrap())
    }

    /// Turn this ConsumableEventResult into an EventResult. 
    /// This will discard and ignore the value of self.consumed.
    pub fn as_normal_result(self) -> EventResult {
        self.result
    }
}

pub type ContainerRenderResult = Cursor;

pub trait ResizeListener {

    fn on_resize(&self, manager: &ContainerManager, event: &ResizeEvent);
}

pub struct ContainerManager {

    canvas: HtmlCanvasElement,
    resize_listener: Option<Box<dyn ResizeListener>>,
    prev_cursor: Option<Cursor>,
    gl: WebGlRenderingContext,
    
    current_container: Option<Rc<RefCell<dyn Container>>>,

    text_renderer: RefCell<TextRenderer>
}

impl ContainerManager {

    pub fn start(canvas: HtmlCanvasElement, resize_listener: Option<Box<dyn ResizeListener>>, leak_self: bool) -> Rc<RefCell<ContainerManager>> {

        let gl = wasmuri_core::get_gl(&canvas);

        let html_canvas = canvas.clone();

        let window = web_sys::window().expect("Should have window");
        let width = window.inner_width().expect("Should be able to call window.innerWidth").as_f64().expect("innerWidth should be f64") as u32;
        let height = window.inner_height().expect("Should be able to call window.innerHeight").as_f64().expect("innerHeight should be f64") as u32;

        html_canvas.set_width(width);
        html_canvas.set_height(height);
        gl.viewport(0, 0, width as i32, height as i32);

        let text_renderer = RefCell::new(TextRenderer::from_canvas(&html_canvas));
        set_event_source(&html_canvas.dyn_into::<HtmlElement>().expect("A canvas should be an HtmlElement"));

        let manager = ContainerManager {
            canvas,
            prev_cursor: None,
            gl,
            resize_listener,

            current_container: None,

            text_renderer
        };

        let manager_cell = Rc::new(RefCell::new(manager));

        start_listen(&manager_cell, &KEY_DOWN_HANDLER);
        start_listen(&manager_cell, &KEY_UP_HANDLER);
        start_listen(&manager_cell, &MOUSE_CLICK_HANDLER);
        start_listen(&manager_cell, &MOUSE_MOVE_HANDLER);
        start_listen(&manager_cell, &MOUSE_SCROLL_HANDLER);
        start_listen(&manager_cell, &RESIZE_HANDLER);
        start_listen(&manager_cell, &UPDATE_HANDLER);
        start_listen(&manager_cell, &RENDER_HANDLER);
        start_listen(&manager_cell, &COPY_HANDLER);
        start_listen(&manager_cell, &PASTE_HANDLER);
        start_listen(&manager_cell, &CUT_HANDLER);

        if leak_self {
            std::mem::forget(Rc::clone(&manager_cell))
        }

        manager_cell
    }

    pub fn set_container_cell(&mut self, new_container: Rc<RefCell<dyn Container>>){
        self.current_container = Some(new_container);
    }

    pub fn set_resize_listener(&mut self, new_listener: Option<Box<dyn ResizeListener>>){
        self.resize_listener = new_listener;
    }

    pub fn get_gl(&self) -> &WebGlRenderingContext {
        &self.gl
    }

    pub fn get_canvas(&self) -> &HtmlCanvasElement {
        &self.canvas
    }

    fn process_result<F: FnMut(&mut dyn Container, &ContainerManager) -> EventResult>(&mut self, mut result_function: F) {

        let maybe_new_container = match &self.current_container {
            Some(container) => {
                let mut borrow_container = container.borrow_mut();
                result_function(&mut *borrow_container, self)
            }, None => None
        };

        match maybe_new_container {
            Some(new_container) => self.set_container_cell(new_container),
            None => {}
        };
    }

    fn with_container<F: FnMut(&mut dyn Container, &ContainerManager)>(&self, mut container_function: F) {
        match &self.current_container {
            Some(container) => {
                let mut borrow_container = container.borrow_mut();
                container_function(&mut *borrow_container, self);
            }, None => {}
        };
    }

    /// Gives a reference to the TextRenderer of this ContainerManager, which is inside a RefCell.
    pub fn get_text_renderer(&self) -> &RefCell<TextRenderer> {
        &self.text_renderer
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
        self.process_result(|container, manager| {
            container.on_key_down(&KeyInfo::new(event.key_event.key(), event.key_event.ctrl_key(), event.key_event.shift_key(), 
                    event.key_event.alt_key(), event.key_event.meta_key()), manager)
        });
    }
}

impl Listener<KeyUpEvent> for ContainerManager {

    fn process(&mut self, event: &KeyUpEvent){
        self.process_result(|container, manager| {
            container.on_key_up(&KeyInfo::new(event.key_event.key(), event.key_event.ctrl_key(), event.key_event.shift_key(), 
                        event.key_event.alt_key(), event.key_event.meta_key()), manager)
        });
    }
}

impl Listener<MouseClickEvent> for ContainerManager {

    fn process(&mut self, event: &MouseClickEvent){
        self.process_result(|container, manager| {
            container.on_mouse_click(ClickInfo::new(event.mouse_event.button(), event.mouse_event.ctrl_key(), 
                        event.mouse_event.shift_key(), event.mouse_event.alt_key(), event.mouse_event.meta_key()), manager)
        });
    }
}

impl Listener<MouseMoveEvent> for ContainerManager {

    fn process(&mut self, event: &MouseMoveEvent){
        self.process_result(|container, manager| container.on_mouse_move(event, manager));
    }
}

impl Listener<MouseScrollEvent> for ContainerManager {

    fn process(&mut self, event: &MouseScrollEvent){
        self.process_result(|container, manager| container.on_mouse_scroll(event, manager));
    }
}

impl Listener<UpdateEvent> for ContainerManager {

    fn process(&mut self, _event: &UpdateEvent){
        self.process_result(|container, manager| container.on_update(manager));
    }
}

impl Listener<ResizeEvent> for ContainerManager {

    fn process(&mut self, event: &ResizeEvent){
        match &self.resize_listener {
            Some(listener) => { listener.on_resize(self, event); },
            None => {

                self.canvas.set_width(event.get_new_width());
                self.canvas.set_height(event.get_new_height());

                self.gl.viewport(0, 0, event.get_new_width() as i32, event.get_new_height() as i32);

                self.with_container(|container, _manager| container.force_render());
            }
        };
    }
}

impl Listener<RenderEvent> for ContainerManager {

    fn process(&mut self, _event: &RenderEvent){

        let mut change_cursor = false;
        let mut result = None;

        self.with_container(|container, manager| {
            let local_result = container.render(&manager.gl, manager);
            
            if manager.prev_cursor.is_none() {
                change_cursor = true;
            } else {
                let prev_cursor = manager.prev_cursor.as_ref().unwrap();
                change_cursor = *prev_cursor != local_result;
            }

            result = Some(local_result);
        });

        if change_cursor {
            let css = self.canvas.style();
            css.set_property("cursor", &result.as_ref().unwrap().to_css_value()).expect("Should be able to set cursor property");
            self.prev_cursor = result;
        }
    }
}

impl Listener<CopyEvent> for ContainerManager {

    fn process(&mut self, event: &CopyEvent) {
        self.with_container(|container, _manager| {
            match event.clipboard_event.clipboard_data() {
                Some(clipboard) => {
                    match container.on_copy() {
                        Some(to_copy) => {
                            match to_copy {
                                ClipboardData::Text(the_text) => {
                                     if clipboard.set_data("text", &the_text).is_err() {
                                          print("Failed to copy data to clipboard during copy event");
                                     }
                                }
                            };
                            event.clipboard_event.prevent_default();
                        }, None => {}
                    };
                }, None => print("No clipboard data on copy event?")
            };
        });
    }
}

impl Listener<PasteEvent> for ContainerManager {

    fn process(&mut self, event: &PasteEvent) {
        self.with_container(|container, _manager| {
            match event.clipboard_event.clipboard_data() {
                Some(clipboard) => {
                    match clipboard.get_data("text") {
                        Ok(the_text) => {
                            container.on_paste(&ClipboardData::Text(the_text));
                        }, Err(_) => { /* No text was pasted, but something else. */ }
                    }
                }, None => print("No clipboard data on paste event?")
            };
        });
    }
}

impl Listener<CutEvent> for ContainerManager {

    fn process(&mut self, event: &CutEvent) {
        self.with_container(|container, _manager| {
            match event.clipboard_event.clipboard_data() {
                Some(clipboard) => {
                    match container.on_cut() {
                        Some(to_cut) => {
                            match to_cut {
                                ClipboardData::Text(the_text) => {
                                     if clipboard.set_data("text", &the_text).is_err() {
                                         print("Failed to copy data to clipboard during cut event");
                                     }
                                }
                            };
                            event.clipboard_event.prevent_default();
                        }, None => {}
                    };
                }, None => print("No clipboard data on cut event?")
            };
        });
    }
}