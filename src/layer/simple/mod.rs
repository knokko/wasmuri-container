use crate::*;

mod update;
mod render;
mod keylistening;
mod mouselistening;
mod clipboardlistening;

use render::RenderManager;
use update::UpdateManager;
use keylistening::KeyListenManager;
use mouselistening::MouseManager;
use clipboardlistening::*;

use std::cell::RefCell;
use std::rc::Rc;

use wasmuri_core::*;

use web_sys::WebGlRenderingContext;

pub struct SimpleLayer {
    
    components: Vec<OuterHandle>,

    key_manager: KeyListenManager,
    mouse_manager: MouseManager,
    clipboard_manager: ClipboardManager,
    update_manager: UpdateManager,
    render_manager: RenderManager,

    mouse_pos: Option<(f32, f32)>,
    last_render_actions: Vec<PassedRenderAction>
}

impl SimpleLayer {

    pub fn new(background_color: Option<Color>) -> SimpleLayer {
        SimpleLayer {
            components: Vec::with_capacity(10),

            render_manager: RenderManager::new(background_color),
            update_manager: UpdateManager::new(),
            clipboard_manager: ClipboardManager::new(),
            key_manager: KeyListenManager::new(),
            mouse_manager: MouseManager::new(),

            mouse_pos: None,
            last_render_actions: Vec::new()
        }
    }

    fn consumable_result(&mut self, consumed: bool) -> ConsumableEventResult {
        let normal_result = self.check_agents();
        match normal_result {
            Some(next_container) => ConsumableEventResult::change_container(next_container),
            None => ConsumableEventResult::consume(consumed)
        }
    }

    fn check_agents(&mut self) -> EventResult {
        let mut components_to_add = Vec::new();
        let mut new_container = None;
        self.components.drain_filter(|outer_handle| {
            let mut handle = outer_handle.get_rc().borrow_mut();
            let mut agent = handle.get_agent();
            if agent.has_changes() {
                agent.clear_changes();

                if agent.requested_container_change() {
                    if new_container.is_some() {
                        print("Warning: 2 components requested a container change during the same event");
                    }
                    new_container = Some(agent.get_new_container());
                }

                if agent.did_request_removal() {
                    return true;
                }

                // The did_request_render will be handled in the render manager of this lay

                components_to_add.append(agent.get_components_to_add());

                // TODO Add support for removing components
            }

            false
        });

        for component in components_to_add {
            self.add_component(component);
        }

        new_container
    }
}

impl Layer for SimpleLayer {

    fn on_mouse_move(&mut self, next_mouse_pos: Option<(f32, f32)>, manager: &ContainerManager) -> ConsumableEventResult {
        self.mouse_manager.fire_mouse_move(self.mouse_pos, next_mouse_pos, manager);
        self.render_manager.on_mouse_move(self.mouse_pos, next_mouse_pos);

        self.mouse_pos = next_mouse_pos;

        // If we rendered something at the mouse position, we assume that the user clicked on this layer and therefore not on the layers behind
        let mut move_result = false;
        if next_mouse_pos.is_some() {
            for render_action in &self.last_render_actions {
                if render_action.get_region().is_float_inside(next_mouse_pos.unwrap()) {
                    move_result = true;
                    break;
                }
            }
        }

        self.consumable_result(move_result)
    }

    fn on_mouse_click(&mut self, click: ClickInfo, manager: &ContainerManager) -> EventResult {
        match self.mouse_pos {
            Some(mouse_pos) => self.mouse_manager.fire_mouse_click(manager, mouse_pos, click), 
            None => self.mouse_manager.fire_mouse_click_outside(manager, click)
        };
        self.check_agents()
    }

    fn on_mouse_scroll(&mut self, delta: f64, manager: &ContainerManager) -> ConsumableEventResult {
        let scroll_result = self.mouse_manager.fire_mouse_scroll(manager, self.mouse_pos, delta);
        self.consumable_result(scroll_result)
    }

    fn on_key_down(&mut self, keys: &KeyInfo, manager: &ContainerManager) -> ConsumableEventResult {
        let key_down_result = self.key_manager.fire_key_down(keys, manager, self.mouse_pos);
        self.consumable_result(key_down_result)
    }

    fn on_key_up(&mut self, keys: &KeyInfo, manager: &ContainerManager) -> ConsumableEventResult {
        let key_up_result = self.key_manager.fire_key_up(keys, manager, self.mouse_pos);
        self.consumable_result(key_up_result)
    }

    fn on_copy(&mut self) -> Option<ClipboardData> {
        self.clipboard_manager.fire_copy_event()
    }

    fn on_paste(&mut self, clipboard: &ClipboardData) -> bool {
        self.clipboard_manager.fire_paste_event(clipboard)
    }

