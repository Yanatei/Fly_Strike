
use std::clone;

use bevy::log::tracing_subscriber::fmt::writer;
use bevy::state::commands;
use bevy::ui::update;
use bevy::{ecs::system::command, log::Level, prelude::*};
use crate::score_plug::GameLevelSpanType;
use crate::{boids_plug::Boid, config::*, score_plug::DurationSpanType};
use crate::event::*;
use bevy_hanabi::prelude::*;

pub struct GameStatePlug;

const ACCEL_VALUE: f32 = -54.0; //重力加速度
const DRAG_VALUE: f32 = 4.; //线性阻力系数

//准备开始游戏计时
#[derive(Resource)]
struct BeforeInGameTimer(Timer);

//游戏中，不需要额外计时器

//过场动画计时 before
#[derive(Resource)]
struct BeforeCutsceneTimer(Timer);
//过场动画计时
#[derive(Resource)]
struct InCutsceneTimer(Timer);
//过场动画计时 after
#[derive(Resource)]
struct AfterCutsceneTimer(Timer);
//加载下一关，不需要额外计时器

//游戏结束动画计时器
#[derive(Resource)]
struct OverCutsceneTimer(Timer);

impl Plugin for GameStatePlug {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.add_plugins(HanabiPlugin);//粒子系统插件
        //初始各状态计时器
        app.insert_resource(BeforeInGameTimer(Timer::new(BEFORE_IN_GAME_DURATION, TimerMode::Once)));
        app.insert_resource(BeforeCutsceneTimer(Timer::new(BEFORE_CUTSCENE_DURATION, TimerMode::Once)));
        app.insert_resource(InCutsceneTimer(Timer::new(IN_CUSTSCENE_DURATION, TimerMode::Once)));
        app.insert_resource(AfterCutsceneTimer(Timer::new(AFTER_CUTSCENE_DURATION, TimerMode::Once)));
        app.insert_resource(OverCutsceneTimer(Timer::new(OVER_CUTSCENE_DURATION, TimerMode::Once)));

        app.add_systems(Startup, setup_system);

        app.add_systems(OnEnter(GameState::BeforeInGame), on_before_in_game); //休息1.6s后开始游戏
        app.add_systems(Update, before_in_game_system.run_if(in_state(GameState::BeforeInGame)));//准备开始游戏

        app.add_systems(OnEnter(GameState::InGame), on_in_game); //游戏开始
        app.add_systems(Update, in_game_system
            .run_if(in_state(GameState::InGame)));//更新游戏状态到 before_cutscene

        app.add_systems(OnEnter(GameState::BeforeCutscene), on_before_cutscene); //休息1.5s后开始过场动画
        app.add_systems(Update, before_cutscene_system.run_if(in_state(GameState::BeforeCutscene)));

        app.add_systems(OnEnter(GameState::InCutscene), on_in_cutscene); //播放过场动画
        app.add_systems(Update, in_cutscene_system.run_if(in_state(GameState::InCutscene)));

        app.add_systems(OnEnter(GameState::AfterCutscene), on_after_cutscene);//休息1.5s后加载下一关
        app.add_systems(Update, after_cutscene_system.run_if(in_state(GameState::AfterCutscene)));

        app.add_systems(OnEnter(GameState::LoadingNext), on_loading_next); //加载下一关

        app.add_systems(OnEnter(GameState::GameOver), on_game_over); //游戏结束
        app.add_systems(Update, game_over_system.run_if(in_state(GameState::GameOver)));
    }
}

