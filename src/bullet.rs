//子弹

use bevy::ecs::system::command;
use bevy::image;
use bevy::{prelude::*, time};
use crate::event::*;
use crate::config::*;

#[derive(Component)]
pub struct Bullet;

#[derive(Resource)]
pub struct BulletImage(pub Handle<Image>);

#[derive(Resource)]
pub struct BulletTimer(pub Timer);

pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BulletTimer(Timer::new(BULLET_TIME_DURATION, time::TimerMode::Repeating)));
        app.add_systems(Startup, setup);
        app.add_systems(Update, bullet_move_system);
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
            translation: Vec3::new(trigger_pos.x, trigger_pos.y+CANNON_HEIGHT, 0.0),
            ..default()
        },
    ));
}