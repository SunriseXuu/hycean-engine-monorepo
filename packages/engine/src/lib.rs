pub mod editor_systems;
pub mod event_bus;

use bevy::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_engine_core() {
    // Let Rust's panic messages be printed to the browser's F12 console
    // Otherwise it will only show obscure WASM memory errors
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // The ID of the HTML element that Bevy will take over
                canvas: Some("#engine".into()),
                // Prevent default right-click menu and other behaviors
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        // Add our custom plugins
        .add_plugins((
            event_bus::EventBusPlugin,
            editor_systems::EditorSystemsPlugin,
        ))
        // Setup a minimal test scene
        .add_systems(Startup, setup_scene)
        .run();
}

// Setup a minimal test scene: a cube, a point light, and a camera
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 1. Spawn a clean green cube using modern Required Components syntax
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.8, 0.6))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // 2. Spawn a point light with shadows
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            // Modern Bevy uses physical light units (lumens/lux).
            // A higher value is required to achieve the same brightness as older versions.
            intensity: 2_000_000.0,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // 3. Spawn your "director's eye" (camera)
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