fn setup_system(
    mut commands: Commands,
    effects: ResMut<Assets<EffectAsset>>
) {
    //初始化粒子系统资源
    create_effects(commands, effects);
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
    mut next_state: ResMut<NextState<GameState>>,
    mut before_in_game_timer: ResMut<BeforeInGameTimer>,
) {
    //准备开始游戏计时1.5s，到时间后切换到InGame状态
    before_in_game_timer.0.tick(time.delta());

    if before_in_game_timer.0.is_finished() {
        next_state.set(GameState::InGame);
        //生成第一关的boids
        commands.trigger(NextLevelBoidsEvent);
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
    mut config: ResMut<GameConfig>,
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<&Boid>,
) {
    //累计时间
    let game_level = config.game_level;
    if game_level < config.MAX_GAME_LEVEL{
        config.elapsed_time[game_level] += time.delta_secs();
    }

    if query.is_empty() {
        next_state.set(GameState::BeforeCutscene);
    }
    
}

fn on_before_cutscene(
    mut before_cutscene_timer: ResMut<BeforeCutsceneTimer>,
) {
    //初始化计时器
   //休息1.5s后开始过场动画
    before_cutscene_timer.0.reset();
}

fn before_cutscene_system(
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
    mut before_cutscene_timer: ResMut<BeforeCutsceneTimer>,
    game_config: Res<GameConfig>,
) {
    //准备过场动画计时，到时间后切换到InCutscene状态
    before_cutscene_timer.0.tick(time.delta());

    if before_cutscene_timer.0.is_finished() {
        if game_config.game_level + 1 >= game_config.MAX_GAME_LEVEL {
            next_state.set(GameState::GameOver);
        }else{
            next_state.set(GameState::InCutscene);
        }
    }
}

fn on_in_cutscene(
    mut commands: Commands,
    mut in_cutscene_timer: ResMut<InCutsceneTimer>,
    mut q_spawner: Query<&mut EffectSpawner>,
    fireworks_sound: Res<FireworksSound>
) {
    //初始化计时器
    in_cutscene_timer.0.reset();
    //播放过场动画
    let mut index = 0;
    for (mut spawner) in q_spawner.iter_mut() {
        spawner.reset();
        spawner.active = true;

        if index < 1 {
            commands.spawn((AudioPlayer(fireworks_sound.0.clone()), PlaybackSettings::DESPAWN));
        }
        index += 1;
    }
}

fn in_cutscene_system(
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
    mut in_cutscene_timer: ResMut<InCutsceneTimer>,
) {
    //过场动画计时，到时间后切换到AfterCutscene状态
    in_cutscene_timer.0.tick(time.delta());

    if in_cutscene_timer.0.is_finished() {
        next_state.set(GameState::AfterCutscene);
    }
}

fn on_after_cutscene(
    mut after_cutscene_timer: ResMut<AfterCutsceneTimer>,
    mut q_spawner: Query<&mut EffectSpawner>,
) {
    //初始化计时器
    after_cutscene_timer.0.reset();
    //结束过场动画
    for (mut spawner) in q_spawner.iter_mut() {
        spawner.active = false;
    }
}

fn after_cutscene_system(
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
    mut after_cutscene_timer: ResMut<AfterCutsceneTimer>,
    game_config: Res<GameConfig>
) {
    //过场动画计时，到时间后切换到LoadingNext状态
    after_cutscene_timer.0.tick(time.delta());

    if after_cutscene_timer.0.is_finished() {
        if game_config.game_level + 1 >= game_config.MAX_GAME_LEVEL {
            next_state.set(GameState::GameOver);
        }else{
            next_state.set(GameState::LoadingNext);
        }
    }
}

fn on_game_over(
    duration_span: Single<&mut TextSpan, With<DurationSpanType>>,
    mut over_cutscene_timer: ResMut<OverCutsceneTimer>,
    mut game_config: ResMut<GameConfig>
) {
    let elapsed = game_config.elapsed_time[game_config.game_level];
    let duration_span = duration_span.into_inner();
    **(duration_span.into_inner()) = format!("{:.2}", elapsed);

    //打印总时间
    let sum_elapsed: f32 = game_config.elapsed_time.iter().sum();
    print!("Game Over! Elapsed time: {:.2} seconds\n", format!("{:.2}", sum_elapsed));

    //初始化计时器
    over_cutscene_timer.0.reset();
    //重置烟花计数器
    game_config.fireworks_count = 0;
}

fn game_over_system(
    time: Res<Time>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut over_cutscene_timer: ResMut<OverCutsceneTimer>,
    mut q_spawner: Query<&mut EffectSpawner>,
    fireworks_sound: Res<FireworksSound>,
    mut game_config: ResMut<GameConfig>
) {
    //目前为空
    over_cutscene_timer.0.tick(time.delta());

    if over_cutscene_timer.0.is_finished() && game_config.fireworks_count < 6 {
        let mut index = 0;
        for (mut spawner) in q_spawner.iter_mut() {
            spawner.reset();
            spawner.active = true;

            if index < 1 && game_config.fireworks_count % 2 == 0 {
                commands.spawn((AudioPlayer(fireworks_sound.0.clone()), PlaybackSettings::DESPAWN));
            }
            index += 1;
        }
        game_config.fireworks_count += 1;
        over_cutscene_timer.0.reset();
    }
}

//加载下一关：
//重新生成boids, 并且给boids增加速度
//切换到InGame状态
fn on_loading_next(
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    mut game_config: ResMut<GameConfig>,
    game_level_span: Single<&mut TextSpan, With<GameLevelSpanType>>,
) {
    //更新关卡数
    game_config.game_level += 1;
    //生成boids
    commands.trigger(NextLevelBoidsEvent);
    next_state.set(GameState::InGame);

    //更新关卡显示
    let game_level_span = game_level_span.into_inner();
    **(game_level_span.into_inner()) = (game_config.game_level + 1).to_string();
}


fn create_effects(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    let first_effect = effects.add(create_first_effect());
    let first_entity = commands.spawn((
        Name::new("FirstEffect"),
        ParticleEffect::new(first_effect),
    )).id();

    let second_effect = effects.add(create_second_effect());
    commands.spawn((
        Name::new("SecondEffect"),
        ParticleEffect::new(second_effect),
        EffectParent::new(first_entity)
    ));

    let third_effect = effects.add(create_third_effect());
    commands.spawn((
        Name::new("ThirdEffect"),
        ParticleEffect::new(third_effect),
        EffectParent::new(first_entity)
    ));
}

fn create_first_effect() -> EffectAsset {
    let writer = ExprWriter::new();
    let init_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::new(0., -300., 0.)).expr(),
        axis: writer.lit(Vec3::Y).expr(),
        radius: writer.lit(2.0).expr(),
        dimension: ShapeDimension::Volume,
    };

    let zero = writer.lit(0.0);
    let y = writer.lit(2000.).uniform(writer.lit(2100.));
    let v= zero.clone().vec3(y, zero);

    let init_val = SetAttributeModifier::new(Attribute::VELOCITY, v.expr());
    let age = writer.lit(0.0).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let rgb = writer.rand(VectorType::VEC3F) * writer.lit(0.9) + writer.lit(0.1);
    let color = rgb.vec4_xyz_w(writer.lit(1.0)).pack4x8snorm();
    let init_trails_color = SetAttributeModifier::new(Attribute::U32_0, color.expr());

    let lifetime = writer.lit(0.9).uniform(writer.lit(1.)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let accel = writer.lit(Vec3::Y * ACCEL_VALUE).expr();
    let update_accel = AccelModifier::new(accel);

    let drag = writer.lit(DRAG_VALUE).expr();
    let update_drag = LinearDragModifier::new(drag);

    let update_spawn_trail = EmitSpawnEventModifier{
        condition: EventEmitCondition::Always,
        count: writer.lit(5u32).expr(),
        child_index: 0,
    };

    let update_spawn_on_die = EmitSpawnEventModifier{
        condition: EventEmitCondition::OnDie,
        count: writer.lit(2000u32).expr(),
        child_index: 1,
    };

    let spawner = SpawnerSettings::once((6.0).into()).with_starts_active(false).with_spawn_duration((0.1).into());

    let mut color_gradient = bevy_hanabi::Gradient::new();
    color_gradient.add_key(0.0, Vec4::new(4.0, 4.0, 4.0, 1.0)); // 一直亮
    color_gradient.add_key(0.5, Vec4::new(4.0, 4.0, 4.0, 1.0)); // 到 50% 还亮
    color_gradient.add_key(1.0, Vec4::new(4.0, 4.0, 4.0, 0.0)); // 最后 50% 透明


    return EffectAsset::new(32, spawner, writer.finish())
        .with_name("FirstEffect")
        .init(init_pos)
        .init(init_val)
        .init(init_age)
        .init(init_trails_color)
        .init(init_lifetime)
        .update(update_accel)
        .update(update_drag)
        .update(update_spawn_trail)
        .update(update_spawn_on_die)
        .render(ColorOverLifetimeModifier{
            //gradient: bevy_hanabi::Gradient::constant(Vec4::ONE),
            gradient: color_gradient,
            blend: ColorBlendMode::Modulate,
            mask: ColorBlendMask::RGBA,
        })
        .render(SizeOverLifetimeModifier {
            gradient: bevy_hanabi::Gradient::constant(Vec3::ONE * 1.0),
            screen_space_size: false,
        });

}

