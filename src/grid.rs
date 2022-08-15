use bevy::{prelude::*};

#[derive(Component)]
struct Tile {
    pub position: Vec2,
    pub visible: bool,
}

pub struct Grid {
    pub size: Vec2,
    pub position: Vec2,
}

pub fn init_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let tilemap_size = Vec2::new(32.0, 32.0);
  
    // create a new quad mesh. this is what we will apply the texture to
    let quad_width = 16.0;

    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        quad_width,
        quad_width,
    ))));

    for x in 0..32u32 {
        for y in 0..32u32 {
            commands.spawn_bundle(PbrBundle {
                mesh: quad_handle.clone(),
                // material: material_handle.clone(),
                //mesh: meshes.add(Mesh::from(shape::Plane { size: 16.0 })),
                material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                transform: Transform {
                    translation: Vec3::new(-(tilemap_size.x as f32 * x as f32) / 2.0, 0.0, -(tilemap_size.y as f32 * y as f32) / 2.0),
                    rotation: Quat::from_rotation_x(-1.5708),
                    ..default()
                },
                ..default()
            });
        }
    }
}
