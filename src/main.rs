use crate::generate::{world::World, Pos};


mod generate;
mod render;



fn main() {
    let mut world: World = World::new();
    world.generate_initial_region();
    world.generate_region(Pos::new(64, 0));
    world.generate_region(Pos::new(64, 64));
    world.generate_region(Pos::new(-64, 64));

    world.export_centroids();
}