//创建第二个粒子效果：烟火拖尾
fn create_second_effect() -> EffectAsset {
    let writer = ExprWriter::new();

    let init_pos = InheritAttributeModifier::new(Attribute::POSITION);

    //速度
    let vel = writer.rand(VectorType::VEC3F);
    let vel = vel * writer.lit(2.) - writer.lit(1.0);
    let vel = vel.normalized();
    let speed = writer.lit(1.);
    let vel = (vel * speed).expr();
    let init_vel = SetAttributeModifier::new(Attribute::VELOCITY, vel);

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(0.2).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let accel = writer.lit(Vec3::Y * ACCEL_VALUE).expr();
    let update_accel = AccelModifier::new(accel);

    let drag = writer.lit(DRAG_VALUE).expr();
    let update_drag = LinearDragModifier::new(drag);

    let spawner = SpawnerSettings::default().with_starts_active(false);

    let mut color_gradient = bevy_hanabi::Gradient::new();
    color_gradient.add_key(0.0, Vec4::new(4.0, 4.0, 4.0, 1.0));
    color_gradient.add_key(0.5, Vec4::new(4.0, 4.0, 4.0, 1.0));
    color_gradient.add_key(1.0, Vec4::new(4.0, 4.0, 4.0, 0.0));

    EffectAsset::new(1000, spawner, writer.finish())
        .with_name("SecondEffect")
        .init(init_pos)
        .init(init_vel)
        .init(init_age)
        .init(init_lifetime)
        .update(update_accel)
        .update(update_drag)
        .render(ColorOverLifetimeModifier{
            gradient: color_gradient,
            blend: ColorBlendMode::Modulate,
            mask: ColorBlendMask::RGBA,
        })
        .render(SizeOverLifetimeModifier {
            gradient: bevy_hanabi::Gradient::constant(Vec3::ONE * 0.1),
            screen_space_size: false,
        })
    
}

