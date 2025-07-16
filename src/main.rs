use crate::generate::world::World;


mod generate;
mod render;



fn main() {
    let mut world: World = World::new();
    world.generate_initial_region();

    world.export_centroids();
}
