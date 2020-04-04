use std::cmp::Ordering;

use std::fmt::{
    Debug,
    Error,
    Formatter
};

use std::hash::Hash;
use std::hash::Hasher;

pub trait RenderPhaseID {

    fn get_crate_name(&self) -> &str;

    fn get_local_name(&self) -> &str;

    fn cmp(&self, other: &dyn RenderPhaseID) -> Ordering {
        let crate_order = self.get_crate_name().cmp(other.get_crate_name());
        if crate_order == Ordering::Equal {
            self.get_local_name().cmp(other.get_local_name())
        } else {
            crate_order
        }
    }
}

impl PartialEq<dyn RenderPhaseID> for dyn RenderPhaseID {

    fn eq(&self, other: &dyn RenderPhaseID) -> bool {
        self.get_crate_name().eq(other.get_crate_name()) && self.get_local_name().eq(other.get_local_name())
    }
}

//impl Eq for Box<dyn RenderPhaseID> {}

impl Eq for dyn RenderPhaseID {}

impl Hash for dyn RenderPhaseID {

    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.get_crate_name().hash(hasher);
        self.get_local_name().hash(hasher);
    }
}

impl Debug for dyn RenderPhaseID {

    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        write!(formatter, "{}:{}", self.get_crate_name(), self.get_local_name())
    }
}

pub struct StaticRenderPhaseID {

    crate_name: &'static str,
    local_name: &'static str
}

impl StaticRenderPhaseID {

    pub const fn new(crate_name: &'static str, local_name: &'static str) -> Self {
        Self { crate_name, local_name }
    }
}

impl RenderPhaseID for StaticRenderPhaseID {

    fn get_crate_name(&self) -> &str {
        &self.crate_name
    }

    fn get_local_name(&self) -> &str {
        &self.local_name
    }
}

pub struct StringRenderPhaseID {

    crate_name: String,
    local_name: String
}

impl StringRenderPhaseID {

    pub fn new(crate_name: String, local_name: String) -> Self {
        Self { crate_name, local_name }
    }
}

impl RenderPhaseID for StringRenderPhaseID {

    fn get_crate_name(&self) -> &str {
        &self.crate_name
    }

    fn get_local_name(&self) -> &str {
        &self.local_name
    }
}