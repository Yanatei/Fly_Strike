use std::time::Duration;

//炮台
use bevy::{prelude::*, time};
use crate::config::*;
use crate::event::*;


#[derive(Component, Resource)]
pub struct Cannon;

#[derive(Component)]
struct AnimationConfig {
    first_index: usize,
    last_index: usize,
    fps: u8,
    frame_timer: Timer,
}
impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_index: first,
            last_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}
pub struct CannonPlugin;

// #[derive(Resource)]
// pub struct FireTimer(Timer);

impl Plugin for CannonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (cannon_move_system, execute_cannon_animations));
    }
}

fn setup(mut commands: Commands, 
    asset_server: Res<AssetServer>,
    game_config: Res<GameConfig>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let pos_x = 0.0;
    let pos_y = -(game_config.window_height/2.0) + CANNON_HEIGHT as f32/2.0;

    //let i_handel = asset_server.load("images/cannon.png");
    let texture = asset_server.load("images/paotai.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(CANNON_IMAGE_HEIGHT), 3, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_config = AnimationConfig::new(0, 2, 10);

    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_config.first_index,
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(CANNON_SIZE)).with_translation(Vec3::new(pos_x, pos_y, 0.0)),
        Cannon,
        animation_config,
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

    let (_, mut cannon_transform, _) = query.single_mut().unwrap();

    if input.just_pressed(KeyCode::Space)  {
        if bullet_timer.0.is_finished() {//子弹发射间隔
            commands.trigger(FireEvent(cannon_transform.translation));
            bullet_timer.0.reset();
        }
    }

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

fn execute_cannon_animations(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut Sprite)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if !input.pressed(KeyCode::ArrowLeft) && !input.pressed(KeyCode::ArrowRight) {
        return;
    }
    for (mut animation_config, mut sprite) in query.iter_mut() {
        animation_config.frame_timer.tick(time.delta());
        if animation_config.frame_timer.is_finished() && let Some(atlas) = &mut sprite.texture_atlas {
            if atlas.index >= animation_config.last_index {
                atlas.index = animation_config.first_index;
            } else {
                atlas.index += 1;
            }

            animation_config.frame_timer.reset();
        }
    }
}