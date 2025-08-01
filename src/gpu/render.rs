use image::{ImageBuffer, Rgba};
use vulkano::{buffer::{BufferContents, BufferUsage}, command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, CopyImageToBufferInfo, RenderPassBeginInfo, SubpassBeginInfo, SubpassContents, SubpassEndInfo}, format::{self, Format}, image::{view::ImageView, Image, ImageCreateInfo, ImageType, ImageUsage}, memory::allocator::{AllocationCreateInfo, MemoryTypeFilter}, pipeline::{graphics::{color_blend::{ColorBlendAttachmentState, ColorBlendState}, input_assembly::InputAssemblyState, multisample::MultisampleState, rasterization::RasterizationState, vertex_input::{Vertex, VertexDefinition}, viewport::{Viewport, ViewportState}, GraphicsPipelineCreateInfo}, layout::PipelineDescriptorSetLayoutCreateInfo, GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo}, render_pass::{Framebuffer, FramebufferCreateInfo, Subpass}, sync::GpuFuture};

use crate::{gpu::GPU, terrain::{sampled_world::SampledWorld, Pos}};


#[derive(Debug, BufferContents, Vertex)]
#[repr(C)]
struct ChunkVertex {
    #[format(R32G32_SFLOAT)]
    position: [f32; 2],
}

impl ChunkVertex {
    pub fn new(x: f32, y: f32) -> Self {
        return Self {
            position: [x, y]
        };
    }

    pub fn from_pos_triangle(tri: [Pos; 3], viewport_size: f64) -> [Self; 3] {
        return [
            tri[0].scaled(1.0/viewport_size).transformed(-0.5, -0.5).into(),
            tri[1].scaled(1.0/viewport_size).transformed(-0.5, -0.5).into(),
            tri[2].scaled(1.0/viewport_size).transformed(-0.5, -0.5).into(),
        ];
    }
}

impl From<Pos> for ChunkVertex {
    fn from(pos: Pos) -> Self {
        return ChunkVertex::new(pos.x as f32, pos.y as f32);
    }
}

















