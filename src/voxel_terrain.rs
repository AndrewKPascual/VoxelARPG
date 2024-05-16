use bevy::{
    prelude::*,
    render::{
        mesh::{Mesh, Indices, PrimitiveTopology}, // Corrected import path for PrimitiveTopology
        render_asset::RenderAssetUsages,
    },
    pbr::PbrBundle,
};

// Removed unused imports

// Define the voxel terrain
pub struct VoxelTerrain {
    pub size: Vec3,
    pub voxel_size: f32,
}

// Implement the Resource trait for VoxelTerrain
impl Resource for VoxelTerrain {}

impl VoxelTerrain {
    // Initialize the voxel terrain with a given size and voxel size
    pub fn new(size: Vec3, voxel_size: f32) -> Self {
        VoxelTerrain { size, voxel_size }
    }

    // Generate the voxel terrain
    pub fn generate(
        &self,
        commands: &mut Commands,
        materials: &mut Assets<StandardMaterial>,
        meshes: &mut Assets<Mesh>,
    ) {
        let half_size = self.size / 2.0;
        for x in (-half_size.x as i32)..(half_size.x as i32) {
            for y in (-half_size.y as i32)..(half_size.y as i32) {
                for z in (-half_size.z as i32)..(half_size.z as i32) {
                    let voxel_position = Vec3::new(x as f32, y as f32, z as f32) * self.voxel_size;
                    // Convert the color to a StandardMaterial directly without using into()
                    let voxel_material = materials.add(StandardMaterial {
                        base_color: Color::rgb(0.4, 0.7, 0.3),
                        ..Default::default()
                    });
                    // Create a new cuboid mesh with the specified size
                    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
                    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![[-0.5, -0.5, 0.5], [0.5, -0.5, 0.5], [0.5, 0.5, 0.5], [-0.5, 0.5, 0.5], [-0.5, -0.5, -0.5], [0.5, -0.5, -0.5], [0.5, 0.5, -0.5], [-0.5, 0.5, -0.5]]);
                    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, -1.0], [0.0, 0.0, -1.0], [0.0, 0.0, -1.0], [0.0, 0.0, -1.0]]);
                    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0], [1.0, 0.0], [0.0, 0.0], [0.0, 1.0], [1.0, 1.0]]);
                    mesh.insert_indices(Indices::U32(vec![0, 2, 1, 0, 3, 2, 1, 2, 5, 2, 6, 5, 5, 6, 4, 6, 7, 4, 4, 7, 0, 7, 3, 0, 3, 7, 2, 7, 6, 2, 4, 0, 1, 4, 1, 5]));
                    let voxel_mesh = meshes.add(mesh);
                    // Spawn the entity with the mesh and material components
                    commands.spawn(PbrBundle {
                        mesh: voxel_mesh.clone(),
                        material: voxel_material.clone(),
                        transform: Transform::from_translation(voxel_position),
                        ..Default::default()
                    });

                    // The generated assets are managed by Bevy's asset system and do not need to be serialized to files
                    // Removed the code that incorrectly attempted to serialize and write handles to files
                    // Removed the code that watched the assets directory for changes
                }
            }
        }
    }
}
