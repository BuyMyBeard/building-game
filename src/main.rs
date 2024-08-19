
use bevy::{input::mouse::MouseButtonInput, prelude::*, window::PrimaryWindow};

use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .init_resource::<CursorWorldCoords>()
        .add_event::<MBHeldEvent>()
        .add_event::<MBReleasedEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, mb_events)
        .add_systems(Update, (hold_body.after(mb_events), release_body.after(mb_events)))
        .add_systems(Update, move_held_body.after(release_body) )
        .add_systems(Update, update_cursor_position)
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, spawn_ground)
        .add_systems(Startup, spawn_triangle)
        .run();
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

#[derive(Event)]
struct MBHeldEvent();

#[derive(Event)]
struct MBReleasedEvent();

fn move_held_body(
    mut query: Query<(&mut Transform, &HeldBody)>,
    mouse_pos: Res<CursorWorldCoords>,
) {
    for (mut transform, held_body) in &mut query {
        transform.translation = (mouse_pos.0 + held_body.offset).xyx(); 
    }
}

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

fn hold_body(
    mut mb_held_evr: EventReader<MBHeldEvent>,
    rapier_context: Res<RapierContext>,
    mut query: Query<(Entity, &Transform, &mut RigidBody)>,
    mouse_pos: Res<CursorWorldCoords>,
    mut commands: Commands,
) -> () {
    if mb_held_evr.read().len() == 0 {
        return;
    }

    let mut query_lens = query.transmute_lens::<(Entity, &Transform)>();

    match project_point(rapier_context, mouse_pos.0, query_lens.query()) {
        Some(entity) => {
            let (_, transform, mut rb) = query.get_mut(entity).unwrap();
            let offset = transform.translation.xy() - mouse_pos.0;
            commands.entity(entity).insert(HeldBody{
                offset,
            });
            *rb = RigidBody::KinematicPositionBased;
        },
        None => {},
    }
}

fn release_body(
    mut mb_released_evr: EventReader<MBReleasedEvent>,
    mut query: Query<(Entity, &mut RigidBody), With<HeldBody>>,
    mut commands: Commands,
) {
    if mb_released_evr.read().len() == 0 {
        return;
    }
    for (entity, mut rb) in &mut query {
        commands.entity(entity).remove::<HeldBody>();
        *rb = RigidBody::Dynamic;
    }
}

#[derive(Resource, Default)]
struct CursorWorldCoords(Vec2);

#[derive(Component)]
struct HeldBody {
    offset: Vec2,
}


#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);
}

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