use bevy::prelude::*;
use bevy::app::AppExit;

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
        .add_plugins(CharacterPlugin)
        // Add the AnimationPlugin to the app
        .add_plugins(AnimationPlugin)
        // Add the CombatPlugin to the app
        .add_plugins(CombatPlugin)
        // Add the ItemPlugin to the app
        .add_plugins(ItemPlugin)
        // Initialize the startup system
        .add_startup_system_to_stage(StartupStage::PreStartup, setup)
        .add_startup_system_to_stage(StartupStage::PreStartup, voxel_terrain_setup)
        // Add systems to the app with the correct schedule label
        .add_systems_to_stage(CoreStage::Update, player_input_system)
        .add_systems_to_stage(CoreStage::Update, exit_on_esc_system)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    // Spawn a 2D camera entity
    commands.spawn_bundle(Camera2dBundle::default());
    // Spawn the player entity
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, -215.0, 0.0),
            scale: Vec3::new(0.5, 0.5, 1.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Player);
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
    keyboard_input: Res<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
