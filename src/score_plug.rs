//计分面板

use bevy::color::palettes::css::{BLUE, YELLOW};
use bevy::{prelude::*};
use crate::event::*;
use crate::config::*;
use crate::game_state::GameState;

#[derive(Component)]
pub struct ScoreType;

#[derive(Component)]
pub struct ScoreSpanType;

#[derive(Component)]
pub struct DurationSpanType;

#[derive(Resource)]
pub struct DurationSpanTimer(pub Timer);

pub struct ScorelPlugin;

impl Plugin for ScorelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0));
        app.insert_resource(DurationSpanTimer(Timer::new(DURATION_SPAN_DURATION, TimerMode::Repeating)));
        app.add_systems(Startup, setup);
        app.add_systems(Update, (score_update_system, elapsed_time_update_system).run_if(in_state(GameState::InGame)));
        app.add_observer(on_scored);//添加观察者，得分时触发
    }
}

fn setup(
    mut commands: Commands,
     global_font: Res<GlobalFont>,
) {
    commands.spawn((
        Node {
            width: percent(15),
            height: percent(13),
            border: UiRect::all(Val::Px(2.0)),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Start,
            // padding: MARGIN.all(),
            // margin: MARGIN.all(),
            ..default()
        },
        //BackgroundColor(BLUE.into()),
    ))
    .with_children(|builder| {
        builder.spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                width: percent(100),
                //padding: MARGIN.all(),
                margin: MARGIN.all(),
                ..default()
            },
            //BackgroundColor(YELLOW.into()),
        ))
        .with_children(|builder|{
            builder.spawn((
                ScoreType,
                Text::new("得分: "),
                TextFont {
                    font: global_font.0.clone(),
                    font_size: SCORE_TEXT_FONT_SIZE,
                    ..default()
                },
                TextColor(SCORE_TEXT_COLOR),
            ))
            .with_child((
                ScoreSpanType,
                TextSpan::new("0"),
                TextFont {
                    font: global_font.0.clone(),
                    font_size: SCORE_TEXT_FONT_SIZE,
                    ..default()
                },
                TextColor(SCORE_TEXT_COLOR),
            ));
        });

        //下一行
        builder.spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                width: percent(100),
                //padding: MARGIN.all(),
                margin: MARGIN.all(),
                ..default()
            },
            //BackgroundColor(YELLOW.into()),
        ))
        .with_children(|builder|{
            builder.spawn((
                Text::new("耗时: "),
                TextFont {
                    font: global_font.0.clone(),
                    font_size: SCORE_TEXT_FONT_SIZE,
                    ..default()
                },
                TextColor(SCORE_TEXT_COLOR),
            ))
            .with_child((
                DurationSpanType,
                TextSpan::new("0.0"),
                TextFont {
                    font: global_font.0.clone(),
                    font_size: SCORE_TEXT_FONT_SIZE,
                    ..default()
                },
                TextColor(SCORE_TEXT_COLOR),
            ))
            .with_child((
                TextSpan::new("s"),
                TextFont {
                    font: global_font.0.clone(),
                    font_size: SCORE_TEXT_FONT_SIZE,
                    ..default()
                },
                TextColor(SCORE_TEXT_COLOR),
            ));
        });
    });
}

fn score_update_system(
    score: ResMut<Score>, 
    score_span_query: Single<&mut TextSpan, With<ScoreSpanType>>
) {
    let score_span = score_span_query.into_inner();
    **(score_span.into_inner()) = score.0.to_string();
}

fn elapsed_time_update_system(
    config: ResMut<GameConfig>,
    duration_span_query: Single<&mut TextSpan, With<DurationSpanType>>,
    mut duration_timer: ResMut<DurationSpanTimer>,
    time: Res<Time>
) {
    duration_timer.0.tick(time.delta());
    if duration_timer.0.is_finished() {
        let duration_span = duration_span_query.into_inner();
        let elapsed = config.elapsed_time[config.game_level + 1];
        **(duration_span.into_inner()) = format!("{:.2}", elapsed);
    }
}

fn on_scored(
    trigger: On<ScoreEvent>,
    mut commands: Commands, sound: Res<ScoreSound>, 
    mut score: ResMut<Score>,
    score_text: Single<(&mut TextSpan, &mut TextColor), With<ScoreSpanType>>
) {
    score.0 += 1;
    commands.spawn((AudioPlayer(sound.0.clone()), PlaybackSettings::DESPAWN));
    let (text, _) = score_text.into_inner();
    **(text.into_inner()) = score.0.to_string();
}