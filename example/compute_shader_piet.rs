//! A compute shader that simulates Conway's Game of Life.
//!
//! Compute shaders use the GPU for computing arbitrary information, that may be independent of what
//! is rendered to the screen.

use bevy::{
    prelude::*,
    render::{extract_component::ExtractComponentPlugin, render_resource::*},
};
use bevy_vello_render::{
    kurbo::Rect,
    screen::{VelloFragment, VelloScene, VelloTarget},
    Affine, Fill, LinearGradient, Point, SceneBuilder, SceneFragment, Stroke, VelloRenderPlugin,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                // uncomment for unthrottled FPS
                // present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..default()
            },
            ..default()
        }))
        .add_plugin(VelloRenderPlugin)
        .add_startup_system(setup)
        .add_system(cube_rotator_system)
        .add_plugin(ExtractComponentPlugin::<VelloScene>::default())
        .add_system(render_fragment)
        .run();
}

// Marks the main pass cube, to which the texture is applied.
#[derive(Component)]
struct MainPassCube;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::STORAGE_BINDING,
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

    // Light
    // NOTE: Currently lights are shared between passes - see https://github.com/bevyengine/bevy/issues/3462
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        ..default()
    });

    let cube_size = 4.0;
    let cube_handle = meshes.add(Mesh::from(shape::Box::new(cube_size, cube_size, cube_size)));

    // This material has the texture that has been rendered.
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle.clone()),
        reflectance: 0.02,
        unlit: false,
        ..default()
    });

    // Main pass cube, with material containing the rendered first pass texture.
    commands.spawn((
        PbrBundle {
            mesh: cube_handle,
            material: material_handle,
            transform: Transform::from_xyz(0.0, 0.0, 1.5)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 5.0)),
            ..default()
        },
        MainPassCube,
    ));

    // The main pass camera.
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn((
        VelloFragment(SceneFragment::default()),
        VelloTarget(image_handle),
    ));
}

fn render_fragment(mut fragment: Query<&mut VelloFragment>, mut frame: Local<usize>) {
    let mut fragment = fragment.single_mut();
    let mut builder = SceneBuilder::for_fragment(&mut fragment.0);
    render_brush_transform(&mut builder, *frame);
    *frame += 1;
}

fn render_brush_transform(sb: &mut SceneBuilder, i: usize) {
    let th = (std::f64::consts::PI / 180.0) * (i as f64);
    let linear = LinearGradient::new((0.0, 0.0), (0.0, 200.0)).stops([
        bevy_vello_render::Color::RED,
        bevy_vello_render::Color::GREEN,
        bevy_vello_render::Color::BLUE,
    ]);
    sb.fill(
        Fill::NonZero,
        Affine::translate((106.0, 106.0)),
        &linear,
        Some(around_center(Affine::rotate(th), Point::new(150.0, 150.0))),
        &Rect::from_origin_size(Point::default(), (300.0, 300.0)),
    );
    sb.stroke(
        &Stroke::new(106.0),
        Affine::IDENTITY,
        &linear,
        Some(around_center(
            Affine::rotate(th + std::f64::consts::PI / 2.),
            Point::new(203.0, 203.0),
        )),
        &Rect::from_origin_size(Point::new(53.0, 53.0), (406.0, 406.0)),
    );
}

fn around_center(xform: Affine, center: Point) -> Affine {
    Affine::translate(center.to_vec2()) * xform * Affine::translate(-center.to_vec2())
}

/// Rotates the outer cube (main pass)
fn cube_rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<MainPassCube>>) {
    for mut transform in &mut query {
        transform.rotate_x(1.0 * time.delta_seconds());
        // transform.rotate_y(0.7 * time.delta_seconds());
    }
}
