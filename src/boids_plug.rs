//飞行物体
use bevy::{prelude::*};
use rand::Rng;
use crate::{config::*};
use crate::event::*;

// Boid 控制参数
#[derive(Resource)]
pub struct BoidConfig {
    speed: f32,         // 速度
    vision: f32,       // 视野范围
    cohesion: f32,     // 聚合力
    alignment: f32,    // 对齐力
    separation: f32,   // 排斥力
    boundary_force: f32, // 边界折返力
    limit_x: f32, // 飞行范围半径
    limit_y: f32, // 飞行范围半径
}

impl Default for BoidConfig {
    fn default() -> Self {
        Self {
            // 基础移动
            speed: 200.0,            // 巡航速度：不能太快，否则肉眼看不清群聚行为
            
            // 视野
            vision: 90.0,           // 探测半径：决定了鸟能看到多远的邻居

            // 三定律权重（比例建议：排斥 > 对齐 > 聚合）
            separation: 80.0,       // 排斥力：最强，防止鸟儿撞在一起
            alignment: 6.0,        // 对齐力：中等，让大家朝一个方向飞
            cohesion: 4.0,         // 聚合力：稍弱，让鸟群有凝聚力但不会挤死

            // 边界约束
            limit_x: 300.0,  // 飞行半径：在这个范围内自由飞行
            limit_y: 200.0,
            boundary_force: 80.0,  // 转向力：快撞墙时，产生一个强力拉回中心
        }
    }
}

#[derive(Component)]
pub struct Boid {
    pub velocity: Vec3,
    pub wander_angle: f32, //想去别的地方"冲动", 连续、可积累
}

impl Boid {
    pub fn new(images_handle: Handle<Image>) -> (Boid, Sprite, Transform) {
        let mut rng = rand::rng();
        let min= -1.0 * 150.0;
        let max = 1.0 * 150.0;
        let location_new = rng.random_range(-50.0 .. 50.0);
        (
            Boid {
                velocity: Vec3::new(rng.random_range(min..max), rng.random_range(min..max), 0.0),
                wander_angle: rng.random_range(0.0 .. std::f32::consts::TAU),
            },
            Sprite {
                image: images_handle,
                color: Color::srgba(1.0, 1.0, 1.0, 0.5),
                ..default()
            },
            Transform {
                translation: Vec3::new(location_new, location_new, 0.0),
                scale: Vec3::new(FLY_SIZE, FLY_SIZE, 0.0) ,
                ..default()
            },
        )
    }
}

pub struct BoidsPlugin;

impl Plugin for BoidsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BoidConfig::default());
        app.add_systems(Startup, setup);
        app.add_systems(Update, boid_system
            .run_if(in_state(GameState::InGame)).run_if(in_state(MenuState::None))
        );
        app.add_observer(on_NextLevelBoidsEvent);
    }
}


fn setup(
         mut config: ResMut<BoidConfig>,
         game_config: Res<GameConfig>,
         asset_server: Res<AssetServer>,
         mut commands: Commands,
) {

    config.limit_x = game_config.window_width / 2.0;
    config.limit_y = game_config.window_height / 2.0;

    //加载图片
    let image_handel = asset_server.load("images/paopao.png");
    commands.insert_resource(BoidsImage(image_handel.clone()));

    // 生成苍蝇/鸟群, 改为由状态触发生成
    // for _ in 0 .. FLY_COUNT {
    //     commands.spawn(Boid::new(image_handel.clone()));
    // }
}

//生成下一关的boids
fn on_NextLevelBoidsEvent(
    trigger: On<NextLevelBoidsEvent>,
    mut commands: Commands,
    boids_image_res: Res<BoidsImage>,
    mut config: ResMut<BoidConfig>,
) {
    //增加速度,生成boids
    config.speed += BOID_SPEED_INCREMENT;
    for _ in 0 .. FLY_COUNT {
        commands.spawn(Boid::new(boids_image_res.0.clone()));
    }
}

