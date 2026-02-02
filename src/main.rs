use bevy::prelude::*;
use crate::bevy_boids::*;
use crate::cannon::*;

mod bevy_boids;
mod config;
mod cannon;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CannonPlugin)
        .add_plugins(BoidsPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}