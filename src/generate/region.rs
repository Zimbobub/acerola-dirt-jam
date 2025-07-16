
use crate::generate::{Pos, Rect};







/// Lazy means only centroids have been generated
pub enum RegionState {
    LazyGenerated,
    FullyGenerated
}


/// A region is a 64x64 vertex area
/// The chunks in a region can extend out into its neighbors
/// A region has 5 voronoi centroids
pub struct Region {
    /// coords is the top left of the region
    /// therefore bottom right is coords.x*64 + 64 ...
    /// coords must always be a multiple of 64
    pub coords: Pos,
    pub state: RegionState,
    pub centroids: [Pos; 5]
}


impl Region {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn get_bounds(&self) -> Rect {
        return Rect::new(
            self.coords.x, self.coords.y,
            self.coords.x+1, self.coords.y+1
        );
    }
}