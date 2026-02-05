# 第一版
    飞行物做成一个泡泡
    可以被子弹击中得分，时长最短的得分更高。 得分排名存储。
## 泡泡
## 炮台、子弹
## 得分
## 菜单
## 排行
## 发布

# 第二版
    飞行物做成肥皂泡和各种小动物，有不同的碰撞体积和飞行速度。
    可以被子弹击中得分，时长最短的得分更高。 得分排名存储。


commands.spawn((
        ScoreType,
        Text::new("得分: "),
        TextFont {
            font: global_font.0.clone(),
            font_size: SCORE_TEXT_FONT_SIZE,
            ..default()
        },
        TextColor(SCORE_TEXT_COLOR),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            ..default()
        },
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