use bevy::app::{App, Startup};

use super::camera::setup_camera;

pub fn level_plugin(app: &mut App) {
  app.add_systems(Startup, setup_camera)
}