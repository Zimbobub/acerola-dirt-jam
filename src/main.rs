use crate::terrain::{region::RegionPos, world_gen::WorldGen, world_save::WorldSave, Pos};





mod gpu;
mod terrain;


fn main() {
    let mut world_save = WorldSave::new();
    world_save.generate_region(RegionPos::default());

    dbg!(&world_save.regions);

    let mut world_gen = WorldGen::init(world_save, Pos::new(1.0, 1.0), 20.0);

    dbg!(&world_gen.centroids);
}
