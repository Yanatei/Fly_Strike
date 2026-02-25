//输入映射控制
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum Action {
    Move,
    Fire,
}

fn spawn_pc_player(mut commands: Commands) {
    let input_map = InputMap::default()
        .insert(KeyCode::KeyA, PlayerAction::Move)
        .insert(KeyCode::KeyD, PlayerAction::Move)
        .insert(KeyCode::ArrowLeft, PlayerAction::Move)
        .insert(KeyCode::ArrowLeft, PlayerAction::Move)
        .insert(KeyCode::Space, PlayerAction::Fire);

    commands.spawn((
        InputManagerBundle::<PlayerAction> {
            input_map,
            action_state: ActionState::default(),
        },
        Player,
    ));
}

fn touch_input_system(
    touches: Res<Touches>,
    mut query: Query<&mut ActionState<PlayerAction>, With<Player>>,
) {
    let mut action_state = query.single_mut();

    // 默认清零移动
    action_state.set_axis(PlayerAction::Move, 0.0);

    for touch in touches.iter() {
        let delta = touch.delta();

        // 滑动阈值
        let threshold = 5.0;

        if delta.x.abs() > threshold {
            // 左右移动
            let direction = delta.x.signum(); // -1 或 1
            action_state.set_axis(PlayerAction::Move, direction);
        }
        else {
            // 如果是刚刚按下且没滑动，认为是点击
            if touch.just_pressed() {
                action_state.press(PlayerAction::Fire);
            }
        }
    }
}