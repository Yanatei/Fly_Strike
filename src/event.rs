use bevy::prelude::*;

//发射子弹
#[derive(Event)]
pub struct FireEvent(pub Vec3);

//计分
#[derive(Event)]
pub struct ScoreEvent;

//生成下一关的飞行物体
#[derive(Event)]
pub struct NextLevelBoidsEvent;

//自动切换到下一个游戏状态
#[derive(Event)]
pub struct AutoNextGameStateEvent;