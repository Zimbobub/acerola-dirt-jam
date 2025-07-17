use crate::terrain::{region::RegionPos, world_save::WorldSave};





mod gpu;
mod terrain;


fn main() {
    let mut world = WorldSave::new();
    world.generate_region(RegionPos::default());
}
