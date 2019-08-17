#[derive(Clone,Copy,std::fmt::Debug)]
pub struct Region {
    min_x: f32,
    min_y: f32,
    max_x: f32,
    max_y: f32
}

impl Region {

    pub fn new(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Region {
        Region {
            min_x,
            min_y,
            max_x,
            max_y
        }
    }

    pub fn intersects_with(&self, other: &Region) -> bool {
        self.min_x < other.max_x && self.min_y < other.max_y && other.min_x < self.max_x && other.min_y < self.max_y
    }

    pub fn is_inside(&self, point: (f32,f32)) -> bool {
        point.0 > self.min_x && point.0 < self.max_x && point.1 > self.min_y && point.1 < self.max_y
    }
}