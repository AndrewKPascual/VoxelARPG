use bevy::prelude::*;
use crate::character_model::{Character, CharacterAssets};
use crate::items::Equipment;

// Define a struct for managing character animation state
#[derive(Component)]
pub struct CharacterAnimation {
    pub current_frame: usize,
    pub total_frames: usize,
    pub frame_duration: f32,
    pub elapsed_time: f32,
}

// Plugin to set up animation systems
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(animate_character_system);
    }
}

// System to animate characters based on elapsed time
fn animate_character_system(
    time: Res<Time>,
    mut query: Query<(&mut CharacterAnimation, &mut Transform, &Equipment, &mut Handle<StandardMaterial>), With<Character>>,
    character_assets: Res<CharacterAssets>,
) {
    for (mut animation, mut transform, equipment, mut material_handle) in query.iter_mut() {
        animation.elapsed_time += time.delta_seconds();

        // Loop the animation based on the total frames and frame duration
        if animation.elapsed_time > animation.frame_duration {
            animation.elapsed_time = 0.0;
            animation.current_frame = (animation.current_frame + 1) % animation.total_frames;

            // Update the character's transform based on the current frame
            // This is a placeholder for actual animation logic
            transform.translation.y = (animation.current_frame as f32) * 0.1;
        }

        // Update the character's material based on the equipped hat
        if let Some(equipped_hat) = &equipment.hat {
            // Update the material handle to the equipped hat's material
            *material_handle = equipped_hat.material_handle.clone();
        } else {
            // If no hat is equipped, use the default character material
            *material_handle = character_assets.character_material.clone();
        }
    }
}