pub fn boid_system(
    time: Res<Time>,
    config: Res<BoidConfig>,
    mut query: Query<(Entity, &mut Transform, &mut Boid)>,
) {
    // === 1️⃣ 快照：避免可变/不可变冲突 ===
    let boid_data: Vec<(Entity, Vec3, Vec3)> = query
        .iter()
        .map(|(e, t, b)| (e, t.translation, b.velocity))
        .collect();

    let mut rng = rand::rng();
    let dt = time.delta_secs();

    // === 2️⃣ 主循环 ===
    for (entity, mut transform, mut boid) in query.iter_mut() {
        let mut center = Vec3::ZERO;
        let mut avg_velocity = Vec3::ZERO;
        let mut separation = Vec3::ZERO;
        let mut neighbors = 0;

        let pos = transform.translation;

        // === 邻居分析（Boids 三定律） ===
        for (other_entity, other_pos, other_vel) in boid_data.iter() {
            if entity == *other_entity {
                continue;
            }

            let diff = *other_pos - pos;
            let dist = diff.truncate().length();

            if dist > 0.01 && dist < config.vision {
                center += *other_pos;
                avg_velocity += *other_vel;

                // 距离越近，排斥越强
                separation += (pos - *other_pos)
                    .normalize_or_zero()
                    / dist.max(1.0);

                neighbors += 1;
            }
        }

        let mut force = Vec3::ZERO;

        if neighbors > 0 {
            let center = center / neighbors as f32;
            let avg_velocity = avg_velocity / neighbors as f32;

            // Cohesion：朝群中心
            force += (center - pos).normalize_or_zero() * config.cohesion;

            // Alignment：朝平均速度微调
            force += (avg_velocity - boid.velocity) * config.alignment;

            // Separation：防止拥挤
            force += separation.normalize_or_zero() * config.separation;
        }

        // === 3️⃣ Wander：持续“想飞别的方向” ===
        let wander_strength = 60.0;
        let wander_change = 1.2;

        boid.wander_angle += rng.random_range(-wander_change..wander_change) * dt;

        let wander_dir = Vec3::new(
            boid.wander_angle.cos(),
            boid.wander_angle.sin(),
            0.0,
        );

        force += wander_dir * wander_strength;

        // === 4️⃣ 软边界（提前 steering，不保证合法） ===
        let margin = 100.0;
        let t1 = 0.1;
        if pos.x > config.limit_x - margin {
            force.x -= (pos.x - (config.limit_x - margin)) * config.boundary_force * t1;
        } else if pos.x < -config.limit_x + margin {
            force.x += (-config.limit_x + margin - pos.x) * config.boundary_force * t1;
        }

        if pos.y > config.limit_y - margin {
            force.y -= (pos.y - (config.limit_y - margin)) * config.boundary_force * t1;
        } else if pos.y < -config.limit_y + margin {
            force.y += (-config.limit_y + margin - pos.y) * config.boundary_force * t1;
        }

        // === 5️⃣ 积分速度 ===
        boid.velocity += force * dt;
        boid.velocity.z = 0.0;

        // 限制最大速度
        boid.velocity = boid.velocity.clamp_length_max(config.speed);

        // 最低巡航速度（防止系统冷却）
        let min_speed = config.speed * 0.4;
        if boid.velocity.length() < min_speed {
            boid.velocity = boid.velocity.normalize_or_zero() * min_speed;
        }

        // === 6️⃣ 更新位置 ===
        transform.translation += boid.velocity * dt;

        // === 7️⃣ 硬边界（世界规则，兜底） ===
        transform.translation.x =
            transform.translation.x.clamp(-config.limit_x, config.limit_x);
        transform.translation.y =
            transform.translation.y.clamp(-config.limit_y, config.limit_y);

        // === 8️⃣ 朝向飞行方向 ===
        if boid.velocity.length_squared() > 0.001 {
            let angle = boid.velocity.y.atan2(boid.velocity.x);
            transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}
