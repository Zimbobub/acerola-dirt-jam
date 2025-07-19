
use spade::{HasPosition, Point2};
use vulkano::buffer::BufferContents;

use crate::terrain::region::{RegionPos, REGION_SIZE};




pub mod world;
pub mod sampled_world;
pub mod region;
pub mod chunk;




#[derive(Debug, Clone, Copy, PartialEq, Default, BufferContents)]
#[repr(C)]
pub struct Pos {
    pub x: f64,
    pub y: f64
}

impl From<RegionPos> for Pos {
    fn from(region_pos: RegionPos) -> Self {
        return Pos::new(region_pos.x as f64 * REGION_SIZE, region_pos.y as f64 * REGION_SIZE);
    }
}


impl<T: Into<f64>> From<Point2<T>> for Pos {
    fn from(pos: Point2<T>) -> Self {
        return Pos::new(pos.x.into(), pos.y.into());
    }
}

impl<T: From<f64>> Into<Point2<T>> for Pos {
    fn into(self) -> Point2<T> {
        return Point2::new(self.x.into(), self.y.into());
    }
}



impl HasPosition for Pos {
    type Scalar = f64;

    fn position(&self) -> spade::Point2<Self::Scalar> {
        return Point2::new(self.x, self.y);
    }
}


impl Pos {
    pub fn new(x: f64, y: f64) -> Self {
        return Self { x: x, y: y };
    }


    pub fn scaled(&self, scale: f64) -> Pos {
        return Pos::new(self.x * scale, self.y * scale);
    }
}