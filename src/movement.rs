use bevy::{prelude::*};

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Velocity {
    pub translation: Vec3,
    pub rotation: f32,
}