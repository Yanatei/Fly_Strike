//use bevy::color::Color;

use std::default;

use bevy::{asset::Handle, audio::AudioSource, color::Color, ecs::{component::Component, resource::Resource}, image::Image, state::state::States, text::Font, time::Timer, ui::Val};

//game config
#[derive(Resource)]
pub struct GameConfig {
    pub window_width: f32,
    pub window_height: f32,

    //计时,单位秒
    pub elapsed_time: [f32; 3],
    //第几关,第几步
    pub game_level: usize,
    pub game_level_index: usize,
    //最大关数
    pub MAX_GAME_LEVEL: usize,

    //游戏结束时的烟花计数器
    pub fireworks_count: usize,
}

impl GameConfig {
    pub fn default() -> Self {
        Self {
            window_width: 300.0,
            window_height: 500.0,
            elapsed_time: [0.0; 3],
            game_level: 0,
            MAX_GAME_LEVEL: 3,
            fireworks_count: 0,
            game_level_index: 0,
        }
    }
    pub fn width_window_size(mut self, width: f32, height: f32) -> Self {
        self.window_width = width;
        self.window_height = height;
        self
    }
}

//游戏状态
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum GameState {
    #[default]
    None,       // 无状态
    BeforeInGame,   // 准备进入游戏
    InGame,     // 游戏中
    InCutscene, // 过场动画中
    Paused, // 暂停
    Leaderboard,//排行榜
    GameOver, // 游戏结束
}

//游戏关卡状态定义, 一共三关
#[derive(Resource)]
pub struct GameStateDef{
    pub game_states: [Vec<GameState>; 3],
}

impl Default for GameStateDef {
    fn default() -> Self {
        Self {
            game_states: [
                vec![GameState::BeforeInGame, GameState::InGame, GameState::InCutscene],
                vec![GameState::BeforeInGame, GameState::InGame, GameState::InCutscene],
                vec![GameState::BeforeInGame, GameState::InGame, GameState::GameOver, GameState::Leaderboard],
            ],
        }
    }
}

//过场动画步骤
#[derive(Debug, Clone, Copy,Eq, PartialEq, Hash, States, Default)]
pub enum CutsceneStepState {
    #[default]
    None,
    BeforeCutscene, // 准备过场动画阶段
    InCutscene, // 过场动画中
    AfterCutscene, // 过场动画结束阶段
}

// 主菜单插件，展示和控制主菜单界面逻辑，1：设置声音大小，2：About界面，3:返回游戏
//目前只有一个菜单界面，点击右上的按钮触发，弹出一个窗口，里面有三个按钮，分别是设置声音大小，About界面，返回游戏
//点击About界面，弹出一个窗口，显示游戏的相关信息
#[derive(Resource,Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum MenuState {
    ManMenu,
    AboutMenu,
    #[default]
    None,
}

#[derive(Component, Debug)]
pub enum MenuAction{
    MainMenu,
    About,
    Exit,
    Back(MenuState),
}

//飞行物体
#[derive(Resource)]
pub struct BoidsImage(pub Handle<Image>);
pub const FLY_COUNT: usize = 3;
pub const FLY_SIZE: f32 = 0.2;
pub const BOID_SPEED_INCREMENT: f32 = 50.0;

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
#[derive(Resource)]
pub struct FireworksSound(pub Handle<AudioSource>);
#[derive(Resource)]
pub struct GameStartedSound(pub Handle<AudioSource>);

//计分面板
pub const SCORE_TEXT_FONT_SIZE: f32 = 19.0;
pub const SCORE_TEXT_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);

//FPS
pub const FPS_TEXT_FONT_SIZE: f32 = 10.0;
pub const FPS_TEXT_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
pub const FPS_TIME_DURATION: std::time::Duration = std::time::Duration::from_millis(1000);

//UI布局
pub const MARGIN: Val = Val::Px(3.);

//过场动画
pub const BEFORE_IN_GAME_DURATION: std::time::Duration = std::time::Duration::from_millis(1500);
pub const BEFORE_CUTSCENE_DURATION: std::time::Duration = std::time::Duration::from_millis(1500);
pub const IN_CUSTSCENE_DURATION: std::time::Duration = std::time::Duration::from_secs(4);
pub const AFTER_CUTSCENE_DURATION: std::time::Duration = std::time::Duration::from_millis(500);
pub const OVER_CUTSCENE_DURATION: std::time::Duration = std::time::Duration::from_millis(200);//游戏结束时激发动画的时长

pub const ABOUT_STR: &str ="
Fly_Strike
Version 0.1

A fast-paced arcade challenge.

Destroy all floating bubbles as quickly as possible.
The faster you finish, the higher your score.

Developed by Orc

Contact:
Email: zheng.yanan84@gmail.com
GitHub: https://github.com/Yanatei
";