pub fn render(gpu: &GPU, world_sample: &SampledWorld) {
    // output image
    let image = Image::new(
        gpu.memory_allocator.clone(),
        ImageCreateInfo {
            image_type: ImageType::Dim2d,
            format: Format::R8G8B8A8_UNORM,
            extent: [1024, 1024, 1],
            usage: ImageUsage::COLOR_ATTACHMENT | ImageUsage::TRANSFER_SRC,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
            ..Default::default()
        },
    ).unwrap();






    // BUFFERS
    const VIEWPORT_SIZE: f64 = 48.0;

    let verticies: Vec<ChunkVertex> = world_sample.chunks.values().map(|chunk| ChunkVertex::from_pos_triangle(chunk.verticies, VIEWPORT_SIZE)).flatten().collect();
    let n_verticies: u32 = verticies.len() as u32;

    for (i, v) in verticies.iter().enumerate() {
        println!("Vertex {} {}", v.position[0], v.position[1]);
        if i % 3 == 2 {
            print!("\n");
        }
    }

    let vertex_buffer = gpu.buffer_from_iter(
        verticies,
        BufferUsage::VERTEX_BUFFER, 
        MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE
    );


    let output_buffer = gpu.buffer_from_iter(
        (0..1024 * 1024 * 4).map(|_| 0u8),
        BufferUsage::TRANSFER_DST,
        MemoryTypeFilter::PREFER_HOST | MemoryTypeFilter::HOST_RANDOM_ACCESS
    );





    /////////// Render pass

    let render_pass = vulkano::single_pass_renderpass!(
        gpu.device.clone(),
        attachments: {
            color: {
                format: Format::R8G8B8A8_UNORM,
                samples: 1,
                load_op: Clear,
                store_op: Store,
            },
        },
        pass: {
            color: [color],
            depth_stencil: {},
        },
    ).unwrap();



    let view = ImageView::new_default(image.clone()).unwrap();
    let framebuffer = Framebuffer::new(
        render_pass.clone(),
        FramebufferCreateInfo {
            attachments: vec![view],
            ..Default::default()
        },
    ).unwrap();











    //////// Pipeline
    

    let vs = super::shaders::vertex_shader::load(gpu.device.clone()).expect("failed to create shader module");
    let fs = super::shaders::fragment_shader::load(gpu.device.clone()).expect("failed to create shader module");



    let viewport = Viewport {
        offset: [0.0, 0.0],
        extent: [1024.0, 1024.0],
        depth_range: 0.0..=1.0,
    };

    let pipeline = {
        // A Vulkan shader can in theory contain multiple entry points, so we have to specify
        // which one.
        let vs = vs.entry_point("main").unwrap();
        let fs = fs.entry_point("main").unwrap();

        let vertex_input_state = ChunkVertex::per_vertex()
            .definition(&vs)
            .unwrap();

        let stages = [
            PipelineShaderStageCreateInfo::new(vs),
            PipelineShaderStageCreateInfo::new(fs),
        ];

        let layout = PipelineLayout::new(
            gpu.device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
                .into_pipeline_layout_create_info(gpu.device.clone())
                .unwrap(),
        )
        .unwrap();

        let subpass = Subpass::from(render_pass.clone(), 0).unwrap();

        GraphicsPipeline::new(gpu.device.clone(), None,
            GraphicsPipelineCreateInfo {
                // The stages of our pipeline, we have vertex and fragment stages.
                stages: stages.into_iter().collect(),
                // Describes the layout of the vertex input and how should it behave.
                vertex_input_state: Some(vertex_input_state),
                // Indicate the type of the primitives (the default is a list of triangles).
                input_assembly_state: Some(InputAssemblyState::default()),
                // Set the fixed viewport.
                viewport_state: Some(ViewportState {
                    viewports: [viewport].into_iter().collect(),
                    ..Default::default()
                }),
                // Ignore these for now.
                rasterization_state: Some(RasterizationState::default()),
                multisample_state: Some(MultisampleState::default()),
                color_blend_state: Some(ColorBlendState::with_attachment_states(
                    subpass.num_color_attachments(),
                    ColorBlendAttachmentState::default(),
                )),
                // This graphics pipeline object concerns the first pass of the render pass.
                subpass: Some(subpass.into()),
                ..GraphicsPipelineCreateInfo::layout(layout)
            },
        )
        .unwrap()
    };












    ///////////// Draw calls

    let mut builder = AutoCommandBufferBuilder::primary(
        gpu.command_buffer_allocator.clone(),
        gpu.queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    ).unwrap();

    unsafe {
        builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![Some([0.0, 0.0, 1.0, 1.0].into())],
                    ..RenderPassBeginInfo::framebuffer(framebuffer.clone())
                },
                SubpassBeginInfo {
                    contents: SubpassContents::Inline,
                    ..Default::default()
                },
            ).unwrap()

            .bind_pipeline_graphics(pipeline.clone()).unwrap()
            .bind_vertex_buffers(0, vertex_buffer.clone()).unwrap()
            .draw(
                n_verticies, 1, 0, 0, // 3 is the number of vertices, 1 is the number of instances
            ).unwrap()

            .end_render_pass(SubpassEndInfo::default()).unwrap()
            .copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(image, output_buffer.clone())).unwrap();
    }

    let command_buffer = builder.build().unwrap();










    //////// Run & Results
    let start = std::time::Instant::now();
    // gpu.run(command_buffer);
    let future = vulkano::sync::now(gpu.device.clone())
        .then_execute(gpu.queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();

    future.wait(None).unwrap();
    println!("Done in {:.2?}", start.elapsed());

    let buffer_content = output_buffer.read().unwrap();
    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();
    image.save("image.png").unwrap();



    println!("Everything succeeded!");
}