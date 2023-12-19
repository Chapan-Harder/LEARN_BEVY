mod asteroids;
mod camera;
mod debug;
mod spaceship;
mod movement;
mod asset_loader;
mod collition;
mod despawn;
mod schedule;

use bevy::prelude::*;

use collition::CollisionPlugin;
use despawn::DespawnPlugin;
use asteroids:: AsteroidPlugin;
use asset_loader::AssetLoaderPlugin;
use camera::CameraPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;
use schedule::SchedulePlugin;

fn main(){
    App::new()
    // Bevy built_ins
    .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.2)))
    .insert_resource(AmbientLight{
        color: Color::default(),
        brightness: 0.85,
    })

    .add_plugins(DefaultPlugins)
    // User configured plugings.
    .add_plugins(AssetLoaderPlugin)
    .add_plugins(CollisionPlugin)
    .add_plugins(SpaceshipPlugin)
    .add_plugins(CameraPlugin)
    .add_plugins(MovementPlugin)
    .add_plugins(AsteroidPlugin)
    .add_plugins(DespawnPlugin)
    .add_plugins(DebugPlugin)
    .add_plugins(SchedulePlugin)
    .run();
}