use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_inspector_egui::WorldInspectorPlugin;
// use bevy_flycam::PlayerPlugin;

mod movement;
mod grid;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(grid::GridPlugin)
        //.add_plugin(PlayerPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
            .add_system(move_units)
            .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands
) {
    // camera
    commands.spawn_bundle(Camera3dBundle {
        projection: OrthographicProjection {
            scale: 500.0,
            scaling_mode: ScalingMode::FixedVertical(1.0),
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(-400.0, 400.0, -400.0).looking_at(Vec3::new(-200.0, 0.0,  -200.0), Vec3::Y),
        ..default()
    });

    // light
    commands.spawn_bundle(DirectionalLightBundle {
        transform: Transform::from_xyz(3.0, 8.0, 5.0).with_rotation(Quat::from_euler(EulerRot::XYZ, 0.5, -2.6, 1.0)),
        ..default()
    });
}

fn move_units() {

}