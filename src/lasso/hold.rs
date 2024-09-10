
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{debug::debug::InspectorParams, project_point};

use super::cursor::{CursorWorldCoords, MBHeldEvent, MBReleasedEvent};

#[derive(Component)]
pub struct HeldBody;

pub fn drag_held_body(
  mut q_held_body: Query<(&mut Velocity, &Transform), With<HeldBody>>,
  res_mouse_pos: Res<CursorWorldCoords>,
  res_inspector_params: Res<InspectorParams>,
) {
  if q_held_body.is_empty() {
    return;
  }
  let (mut velocity, transform) = q_held_body.single_mut();
  let ab = res_mouse_pos.0 - transform.translation.xy();
  let dist = ab.length_squared();
  let norm_vec = ab.normalize_or_zero();
  let new_velocity = norm_vec * quadratic_curve(dist, res_inspector_params.growth);
  println!("dist: {}, norm_vec: {}, new_velocity: {}", dist, norm_vec, new_velocity);
  velocity.linvel = new_velocity.clamp_length_max(res_inspector_params.max_speed);
}

fn step_curve(x: f32, step: f32, threshold: f32) -> f32 {
  if x >= threshold {
      return step;
  }
  return 0.0;
}

fn quadratic_curve(x: f32, a: f32) -> f32 {
  return a * x * x;
}

pub fn hold_body(
  mut evr_mb_held: EventReader<MBHeldEvent>,
  rapier_context: Res<RapierContext>,
  mut q_rb: Query<(Entity, &Transform, &mut GravityScale, &mut Damping)>,
  res_mouse_pos: Res<CursorWorldCoords>,
  mut commands: Commands,
  res_inspector_params: Res<InspectorParams>,
) -> () {
  if evr_mb_held.read().len() == 0 {
      return;
  }

  let mut ql_rb = q_rb.transmute_lens::<(Entity, &Transform)>();

  match project_point(rapier_context, res_mouse_pos.0, ql_rb.query()) {
      Some(entity) => {
          let (_, __, mut gravity, mut damping) = q_rb.get_mut(entity).unwrap();
          // let offset = transform.translation.xy() - res_mouse_pos.0;
          commands.entity(entity).insert(HeldBody);
          gravity.0 = 0.0;
          damping.angular_damping = res_inspector_params.angular_damping;
      },
      None => {},
  }
}

pub fn release_body(
  mut mb_released_evr: EventReader<MBReleasedEvent>,
  mut query: Query<(Entity, &mut GravityScale, &mut Velocity, &mut Damping), With<HeldBody>>,
  mut commands: Commands,
) {
  if mb_released_evr.read().len() == 0 || query.is_empty() {
      return;
  }

  let (entity, mut gravity, mut velocity, mut damping) = query.single_mut();
  commands.entity(entity).remove::<HeldBody>();
  gravity.0 = 1.0;
  velocity.linvel = Vec2::ZERO;
  damping.angular_damping = 0.0;
}
