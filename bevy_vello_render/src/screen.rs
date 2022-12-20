use bevy::{
    prelude::{Component, Handle, Image},
    render::extract_component::ExtractComponent,
};
use vello::Scene;

#[derive(Component)]
pub struct VelloScene(pub Scene, pub Handle<Image>);

#[derive(Component)]
pub struct VelloTarget(pub Handle<Image>);

#[derive(Component)]
// In the future, this will probably connect to the bevy heirarchy with an Affine component
pub struct VelloFragment(pub vello::SceneFragment);

impl ExtractComponent for VelloScene {
    type Query = (&'static VelloFragment, &'static VelloTarget);

    type Filter = ();

    fn extract_component((fragment, target): bevy::ecs::query::QueryItem<'_, Self::Query>) -> Self {
        let mut scene = vello::Scene::default();
        let mut builder = vello::SceneBuilder::for_scene(&mut scene);
        builder.append(&fragment.0, None);
        builder.finish();
        Self(scene, target.0.clone())
    }
}
