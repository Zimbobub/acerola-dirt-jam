
use crate::generate::{Pos, Rect};


/// Lazy means only centroids have been generated
#[derive(Debug, Clone)]
pub struct LazyRegion {
    /// coords is the top left of the region
    /// therefore bottom right is coords.x*64 + 64 ...
    /// coords must always be a multiple of 64
    pub coords: Pos,
    pub centroids: [Pos; 5]
}


#[derive(Debug, Clone)]
pub struct FullRegion {
    /// coords is the top left of the region
    /// therefore bottom right is coords.x*64 + 64 ...
    /// coords must always be a multiple of 64
    pub coords: Pos,
    pub centroids: [Pos; 5]
}





/// A region is a 64x64 vertex area
/// The chunks in a region can extend out into its neighbors
/// A region has 5 voronoi centroids
#[derive(Debug, Clone)]
pub enum Region {
    LazyGenerated(LazyRegion),
    FullyGenerated(FullRegion)
}


impl Region {
    /// Lazy generates the centroids
    pub fn init(coords: Pos) -> Self {
        let centroids: [Pos; 5] = core::array::from_fn(|_| {
            return Pos::new(rand::random_range(coords.x..coords.x+64), rand::random_range(coords.y..coords.y+64));
        });

        println!("{} to {}", (coords.x..coords.x+64).start, (coords.x..coords.x+64).end-1);
        println!("    {:?}", centroids);

        return Self::LazyGenerated(LazyRegion { coords: coords, centroids: centroids });
    }

    /// TODO: generate chunks in this
    pub fn fully_generate(&self) -> Option<Self> {
        match self {
            Self::FullyGenerated(_) => return None,
            Self::LazyGenerated(region) => {
                return Some(Self::FullyGenerated(FullRegion {
                    coords: region.coords,
                    centroids: region.centroids
                }));
            }
        };
    }

    pub fn coords(&self) -> Pos {
        match self {
            Self::LazyGenerated(region) => return region.coords,
            Self::FullyGenerated(region) => return region.coords,
        };
    }

    pub fn centroids(&self) -> [Pos; 5] {
        match self {
            Self::LazyGenerated(region) => return region.centroids,
            Self::FullyGenerated(region) => return region.centroids,
        };
    }


    pub fn get_neighbor_coords(&self) -> [Pos; 8] {
        return [
            self.coords().translate(Pos::new(-64, -64)),
            self.coords().translate(Pos::new(-64, 0)),
            self.coords().translate(Pos::new(-64, 64)),

            self.coords().translate(Pos::new(0, -64)),
            self.coords().translate(Pos::new(0, 64)),

            self.coords().translate(Pos::new(64, -64)),
            self.coords().translate(Pos::new(64, 0)),
            self.coords().translate(Pos::new(64, 64)),
        ]
    }


    pub fn get_bounds(&self) -> Rect {
        return Rect::new(
            self.coords().x, self.coords().y,
            self.coords().x+64, self.coords().y+64
        );
    }
}