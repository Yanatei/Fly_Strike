//计分面板

use bevy::color::palettes::css::{BLUE, YELLOW};
use bevy::{prelude::*};
use crate::event::*;
use crate::config::*;

#[derive(Component)]
pub struct ScoreType;

#[derive(Component)]
pub struct ScoreSpanType;

pub struct ScorelPlugin;

impl Plugin for ScorelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0));
        app.add_systems(Startup, setup);
        app.add_systems(Update, score_update_system);
        app.add_observer(on_scored);//添加观察者，得分时触发
    }
}

fn setup(
    mut commands: Commands,
     global_font: Res<GlobalFont>,
) {
    commands.spawn((
        // ScoreType,
        // TextColor(SCORE_TEXT_COLOR),
        Node {
            width: percent(15),
            height: percent(10),
            border: UiRect::all(Val::Px(2.0)),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Start,
            padding: MARGIN.all(),
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
    });
}

fn score_update_system(
    score: ResMut<Score>, 
    score_span_query: Single<&mut TextSpan, With<ScoreSpanType>>
) {
    let score_span = score_span_query.into_inner();
    **(score_span.into_inner()) = score.0.to_string();
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