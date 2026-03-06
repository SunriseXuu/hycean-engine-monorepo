use bevy::prelude::*;
use serde::Deserialize;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

// 全局信箱
static COMMAND_QUEUE: Mutex<Vec<EditorCommand>> = Mutex::new(Vec::new());

// 跨界指令协议
#[derive(Deserialize, Debug, Clone)]
pub enum EditorCommand {
    UpdateCubeColor { r: f32, g: f32, b: f32 },
}

// Bevy 内部事件
#[derive(Event)]
pub struct EngineEvent(pub EditorCommand);

// 暴露给 JS 的唯一总线入口
// 注意：只要这里加了 #[wasm_bindgen]，无论它藏在哪个模块，最后都会被提升到 JS 的顶层！
#[wasm_bindgen]
pub fn dispatch_command(json_str: &str) {
    match serde_json::from_str::<EditorCommand>(json_str) {
        Ok(cmd) => {
            if let Ok(mut queue) = COMMAND_QUEUE.lock() {
                queue.push(cmd);
            }
        }
        Err(e) => info!("[WASM] 前端传来的指令解析失败: {}", e),
    }
}

// =======================================================
// 【核心设计】：将邮局打包成一个 Bevy Plugin！
// 任何人想用这个事件总线，只需要在 App 里 add_plugins(EventBusPlugin) 即可
// =======================================================
pub struct EventBusPlugin;

impl Plugin for EventBusPlugin {
    fn build(&self, app: &mut App) {
        // 注册事件，并安排“收信员”每帧去信箱取信
        app.add_systems(Update, receive_commands_system);
    }
}

// 收信员逻辑
fn receive_commands_system(mut commands: Commands) {
    if let Ok(mut queue) = COMMAND_QUEUE.lock() {
        for cmd in queue.drain(..) {
            commands.trigger(EngineEvent(cmd));
        }
    }
}