    fn on_cut(&mut self) -> Option<ClipboardData> {
        self.clipboard_manager.fire_cut_event()
    }

    fn on_update(&mut self, manager: &ContainerManager) -> EventResult {
        self.update_manager.fire_update(manager);

        self.check_agents()
    }

    fn predict_render(&mut self) -> Vec<PlannedRenderAction> {
        self.render_manager.predict_render()
    }

    fn force_partial_render(&mut self, regions: &[Region]) -> Vec<PlannedRenderAction> {
        self.render_manager.force_partial_render(regions)
    }

    fn on_render(&mut self, gl: &WebGlRenderingContext, manager: &ContainerManager) -> RenderResult {
        let render_result = self.render_manager.render(gl, manager, self.mouse_pos);

        // TODO Hm... what about components that did not re-render?
        self.last_render_actions = render_result.1;

        self.check_agents().expect_none("A component attempted to replace the current container during a render event");

        render_result.0
    }

    fn force_render(&mut self){
        self.render_manager.force_full_render();
    }

    fn add_component(&mut self, component: Rc<RefCell<dyn Component>>) {
        let behaviors = component.borrow_mut().create_behaviors();
        for behavior in &behaviors {
            let mut agent = SimpleLayerAgent::new(self);
            behavior.borrow_mut().attach(&mut agent);
            let store = agent.store;

            for request in &store.render_requests {
                self.render_manager.claim_space(request.region, request.trigger, request.phase_id, request.opacity, Rc::downgrade(&behavior));
            }

            match store.key_down_space {
                Some(region) => {
                    self.key_manager.add_region_key_down_listener(Rc::downgrade(&behavior), region);
                }, None => {}
            };

            match store.key_up_space {
                Some(region) => {
                    self.key_manager.add_region_key_up_listener(Rc::downgrade(&behavior), region);
                }, None => {}
            };

            match store.key_down_priority {
                Some(priority) => {
                    self.key_manager.add_global_key_down_listener(Rc::downgrade(&behavior), priority);
                }, None => {}
            };

            match store.key_up_priority {
                Some(priority) => {
                    self.key_manager.add_global_key_up_listener(Rc::downgrade(&behavior), priority);
                }, None => {}
            };

            match store.mouse_click_space {
                Some(space) => {
                    self.mouse_manager.add_click_space_listener(Rc::downgrade(&behavior), space);
                }, None => {}
            };
            
            if store.mouse_click_global {
                self.mouse_manager.add_full_click_listener(Rc::downgrade(&behavior));
            }

            match store.mouse_scroll_space {
                Some(space) => {
                    self.mouse_manager.add_scroll_space_listener(Rc::downgrade(&behavior), space);
                }, None => {}
            };

            match store.mouse_scroll_priority {
                Some(priority) => {
                    self.mouse_manager.add_full_scroll_listener(Rc::downgrade(&behavior), priority);
                }, None => {}
            };

            match store.mouse_move_space {
                Some(space) => {
                    self.mouse_manager.add_move_space_listener(Rc::downgrade(&behavior), space);
                }, None => {}
            };

            match store.mouse_move_in_out_space {
                Some(space) => {
                    self.mouse_manager.add_in_out_move_listener(Rc::downgrade(&behavior), space);
                }, None => {}
            };

            if store.copy_priority.is_some() {
                self.clipboard_manager.add_copy_listener(Rc::downgrade(&behavior), store.copy_priority.unwrap());
            }

            if store.paste_priority.is_some() {
                self.clipboard_manager.add_paste_listener(Rc::downgrade(&behavior), store.paste_priority.unwrap());
            }

            if store.cut_priority.is_some() {
                self.clipboard_manager.add_cut_listener(Rc::downgrade(&behavior), store.cut_priority.unwrap());
            }

            if store.mouse_move_global {
                self.mouse_manager.add_full_move_listener(Rc::downgrade(&behavior));
            }

            if store.receive_updates {
                self.update_manager.add_listener(Rc::downgrade(&behavior));
            }
        }

        self.components.push(OuterHandle::new(component, behaviors));
    }
}

struct RenderRequest {

    region: Region,
    trigger: RenderTrigger,
    phase_id: &'static dyn RenderPhaseID,
    opacity: RenderOpacity
}

struct SimpleLayerAgentStore {

    render_requests: Vec<RenderRequest>,

    key_down_space: Option<Region>,
    key_up_space: Option<Region>,

    key_down_priority: Option<i8>,
    key_up_priority: Option<i8>,

    mouse_click_space: Option<Region>,
    mouse_click_global: bool,

