use bevy::prelude::{App, Plugin};

/// Contains the Bevy interface to the Piet renderer.
#[derive(Default)]
pub struct PietRenderPlugin;

impl Plugin for PietRenderPlugin {
    fn build(&self, app: &mut App) {}
}
