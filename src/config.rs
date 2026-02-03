//use bevy::color::Color;

use bevy::ecs::resource::Resource;

//game config
#[derive(Resource)]
pub struct GameConfig {
    pub window_width: f32,
    pub window_height: f32,
}


//飞行物体
pub const FLY_COUNT: usize = 30;
pub const FLY_SIZE: f32 = 0.2;
//pub const FLY_COLOR: Color = Color::srgb(0.0, 0.7, 0.0);

//炮台
pub const CANNON_HEIGHT: f32 = 0.0;
pub const CANNON_TIME_DURATION: std::time::Duration = std::time::Duration::from_millis(5);
pub const BULLET_TIME_DURATION: std::time::Duration = std::time::Duration::from_millis(1000); //子弹发射间隔时间t

//子弹
pub const BULLET_SPEED: f32 = 200.0;