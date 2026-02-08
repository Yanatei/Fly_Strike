
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

//过场动画计时
#[derive(Resource)]
struct LevelCompleteTimer(Timer);

impl Plugin for GameStatePlug {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.add_plugins(HanabiPlugin);//粒子系统插件
        app.add_systems(Startup, setup_system);
        app.add_systems(Update, update_state_system.run_if(in_state(GameState::InGame)));
        app.add_systems(OnEnter(GameState::GameOver), on_game_over);
        app.add_systems(OnEnter(GameState::LevelComplete), on_level_complete); 
        app.add_systems(Update, in_level_complete_system.run_if(in_state(GameState::LevelComplete)));//过场动画
        app.add_systems(OnEnter(GameState::LoadingNext), on_loading_next);
    }
}

fn setup_system(
    commands: Commands,
    effects: ResMut<Assets<EffectAsset>>
) {
    //初始化粒子系统资源
    create_effects(commands, effects);
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
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Y).expr(),
        radius: writer.lit(30.0).expr(),
        dimension: ShapeDimension::Volume,
    };

    let zero = writer.lit(Vec3::ZERO);
    let y = writer.lit(140.).uniform(writer.lit(160.));
    let v= zero.clone().vec3(y, zero);

    let init_val = SetAttributeModifier::new(Attribute::VELOCITY, v.expr());
    let age = writer.lit(0.0).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let rgb = writer.rand(VectorType::VEC3F) * writer.lit(0.9) + writer.lit(0.1);
    let color = rgb.vec4_xyz_w(writer.lit(1.0)).pack4x8snorm();
    let init_trails_color = SetAttributeModifier::new(Attribute::U32_0, color.expr());

    let lifetime = writer.lit(0.8).uniform(writer.lit(1.2)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let accel = writer.lit(Vec3::Y * -16.).expr();
    let update_accel = AccelModifier::new(accel);

    let drag = writer.lit(4.0).expr();
    let update_drag = LinearDragModifier::new(drag);

    let update_spawn_trail = EmitSpawnEventModifier{
        condition: EventEmitCondition::Always,
        count: writer.lit(5u32).expr(),
        child_index: 0,
    };

    let update_spawn_on_die = EmitSpawnEventModifier{
        condition: EventEmitCondition::OnDie,
        count: writer.lit(1000u32).expr(),
        child_index: 1,
    };

    let spawner = SpawnerSettings::rate((1.0, 3.0).into());

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
            gradient: bevy_hanabi::Gradient::constant(Vec4::ONE),
            blend: ColorBlendMode::Overwrite,
            mask: ColorBlendMask::RGBA,
        })
        .render(SizeOverLifetimeModifier {
            gradient: bevy_hanabi::Gradient::constant(Vec3::ONE * 0.1),
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

    let accel = writer.lit(Vec3::Y * -16.).expr();
    let update_accel = AccelModifier::new(accel);

    let drag = writer.lit(4.).expr();
    let update_drag = LinearDragModifier::new(drag);

    let spawner = SpawnerSettings::default();

    let mut color_gradient = bevy_hanabi::Gradient::new();
    color_gradient.add_key(0.0, Vec4::new(4.0, 4.0, 4.0, 1.0));
    color_gradient.add_key(0.8, Vec4::new(4.0, 4.0, 4.0, 1.0));
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
            gradient: bevy_hanabi::Gradient::constant(Vec3::ONE * 0.02),
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
    let speed = writer.lit(40.).uniform(writer.lit(60.));
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

    let lifetime = writer.lit(0.8).uniform(writer.lit(1.2)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let accel = writer.lit(Vec3::Y * -16.).expr();
    let update_accel = AccelModifier::new(accel);

    let drag = writer.lit(4.0).expr();
    let update_drag = LinearDragModifier::new(drag);

    let orient = OrientModifier::new(OrientMode::AlongVelocity);

    let spawner = SpawnerSettings::default();

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
            gradient: bevy_hanabi::Gradient::constant(Vec3::new(0.2, 0.05, 0.05)),
            screen_space_size: false,
        })
        .render(orient)
}

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
    //播放动画



    //动画计时，到时间后加载下一关
    level_timer.0.tick(time.delta());
    if level_timer.0.is_finished() {
        commands.remove_resource::<LevelCompleteTimer>();
        next_state.set(GameState::LoadingNext);
    }
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
