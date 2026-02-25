//子弹
use bevy::math::bounding::BoundingCircle;
use bevy::{prelude::*, time};
use bevy::math::bounding::IntersectsVolume;
use crate::event::*;
use crate::config::*;
use crate::boids_plug::Boid;

#[derive(Component)]
pub struct Bullet;

#[derive(Resource)]
pub struct BulletImage(pub Handle<Image>);

pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BulletTimer(Timer::new(BULLET_TIME_DURATION, TimerMode::Once)));
        app.add_systems(Startup, setup);
        app.add_systems(Update, (bullet_move_system, collision_system).chain()
            .run_if(not(in_state(GameState::Paused).or(in_state(GameState::Leaderboard)).or(not(in_state(MenuState::None)))))
        );
        app.add_observer(on_fired);//添加观察者，开火时触发
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image_handle = asset_server.load("images/bullet.png");
    commands.insert_resource(BulletImage(image_handle.clone()));
}

fn bullet_move_system(
    time: Res<Time>, 
    mut query: Query<(Entity, &mut Transform, &mut Bullet)>,
    mut commands: Commands,
    game_config: Res<GameConfig>,
) {
    for (entity, mut transform, _) in query.iter_mut() {
        
        transform.translation.y += BULLET_SPEED * time.delta_secs();

        if transform.translation.y > game_config.window_height {
            commands.entity(entity).despawn();
        }
    }
}

fn on_fired(
    trigger: On<FireEvent>,
    mut commands: Commands,
    image: Res<BulletImage>,
) {
    //生成子弹
    let trigger_pos = trigger.0;
    commands.spawn((
        Bullet,
        Sprite {
            image: image.0.clone(),
            ..default()
        },
        Transform {
            scale: Vec3::new(1.0, 1.0, 1.0),
            translation: Vec3::new(trigger_pos.x, trigger_pos.y + CANNON_HEIGHT as f32/2.0, 0.0),
            ..default()
        },
    ));
}

fn collision_system(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &Transform, &mut Bullet)>,
    mut boids_query: Query<(Entity, &Transform, &mut Boid)>,
    boids_image: Res<BoidsImage>,
    images: Res<Assets<Image>>,
) {
    let mut boid_radius: f32 = 0.0;
    //获取飞鸟的尺寸
    if let Some(image) = images.get(&boids_image.0) {
        boid_radius = image.size().x as f32 * FLY_SIZE / 2.0;
    }
    
    for (_, b_transform, _) in bullet_query.iter_mut() {
        for (entity, transform, _) in boids_query.iter_mut() {
            let boid_bound = BoundingCircle::new(transform.translation.truncate(), boid_radius);
            let bullet_bound = BoundingCircle::new(b_transform.translation.truncate(), BULLET_SIZE/2.0);
            if boid_bound.intersects(&bullet_bound) {
                commands.entity(entity).despawn();
                //击中得分
                commands.trigger(ScoreEvent);
            }
        }
    }
    
}