//创建第三个粒子效果：爆炸效果
fn create_third_effect() -> EffectAsset {
    let writer = ExprWriter::new();
    let init_pos = InheritAttributeModifier::new(Attribute::POSITION);

    let init_color = SetAttributeModifier::new(
        Attribute::COLOR,
        writer.parent_attr(Attribute::U32_0).expr(),
    );

    let center = writer.attr(Attribute::POSITION);
    let speed = writer.lit(320.).uniform(writer.lit(620.));
    let dir = writer.rand(VectorType::VEC3F)
        .mul(writer.lit(2.0))
        .sub(writer.lit(1.0))
        .normalized();
    let init_val = SetAttributeModifier::new(
        Attribute::VELOCITY,
        (center + dir * speed).expr(),
    );
    
    let age = writer.lit(0.0).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(1.8).uniform(writer.lit(3.2)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let accel = writer.lit(Vec3::Y * ACCEL_VALUE).expr();
    let update_accel = AccelModifier::new(accel);

    let drag = writer.lit(DRAG_VALUE).expr();
    let update_drag = LinearDragModifier::new(drag);

    let orient = OrientModifier::new(OrientMode::AlongVelocity);

    let spawner = SpawnerSettings::default().with_starts_active(false);

    let mut color_gradient = bevy_hanabi::Gradient::new();
    color_gradient.add_key(0.0, Vec4::new(4.0, 4.0, 4.0, 1.0));
    color_gradient.add_key(0.6, Vec4::new(4.0, 4.0, 4.0, 1.0));
    color_gradient.add_key(1.0, Vec4::new(4.0, 4.0, 4.0, 0.0));

    EffectAsset::new(10000, spawner, writer.finish())
        .with_name("ThirdEffect")
        .init(init_pos)
        .init(init_color)
        .init(init_val)
        .init(init_age)
        .init(init_lifetime)
        .update(update_accel)
        .update(update_drag)
        .render(ColorOverLifetimeModifier{
            gradient: color_gradient,
            blend: ColorBlendMode::Modulate,
            mask: ColorBlendMask::RGBA,
        })
        .render(SizeOverLifetimeModifier {
            gradient: bevy_hanabi::Gradient::constant(Vec3::new(1.0, 1.0, 0.05)),
            screen_space_size: false,
        })
        .render(orient)
}