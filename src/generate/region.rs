
use crate::generate::{Pos, Rect};







/// Lazy means only centroids have been generated
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionState {
    LazyGenerated,
    FullyGenerated
}


/// A region is a 64x64 vertex area
/// The chunks in a region can extend out into its neighbors
/// A region has 5 voronoi centroids
#[derive(Debug, Clone)]
pub struct Region {
    /// coords is the top left of the region
    /// therefore bottom right is coords.x*64 + 64 ...
    /// coords must always be a multiple of 64
    pub coords: Pos,
    pub state: RegionState,
    pub centroids: [Pos; 5]
}


impl Region {
    pub fn generate(coords: Pos) -> Option<Self> {
        let centroids: [Pos; 5] = core::array::from_fn(|_| {
            return Pos::new(rand::random_range(coords.x..coords.x+64), rand::random_range(coords.y..coords.y+64));
        });

        println!("{} to {}", (coords.x..coords.x+64).start, (coords.x..coords.x+64).end-1);
        println!("    {:?}", centroids);

        return Some(Self { coords: coords, state: RegionState::LazyGenerated, centroids: centroids });
    }



    pub fn get_bounds(&self) -> Rect {
        return Rect::new(
            self.coords.x, self.coords.y,
            self.coords.x+64, self.coords.y+64
        );
    }
}