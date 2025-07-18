use vulkano::{buffer::{Buffer, BufferCreateInfo, BufferUsage}, memory::allocator::{AllocationCreateInfo, MemoryTypeFilter}};

use crate::{gpu::GPU, terrain::{region::RegionPos, sampled_world::SampledWorld, world::World, Pos}};





mod gpu;
mod terrain;


fn main() {
    let mut world = World::new();
    world.generate_region(RegionPos::default());

    dbg!(&world);
    let mut world_sample = SampledWorld::init(world, Pos::new(1.0, 1.0), 20.0);
    // dbg!(&world_gen.centroids);

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
