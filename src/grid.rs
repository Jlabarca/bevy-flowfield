use crate::{movement::Velocity};
use bevy::{prelude::*};
use noise::{NoiseFn, Perlin, Turbulence};


pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_grid)
        // .add_system(move_system)
        // .add_system(velocity_system)
        .register_type::<Tile>()
        .register_type::<Velocity>()
        ;
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tile {
    pub x: f32,
    pub y: f32,
    pub visible: bool,
}


#[derive(Bundle)]
struct TileBundle {
    tile: Tile,
    velocity: Velocity,
    #[bundle]
    pbr: PbrBundle,
}

const DEFAULT_COLOR: Color = Color::rgb(0.3, 0.5, 0.3);

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
            let position = Vec3::new(-(tilemap_size.x as f32 * x as f32) / 2.0, 0.0, -(tilemap_size.y as f32 * y as f32) / 2.0);

            commands.spawn_bundle(TileBundle {
                tile: Tile {
                    x: x as f32,
                    y: y as f32,
                    visible: true
                },
                velocity: Velocity {
                    translation: Vec3::ZERO,
                    rotation: 0.0,
                },
                pbr: PbrBundle {
                    mesh: quad_handle.clone(),
                    material: materials.add(DEFAULT_COLOR.into()),
                    transform: Transform {
                        translation: position,
                        rotation: Quat::from_rotation_x(-1.5708),
                        ..default()
                    },
                    ..default()
                }
            });
        }
    }
}

// move up and down the tiles - just for fun
fn velocity_system(mut tiles: Query<(&mut Velocity, &Transform)>) {
    let perlin = Perlin::default();
    let turbulence = Turbulence::<Perlin>::new(perlin);
    tiles.par_for_each_mut(32, |(mut velocity, transform) | {

       
        // if rng.gen_range(-1.0..1.0) > 0.77
        // {
        //     return
        // }

        // if velocity.translation.y >= 0.0 && transform.translation.y < 1.0 {
        //     velocity.translation = Vec3::new(0.0, 4.0, 0.0);
        // } else if transform.translation.y > -1.0 {
        //     velocity.translation = Vec3::new(0.0, -4.0, 0.0);
        // } else {
        //     velocity.translation = Vec3::ZERO;
        // }

        // if rng.gen_range(-1.0..1.0) > 0.9 {
        //     velocity.translation = Vec3::new(0.0, 4.0, 0.0);
        // } else if rng.gen_range(-1.0..1.0) < -0.9 || transform.translation.y > -7.0{
        //     velocity.translation = Vec3::new(0.0, -4.0, 0.0);
        // }


        // let mut rng = rand::thread_rng();
        // velocity.translation.y = rng.gen_range(-11.0..11.0);

        velocity.translation.y = turbulence.get([transform.translation.x as f64, transform.translation.z as f64]) as f32

    });
}

fn move_system(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    let delta = time.delta_seconds();

    for (velocity, mut transform) in &mut query {
        transform.translation += delta * velocity.translation;
        transform.rotate_z(velocity.rotation * delta);
    }
}