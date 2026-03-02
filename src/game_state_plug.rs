
use std::clone;

use bevy::log;
use bevy::log::tracing_subscriber::fmt::writer;
use bevy::state::commands;
use bevy::ui::update;
use bevy::{ecs::system::command, log::Level, prelude::*};
use bevy_hanabi::prelude::*;
use crate::custscene_public::CutSceneTimers;
use crate::score_plug::GameLevelSpanType;
use crate::{boids_plug::Boid, config::*, score_plug::DurationSpanType};
use crate::event::*;
use crate::cutscene_plug::*;
use crate::cutscene_mobile_plug::*;

pub struct GameStatePlug;

//准备开始游戏计时
#[derive(Resource)]
struct BeforeInGameTimer(Timer);
//游戏结束动画计时器
// #[derive(Resource)]
// struct OverCutsceneTimer(Timer);

//展示得分板时器
#[derive(Resource)]
struct LeaderBoardTimer(Timer);

impl Plugin for GameStatePlug {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        if cfg!(any(target_os="windows", target_os="linux", target_os="macos")){
            app.add_plugins(HanabiPlugin);//粒子系统插件
            app.add_plugins(CutScenePlugin);//过场动画插件
        }else if cfg!(any(target_os="android", target_os="ios")){
            app.add_plugins(CutSceneMobilePlugin);//Sprite Sheet动画
        }
        //app.add_plugins(CutSceneMobilePlugin);//Sprite Sheet动画
        //游戏关卡数据
        app.insert_resource(BeforeInGameTimer(Timer::new(BEFORE_IN_GAME_DURATION, TimerMode::Once)));
        app.insert_resource(GameStateDef::default());

        //初始化
        app.add_systems(Startup, setup_system);
        //游戏开始前，休息1.5s
        app.add_systems(OnEnter(GameState::BeforeInGame), on_before_in_game);
        app.add_systems(Update, before_in_game_system
            .run_if(in_state(GameState::BeforeInGame).and(in_state(MenuState::None)))
        );
        //开始游戏时，给个声音提醒
        app.add_systems(OnEnter(GameState::InGame), on_in_game);
        //进入过场动画，初始化过场动画状态
        app.add_systems(OnEnter(GameState::InCutscene), on_in_cutscene);
        //进入展示排名榜状态
        app.add_systems(OnEnter(GameState::Leaderboard), on_leaderboard);
        //游戏结束
        app.add_systems(OnEnter(GameState::GameOver), on_game_over);
        //游戏退出
        app.add_systems(OnEnter(GameState::Exit), on_game_exit);

        //游戏中状态，实时逻辑处理
        //展示游戏时长，检测关卡结束
        app.add_systems(Update, in_game_system
            .run_if(in_state(GameState::InGame).and(in_state(MenuState::None)))
        );
        //GameOver状态，多个烟火逻辑
        // app.add_systems(Update, game_over_system
        //     .run_if(in_state(GameState::GameOver))
        // );
        //Leaderboard状态，展示9秒，然后进入下一状态
        app.add_systems(Update, in_leaderboard_system
            .run_if(in_state(GameState::Leaderboard))
        );
        //注册游戏状态切换观察者
        app.add_observer(on_auto_next_game_state_event);
    }
}

fn setup_system(
    mut commands: Commands,
) {
    //commands.insert_resource(OverCutsceneTimer(Timer::new(OVER_CUTSCENE_DURATION, TimerMode::Once)));
    commands.insert_resource(LeaderBoardTimer(Timer::new(LEADERBOARD_DURATION, TimerMode::Once)));
}

fn on_before_in_game(
    mut before_in_game_timer: ResMut<BeforeInGameTimer>,
) {
    //初始化计时器
    before_in_game_timer.0.reset();
}

fn before_in_game_system(
    mut commands: Commands,
    time: Res<Time>,
    mut before_in_game_timer: ResMut<BeforeInGameTimer>,
) {
    //准备开始游戏计时1.5s，到时间后切换到InGame状态
    before_in_game_timer.0.tick(time.delta());

    if before_in_game_timer.0.is_finished() {
        //更新游戏关卡显示
        commands.trigger(GameLevelEvent);
        //生成boids
        commands.trigger(NextLevelBoidsEvent);
        //切换到下一游戏状态
        commands.trigger(AutoNextGameStateEvent);
    }
}

fn on_in_game(
    mut commands: Commands,
    game_started_sound: Res<GameStartedSound>,
) {
    //播放开始音效
    commands.spawn((AudioPlayer(game_started_sound.0.clone()), PlaybackSettings::DESPAWN));
}

