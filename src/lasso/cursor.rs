use bevy::{input::mouse::MouseButtonInput, window::PrimaryWindow};
use::bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::level::camera::MainCamera;

#[derive(Event)]
pub struct MBHeldEvent();

#[derive(Event)]
pub struct MBReleasedEvent();

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct CursorWorldCoords(pub Vec2);

pub fn mb_events(
  mut evw_mb_held: EventWriter<MBHeldEvent>,
  mut evw_mb_released: EventWriter<MBReleasedEvent>,
  mut evr_mb: EventReader<MouseButtonInput>,
) {
  use bevy::input::ButtonState;
  for ev in evr_mb.read() {
      if ev.button != MouseButton::Left { 
          return;
      }
      match ev.state {
          ButtonState::Pressed => {
              evw_mb_held.send(MBHeldEvent());
          }
          ButtonState::Released => {
              evw_mb_released.send(MBReleasedEvent());
          }
      }
  }
}


pub fn update_cursor_position(
  mut mouse_coords: ResMut<CursorWorldCoords>,
  q_window: Query<&Window, With<PrimaryWindow>>,
  q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
  let (camera, camera_transform) = q_camera.single();

  let window = q_window.single();

  if let Some(world_position) = window.cursor_position()
      .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
      .map(|ray| ray.origin.truncate())
  {
      mouse_coords.0 = world_position;
  }
}