use bevy::prelude::*;
use crate::config::*;
use crate::event::*;
use crate::custscene_public::*;

const ACCEL_VALUE: f32 = -54.0; //重力加速度
const DRAG_VALUE: f32 = 4.; //线性阻力系数


pub struct CutSceneMobilePlugin;

impl Plugin for CutSceneMobilePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<CutsceneStepState>();
        //初始化CutSceneTimerConfig
        app.insert_resource(CutSceneTimers::default());
        app.insert_resource(CutSceneStateDef::default());
        app.add_systems(Startup, setup_system);

        app.add_systems(OnEnter(CutsceneStepState::BeforeCutscene), on_before_cutscene);
        app.add_systems(OnEnter(CutsceneStepState::InCutscene), on_in_cutscene);
        app.add_systems(OnEnter(CutsceneStepState::AfterCutscene), on_after_cutscene);

        //退出最后一个烟花状态时，触发修改游戏状态
        app.add_systems(OnExit(CutsceneStepState::AfterCutscene), on_exit_after_cutscene);

        //注册到游戏状态上，更新动画的状态
        app.add_systems(Update, cutscene_state_system
            .run_if(in_state(GameState::InCutscene))
        );
    }
}

fn setup_system(
    mut commands: Commands, 
    game_config: Res<GameConfig>,
) {

}

fn cutscene_state_system(
    mut cutscene_timers: ResMut<CutSceneTimers>,
    mut next_state: ResMut<NextState<CutsceneStepState>>,
    cutscene_def: ResMut<CutSceneStateDef>,
    time: Res<Time>,
){
    let index = cutscene_timers.cur_index;
    let times = &mut cutscene_timers.timers;
    let delta = time.delta();

    times[index].tick(delta);

    let mut next_status = CutsceneStepState::None;
    let mut next_index = index;
    if times[index].just_finished() {
        if index + 1 >= cutscene_def.state.len() {
            next_status = CutsceneStepState::None;
            next_index = 0;
        }else{
            next_status = cutscene_def.state[index+1];
            next_index = index + 1;
        }
        next_state.set(next_status);
        cutscene_timers.cur_index = next_index;
    }
}

fn on_exit_after_cutscene(
    mut commands: Commands,
){
    commands.trigger(AutoNextGameStateEvent);
}

//初始化当前阶段计时器
fn reset_cutscene_timer(cutscene_timers: &mut CutSceneTimers){
    let cur_index = cutscene_timers.cur_index;
    cutscene_timers.timers[cur_index].reset();
}

fn on_before_cutscene(
    mut cutscene_timers: ResMut<CutSceneTimers>,
) {
    //初始化计时器
    reset_cutscene_timer(&mut cutscene_timers);
}
fn on_in_cutscene(
    mut commands: Commands,
    mut cutscene_timers: ResMut<CutSceneTimers>,
    fireworks_sound: Res<FireworksSound>,
) {
    //初始化计时器
    reset_cutscene_timer(&mut cutscene_timers);
    //激活动画
    
}

fn on_after_cutscene(
    mut cutscene_timers: ResMut<CutSceneTimers>,
){
    //初始化计时器
    reset_cutscene_timer(&mut cutscene_timers);
}