fn in_game_system(
    time: Res<Time>,
    mut commands: Commands,
    mut config: ResMut<GameConfig>,
    query: Query<&Boid>,
) {
    //累计时间
    let game_level = config.game_level;
    if game_level < config.max_game_level{
        config.elapsed_time[game_level] += time.delta_secs();
    }

    if query.is_empty() {
        //飞行物被消灭完了,进入下一个游戏状态
        commands.trigger(AutoNextGameStateEvent);
    }
}

//OnEnter排行榜
//展示得分和总用时
fn on_leaderboard(
    mut commands: Commands,
    score: ResMut<Score>,
    mut game_config: ResMut<GameConfig>,
    global_font: Res<GlobalFont>,
    mut leaderboard_timer: ResMut<LeaderBoardTimer>
) {
    let totol_time = game_config.elapsed_time.iter().sum::<f32>();
    let str = format!("得分:{:.2}\n用时:{:.2}s", score.score, totol_time);
    commands.spawn((
        Text2d::new(str),
        TextFont {
            font: global_font.0.clone(),
            font_size: 60.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    leaderboard_timer.0.reset();
}

fn in_leaderboard_system(
    mut commands: Commands,
    mut leaderboard_timer: ResMut<LeaderBoardTimer>,
    time: Res<Time>,
){
    leaderboard_timer.0.tick(time.delta());
    if leaderboard_timer.0.is_finished() {
        commands.trigger(AutoNextGameStateEvent);
    }
}

//驱动游戏状态按顺序变化
fn on_auto_next_game_state_event(
    trigger: On<AutoNextGameStateEvent>,
    mut cur_state: ResMut<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_config: ResMut<GameConfig>,
    game_state_def: Res<GameStateDef>,
){
    let cur_level = game_config.game_level;
    let cur_level_index = game_config.game_level_index;
    let mut next_level = cur_level;
    let mut next_level_index = cur_level_index;
    let status_length = game_state_def.game_states[cur_level].len();

    if cur_level_index + 1 <  status_length {//切换到下一个状态
        next_level_index = cur_level_index + 1;
    }
    else if cur_level_index == status_length - 1 {//切换到下一关
        next_level_index = 0;
        //最后一关
        if next_level + 1 < game_config.max_game_level {
            next_level = cur_level + 1;
        }
    }else{
        log::error!("advance_state_to_next error");
    }
    
    game_config.game_level = next_level;
    game_config.game_level_index = next_level_index;
    next_state.set(game_state_def.game_states[next_level][next_level_index].clone());
    print!("game_level={},game_level_index={}, cur_state={:?}, next_state={:?}\n", next_level, next_level_index, cur_state, next_state);
}

fn on_in_cutscene(
    mut cutscene_state: ResMut<NextState<CutsceneStepState>>,
    mut cutscene_timers: ResMut<CutSceneTimers>,
) {
    //设置关卡动画的首个状态
    //更新CutSceneTimers当前状态
    cutscene_timers.cur_state = CutsceneStepState::BeforeCutscene;
    cutscene_state.set(CutsceneStepState::BeforeCutscene);
}

fn on_game_over(
    duration_span: Single<&mut TextSpan, With<DurationSpanType>>,
    // mut over_cutscene_timer: ResMut<OverCutsceneTimer>,
    mut game_config: ResMut<GameConfig>
) {
    let elapsed = game_config.elapsed_time[game_config.game_level];
    let duration_span = duration_span.into_inner();
    **(duration_span.into_inner()) = format!("{:.2}", elapsed);

    //打印总时间
    let sum_elapsed: f32 = game_config.elapsed_time.iter().sum();
    print!("Game Over! Elapsed time: {:.2} seconds\n", format!("{:.2}", sum_elapsed));

    //over_cutscene_timer.0.reset();
}

fn game_over_system(
    time: Res<Time>,
    mut commands: Commands,
    //mut over_cutscene_timer: ResMut<OverCutsceneTimer>,
) {
    // //目前为空
    // over_cutscene_timer.0.tick(time.delta());
    // //动画效果结束，切换到下一状态
    // if over_cutscene_timer.0.is_finished(){
    //     commands.trigger(AutoNextGameStateEvent);
    // }
}

fn on_game_exit(
    mut message_writer: MessageWriter<AppExit>,
){
    message_writer.write(AppExit::Success);
}

//加载下一关：
//重新生成boids, 并且给boids增加速度
//切换到InGame状态
// fn on_loading_next(
//     mut next_state: ResMut<NextState<GameState>>,
//     mut commands: Commands,
//     mut game_config: ResMut<GameConfig>,
//     game_level_span: Single<&mut TextSpan, With<GameLevelSpanType>>,
// ) {
//     //更新关卡数
//     game_config.game_level += 1;
//     //生成boids
//     commands.trigger(NextLevelBoidsEvent);
//     next_state.set(GameState::InGame);

//     //更新关卡显示
//     let game_level_span = game_level_span.into_inner();
//     **(game_level_span.into_inner()) = (game_config.game_level + 1).to_string();
// }


