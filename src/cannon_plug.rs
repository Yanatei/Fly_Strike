use std::time::Duration;

use bevy::asset::transformer;
//炮台
use bevy::{log, prelude::*, time};
use leafwing_input_manager::prelude::ActionState;
use crate::{config::*, player_plug};
use crate::event::*;
use crate::player_plug::{Player, PlayerAction};


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
        app.add_systems(Update, (cannon_move_system_player,execute_cannon_animations)
            .run_if(not(in_state(GameState::Paused).or(in_state(GameState::Leaderboard)).or(not(in_state(MenuState::None)))))
        );
        app.add_observer(on_cannon_re_location_event);
    }
}

fn setup(mut commands: Commands, 
    asset_server: Res<AssetServer>,
    game_config: Res<GameConfig>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let pos_x = 0.0;
    let pos_y = get_cannon_y(game_config.window_height);

    //let i_handel = asset_server.load("images/cannon.png");
    let texture = asset_server.load("images/paotai.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(CANNON_IMAGE_HEIGHT), 3, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_config = AnimationConfig::new(CANNON_PARAMETER.0, CANNON_PARAMETER.1, CANNON_PARAMETER.2);

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

pub fn cannon_move_system_player(
    time: Res<Time>,
    mut bullet_timer: ResMut<BulletTimer>,
    mut cannot_query: Query<(Entity, &mut Transform, &mut Cannon)>,
    player_query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut commands: Commands,
    game_config: Res<GameConfig>,
){
    bullet_timer.0.tick(time.delta());

    let (_, mut cannon_transform, _) = cannot_query.single_mut().unwrap();
    if player_query.single().is_err() {
        log::info!("cannon_move_system_player player_query is empty!!!");
        return;
    }

    let player = player_query.single().unwrap();
    if player.pressed(&PlayerAction::Fire)  {
        if bullet_timer.0.is_finished() {//子弹发射间隔
            commands.trigger(FireEvent(cannon_transform.translation));
            bullet_timer.0.reset();
        }
    }

    if player.pressed(&PlayerAction::MoveLeft) {
        cannon_transform.translation.x -= CANNON_SPEED * time.delta_secs();
    }

    if player.pressed(&PlayerAction::MoveRight) {
        cannon_transform.translation.x += CANNON_SPEED * time.delta_secs();
    }

    cannon_transform.translation.x = cannon_transform.translation.x.clamp(-game_config.window_width/2.0, game_config.window_width/2.0);
}

fn execute_cannon_animations(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut Sprite)>,
    player_query: Single<&ActionState<PlayerAction>, With<Player>>,
) {
    if !(player_query.pressed(&PlayerAction::MoveLeft) || player_query.pressed(&PlayerAction::MoveRight)) {
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

fn on_cannon_re_location_event(
    trigger: On<CannonReLocationEvent>,
    mut query: Single<(Entity, &mut Transform, &mut Cannon)>,
    game_config: Res<GameConfig>,
){
    let (_, mut transform, _) = query.into_inner();
    transform.translation.y = get_cannon_y(game_config.window_height);
}

fn get_cannon_y(window_height: f32) -> f32 {
    let pos_y = -(window_height/2.0) + CANNON_HEIGHT as f32/2.0;
    pos_y
}
