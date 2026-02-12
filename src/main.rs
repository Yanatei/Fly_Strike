use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::boids_plug::*;
use crate::cannon_plug::*;
use crate::bullet_plug::*;
use crate::config::*;
use crate::score_plug::*;
use crate::fps_plug::*;
use crate::game_state_plug::*;
use crate::menu_plug::*;

mod boids_plug;
mod config;
mod cannon_plug;
mod event;
mod bullet_plug;
mod score_plug;
mod fps_plug;
mod game_state_plug;
mod menu_plug;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameStatePlug)
        .add_plugins(MenuPlug)
        .add_plugins(CannonPlugin)
        .add_plugins(BoidsPlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(ScorelPlugin)
        .add_plugins(FpsPlugin)
        .add_systems(PreStartup, pre_startup)
        .add_systems(Startup, setup)
        .run();
}

fn pre_startup(
    window: Single<&Window, With<PrimaryWindow>>,
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2d);
    load_font(&mut commands, &asset_server);

    //获取窗口宽高
    let width = window.width();
    let height = window.height();
    commands.insert_resource(GameConfig::default().width_window_size(width, height));
    
    //音效
    let sound = asset_server.load("sounds/score.wav");
    commands.insert_resource(ScoreSound(sound.clone()));

    let sound = asset_server.load("sounds/fireworks.wav");
    commands.insert_resource(FireworksSound(sound.clone()));

    let sound = asset_server.load("sounds/game_started.wav");
    commands.insert_resource(GameStartedSound(sound.clone()));

}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
}


fn load_font(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let handle = asset_server.load("fonts/default_zh.ttf");
    commands.insert_resource(GlobalFont(handle));
}
