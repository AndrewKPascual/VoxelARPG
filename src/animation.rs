use bevy::prelude::*;
use crate::character_model::Character;

// Define a struct for managing character animation state
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
    mut query: Query<(&mut CharacterAnimation, &mut Transform), With<Character>>,
) {
    for (mut animation, mut transform) in query.iter_mut() {
        animation.elapsed_time += time.delta_seconds();

        // Loop the animation based on the total frames and frame duration
        if animation.elapsed_time > animation.frame_duration {
            animation.elapsed_time = 0.0;
            animation.current_frame = (animation.current_frame + 1) % animation.total_frames;

            // Update the character's transform based on the current frame
            // This is a placeholder for actual animation logic
            transform.translation.y = (animation.current_frame as f32) * 0.1;
        }
    }
}
