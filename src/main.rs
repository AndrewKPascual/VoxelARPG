use bevy::{
    prelude::*,
    core_pipeline::core_2d::Camera2dBundle,
    input::{InputPlugin, ButtonInput},
    app::AppExit,
};

mod voxel_terrain;
use voxel_terrain::VoxelTerrain;

// Import the character plugin module
mod character_model;
use character_model::CharacterPlugin;

// Import the animation plugin module
mod animation;
use animation::AnimationPlugin;

// Import the combat plugin module
mod combat;
use combat::CombatPlugin;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(InputPlugin) // Removed as DefaultPlugins already includes InputPlugin
        .insert_resource(VoxelTerrain::new(Vec3::new(100.0, 100.0, 100.0), 1.0))
        // Add the CharacterPlugin to the app
        .add_plugin(CharacterPlugin)
        // Add the AnimationPlugin to the app
        .add_plugin(AnimationPlugin)
        // Add the CombatPlugin to the app
        .add_plugin(CombatPlugin)
        // Initialize the startup system
        .add_systems(Startup, setup)
        .add_systems(Startup, voxel_terrain_setup)
        .add_systems(Update, player_input_system)
        .add_systems(Update, exit_on_esc_system)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    // Spawn a 2D camera entity
    commands.spawn(Camera2dBundle::default());
}

fn voxel_terrain_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    voxel_terrain: Res<VoxelTerrain>,
) {
    voxel_terrain.generate(&mut commands, &mut materials, &mut meshes);
}

fn player_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>, // Changed from Input<KeyCode> to ButtonInput<KeyCode>
    mut query: Query<&mut Transform, With<Player>>,
) {
    for mut transform in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            transform.translation.y += 2.;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            transform.translation.y -= 2.;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= 2.;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            transform.translation.x += 2.;
        }
    }
}

fn exit_on_esc_system(
    keyboard_input: Res<ButtonInput<KeyCode>>, // Changed from Input<KeyCode> to ButtonInput<KeyCode>
    mut exit: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
