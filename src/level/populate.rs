use::bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn spawn_cube_pile(mut commands: Commands) {
  const WIDTH: i32 = 13;
  const HEIGHT: i32 = 10;
  const CUBESIZE: f32 = 30.0;
  const HALF_CUBESIZE: f32 = CUBESIZE / 2.0;
  const STARTING_OFFSET_X: f32 = (WIDTH / 2) as f32 * CUBESIZE * -1.0;
  const STARTING_OFFSET_Y: f32 = - 50.0;
  let mut i = 0;

  while i < WIDTH * HEIGHT {
      let column = i % WIDTH;
      let row = i / WIDTH;
      commands
          .spawn(RigidBody::Dynamic)
          .insert(Collider::cuboid(HALF_CUBESIZE, HALF_CUBESIZE))
          .insert(Velocity::zero())
          .insert(GravityScale(1.0))
          .insert(Damping::default())
          .insert(ColliderMassProperties::MassProperties(MassProperties{
              local_center_of_mass: Vec2::ZERO,
              principal_inertia: 3000.0,
              mass: 10.0,
          }))
          .insert(TransformBundle::from(Transform::from_xyz(column as f32 * CUBESIZE + STARTING_OFFSET_X, row as f32 * CUBESIZE + STARTING_OFFSET_Y, 0.0)));
      i += 1;
  }
}

pub fn spawn_ground(mut commands: Commands) {
  commands
      .spawn(Collider::cuboid(500.0, 50.0))
      .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));
}

pub fn spawn_triangle(mut commands: Commands) {
  commands
      .spawn(RigidBody::Dynamic)
      .insert(Collider::triangle(Vec2::new(-100.0, 0.0), Vec2::new(100.0, 0.0), Vec2::new(0.0, 100.0)))
      .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
}