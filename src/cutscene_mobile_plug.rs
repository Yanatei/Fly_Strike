use bevy::asset::transformer;
use bevy::camera::visibility;
use bevy::log;
use bevy::prelude::*;
use crate::config::*;
use crate::event::*;
use crate::custscene_public::*;

#[derive(Component)]
struct SpriteSheetFrieWorks;
#[derive(Component)]
struct SpriteSheetFrieWorksLast;

pub struct CutSceneMobilePlugin;

impl Plugin for CutSceneMobilePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<CutsceneStepState>();
        //初始化CutSceneTimerConfig
        app.insert_resource(CutSceneTimers::default());
        app.insert_resource(CutSceneStateDef::default());//过关动画状态定义
        app.insert_resource(CutSceneGameOverStateDef::default());//结束动画状态定义
        app.add_systems(Startup, setup_system);

        app.add_systems(OnEnter(CutsceneStepState::BeforeCutscene), on_before_cutscene);
        app.add_systems(OnEnter(CutsceneStepState::InCutscene), on_in_cutscene);
        app.add_systems(OnEnter(CutsceneStepState::AfterCutscene), on_after_cutscene);
        app.add_systems(OnEnter(CutsceneStepState::InGameOverCutscene), on_in_gameover_cutscene);
         //注册到游戏状态上，更新动画的状态
        app.add_systems(Update, (cutscene_state_system)
            .run_if(in_state(GameState::InCutscene))
        );

        //退出最后一个烟花状态时，触发修改游戏状态
        app.add_systems(OnExit(CutsceneStepState::AfterCutscene), on_exit_after_cutscene);
        
        //注册到动画状态上，更新动画索引
        app.add_systems(Update, (execute_fireworks_animations)
            .run_if(in_state(CutsceneStepState::InCutscene))
        );
        //GameOver时，烟花状态控制
        app.add_systems(OnEnter(GameState::GameOver), on_game_over_state);
        app.add_systems(Update, game_over_state_system
            .run_if(in_state(GameState::GameOver)).run_if(not(in_state(CutsceneStepState::None)))
        );
        //注册到游戏结束状态上，更新结束动画的状态
        app.add_systems(Update, cutscene_gameover_state_system
            .run_if(in_state(CutsceneStepState::InGameOverCutscene))
        );
    }
}

fn on_in_gameover_cutscene(
    mut cutscene_timers: ResMut<CutSceneTimers>,
    query_last_fireworks: Query<(&mut Sprite, &mut Visibility), With<SpriteSheetFrieWorksLast>>,
){
    let Some(cur_timer) = cutscene_timers.get_cur_timer() else {
        log::info!("on_in_gameover_cutscene cutscene_timer is empty!!");
        return;
    };
    //开始Sprite动画
    for (mut sprite, mut visibility) in query_last_fireworks{
        *visibility = Visibility::Visible;
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = 0;
        }
    }
    cur_timer.reset();
}

fn on_game_over_state(
    mut cutscene_state: ResMut<NextState<CutsceneStepState>>,
    mut cutscene_timer: ResMut<CutSceneTimers>,
    mut game_config: ResMut<GameConfig>,
){
    //设置当前首个状态
    cutscene_timer.cur_state = CutsceneStepState::BeforeCutscene;
    cutscene_state.set(CutsceneStepState::BeforeCutscene);

    //初始化计时器
    if let Some(t_timer) = cutscene_timer.get_cur_timer() {
        t_timer.reset();
    }
    //重置烟花计数器
    game_config.fireworks_count = 0;
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_config: Res<GameConfig>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    //加载SpriteSheet
    let texture: Handle<Image> = asset_server.load("images/fireworks_sheet.png");
    let texture_last:Handle<Image> = asset_server.load("images/fireworks_last_sheet.png");
    let layout = TextureAtlasLayout::from_grid(FIREWORKS_IMAGE_SIZE, 10, 11, None, None);
    let layout_last = TextureAtlasLayout::from_grid(FIREWORKS_IMAGE_SIZE, 10, 11, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let texture_atlas_last_layout = texture_atlas_layouts.add(layout_last);
    let animation_config = AnimationConfig::new(FIREWORKS_PARAMETER.0, FIREWORKS_PARAMETER.1, FIREWORKS_PARAMETER.2);
    let animation_last_config = AnimationConfig::new(FIREWORKS_LAST_PARAMETER.0, FIREWORKS_LAST_PARAMETER.1, FIREWORKS_LAST_PARAMETER.2);

    let pos_x = 0.;
    let pos_y = 0.;
    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_config.first_index,
            }),
            ..default()
        },
        Visibility::Hidden,
        Transform::from_scale(Vec3::splat(FIREWORKS_SIZE)).with_translation(Vec3::new(pos_x, pos_y, 0.0)),
        SpriteSheetFrieWorks,
        animation_config,
    ));
    commands.spawn((
        Sprite {
            image: texture_last.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_last_layout.clone(),
                index: animation_last_config.first_index,
            }),
            ..default()
        },
        Visibility::Hidden,
        Transform::from_scale(Vec3::splat(FIREWORKS_SIZE)).with_translation(Vec3::new(pos_x, pos_y, 0.0)),
        SpriteSheetFrieWorksLast,
        animation_last_config,
    ));
}

