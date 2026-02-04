//炮台
use bevy::{prelude::*, time, window::PrimaryWindow};
use crate::config::*;
use crate::event::*;


#[derive(Component, Resource)]
pub struct Cannon;

// #[derive(Resource)]
// pub struct CannonConfig {
//     pub image_handel : Handle<Image>,
// }

pub struct CannonPlugin;

// #[derive(Resource)]
// pub struct FireTimer(Timer);

impl Plugin for CannonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, cannon_move_system);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    //let window_width = window.width();
    let window_height = window.height();
    let pos_x = 0.0;
    let pos_y = -(window_height/2.0) + CANNON_HEIGHT;

    let i_handel = asset_server.load("images/cannon.png");

    commands.spawn((
        Cannon,
        Sprite {
            image: i_handel.clone(),
            ..default()
        },
        Transform {
                translation: Vec3::new(pos_x, pos_y, 0.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
        },
    ));
}

fn cannon_move_system(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut bullet_timer: ResMut<BulletTimer>,
    mut query: Query<(Entity, &mut Transform, &mut Cannon)>,
    mut commands: Commands,
    game_config: Res<GameConfig>,
){
    bullet_timer.0.tick(time.delta());

    if input.just_pressed(KeyCode::Space)  {
        if bullet_timer.0.is_finished() {//子弹发射间隔
            let (_, transform, _) = query.single_mut().unwrap();
            commands.trigger(FireEvent(transform.translation));
            bullet_timer.0.reset();
        }
    }

    let (_, mut cannon_transform, _) = query.single_mut().unwrap();

    let mut keys = [KeyCode::ArrowLeft, KeyCode::KeyA];
    if keys.iter().any(|&key| input.pressed(key)) {
        cannon_transform.translation.x -= CANNON_SPEED * time.delta_secs();
    }

    keys = [KeyCode::ArrowRight, KeyCode::KeyD];
    if keys.iter().any(|&key| input.pressed(key)) {
        cannon_transform.translation.x += CANNON_SPEED * time.delta_secs();
    }

    cannon_transform.translation.x = cannon_transform.translation.x.clamp(-game_config.window_width/2.0, game_config.window_width/2.0);
}
