use bevy::ecs::system::command;
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

#[derive(Resource)]
pub struct CannonTimer(Timer);

impl Plugin for CannonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.insert_resource(CannonTimer(Timer::new(CANNON_TIME_DURATION, time::TimerMode::Repeating)));
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
    // commands.insert_resource(CannonConfig{
    //     image_handel : i_handel.clone()
    // });

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
    mut cannon_time: ResMut<CannonTimer>,
    mut query: Query<(Entity, &mut Transform, &mut Cannon)>,
    mut commands: Commands
){
    cannon_time.0.tick(time.delta());

    if input.just_pressed(KeyCode::Space) && cannon_time.0.just_finished(){
        let (_, transform, _) = query.single_mut().unwrap();
        commands.trigger(FireEvent(transform.translation));
    }
}