fn cutscene_state_system(
    mut cutscene_timers: ResMut<CutSceneTimers>,
    mut next_state: ResMut<NextState<CutsceneStepState>>,
    mut cutscene_def: ResMut<CutSceneStateDef>,
    time: Res<Time>,
){
    let t_times = cutscene_timers.get_cur_timer();
    let delta = time.delta();
    let Some(times) = t_times else {
        log::info!("cutscene_state_system cutscene_timers is empty!!");
        return;
    };
    times.tick(delta);

    let index = cutscene_def.cur_index;
    let mut next_status = CutsceneStepState::None;
    let mut next_index = index;
    if times.just_finished() {
        if index + 1 >= cutscene_def.state.len() {
            next_status = CutsceneStepState::None;
            next_index = 0;
        }else{
            next_index = index + 1;
            next_status = cutscene_def.state[next_index];
        }
        next_state.set(next_status);
        cutscene_def.cur_index = next_index;
        cutscene_timers.cur_state = next_status;
    }
}

fn on_exit_after_cutscene(
    mut commands: Commands,
){
    commands.trigger(AutoNextGameStateEvent);
}

//初始化当前阶段计时器
fn reset_cutscene_timer(cutscene_timers: &mut CutSceneTimers){
    let Some(t_timer) = cutscene_timers.get_cur_timer() else {
        log::info!("reset_cutscene_timer cutscene_timers is empty!!");
        return;
    };
    t_timer.reset();
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
    query_fireworks: Query<(&mut Sprite, &mut Visibility, &mut AnimationConfig), With<SpriteSheetFrieWorks>>
) {
    //初始化计时器
    reset_cutscene_timer(&mut cutscene_timers);
    //激活动画, 播放声音
    commands.spawn((AudioPlayer(fireworks_sound.0.clone()), PlaybackSettings::DESPAWN));
    for (mut sprite, mut visibility, mut animaton_config) in query_fireworks{
        *visibility = Visibility::Visible;
        animaton_config.frame_timer.reset();
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = 0;
        }
    }
}

fn on_after_cutscene(
    mut cutscene_timers: ResMut<CutSceneTimers>,
    query_fireworks: Query<&mut Visibility, Or<(With<SpriteSheetFrieWorks>, With<SpriteSheetFrieWorksLast>)>>,
){
    //初始化计时器
    reset_cutscene_timer(&mut cutscene_timers);
    //结束Sprite动画
    for mut visibility in query_fireworks{
        *visibility = Visibility::Hidden;
    }
}

fn execute_fireworks_animations(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut Sprite), With<SpriteSheetFrieWorks>>,
) {
    for (mut animation_config, mut sprite) in query.iter_mut() {
        animation_config.frame_timer.tick(time.delta());
        if animation_config.frame_timer.is_finished() && let Some(atlas) = &mut sprite.texture_atlas {
            if atlas.index + 1 >= animation_config.last_index {
               break;
            }
            atlas.index += 1;
            animation_config.frame_timer.reset();
        }
    }
}

fn game_over_state_system(
    mut cutscene_timers: ResMut<CutSceneTimers>,
    mut next_state: ResMut<NextState<CutsceneStepState>>,
    mut cutscene_gameover_def: ResMut<CutSceneGameOverStateDef>,
    time: Res<Time>,
){
    let t_times = cutscene_timers.get_cur_timer();
    let delta = time.delta();
    let Some(times) = t_times else {
        log::info!("cutscene_gameover_state_system cutscene_timers is empty!!");
        return;
    };
    times.tick(delta);

    let index = cutscene_gameover_def.cur_index;
    let mut next_status = CutsceneStepState::None;
    let mut next_index = index;
    if times.just_finished() {
        if index + 1 >= cutscene_gameover_def.state.len() {
            next_status = CutsceneStepState::None;
            next_index = 0;
        }else{
            next_index = index + 1;
            next_status = cutscene_gameover_def.state[next_index];
        }
        next_state.set(next_status);
        cutscene_gameover_def.cur_index = next_index;
        cutscene_timers.cur_state = next_status;
    }
}

fn cutscene_gameover_state_system(
    time: Res<Time>,
    mut commands: Commands,
    fireworks_sound: Res<FireworksSound>,
    mut game_config: ResMut<GameConfig>,
    mut query: Query<(&mut AnimationConfig, &mut Sprite), With<SpriteSheetFrieWorksLast>>,
) {
    for (mut animation_config, mut sprite) in query.iter_mut() {
        animation_config.frame_timer.tick(time.delta());
        if animation_config.frame_timer.is_finished() && let Some(atlas) = &mut sprite.texture_atlas {
            if atlas.index + 1 >= animation_config.last_index {
               break;
            }
            atlas.index += 1;
            animation_config.frame_timer.reset();

            //音效
            if game_config.fireworks_count < 60 {
                if game_config.fireworks_count % 20 == 0 {
                    commands.spawn((AudioPlayer(fireworks_sound.0.clone()), PlaybackSettings::DESPAWN));
                }
            }
            game_config.fireworks_count += 1;
        }
    }
    
}