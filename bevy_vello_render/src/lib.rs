use bevy::{
    prelude::{App, FromWorld, Image, Plugin, Query, Res, ResMut, Resource, World},
    render::{
        render_asset::RenderAssets,
        renderer::{RenderDevice, RenderQueue},
        RenderApp, RenderStage,
    },
};

use screen::VelloScene;
use vello::Renderer;
pub mod screen;

pub use vello::kurbo::{Affine, BezPath, Ellipse, PathEl, Point, Rect};
pub use vello::peniko::*;
pub use vello::*;

#[derive(Resource)]
struct VelloRenderer(Renderer);

impl FromWorld for VelloRenderer {
    fn from_world(world: &mut World) -> Self {
        let device = world.get_resource::<RenderDevice>().unwrap();
        VelloRenderer(Renderer::new(device.wgpu_device()).unwrap())
    }
}

/// Contains the Bevy interface to the Vello renderer.
#[derive(Default)]
pub struct VelloRenderPlugin;

impl Plugin for VelloRenderPlugin {
    fn build(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else { return };
        render_app.init_resource::<VelloRenderer>();
        // This should probably use the render graph, but working out the dependencies there is awkward
        render_app.add_system_to_stage(RenderStage::Render, render_scenes);
    }
}

fn render_scenes(
    mut renderer: ResMut<VelloRenderer>,
    mut scenes: Query<&VelloScene>,
    gpu_images: Res<RenderAssets<Image>>,
    device: Res<RenderDevice>,
    queue: Res<RenderQueue>,
) {
    for scene in &mut scenes {
        let gpu_image = gpu_images.get(&scene.1).unwrap();

        renderer
            .0
            .render_to_texture(
                device.wgpu_device(),
                &*queue,
                &scene.0,
                &gpu_image.texture_view,
                gpu_image.size.x as u32,
                gpu_image.size.y as u32,
            )
            .unwrap();
    }
}
