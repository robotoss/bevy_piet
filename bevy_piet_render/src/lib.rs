use bevy::{prelude::{App, Plugin, Resource}, render::renderer::RenderDevice};
use piet_wgsl::Renderer;
mod render;

#[derive(Resource,)]
pub struct PietRenderResources {
    pub render: Renderer,
}

/// Contains the Bevy interface to the Piet renderer.
#[derive(Default)]
pub struct PietRenderPlugin;

impl Plugin for PietRenderPlugin {
    fn build(&self, app: &mut App) {
        let mut render_app = App::empty();
       let render_device = app.world.resource::<RenderDevice>().clone();

        let render = Renderer::new(render_device.wgpu_device()).expect("Can't create new Renderer");

        render_app.insert_resource(PietRenderResources { render });
    }
}

pub async fn run_render(
) -> Result<(), Box<dyn std::error::Error>> {

    Ok(())
}
