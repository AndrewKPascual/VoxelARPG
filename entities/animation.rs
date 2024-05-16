use bevy::prelude::*;

// Define a struct for our character animation
pub struct CharacterAnimation {
    pub current_frame: usize,
    pub total_frames: usize,
    pub frame_duration: f32, // in seconds
    pub elapsed_time: f32, // in seconds
}

// Define a struct to hold our animation assets
pub struct AnimationAssets {
    pub idle_frames: Vec<Handle<Mesh>>,
    pub walk_frames: Vec<Handle<Mesh>>,
}

// Plugin to set up animation assets and systems
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AnimationAssets {
                idle_frames: Vec::new(),
                walk_frames: Vec::new(),
            })
            .add_startup_system(setup_animations)
            .add_system(animate_character_system);
    }
}

// System to set up animation assets
fn setup_animations(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut animation_assets: ResMut<AnimationAssets>,
) {
    // Load or create meshes for each frame of the idle animation
    // For simplicity, we'll use the same mesh for all frames
    let idle_mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    animation_assets.idle_frames.push(idle_mesh.clone());

    // Load or create meshes for each frame of the walk animation
    // For simplicity, we'll use the same mesh but scaled differently for all frames
    let walk_mesh = meshes.add(Mesh::from(shape::Cube { size: 1.2 }));
    animation_assets.walk_frames.push(walk_mesh.clone());
}

// System to animate the character
fn animate_character_system(
    time: Res<Time>,
    mut query: Query<(&mut CharacterAnimation, &mut Handle<Mesh>)>,
    animation_assets: Res<AnimationAssets>,
) {
    for (mut animation, mut mesh_handle) in query.iter_mut() {
        animation.elapsed_time += time.delta_seconds();

        if animation.elapsed_time > animation.frame_duration {
            animation.elapsed_time = 0.0;
            animation.current_frame = (animation.current_frame + 1) % animation.total_frames;

            // Update the mesh handle to the next frame
            *mesh_handle = match animation.current_frame {
                0 => animation_assets.idle_frames[0].clone(),
                _ => animation_assets.walk_frames[0].clone(),
            };
        }
    }
}
