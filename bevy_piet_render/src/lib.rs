use bevy::{prelude::{App, Plugin, Resource}};
use piet_wgsl::prelude::Engine;
use wgpu::Limits;
mod render;

#[derive(Resource,)]
pub struct PietRenderResources {
    pub engine: Engine,
}

/// Contains the Bevy interface to the Piet renderer.
#[derive(Default)]
pub struct PietRenderPlugin;

impl Plugin for PietRenderPlugin {
    fn build(&self, app: &mut App) {
        let mut render_app = App::empty();
        let engine = Engine::new();
        render_app.insert_resource(PietRenderResources { engine });
    }
}

pub async fn run_render(
    // device: &RenderDevice,
    // queue: &RenderQueue,
) -> Result<(), Box<dyn std::error::Error>> {
    let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
    let adapter = instance.request_adapter(&Default::default()).await.unwrap();
    let features = adapter.features();
    let mut limits = Limits::default();
    limits.max_storage_buffers_per_shader_stage = 16;
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: features & wgpu::Features::TIMESTAMP_QUERY,
                limits,
            },
            None,
        )
        .await?;


    let mut engine = Engine::new();
    render::do_render(&device, &queue, &mut engine).await?;

    Ok(())
}
