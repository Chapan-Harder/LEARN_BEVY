use bevy::prelude::*;

use crate::collition::Collider;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use crate::asset_loader::SceneAssets;
use crate::schedule::InGameSet;

const STARTING_TRANSITION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 3.5;
const SPACESHIP_ROLL_SPEED: f32 = 3.5;
const SPACESHIP_RADIUS: f32 = 5.0;
const MISSILES_SPEED: f32 = 70.0;
const MISSILES_FORWARD_SPAWN_SCALAR: f32 = 7.5;
const MISSILES_RADIUS: f32 = 1.0;

#[derive(Component, Debug)]
pub struct Spaceship;

#[derive(Component, Debug)]
pub struct SpaceshipShield;

#[derive(Component, Debug)]
pub struct SapaceshipMission;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship).add_systems(Update, /*(spaceship_movement_controls, spaceship_weapon_control),*/ (spaceship_movement_controls, spaceship_weapon_control, spaceship_shield_control).chain().in_set(InGameSet::UserInput));
    }
}

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>){
    commands.spawn((MovingObjectBundle{
        velocity: Velocity::new(Vec3::ZERO),
        acceleration: Acceleration::new(Vec3::ZERO),
        collider: Collider::new(SPACESHIP_RADIUS),
        model: SceneBundle{scene: scene_assets.spaceship.clone(),
        transform: Transform::from_translation(STARTING_TRANSITION),
        ..default()},
    },
    Spaceship
    ));
}

fn spaceship_movement_controls(mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>, keyboard_input: Res<Input<KeyCode>>, time: Res<Time>){
    let (mut transform, mut velocity) = query.single_mut();
    let Ok((mut transform, mut velocity)) = query.get_single_mut() else{return;};

    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    if keyboard_input.pressed(KeyCode::D){
        rotation = -SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    }
    else if keyboard_input.pressed(KeyCode::A){
        rotation = SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::S){
        movement = -SPACESHIP_SPEED;
    }
    else if keyboard_input.pressed(KeyCode::W){
        movement = SPACESHIP_SPEED;
    }

    if keyboard_input.pressed(KeyCode::Left){
        roll = -SPACESHIP_ROLL_SPEED * time.delta_seconds();
    }
    else if keyboard_input.pressed(KeyCode::Right){
        roll = SPACESHIP_ROLL_SPEED * time.delta_seconds();
    }

    // Rotate around the Y-axis.
    transform.rotate_y(rotation);
    // Rotate around the local Z-axis.
    transform.rotate_local_z(roll);

    // Update the spaceshipe's velocity based on new direction.
    velocity.value = -transform.forward() * movement;
}

fn spaceship_weapon_control(mut commands: Commands, query: Query<&Transform, With<Spaceship>>, keyboard_input: Res<Input<KeyCode>>, scene_assets: Res<SceneAssets>, time: Res<Time>){
    let transform = query.single();
    let Ok(transform) = query.get_single() else{return;};

    if keyboard_input.pressed(KeyCode::Space){
        commands.spawn((MovingObjectBundle{
            velocity: Velocity::new(-transform.forward() * MISSILES_SPEED),
            acceleration: Acceleration::new(Vec3::ZERO),
            collider: Collider::new(MISSILES_RADIUS),
            model: SceneBundle{
                scene: scene_assets.missiles.clone(),
                transform: Transform::from_translation(transform.translation + -transform.forward() * MISSILES_FORWARD_SPAWN_SCALAR * time.delta_seconds()),
                ..default()
            }
        }, SapaceshipMission));
    }
}

fn spaceship_shield_control(mut commands: Commands,query: Query<Entity, With<Spaceship>>, keyboard_input: Res<Input<KeyCode>>){
    let Ok(spaceship) = query.get_single() else{return;};

    if keyboard_input.pressed(KeyCode::Tab){
        commands.entity(spaceship).insert(SpaceshipShield);
    }
}