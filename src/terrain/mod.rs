
use crate::terrain::region::{RegionPos, REGION_SIZE};




pub mod world_save;
pub mod world_gen;
pub mod region;
pub mod chunk;




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