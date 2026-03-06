use bevy::prelude::*;
// 引入兄弟模块里定义的指令和事件
use crate::event_bus::{EditorCommand, EngineEvent};

// 同样，把所有的业务系统打包成一个 Plugin
pub struct EditorSystemsPlugin;

impl Plugin for EditorSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(process_commands_system);
    }
}

fn process_commands_system(
    event: On<EngineEvent>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<&MeshMaterial3d<StandardMaterial>>,
) {
    match event.0.clone() {
        EditorCommand::UpdateCubeColor { r, g, b } => {
            info!("[Bevy Editor] 执行变色: R:{}, G:{}, B:{}", r, g, b);
            for material_handle in query.iter() {
                if let Some(material) = materials.get_mut(&material_handle.0) {
                    material.base_color = Color::srgb(r, g, b);
                }
            }
        }
    }
}
