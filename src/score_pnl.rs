//计分面板

#[device(component)]
pub struct ScoreType;

#[device(resource)]
pub struct Score(pub u32);

pub struct ScorelPlugin;

impl Plugin for ScorelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0));
        app.add_systems(Startup, setup);
        app.add_systems(Update, score_update_system);
    }
}

fn setup(
    mut commands: Commands,
     window: Single<&Window, With<PrimaryWindow>>,
     asset_server: Res<AssetServer>,
) {
    //初始化计分面板
    let window_width = window.width();
    let window_height = window.height();
    let pos_x = 0.0;
    let pos_y = 0.0;

    command.spawn((
        ScoreType,
        Text::new("得分: 0"),
        TextFont {
            font: global_font.0.clone(),
            font_size: SCORE_TEXT_FONT_SIZE,
            ..default()
        },
        TextColor(Color::srgb(0.5, 0.5, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(game_board_point.y),
            left: Val::Px(game_board_point.x - STATS_BOARD_WIDTH),
            ..default()
        },
    ));
}