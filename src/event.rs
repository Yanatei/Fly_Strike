use bevy::prelude::*;

//发射子弹
#[derive(Event)]
pub struct FireEvent(pub Vec3);

//计分
#[derive(Event)]
pub struct ScoreEvent;

//关卡显示
#[derive(Event)]
pub struct GameLevelEvent;

//生成下一关的飞行物体
#[derive(Event)]
pub struct NextLevelBoidsEvent;

//自动切换到下一个游戏状态
#[derive(Event)]
pub struct AutoNextGameStateEvent;

//重新调整炮台的位置(窗口尺寸发生改变)
#[derive(Event)]
pub struct CannonReLocationEvent;

//重新调整飞鸟的飞行限制(窗口尺寸发生改变)
#[derive(Event)]
pub struct BoidsReLimitEvent;