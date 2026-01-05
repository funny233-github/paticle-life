use crate::input_focus::InputFocus;
use bevy::prelude::*;

/// 相机移动控制参数
#[derive(Resource, Clone, Copy)]
pub struct CameraMoveConfig {
    /// 相机移动速度
    pub speed: f32,
    /// 缩放速度
    pub zoom_speed: f32,
    /// 最小缩放倍数
    pub min_scale: f32,
    /// 最大缩放倍数
    pub max_scale: f32,
}

impl Default for CameraMoveConfig {
    fn default() -> Self {
        Self {
            speed: 400.0,
            zoom_speed: 1.0,
            min_scale: 0.01,
            max_scale: 50.0,
        }
    }
}

/// 相机移动和缩放控制
///
/// 功能：
/// - WASD: 移动相机
/// - +/-: 缩放相机
/// - 只在游戏焦点时响应输入
pub fn move_camera(
    mut camera: Query<(&mut Transform, &Camera), With<Camera2d>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    input_focus: Res<InputFocus>,
    config: Res<CameraMoveConfig>,
) {
    // 只在游戏焦点时处理相机移动
    if !input_focus.is_game() {
        return;
    }

    let Ok((mut transform, _camera)) = camera.single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;
    let current_scale = transform.scale;

    // 移动控制
    if keys.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    // 应用移动
    if direction != Vec3::ZERO {
        transform.translation += direction.normalize() * config.speed * current_scale * time.delta_secs();
    }

    // 缩放控制
    if keys.pressed(KeyCode::Minus) || keys.pressed(KeyCode::NumpadAdd) {
        transform.scale *= 1.0 + config.zoom_speed * time.delta_secs();
    }
    if keys.pressed(KeyCode::Equal) || keys.pressed(KeyCode::NumpadSubtract) {
        transform.scale *= 1.0 - config.zoom_speed * time.delta_secs();
    }

    // 限制缩放范围
    transform.scale = transform.scale.clamp(
        Vec3::splat(config.min_scale),
        Vec3::splat(config.max_scale),
    );
}

/// 插件：注册相机移动系统
pub struct CameraMovePlugin;

impl Plugin for CameraMovePlugin {
    fn build(&self, app: &mut App) {
        // 插入默认的相机移动配置
        app.insert_resource(CameraMoveConfig::default());

        // 注册相机移动系统
        app.add_systems(Update, move_camera);
    }
}
