use bevy::{prelude::*, time};
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use crate::config::*;

pub struct FpsPlugin;

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct FpsTextSpan;

#[derive(Resource)]
pub struct FpsTimer(Timer);

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        app.insert_resource(FpsTimer(Timer::new(FPS_TIME_DURATION, time::TimerMode::Repeating)));
        app.add_systems(Startup, setup);
        app.add_systems(Update, update_fps_text);
    }
}

//显示fps，注册组件
fn setup(
    global_font: Res<GlobalFont>, 
    mut commands: Commands,
    game_config: Res<GameConfig>,
) {
    commands.spawn((
        FpsText,
        Text::new("FPS: "),
        TextFont {
            font: global_font.0.clone(),
            font_size: FPS_TEXT_FONT_SIZE,
            ..default()
        },
        TextColor(FPS_TEXT_COLOR),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(game_config.window_width - 50.0),
            ..default()
        },
    ))
    .with_child((
        FpsTextSpan,
        TextSpan::new("0"),
        TextFont {
            font: global_font.0.clone(),
            font_size: FPS_TEXT_FONT_SIZE,
            ..default()
        },
        TextColor(FPS_TEXT_COLOR),
    ));
    
}

fn update_fps_text(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut TextSpan, With<FpsTextSpan>>,
    mut fps_timer: ResMut<FpsTimer>,
    time: Res<Time>,
) {
    fps_timer.0.tick(time.delta());

    if !fps_timer.0.just_finished() {
        return;
    }

    if let Some(fps) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.smoothed())
    {
        for text in &mut query {
            **(text.into_inner()) = format!("{:.0}", fps);
        }
    }
}