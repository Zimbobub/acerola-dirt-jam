


use spade::Triangulation;

use crate::{gpu::GPU, terrain::{region::RegionPos, sampled_world::SampledWorld, world::World, Pos}};





mod gpu;
mod terrain;


fn main() {
    let mut world = World::new();
    world.generate_region(RegionPos::new(0, 0));
    println!("{} total chunks from {} verticies", world.triangulation.num_inner_faces(), world.regions.values().map(|v| v.len()).sum::<usize>());

    let world_sample = SampledWorld::init(world, Pos::new(0.0, 0.0), 1024.0);
    println!("{} chunks in render distance", world_sample.chunks.len());

    for chunk in world_sample.chunks.values() {
        println!("Chunk ({}, {}) ({}, {}) ({}, {})", chunk.verticies[0].x, chunk.verticies[0].y, chunk.verticies[1].x, chunk.verticies[1].y, chunk.verticies[2].x, chunk.verticies[2].y);
    }

    let gpu = GPU::init();

    gpu::render::render(&gpu, &world_sample);

}
