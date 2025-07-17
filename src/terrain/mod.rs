use core::ops::Range;




pub mod world_save;
pub mod world_gen;
pub mod chunk;


pub const REGION_SIZE: f64 = 64.0;
pub const REGION_CHUNKS: Range<usize> = 64..128;



#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct RegionPos {
    pub x: i32,
    pub y: i32
}

impl RegionPos {
    pub fn new(x: i32, y: i32) -> Self {
        return Self { x: x, y: y };
    }
}




#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Pos {
    pub x: f64,
    pub y: f64
}

impl From<RegionPos> for Pos {
    fn from(region_pos: RegionPos) -> Self {
        return Pos::new(region_pos.x as f64 * REGION_SIZE, region_pos.y as f64 * REGION_SIZE);
    }
}

impl Pos {
    pub fn new(x: f64, y: f64) -> Self {
        return Self { x: x, y: y };
    }
}