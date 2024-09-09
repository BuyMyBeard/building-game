use bevy_inspector_egui::{prelude::*, quick::ResourceInspectorPlugin};
use bevy::prelude::*;

use crate::lasso::cursor::CursorWorldCoords;

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct InspectorParams {
    #[inspector(min = 0.0, max = 100.0)]
    pub threshold: f32,
    #[inspector(min = 0.0, max = 10000000.0)]
    pub force: f32,
}

pub fn debug_plugin(app: &mut App) {
    app.init_resource::<InspectorParams>()
    .register_type::<InspectorParams>()
    .add_plugins(ResourceInspectorPlugin::<CursorWorldCoords>::default())
    .add_plugins(ResourceInspectorPlugin::<InspectorParams>::default());
}