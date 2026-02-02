//子弹

use bevy::{prelude::*, time};
use crate::event::*;

#[derive(Component)]
pub struct Bullet;

#[derive(Resource)]
pub struct BulletImage(pub Handle<Image>);

#[derive(Resource)]
pub struct BulletTimer(pub Timer);

pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BulletTimer(Timer::new(std::time::Duration::from_millis(100), time::TimerMode::Repeating)));
        app.add_systems(Startup, setup);
        app.add_systems(Update, bullet_move_system);
        app.add_observer(on_fired);//添加观察者，开火时触发
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image_handle = asset_server.load("images/bullet.png");
    commands.insert_resource(BulletImage(image_handle.clone()));
}

fn bullet_move_system(time: Res<Time>, mut query: Query<(Entity, &mut Transform, &mut Bullet)>) {
    
}

fn on_fired(
    _collided: On<FireEvent>,
    mut commands: Commands
) {
    println!("子弹发射");
}