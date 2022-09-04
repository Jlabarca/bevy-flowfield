use bevy::{prelude::*, window::PresentMode};
use bevy_mod_raycast::{
    DefaultPluginState, DefaultRaycastingPlugin, RayCastMesh, RayCastMethod, RayCastSource,
    RaycastSystem,
};

use crate::{grid::*};

pub struct RaycastPlugin;

impl Plugin for RaycastPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(WindowDescriptor {
            present_mode: PresentMode::AutoNoVsync, // Reduces input lag.
            ..Default::default()
        })
        // The DefaultRaycastingPlugin bundles all the functionality you might need into a single
        // plugin. This includes building rays, casting them, and placing a debug cursor at the
        // intersection. For more advanced uses, you can compose the systems in this plugin however
        // you need. For example, you might exclude the debug cursor system.
        .add_plugin(DefaultRaycastingPlugin::<RaycastSet>::default())
        // You will need to pay attention to what order you add systems! Putting them in the wrong
        // order can result in multiple frames of latency. Ray casting should probably happen near
        // start of the frame. For example, we want to be sure this system runs before we construct
        // any rays, hence the ".before(...)". You can use these provided RaycastSystem labels to
        // order your systems with the ones provided by the raycasting plugin.
        .add_system_to_stage(
            CoreStage::First,
            update_raycast_with_cursor.before(RaycastSystem::BuildRays::<RaycastSet>),
        )
        .add_system(add_rs_to_tiles)
        ;
    }
}

/// This is a unit struct we will use to mark our generic `RayCastMesh`s and `RayCastSource` as part
/// of the same group, or "RayCastSet". For more complex use cases, you might use this to associate
/// some meshes with one ray casting source, and other meshes with a different ray casting source."
struct RaycastSet;

// Update our `RayCastSource` with the current cursor position every frame.
fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RayCastSource<RaycastSet>>,
) {
    // Grab the most recent cursor event if it exists:
    let cursor_position = match cursor.iter().last() {
        Some(cursor_moved) => cursor_moved.position,
        None => return,
    };

    for mut pick_source in &mut query {
        pick_source.cast_method = RayCastMethod::Screenspace(cursor_position);
    }
}



/// Add RaycastSet to Tiles
pub fn add_rs_to_tiles(
    mut commands: Commands,
    mut query: Query<(Entity, Added<Tile>)>,
) {
    for (entity, _) in &mut query {
        commands.entity(entity).insert( RayCastMesh::<RaycastSet>::default());
    }
}