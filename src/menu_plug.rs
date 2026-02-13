use bevy::{asset::LoadedUntypedAsset, color::palettes::css::*, log, platform::hash, prelude::*};
use crate::config::*;

pub struct MenuPlug;

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

// #[derive(Resource)]
// struct AboutTextHandle(Handle<LoadedUntypedAsset>);

#[derive(Component)]
pub struct MainButton;

#[derive(Component)]
pub struct OnManMenuScreen;

#[derive(Component)]
pub struct OnAboutMenuScreen;

impl Plugin for MenuPlug {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>(); // 初始化菜单状态
        app.add_systems(Startup, setup);//生成游戏界面右上角的菜单按钮，同时初始化菜单状态为None
        app.add_systems(OnEnter(MenuState::ManMenu), create_man_menu); // 进入主菜单时创建主菜单界面
        app.add_systems(OnEnter(MenuState::AboutMenu), create_about_menu);// 进入About菜单时创建About界面
        // app.add_systems(OnEnter(MenuState::None), on_back_menu);// 菜单关闭时回到游戏状态
        app.add_systems(Update, man_menu_action);// 在游戏状态下监听菜单按钮的交互事件
        app.add_systems(Update, menu_action.run_if(in_state(GameState::Menu)));// 在游戏状态下监听菜单按钮的交互事件
    } 
}

fn setup(mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut menu_state: ResMut<NextState<MenuState>>
) {
    let man_menu_icon = asset_server.load("images/main_menu_icon.png");

    let margin_default =Val::Px(5.0);
    menu_state.set(MenuState::None);
    //生成右上角的菜单按钮
    let button_node = Node {
        position_type: PositionType::Absolute,
        top: Val::Px(10.0),
        right: Val::Px(5.0),
        width: Val::Percent(3.0),
        height: Val::Auto,
        aspect_ratio: Some(1.0),
        border: UiRect::all(Val::Px(2.0)),
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        ..default()
    };

    let icon_node = Node {
        width: percent(100),
        height: percent(100),
        border: UiRect::all(Val::Px(2.0)),
        padding: margin_default.all(),
        margin: margin_default.all(),
        ..default()
    };

    commands.spawn((
        MainButton,
        Button,
        button_node.clone(),
        BackgroundColor(Color::srgba(1.0, 0.0, 0.0, 1.0)),
        MenuAction::MainMenu,
        children![
            (ImageNode::new(man_menu_icon), icon_node.clone()),
        ]
    ));
}

fn create_man_menu(
    mut commands: Commands,
) {
    let width_s = 0.3;
    let height_s = 0.5;
    let top_s = (1.0 - height_s) / 2.0;
    let left_s = (1.0 - width_s) / 2.0;

    let button_node = Node {
        width: percent(80),
        height: percent(30),
        margin: Val::Px(5.0).all(),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = (
        TextFont {
            font_size: 33.0,
            ..default()
        },
        TextColor(Color::from(NAVY)),
    );

    commands.spawn((
        DespawnOnExit(MenuState::ManMenu),
        OnManMenuScreen,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(top_s * 100.0),
            left: Val::Percent(left_s * 100.0),
            width: Val::Percent(width_s * 100.0),
            height: Val::Percent(height_s * 100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(1.0, 1.0, 0.0, 1.0)),
        children![
            (
                Node {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    width: percent(80),
                    height: percent(80),
                    border: UiRect::all(Val::Px(2.0)),
                    padding: Val::Px(3.0).all(),
                    margin: Val::Px(3.0).all(),
                    ..default()
                },
                BackgroundColor(Color::srgba(1.0, 0.5, 0.0, 1.0)),
                children![
                    (
                        Button, button_node.clone(), BackgroundColor(Color::srgba(0.0, 1.0, 0.0, 1.0)), MenuAction::About,
                        children![(
                            Text::new("About"), button_text_style.clone(),
                        )]
                    ),
                    (
                        Button, button_node.clone(), BackgroundColor(Color::srgba(0.0, 1.0, 0.0, 1.0)), MenuAction::Back(MenuState::None),
                        children![(
                            Text::new("Back"), button_text_style.clone(),
                        )]
                    ),
                    (
                        Button, button_node.clone(), BackgroundColor(Color::srgba(0.0, 1.0, 0.0, 1.0)), MenuAction::Exit,
                        children![(
                            Text::new("Exit"), button_text_style.clone(),
                        )]
                    ),
                ],
            ),
        ]
    ));
}

fn create_about_menu(
    mut commands: Commands,
) {
    let width_s = 0.5;
    let height_s = 0.7;
    let top_s = (1.0 - height_s) / 2.0;
    let left_s = (1.0 - width_s) / 2.0;

    let title_height = 0.05;
    let content_height = 1.0 - title_height;

    let button_node = Node {
        width: percent(2),
        height: percent(100),
        margin: Val::Px(1.0).all(),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = (
        TextFont {
            font_size: 33.0,
            ..default()
        },
        TextColor(Color::from(BLACK)),
    );

    commands.spawn((
        //windows root
        DespawnOnExit(MenuState::AboutMenu),
        Node {
            width: Val::Percent(80.0),
            height: Val::Percent(80.0),
            position_type: PositionType::Absolute,
            top: Val::Percent(top_s * 100.0),
            left: Val::Percent(left_s * 100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Start,
            ..default()
        },
        BackgroundColor(Color::from(WHITE)),
    ))
    .with_children(|root_node|{
        //title
        root_node.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(title_height * 100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: Val::Px(5.0).all(),
                ..default()
            },
            BackgroundColor(Color::from(BLUE)),
        ))
        .with_children(|title_node|{
            title_node.spawn((
                Text::new("About"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
            ));

            title_node.spawn((
                Button,
                button_node.clone(),
                BackgroundColor(Color::srgba(1.0, 0.0, 0.0, 1.0)),
                MenuAction::Back(MenuState::ManMenu),
                children![(
                    Text::new("X"), button_text_style.clone(),
                )],
            ));
        });

        //content
        root_node.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(content_height * 100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: Val::Px(5.0).all(),
                ..default()
            },
            BackgroundColor(Color::from(WHITE_SMOKE)),
            Text::new(ABOUT_STR),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::from(BLACK)),
        ));
    });
}

fn on_back_menu(
    mut game_state: ResMut<NextState<GameState>>
) {
    game_state.set(GameState::InGame);
}

fn man_menu_action(
    mut commands: Commands,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
    interaction_query: Single<(&Interaction, &MenuAction), (Changed<Interaction>, With<MainButton>)>
){
    let (interaction, menu_action) = interaction_query.into_inner();
    if *interaction == Interaction::Pressed {
        game_state.set(GameState::Menu);
    }
}

fn menu_action(
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut message_writer: MessageWriter<AppExit>,
    interaction_query: Query<(&Interaction, &MenuAction), (Changed<Interaction>, With<Button>)> 
){
    for ( interaction, menu_action) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            log::info!("menu action: {:#?}", menu_action);
            match menu_action {
                MenuAction::MainMenu => {
                    menu_state.set(MenuState::ManMenu);
                },
                MenuAction::About => {
                    menu_state.set(MenuState::AboutMenu);
                },
                MenuAction::Back(in_state) => {
                    menu_state.set(*in_state);
                    if *in_state == MenuState::None {
                        game_state.set(GameState::InGame);
                    }
                },
                MenuAction::Exit => {
                    message_writer.write(AppExit::Success);
                }
                _ => {
                    log::error!("unknown menu action");
                }
            }
        }
    }
}
