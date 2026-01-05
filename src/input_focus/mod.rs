use bevy::prelude::*;

/// 游戏输入焦点状态
#[derive(Resource, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum InputFocus {
    /// 游戏焦点：接收游戏操作输入
    #[default]
    Game,
    /// 控制台焦点：接收控制台输入
    Console,
}

impl InputFocus {
    /// 切换焦点
    pub fn toggle(&mut self) {
        *self = match *self {
            InputFocus::Game => InputFocus::Console,
            InputFocus::Console => InputFocus::Game,
        };
    }

    /// 检查是否为游戏焦点
    pub fn is_game(&self) -> bool {
        *self == InputFocus::Game
    }

    /// 检查是否为控制台焦点
    pub fn is_console(&self) -> bool {
        *self == InputFocus::Console
    }

    /// 设置为游戏焦点
    pub fn set_game(&mut self) {
        *self = InputFocus::Game;
    }

    /// 设置为控制台焦点
    pub fn set_console(&mut self) {
        *self = InputFocus::Console;
    }
}

/// 切换输入焦点
///
/// 使用 Backquote (`) 键在游戏和控制台焦点之间切换
pub fn toggle_input_focus(
    keys: Res<ButtonInput<KeyCode>>,
    mut input_focus: ResMut<InputFocus>,
) {
    if keys.just_pressed(KeyCode::Backquote) {
        input_focus.toggle();
        bevy::log::info!(
            "Input focus switched to: {}",
            if input_focus.is_game() { "Game" } else { "Console" }
        );
    }
}

/// 释放输入焦点（当控制台关闭时，确保焦点回到游戏）
///
/// 使用 ESC 键从控制台焦点回到游戏焦点
pub fn release_console_focus(
    keys: Res<ButtonInput<KeyCode>>,
    mut input_focus: ResMut<InputFocus>,
) {
    // 当在控制台按 ESC 时，关闭控制台并回到游戏焦点
    if input_focus.is_console() && keys.just_pressed(KeyCode::Escape) {
        input_focus.set_game();
        bevy::log::info!("Console closed, input focus returned to Game");
    }
}

/// 插件：注册输入焦点系统
pub struct InputFocusPlugin;

impl Plugin for InputFocusPlugin {
    fn build(&self, app: &mut App) {
        // 插入默认的输入焦点资源
        app.insert_resource(InputFocus::default());

        // 注册系统
        app.add_systems(Update, toggle_input_focus);
        app.add_systems(Update, release_console_focus);
    }
}
