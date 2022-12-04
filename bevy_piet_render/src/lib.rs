use bevy::{
    prelude::{App, Plugin, Resource},
    render::renderer::RenderDevice,
};
use piet_scene::{Scene, SceneBuilder};
use piet_wgsl::Renderer;
use wgpu::{Device, Queue, TextureView};
mod render;
mod test_scene;

#[derive(Resource)]
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

pub fn run_render(
    mut render: Renderer,
    device: &Device,
    queue: &Queue,
    texture: &TextureView,
    width: u32,
    height: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut scene = Scene::default();

    let mut builder = SceneBuilder::for_scene(&mut scene);
    test_scene::render_funky_paths(&mut builder);
    builder.finish();

    render
        .render_to_texture(device, queue, &scene, texture, width, height)
        .expect("failed to render to texture");

    Ok(())
}
