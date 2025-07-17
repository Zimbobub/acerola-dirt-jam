use crate::terrain::{world::World, RegionPos};





mod gpu;
mod terrain;


fn main() {
    let mut world = World::new();
    world.generate_region(RegionPos::default());
}
