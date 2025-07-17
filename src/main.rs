use crate::terrain::{world_save::WorldSave, RegionPos};





mod gpu;
mod terrain;


fn main() {
    let mut world = WorldSave::new();
    world.generate_region(RegionPos::default());
}
