use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::boids_plug::*;
use crate::cannon_plug::*;
use crate::bullet_plug::*;
use crate::config::*;
use crate::score_plug::*;
use crate::fps_plug::*;
use crate::game_state::*;

mod boids_plug;
mod config;
mod cannon_plug;
mod event;
mod bullet_plug;
mod score_plug;
mod fps_plug;
mod game_state;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins(CannonPlugin)
        .add_plugins(BoidsPlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(ScorelPlugin)
        .add_plugins(FpsPlugin)
        .add_systems(PreStartup, pre_startup)
        .add_systems(Startup, setup)
        .add_systems(Update, update_system.run_if(in_state(GameState::InGame)))
        .add_systems(OnEnter(GameState::GameOver), on_game_over)
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
    commands.insert_resource(
        GameConfig{
            window_width: width,
            window_height: height,
            elapsed_time: [0.0; 3],
            game_level: 0,
        }
    );
    
    //音效
    let sound = asset_server.load("sounds/score.wav");
    commands.insert_resource(ScoreSound(sound.clone()));

}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
}

fn update_system(
    time: Res<Time>,
    mut config: ResMut<GameConfig>,
    cur_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<&Boid>,
) {
    //累计时间
    let game_level = config.game_level + 1;
    config.elapsed_time[game_level] += time.delta_secs();

    //判断游戏是否结束
    //print!("query.is_empty()={}, game_state={:?}\n", query.is_empty(), *cur_state);
    if query.is_empty() && *cur_state == GameState::InGame {
        next_state.set(GameState::GameOver);
    }
}

fn load_font(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let handle = asset_server.load("fonts/default_zh.ttf");
    commands.insert_resource(GlobalFont(handle));
}


fn on_game_over(
    config: Res<GameConfig>,
    duration_span: Single<&mut TextSpan, With<DurationSpanType>>,
) {
    let elapsed = config.elapsed_time[config.game_level + 1];
    let duration_span = duration_span.into_inner();
    **(duration_span.into_inner()) = format!("{:.2}", elapsed);
}