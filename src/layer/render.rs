use crate::*;

use wasmuri_core::Region;

pub type RenderPriority = i8;

#[derive(Clone,Copy)]
pub enum RenderTrigger {

    Request,
    MouseInOut,
    MouseMoveInside,
    MouseMove,
    Always
}

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
pub enum RenderOpacity {

    /// Each pixel in the render area is fully opaque
    Solid,

    /// Each pixel in the render area is either fully opaque or fully transparent.
    /// Which pixels are opaque and which are transparent only changes when the viewport changes.
    StaticSolidOrNothing,

    /// Each pixel in the render area is either fully opaque or fully transparent.
    /// Which pixels are opaque and which are transparent can change at any time.
    DynamicSolidOrNothing,

    /// Each pixel can have any transparency
    Mixed
}

#[derive(Clone,Copy)]
pub struct PlannedRenderAction {

    region: Region,
    opacity: RenderOpacity
}

impl PlannedRenderAction {

    pub fn new(region: Region, opacity: RenderOpacity) -> Self {
        Self {
            region,
            opacity
        }
    }

    pub fn get_region(&self) -> Region {
        self.region
    }

    pub fn get_opacity(&self) -> RenderOpacity {
        self.opacity
    }
}

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
pub struct PassedRenderAction {

    region: Region
}

impl PassedRenderAction {

    pub fn new(region: Region) -> Self {
        Self {
            region
        }
    }

    pub fn get_region(&self) -> Region {
        self.region
    }
}

#[derive(Clone)]
pub struct RenderResult {

    cursor: Option<Cursor>
}

impl RenderResult {

    pub fn new(cursor: Option<Cursor>) -> Self {
        RenderResult {
            cursor
        }
    }

    pub fn with_cursor(cursor: Cursor) -> Self {
        Self::new(Some(cursor))
    }

    pub fn without_cursor() -> Self {
        Self::new(None)
    }

    pub fn get_cursor(&self) -> Option<Cursor> {
        self.cursor.clone()
    }
}