//use bevy::color::Color;

use bevy::{asset::Handle, audio::AudioSource, color::Color, ecs::resource::Resource, image::Image, text::Font, time::Timer, ui::Val};

//game config
#[derive(Resource)]
pub struct GameConfig {
    pub window_width: f32,
    pub window_height: f32,
}

//飞行物体
#[derive(Resource)]
pub struct BoidsImage(pub Handle<Image>);
pub const FLY_COUNT: usize = 30;
pub const FLY_SIZE: f32 = 0.2;
//pub const FLY_COLOR: Color = Color::srgb(0.0, 0.7, 0.0);

//炮台
pub const CANNON_HEIGHT: f32 = 0.0;
pub const BULLET_TIME_DURATION: std::time::Duration = std::time::Duration::from_millis(1000); //子弹发射间隔时间t
pub const CANNON_SPEED: f32 = 300.0;

//子弹
pub const BULLET_SIZE: f32 = 2.0;
pub const BULLET_SPEED: f32 = 900.0;
#[derive(Resource)]
pub struct BulletTimer(pub Timer);

//字体
#[derive(Resource)]
pub struct GlobalFont(pub Handle<Font>);

//计分
#[derive(Resource)]
pub struct Score(pub u32);

//音效
#[derive(Resource)]
pub struct ScoreSound(pub Handle<AudioSource>);

//计分面板
pub const SCORE_TEXT_FONT_SIZE: f32 = 25.0;
pub const SCORE_TEXT_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);

//FPS
pub const FPS_TEXT_FONT_SIZE: f32 = 10.0;
pub const FPS_TEXT_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
pub const FPS_TIME_DURATION: std::time::Duration = std::time::Duration::from_millis(1000);

//布局
pub const MARGIN: Val = Val::Px(6.);