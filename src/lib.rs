use bevy::{
    prelude::*,
    core_pipeline::core_2d::Camera2dBundle,
    app::{App, AppExit},
    input::{keyboard::KeyCode, Input},
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

// Import the items plugin module
mod items;
use items::ItemPlugin;

#[derive(Component)]
struct Player;

pub fn run_app() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(VoxelTerrain::new(Vec3::new(100.0, 100.0, 100.0), 1.0))
        // Add the CharacterPlugin to the app
        .add_plugin(CharacterPlugin)
        // Add the AnimationPlugin to the app
        .add_plugin(AnimationPlugin)
        // Add the CombatPlugin to the app
        .add_plugin(CombatPlugin)
        // Add the ItemPlugin to the app
        .add_plugin(ItemPlugin)
        // Initialize the startup system
        .add_startup_system(setup)
        .add_startup_system(voxel_terrain_setup)
        // Add systems to the app
        .add_system(player_input_system)
        .add_system(exit_on_esc_system)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    // Spawn a 2D camera entity
    commands.spawn_bundle(Camera2dBundle::default());
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
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    for mut transform in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += 2.;
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= 2.;
        }
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= 2.;
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += 2.;
        }
    }
}

fn exit_on_esc_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
