
use bevy::{input::mouse::MouseButtonInput, prelude::*, window::PrimaryWindow};

use bevy_inspector_egui::{prelude::ReflectInspectorOptions, quick::{ResourceInspectorPlugin, WorldInspectorPlugin}, InspectorOptions};
use bevy_rapier2d::prelude::*;

mod lasso;
mod level;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .init_resource::<InspectorParams>()
        .register_type::<InspectorParams>()
        .add_plugins(ResourceInspectorPlugin::<CursorWorldCoords>::default())
        .add_plugins(ResourceInspectorPlugin::<InspectorParams>::default())
        .add_event::<MBHeldEvent>()
        .add_event::<MBReleasedEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, mb_events)
        .add_systems(Update, (hold_body.after(mb_events), release_body.after(mb_events)))
        .add_systems(Update, drag_held_body.after(release_body) )
        .add_systems(Update, update_cursor_position)
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, spawn_ground)
        .add_systems(Startup, spawn_cube_pile)
        //.add_systems(Startup, spawn_triangle)
        .run();
}

fn spawn_cube_pile(mut commands: Commands) {
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
            .insert(ExternalForce{force: Vec2::ZERO, torque: 0.0})
            .insert(GravityScale(1.0))
            .insert(ColliderMassProperties::MassProperties(MassProperties{
                local_center_of_mass: Vec2::ZERO,
                principal_inertia: 3000.0,
                mass: 10.0,
            }))
            .insert(TransformBundle::from(Transform::from_xyz(column as f32 * CUBESIZE + STARTING_OFFSET_X, row as f32 * CUBESIZE + STARTING_OFFSET_Y, 0.0)));
        i += 1;
    }
}

fn spawn_ground(mut commands: Commands) {
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));
}

fn spawn_triangle(mut commands: Commands) {
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::triangle(Vec2::new(-100.0, 0.0), Vec2::new(100.0, 0.0), Vec2::new(0.0, 100.0)))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
}

/* Project a point inside of a system. */
fn project_point(rapier_context: Res<RapierContext>, point: Vec2, query_lens: Query<(Entity, &Transform)>) -> Option<Entity>{
    let filter: QueryFilter = QueryFilter::only_dynamic();

    let mut contacted_entities: Vec<(Entity, f32)> = Vec::new();

    rapier_context.intersections_with_point(point, filter, |entity| {
        let transform = query_lens.get(entity).unwrap().1;
        let distance = point.distance_squared(transform.translation.xy());
        contacted_entities.push((entity, distance));
        true
    });

    if contacted_entities.len() == 0 {
        return None;
    }

    let entity = contacted_entities.iter().min_by(|x, y: &&(Entity, f32)| x.1.partial_cmp(&y.1).unwrap()).unwrap().0;

    return Some(entity);
}

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct InspectorParams {
    #[inspector(min = 0.0, max = 100.0)]
    threshold: f32,
    #[inspector(min = 0.0, max = 10000000.0)]
    force: f32,
}
