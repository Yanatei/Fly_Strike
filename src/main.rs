use bevy::log;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::window::WindowResized;
use crate::boids_plug::*;
use crate::cannon_plug::*;
use crate::bullet_plug::*;
use crate::config::*;
use crate::event::BoidsReLimitEvent;
use crate::event::CannonReLocationEvent;
use crate::score_plug::*;
use crate::fps_plug::*;
use crate::game_state_plug::*;
use crate::menu_plug::*;
use crate::player_plug::*;

mod boids_plug;
mod config;
mod cannon_plug;
mod event;
mod bullet_plug;
mod score_plug;
mod fps_plug;
mod game_state_plug;
mod menu_plug;
mod cutscene_plug;
mod cutscene_mobile_plug;
mod custscene_public;
mod player_plug;


#[bevy_main]
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
        .add_plugins(PlayerPlug)
        .add_systems(PreStartup, pre_startup)
        .add_systems(Startup, setup)
        .add_systems(Update, on_resize_window)
        .run();
}

fn pre_startup(
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    //设置窗口大小
    let window_config = get_platform_window_config();
    window_config.apply_to(&mut window);
    window_config.log();

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
    mut game_state: ResMut<NextState<GameState>>
) {
    game_state.set(GameState::BeforeInGame);
}


fn load_font(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let handle = asset_server.load("fonts/default_zh.ttf");
    commands.insert_resource(GlobalFont(handle));
}

fn on_resize_window(
    mut commands: Commands,
    mut game_config: ResMut<GameConfig>,
    mut resize_reader: MessageReader<WindowResized>,
){
    for e in resize_reader.read() {
        // When resolution is being changed
        game_config.window_width = e.width;
        game_config.window_height = e.height;

        log::info!("window resized, width={}, height={}\n", game_config.window_width, game_config.window_height);
        //炮台重新定位
        commands.trigger(CannonReLocationEvent);
        //飞鸟限制范围更新
        commands.trigger(BoidsReLimitEvent);
    }
}