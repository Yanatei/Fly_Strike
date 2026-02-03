use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::bevy_boids::*;
use crate::cannon::*;
use crate::bullet::*;
use crate::config::*;

mod bevy_boids;
mod config;
mod cannon;
mod event;
mod bullet;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CannonPlugin)
        .add_plugins(BoidsPlugin)
        .add_plugins(BulletPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    commands.spawn(Camera2d);

    //获取窗口宽高
    let width = window.width();
    let height = window.height();
    commands.insert_resource(
        GameConfig{
            window_width: width,
            window_height: height,
        }
    );
}