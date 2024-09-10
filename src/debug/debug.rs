use bevy_inspector_egui::{prelude::*, quick::ResourceInspectorPlugin};
use bevy::prelude::*;

use crate::lasso::cursor::CursorWorldCoords;

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct InspectorParams {
    #[inspector(min = 1000.0, max = 10000.0)]
    pub max_speed: f32,
    #[inspector(min = 0.0, max = 1.0)]
    pub growth: f32,
    #[inspector(min = 0.0, max = 100.0)]
    pub angular_damping: f32,
}

impl Default for InspectorParams {
    fn default() -> Self {
        InspectorParams{
            max_speed: 5000.0,
            growth: 0.01,
            angular_damping: 10.0,
        }
    }
}

pub fn debug_plugin(app: &mut App) {
    app.init_resource::<InspectorParams>()
    .register_type::<InspectorParams>()
    .add_plugins(ResourceInspectorPlugin::<CursorWorldCoords>::default())
    .add_plugins(ResourceInspectorPlugin::<InspectorParams>::default());
}