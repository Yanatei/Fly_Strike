
use bevy::state::commands;
use bevy::{ecs::system::command, log::Level, prelude::*};
use crate::score_plug::GameLevelSpanType;
use crate::{boids_plug::Boid, config::*, game_state_plug, score_plug::DurationSpanType};
use crate::event::*;

pub struct GameStatePlug;

impl Plugin for GameStatePlug {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.add_systems(Update, update_state_system.run_if(in_state(GameState::InGame)));
        app.add_systems(OnEnter(GameState::GameOver), on_game_over);
        app.add_systems(OnEnter(GameState::LevelComplete), on_level_complete); 
        app.add_systems(Update, in_level_complete_system.run_if(in_state(GameState::LevelComplete)));//过场动画
        app.add_systems(OnEnter(GameState::LoadingNext), on_loading_next);
    }
}

//过场动画计时
#[derive(Resource)]
struct LevelCompleteTimer(Timer);

// fn over_level_complete(
//     time: Res<Time>,
//     mut next_state: ResMut<NextState<GameState>>,
//     mut timer: ResMut<LevelCompleteTimer>,
//     mut commands: Commands,
// ) {
//     timer.0.tick(time.delta());
//     if timer.0.is_finished() {
//         commands.remove_resource::<LevelCompleteTimer>();
//         next_state.set(GameState::LoadingNext);
//     }
// }

fn on_level_complete(
    mut commands: Commands
) {
    commands.insert_resource(LevelCompleteTimer(Timer::new(LEVEL_COMPLETE_DURATION, TimerMode::Once)));
}

fn in_level_complete_system(
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
    mut level_timer: ResMut<LevelCompleteTimer>,
    mut commands: Commands,
) {
    level_timer.0.tick(time.delta());
    if level_timer.0.is_finished() {
        commands.remove_resource::<LevelCompleteTimer>();
        next_state.set(GameState::LoadingNext);
    }

    //播放动画
    
}

fn on_game_over(
    config: Res<GameConfig>,
    duration_span: Single<&mut TextSpan, With<DurationSpanType>>,
) {
    let elapsed = config.elapsed_time[config.game_level];
    let duration_span = duration_span.into_inner();
    **(duration_span.into_inner()) = format!("{:.2}", elapsed);

    let sum_elapsed: f32 = config.elapsed_time.iter().sum();
    print!("Game Over! Elapsed time: {:.2} seconds\n", format!("{:.2}", sum_elapsed));
}

fn update_state_system(
    time: Res<Time>,
    mut config: ResMut<GameConfig>,
    cur_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<&Boid>,
) {
    //累计时间
    let game_level = config.game_level;
    if game_level < config.MAX_GAME_LEVEL{
        config.elapsed_time[game_level] += time.delta_secs();
    }

    //判断游戏是否结束
    //print!("query.is_empty()={}, game_state={:?}\n", query.is_empty(), *cur_state);
    if *cur_state == GameState::InGame {
       if query.is_empty() {
           if config.game_level + 1 >= config.MAX_GAME_LEVEL {
               //最后一关结束，游戏结束
               next_state.set(GameState::GameOver);
           } else {
               //进入下一关
               config.game_level += 1;
               next_state.set(GameState::LevelComplete);
           }
       }
    }
}

//加载下一关：
//重新生成boids, 并且给boids增加速度
//切换到InGame状态
fn on_loading_next(
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    game_config: Res<GameConfig>,
    game_level_span: Single<&mut TextSpan, With<GameLevelSpanType>>,
) {
    //生成boids
    commands.trigger(NextLevelBoidsEvent);
    next_state.set(GameState::InGame);

    //更新关卡显示
    let game_level_span = game_level_span.into_inner();
    **(game_level_span.into_inner()) = (game_config.game_level + 1).to_string();
}
