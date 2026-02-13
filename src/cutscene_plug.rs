use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use crate::config::*;
use crate::event::*;

const ACCEL_VALUE: f32 = -54.0; //重力加速度
const DRAG_VALUE: f32 = 4.; //线性阻力系数

#[derive(Resource)]
struct CutSceneTimers{
    pub timers: [Timer; 3],
    pub cur_index: usize,
}

impl Default for CutSceneTimers {
    fn default() -> Self {
        Self {
            timers: [
                Timer::new(BEFORE_CUTSCENE_DURATION, TimerMode::Once),
                Timer::new(IN_CUSTSCENE_DURATION, TimerMode::Once),
                Timer::new(AFTER_CUTSCENE_DURATION, TimerMode::Once),
            ], 
            cur_index: 0,
        }
    }
}

#[derive(Resource, Clone)]
struct CutSceneStateDef{
    pub state: [CutsceneStepState; 3],
}

impl Default for CutSceneStateDef {
    fn default() -> Self {
        Self { 
            state: [
                CutsceneStepState::BeforeCutscene,
                CutsceneStepState::InCutscene,
                CutsceneStepState::AfterCutscene,
            ],
        }
    }
}

pub struct CutScenePlugin;

impl Plugin for CutScenePlugin {
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
        app.add_systems(OnExit(CutsceneStepState::AfterCutscene), onExit_after_cutscene);

        app.add_systems(Update, cutscene_system
            .run_if(in_state(GameState::InCutscene))
        );
    }
}

fn setup_system(
    mut commands: Commands, 
    effects: ResMut<Assets<EffectAsset>>
) {
    //初始化粒子系统资源
    create_effects(commands, effects);
}

fn cutscene_system(
    mut commands: Commands,
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
    if times[index].is_finished() {
        if index + 1 >= cutscene_def.state.len() {
            next_status = CutsceneStepState::None;
        }else{
            next_status = cutscene_def.state[index+1].clone();
        }
        next_state.set(next_status);
    }
}

fn onExit_after_cutscene(
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
    mut commands: Commands,
    mut cutscene_timers: ResMut<CutSceneTimers>,
) {
    //初始化计时器
    reset_cutscene_timer(&mut cutscene_timers);
}
fn on_in_cutscene(
    time: Res<Time>,
    mut cutscene_timers: ResMut<CutSceneTimers>,
) {
    //初始化计时器
    reset_cutscene_timer(&mut cutscene_timers);
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