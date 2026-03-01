use bevy::log;
use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use crate::config::*;
use crate::event::*;
use crate::custscene_public::*;

const ACCEL_VALUE: f32 = -54.0; //重力加速度
const DRAG_VALUE: f32 = 4.; //线性阻力系数

pub struct CutScenePlugin;

impl Plugin for CutScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<CutsceneStepState>();
        //初始化CutSceneTimerConfig
        app.insert_resource(CutSceneTimers::default());
        app.insert_resource(CutSceneStateDef::default());//关卡过关动画状态定义
        app.insert_resource(CutSceneGameOverStateDef::default());//游戏结束时状态定义
        app.add_systems(Startup, setup_system);

        app.add_systems(OnEnter(CutsceneStepState::BeforeCutscene), on_before_cutscene);
        app.add_systems(OnEnter(CutsceneStepState::InCutscene), on_in_cutscene);
        app.add_systems(OnEnter(CutsceneStepState::AfterCutscene), on_after_cutscene);
        //注册到游戏状态上，更新动画的状态
        app.add_systems(Update, cutscene_state_system
            .run_if(in_state(GameState::InCutscene))
        );

        //退出最后一个烟花状态时，触发修改游戏状态
        app.add_systems(OnExit(CutsceneStepState::AfterCutscene), on_exit_after_cutscene);

        //GameOver时，烟花状态控制
        app.add_systems(OnEnter(GameState::GameOver), on_game_over_state);
        app.add_systems(Update, game_over_state_system
            .run_if(in_state(GameState::GameOver))
        );
        //注册到游戏结束状态上，更新结束动画的状态
        app.add_systems(Update, cutscene_gameover_state_system
            .run_if(in_state(CutsceneStepState::InGameOverCutscene))
        );
    }
}

fn on_game_over_state(
    mut cutscene_timer: ResMut<CutSceneTimers>,
    mut game_config: ResMut<GameConfig>,
){
    //初始化计时器
    if let Some(t_timer) = cutscene_timer.get_cur_timer() {
        t_timer.reset();
    }
    //重置烟花计数器
    game_config.fireworks_count = 0;
    //设置当前首个状态
    // cutscene_timer.cur_state = CutsceneStepState::InGameOverCutscene;
}

fn setup_system(
    mut commands: Commands, 
    effects: ResMut<Assets<EffectAsset>>,
    game_config: Res<GameConfig>,
) {
    //初始化粒子系统资源
    create_effects(commands, effects, game_config);
}

fn cutscene_gameover_state_system(
    time: Res<Time>,
    mut commands: Commands,
    mut cutscene_timer: ResMut<CutSceneTimers>,
    mut q_spawner: Query<&mut EffectSpawner>,
    fireworks_sound: Res<FireworksSound>,
    mut game_config: ResMut<GameConfig>
) {
    //目前为空
    //初始化计时器
    let Some(t_timer) = cutscene_timer.get_cur_timer() else {
        log::info!("game_over_state_system cutscene_timer is empty!!");
        return;
    };
    t_timer.tick(time.delta());

    if t_timer.is_finished() && game_config.fireworks_count < 6 {
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
        t_timer.reset();
    }
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
        log::info!("cutscene_state_system cutscene_timers is empty!!, cur_state={:?}", cutscene_timers.cur_state);
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

fn on_exit_after_cutscene(
    mut commands: Commands,
){
    commands.trigger(AutoNextGameStateEvent);
}

//初始化当前阶段计时器
fn reset_cutscene_timer(cutscene_timers: &mut CutSceneTimers){
    let Some(t_timer) = cutscene_timers.get_cur_timer() else {
        log::info!("reset_cutscene_timer cutscene_timers is empty!!, cur_state={:?}", cutscene_timers.cur_state);
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
    mut q_spawner: Query<&mut EffectSpawner>,
    fireworks_sound: Res<FireworksSound>,
) {
    //初始化计时器
    reset_cutscene_timer(&mut cutscene_timers);
    //激活动画
    let mut index = 0;
    for mut spawner in q_spawner.iter_mut() {
        spawner.reset();
        spawner.active = true;

        if index < 1 {
            commands.spawn((AudioPlayer(fireworks_sound.0.clone()), PlaybackSettings::DESPAWN));
        }
        index += 1;
    }
}

fn on_after_cutscene(
    mut cutscene_timers: ResMut<CutSceneTimers>,
){
    //初始化计时器
    reset_cutscene_timer(&mut cutscene_timers);
}

fn create_effects(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    game_config: Res<GameConfig>,
) {
    let window_height = game_config.window_height;
    let first_effect = effects.add(create_first_effect(window_height));
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

fn create_first_effect(window_height: f32) -> EffectAsset {
    let pos_y = 0. - window_height/3.;
    let writer = ExprWriter::new();
    let init_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::new(0., pos_y, 0.)).expr(),
        axis: writer.lit(Vec3::Y).expr(),
        radius: writer.lit(2.0).expr(),
        dimension: ShapeDimension::Volume,
    };

    let zero = writer.lit(0.0);
    let y = writer.lit(1600.).uniform(writer.lit(1700.));
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

    EffectAsset::new(1000, spawner, writer.finish())
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