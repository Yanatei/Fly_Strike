//use bevy::color::Color;

use bevy::{asset::Handle, audio::AudioSource, color::Color, ecs::resource::Resource, image::Image, state::state::States, text::Font, time::Timer, ui::Val};

//game config
#[derive(Resource)]
pub struct GameConfig {
    pub window_width: f32,
    pub window_height: f32,

    //计时,单位秒
    pub elapsed_time: [f32; 3],
    //第几关
    pub game_level: usize,
    //最大关数
    pub MAX_GAME_LEVEL: usize,
}

impl GameConfig {
    pub fn default() -> Self {
        Self {
            window_width: 300.0,
            window_height: 500.0,
            elapsed_time: [0.0; 3],
            game_level: 0,
            MAX_GAME_LEVEL: 3,
        }
    }
    pub fn width_window_size(mut self, width: f32, height: f32) -> Self {
        self.window_width = width;
        self.window_height = height;
        self
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum GameState {
    Menu,   // 主菜单
    #[default]
    InGame,     // 游戏中
    LevelComplete,  // 过关停顿 + 动画
    LoadingNext,    // 
    Paused, // 暂停
    GameOver, // 游戏结束
}

//飞行物体
#[derive(Resource)]
pub struct BoidsImage(pub Handle<Image>);
pub const FLY_COUNT: usize = 30;
pub const FLY_SIZE: f32 = 0.2;
//pub const FLY_COLOR: Color = Color::srgb(0.0, 0.7, 0.0);

//炮台
pub const CANNON_SIZE: f32 = 0.4;
pub const CANNON_IMAGE_HEIGHT: u32 = 200; //炮台原始图片高度 px
pub const CANNON_HEIGHT: f32 = CANNON_IMAGE_HEIGHT as f32 * CANNON_SIZE; //炮台高度 px
pub const CANNON_PARAMETER: (usize, usize, u8) = (0, 2, 9); //第一帧、最后一帧、频率
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
pub const DURATION_SPAN_DURATION: std::time::Duration = std::time::Duration::from_millis(100);

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

//UI布局
pub const MARGIN: Val = Val::Px(3.);

//过场动画
pub const LEVEL_COMPLETE_DURATION: std::time::Duration = std::time::Duration::from_secs(6);
