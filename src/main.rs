
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use debug::debug::debug_plugin;
use lasso::lasso::lasso_plugin;
use level::level::level_plugin;

mod lasso;
mod level;
mod debug;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(lasso_plugin)
        .add_plugins(level_plugin)
        .add_plugins(debug_plugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())        //.add_systems(Startup, spawn_triangle)
        .run();
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