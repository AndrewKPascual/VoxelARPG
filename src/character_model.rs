use bevy::{
    prelude::*,
    math::primitives::Cuboid,
};
use crate::animation::CharacterAnimation;

// Define a struct for our character
#[derive(Component)]
pub struct Character;

// Define a struct for the hat
#[derive(Component)]
pub struct Hat;

// Define a struct to hold our character assets
#[derive(Resource, Clone)]
pub struct CharacterAssets {
    pub character_mesh: Handle<Mesh>,
    pub character_material: Handle<StandardMaterial>,
    pub hat_mesh: Handle<Mesh>,
    pub hat_material: Handle<StandardMaterial>,
}

// Plugin to set up character assets and spawning
pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CharacterAssets {
                character_mesh: Default::default(),
                character_material: Default::default(),
                hat_mesh: Default::default(),
                hat_material: Default::default(),
            })
            .add_startup_system(setup_characters);
    }
}

// System to set up character assets
fn setup_characters(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut character_assets: ResMut<CharacterAssets>,
) {
    // Create a cuboid mesh for the character body
    character_assets.character_mesh = meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0)));
    // Create a material for the character
    character_assets.character_material = materials.add(Color::rgb(0.8, 0.7, 0.6).into());

    // Create a smaller cuboid mesh for the hat
    character_assets.hat_mesh = meshes.add(Mesh::from(Cuboid::new(0.5, 0.5, 0.5)));
    // Create a material for the hat
    character_assets.hat_material = materials.add(Color::rgb(0.1, 0.1, 0.1).into());

    // Spawn the character entity with the mesh, material, and animation component
    commands
        .spawn_bundle(PbrBundle {
            mesh: character_assets.character_mesh.clone(),
            material: character_assets.character_material.clone(),
            ..Default::default()
        })
        .insert(Character)
        .insert(CharacterAnimation {
            current_frame: 0,
            total_frames: 2, // Assuming we have 2 frames for simplicity
            frame_duration: 0.5, // Change the frame every 0.5 seconds
            elapsed_time: 0.0,
        });

    // Spawn the hat entity with the mesh and material
    commands
        .spawn_bundle(PbrBundle {
            mesh: character_assets.hat_mesh.clone(),
            material: character_assets.hat_material.clone(),
            transform: Transform::from_xyz(0.0, 0.75, 0.0), // Position the hat above the character
            ..Default::default()
        })
        .insert(Hat);
}
