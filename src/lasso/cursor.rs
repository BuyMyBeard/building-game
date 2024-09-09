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
  mut mb_held_evw: EventWriter<MBHeldEvent>,
  mut mb_released_evw: EventWriter<MBReleasedEvent>,
  mut mb_evr: EventReader<MouseButtonInput>,
) {
  use bevy::input::ButtonState;
  for ev in mb_evr.read() {
      if ev.button != MouseButton::Left { 
          return;
      }
      match ev.state {
          ButtonState::Pressed => {
              mb_held_evw.send(MBHeldEvent());
          }
          ButtonState::Released => {
              mb_released_evw.send(MBReleasedEvent());
          }
      }
  }
}


pub fn update_cursor_position(
  mut mouse_coords: ResMut<CursorWorldCoords>,
  // query to get the window (so we can read the current cursor position)
  q_window: Query<&Window, With<PrimaryWindow>>,
  // query to get camera transform
  q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
  // get the camera info and transform
  // assuming there is exactly one main camera entity, so Query::single() is OK
  let (camera, camera_transform) = q_camera.single();

  // There is only one primary window, so we can similarly get it from the query:
  let window = q_window.single();

  // check if the cursor is inside the window and get its position
  // then, ask bevy to convert into world coordinates, and truncate to discard Z
  if let Some(world_position) = window.cursor_position()
      .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
      .map(|ray| ray.origin.truncate())
  {
      mouse_coords.0 = world_position;
  }
}