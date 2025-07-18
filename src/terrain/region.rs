use core::ops::Range;






pub const REGION_SIZE: f64 = 64.0;
pub const REGION_CHUNKS: Range<usize> = 4..8;



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RegionPos {
    pub x: i32,
    pub y: i32
}

impl RegionPos {
    pub fn new(x: i32, y: i32) -> Self {
        return Self { x: x, y: y };
    }
}
