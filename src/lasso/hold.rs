
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{debug::debug::InspectorParams, project_point};

use super::cursor::{CursorWorldCoords, MBHeldEvent, MBReleasedEvent};

#[derive(Component)]
pub struct HeldBody {
    offset: Vec2,
}

// pub fn move_held_body(
//   mut query: Query<(&mut Transform,  &HeldBody)>,
//   mouse_pos: Res<CursorWorldCoords>,
// ) {
//   for (mut transform, held_body) in &mut query {
//       transform.translation = (mouse_pos.0 + held_body.offset).xyx(); 
//   }
// }

pub fn drag_held_body(
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

pub fn hold_body(
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

pub fn release_body(
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