    mouse_scroll_space: Option<Region>,
    mouse_scroll_priority: Option<i8>,

    copy_priority: Option<i8>,
    paste_priority: Option<i8>,
    cut_priority: Option<i8>,

    mouse_move_space: Option<Region>,
    mouse_move_in_out_space: Option<Region>,
    mouse_move_global: bool,

    receive_updates: bool
}

impl SimpleLayerAgentStore {

    fn new() -> Self {
        Self {
            render_requests: Vec::with_capacity(1),

            key_down_space: None,
            key_up_space: None,

            key_down_priority: None,
            key_up_priority: None,

            mouse_click_space: None,
            mouse_click_global: false,

            mouse_scroll_space: None,
            mouse_scroll_priority: None,

            mouse_move_space: None,
            mouse_move_in_out_space: None,
            mouse_move_global: false,

            copy_priority: None,
            paste_priority: None,
            cut_priority: None,

            receive_updates: false
        }
    }
}

pub struct SimpleLayerAgent<'a> {

    layer: &'a SimpleLayer,
    store: SimpleLayerAgentStore
}

impl<'a> SimpleLayerAgent<'a> {

    fn new(layer: &'a SimpleLayer) -> Self {
        Self {
            layer,
            store: SimpleLayerAgentStore::new()
        }
    }
}

impl<'a> LayerAgent for SimpleLayerAgent<'a> {

    fn claim_render_space(&mut self, region: Region, trigger: RenderTrigger, opacity: RenderOpacity, phase_id: &'static dyn RenderPhaseID) -> Result<(),RenderRequestError> {

        if !self.layer.render_manager.can_claim(region) {
            return Err(RenderRequestError::RegionAlreadyClaimed);
        }

        if !self.layer.render_manager.knows_render_phase(phase_id) {
            return Err(RenderRequestError::UnregisteredRenderPhase);
        }

        self.store.render_requests.push(RenderRequest {region, trigger, phase_id, opacity});
        Ok(())
    }

    fn claim_key_down_space(&mut self, region: Region) -> Result<(),()> {

        if !self.layer.key_manager.can_claim_down(region) {
            return Err(());
        }

        self.store.key_down_space = Some(region);
        Ok(())
    }

    fn claim_key_up_space(&mut self, region: Region) -> Result<(),()> {

        if !self.layer.key_manager.can_claim_up(region) {
            return Err(());
        }

        self.store.key_up_space = Some(region);
        Ok(())
    }

    fn claim_key_listen_space(&mut self, region: Region) -> Result<(),()> {
        if !self.layer.key_manager.can_claim_down(region) && !self.layer.key_manager.can_claim_up(region) {
            return Err(());
        }

        self.store.key_down_space = Some(region);
        self.store.key_up_space = Some(region);
        Ok(())
    }

    fn make_key_down_listener(&mut self, priority: i8){
        self.store.key_down_priority = Some(priority);
    }

    fn make_key_up_listener(&mut self, priority: i8){
        self.store.key_up_priority = Some(priority);
    }

    fn make_key_listener(&mut self, priority: i8){
        self.store.key_down_priority = Some(priority);
        self.store.key_up_priority = Some(priority);
    }

    fn claim_mouse_click_space(&mut self, region: Region) -> Result<(),()> {
        if !self.layer.mouse_manager.can_claim_click_space(region) {
            return Err(());
        }

        self.store.mouse_click_space = Some(region);
        Ok(())
    }

    fn claim_mouse_scroll_space(&mut self, region: Region) -> Result<(),()> {
        if !self.layer.mouse_manager.can_claim_scroll_space(region) {
            return Err(());
        }

        self.store.mouse_scroll_space = Some(region);
        Ok(())
    }

    fn make_mouse_scroll_listener(&mut self, priority: i8) {
        self.store.mouse_scroll_priority = Some(priority);
    }

    fn claim_mouse_move_space(&mut self, region: Region){
        self.store.mouse_move_space = Some(region);
    }

    fn claim_mouse_in_out_space(&mut self, region: Region){
        self.store.mouse_move_in_out_space = Some(region);
    }

    fn make_mouse_move_listener(&mut self){
        self.store.mouse_move_global = true;
    }

    fn make_mouse_click_listener(&mut self){
        self.store.mouse_click_global = true;
    }

    fn make_copy_listener(&mut self, priority: i8) {
        self.store.copy_priority = Some(priority);
    }

    fn make_paste_listener(&mut self, priority: i8) {
        self.store.paste_priority = Some(priority);
    }

    fn make_cut_listener(&mut self, priority: i8) {
        self.store.cut_priority = Some(priority);
    }

    fn make_update_listener(&mut self){
        self.store.receive_updates = true;
    }
}