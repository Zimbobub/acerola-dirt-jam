


use spade::Triangulation;

use crate::{terrain::{region::RegionPos, sampled_world::SampledWorld, world::World, Pos}};





mod gpu;
mod terrain;


fn main() {
    let mut world = World::new();
    world.generate_region(RegionPos::new(0, 0));
    println!("{} total chunks from {} verticies", world.triangulation.num_inner_faces(), world.regions.values().map(|v| v.len()).sum::<usize>());

    let world_sample = SampledWorld::init(world, Pos::new(0.0, 0.0), 1024.0);
    println!("{} chunks in render distance", world_sample.chunks.len());

    // let gpu = GPU::init();



    // delaunay triangulation
    // let centroids_buffer = gpu.buffer_from_iter(
    //     world_gen.centroids.clone(),
    //     BufferUsage::UNIFORM_BUFFER, 
    //     MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE
    // );

    // let triangles_buffer = gpu.buffer_from_iter(
    //     (0..world_gen.centroids.len()).map(|_| 0),
    //     BufferUsage::UNIFORM_BUFFER, 
    //     MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE
    // );
}
