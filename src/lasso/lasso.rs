use bevy::{input::mouse::MouseButtonInput, prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::InspectorOptions;
use crate::{level::camera::MainCamera, ReflectInspectorOptions};
use bevy_rapier2d::prelude::*;

fn move_held_body(
  mut query: Query<(&mut Transform,  &HeldBody)>,
  mouse_pos: Res<CursorWorldCoords>,
) {
  for (mut transform, held_body) in &mut query {
      transform.translation = (mouse_pos.0 + held_body.offset).xyx(); 
  }
}

fn drag_held_body(
  mut query: Query<(&mut ExternalForce, &Transform), With<HeldBody>>,
  mouse_pos: Res<CursorWorldCoords>,
  inspector_params: Res<InspectorParams>,
) {
  for (mut force, transform) in &mut query {
      let ab = mouse_pos.0 - transform.translation.xy();
      let dist = ab.length_squared();
      let norm_vec = ab.normalize_or_zero();
      let new_force = norm_vec * step_curve(dist, inspector_params.force, inspector_params.threshold);
      force.force = new_force;
  }
}

// fn linear_curve<F: Float>(x: F, m: F) -> F {
//     return m * x;
// }

fn step_curve(x: f32, step: f32, threshold: f32) -> f32 {
  if x >= threshold {
      return step;
  }
  return 0.0;
}

fn hold_body(
  mut mb_held_evr: EventReader<MBHeldEvent>,
  rapier_context: Res<RapierContext>,
  mut query: Query<(Entity, &Transform, &mut GravityScale)>,
  mouse_pos: Res<CursorWorldCoords>,
  mut commands: Commands,
) -> () {
  if mb_held_evr.read().len() == 0 {
      return;
  }

  let mut query_lens = query.transmute_lens::<(Entity, &Transform)>();

  match project_point(rapier_context, mouse_pos.0, query_lens.query()) {
      Some(entity) => {
          let (_, transform, mut gravity) = query.get_mut(entity).unwrap();
          let offset = transform.translation.xy() - mouse_pos.0;
          commands.entity(entity).insert(HeldBody{
              offset,
          });
          gravity.0 = 0.0;
      },
      None => {},
  }
}

fn release_body(
  mut mb_released_evr: EventReader<MBReleasedEvent>,
  mut query: Query<(Entity, &mut GravityScale, &mut ExternalForce), With<HeldBody>>,
  mut commands: Commands,
) {
  if mb_released_evr.read().len() == 0 {
      return;
  }
  for (entity, mut gravity, mut force) in &mut query {
      commands.entity(entity).remove::<HeldBody>();
      gravity.0 = 1.0;
      force.force = Vec2::ZERO;
  }
}

#[derive(Component)]
pub struct HeldBody {
    offset: Vec2,
}

#[derive(Event)]
pub struct MBHeldEvent();

#[derive(Event)]
pub struct MBReleasedEvent();

fn update_cursor_position(
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

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct CursorWorldCoords(Vec2);

fn mb_events(
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

pub fn lasso_plugin(app: &mut App) {
  app
    .init_resource::<CursorWorldCoords>()
    .register_type::<CursorWorldCoords>()
    .add_event::<MBHeldEvent>()
    .add_event::<MBReleasedEvent>()
    .add_systems(Update, mb_events)
    .add_systems(Update, (hold_body.after(mb_events), release_body.after(mb_events)))
    .add_systems(Update, drag_held_body.after(release_body) )
    .add_systems(Update, update_cursor_position);
}