use bevy::{app::Plugin, reflect::Reflect};
use bevy::{log, prelude::*};
//输入映射控制
use leafwing_input_manager::prelude::*;

use crate::cannon_plug::cannon_move_system_player;
use crate::config::{GameConfig, GameState, MenuState};

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PlayerAction {
    MoveLeft,
    MoveRight,
    Fire,
}

pub struct PlayerPlug;

#[derive(Component)]
pub struct Player;

impl Plugin for PlayerPlug {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
        app.add_systems(Startup, setup);
        if cfg!(any(target_os="android", target_os="ios")){
            app.add_systems(Update, mobile_touch_input_system.before(cannon_move_system_player)
                .run_if(not(in_state(GameState::Paused).or(in_state(GameState::Leaderboard)).or(not(in_state(MenuState::None)))))
            );
        }
    }
}

fn setup(
mut commands: Commands
){
    let input_map = InputMap::new([
        (PlayerAction::MoveLeft, KeyCode::KeyA),
        (PlayerAction::MoveLeft, KeyCode::ArrowLeft),
        (PlayerAction::MoveRight, KeyCode::KeyD),
        (PlayerAction::MoveRight, KeyCode::ArrowRight),
        (PlayerAction::Fire, KeyCode::Space),
        (PlayerAction::Fire, KeyCode::ArrowUp)
    ]);
    //只有桌面环境注册按键映射
    if cfg!(any(target_os="windows", target_os="linux", target_os="macos")) {
        commands.spawn(input_map).insert(Player);
    }else{
        commands.spawn((
            Player,
            ActionState::<PlayerAction>::default(),
        ));
    }
}

fn mobile_touch_input_system(
    mut player_query: Query<&mut ActionState<PlayerAction>, With<Player>>,
    touches: Res<Touches>,
    game_config: Res<GameConfig>
) {
    let action_state = player_query.single_mut();
    if action_state.is_err() {
        log::info!("mobile_touch_input_system player_query is empty!!!");
        return;
    }
    let mut action_state = action_state.unwrap();
    action_state.release(&PlayerAction::MoveLeft);
    action_state.release(&PlayerAction::MoveRight);
    action_state.release(&PlayerAction::Fire);

    for touch in touches.iter() {
        // let position = touch.position();

        // let screen_width = game_config.window_width;

        // if position.x < screen_width / 2.0 {
        //     // 左半屏控制移动
        //     if position.x < screen_width / 4.0 {
        //         action_state.press(&PlayerAction::MoveLeft);
        //     } else {
        //         action_state.press(&PlayerAction::MoveRight);
        //     }
        // } else {
        //     // 右半屏开火
        //     action_state.press(&PlayerAction::Fire);
        // }
        //手机端只开火，不移动炮台
        action_state.press(&PlayerAction::Fire);
    }
}