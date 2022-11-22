use bevy::{prelude::{App, Plugin, Resource}, render::renderer::{RenderDevice, RenderQueue}};
use piet_wgsl::prelude::Engine;
mod render;

#[derive(Resource,)]
pub struct PietRenderResources {
    pub engine: Engine,
}

/// Contains the Bevy interface to the Piet renderer.
#[derive(Default)]
pub struct PietRenderPlugin;

impl Plugin for PietRenderPlugin {
    fn build(&self, _app: &mut App) {
        let mut render_app = App::empty();
        let engine = Engine::new();
        render_app.insert_resource(PietRenderResources { engine });
    }
}

pub async fn run_render(
    device: &RenderDevice,
    queue: &RenderQueue,
) -> Result<(), Box<dyn std::error::Error>> {
    

    let mut engine = Engine::new();
    render::do_render(&device.wgpu_device(), &queue, &mut engine).await?;

    Ok(())
}
