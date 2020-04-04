use super::*;

use std::collections::HashMap;

pub struct RenderPhaseStore {

    phase_map: HashMap<Box<dyn RenderPhaseID>, Box<dyn RenderPhase>>
}

impl RenderPhaseStore {

    pub fn new() -> Self {
        Self {
            phase_map: HashMap::new()
        }
    }

    pub fn register(&mut self, id: Box<dyn RenderPhaseID>, phase: Box<dyn RenderPhase>) {
        self.phase_map.insert(id, phase).expect_none("Duplicate render phase id")
    }

    fn get<'a>(&'a self, id: &'a dyn RenderPhaseID) -> Option<&'a dyn RenderPhase> {
        match self.phase_map.get(id) {
            Some(phase_box) => Some(phase_box.as_ref()),
            None => None
        }
    }
}