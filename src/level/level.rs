use bevy::app::{App, Startup};

use super::{camera::setup_camera, populate::{spawn_cube_pile, spawn_ground}};

pub fn level_plugin(app: &mut App) {
  app.add_systems(Startup, setup_camera)
    .add_systems(Startup, spawn_ground)
    .add_systems(Startup, spawn_cube_pile);
}