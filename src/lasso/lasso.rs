use bevy::prelude::*;

use super::{cursor::{mb_events, update_cursor_position, CursorWorldCoords, MBHeldEvent, MBReleasedEvent}, hold::{drag_held_body, hold_body, release_body}};

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