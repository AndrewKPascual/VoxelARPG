use bevy::{
    prelude::*,
    ecs::schedule::CoreStage,
};

// Define the types of items available in the game
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ItemType {
    Hat,
    Weapon,
    Armor,
}

// Define the properties of an item
#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub item_type: ItemType,
    pub effects: ItemEffects,
    pub mesh_handle: Handle<Mesh>, // Added field for the mesh handle
    pub material_handle: Handle<StandardMaterial>, // Added field for the material handle
}

// Define the effects that an item can have on the character
#[derive(Debug, Clone)]
pub struct ItemEffects {
    pub health_bonus: i32,
    pub attack_bonus: i32,
    pub defense_bonus: i32,
}

// Define the inventory to manage and store items
#[derive(Debug, Default, Clone, Component)]
pub struct Inventory {
    pub items: Vec<Item>,
}

// Define the equipment system to allow characters to equip items
#[derive(Debug, Default, Clone, Component)]
pub struct Equipment {
    pub hat: Option<Item>,
    pub weapon: Option<Item>,
    pub armor: Option<Item>,
}

// Plugin to set up item systems
pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set_to_stage(CoreStage::Update, SystemSet::new()
                .with_system(add_item_system)
                .with_system(remove_item_system)
                .with_system(use_item_system));
    }
}

// System to add items to the inventory
fn add_item_system(
    mut query: Query<&mut Inventory>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Logic for adding items to the inventory would be implemented here
    // This is a placeholder example of adding an item to the first inventory found
    for mut inventory in query.iter_mut() {
        let hat_material_handle = materials.add(StandardMaterial {
            base_color: Color::rgb(0.1, 0.1, 0.1),
            ..Default::default()
        });
        // Corrected the creation of the Cuboid mesh with the half_size field
        let hat_mesh_handle = meshes.add(Mesh::from(Cuboid { half_size: Vec3::new(0.25, 0.25, 0.25) }));

        let item = Item {
            name: "Mystic Hat".to_string(),
            item_type: ItemType::Hat,
            effects: ItemEffects {
                health_bonus: 5,
                attack_bonus: 2,
                defense_bonus: 3,
            },
            mesh_handle: hat_mesh_handle, // Assign the mesh handle
            material_handle: hat_material_handle, // Assign the material handle
        };
        inventory.items.push(item);
        break; // Only add to the first inventory for this example
    }
}

// System to remove items from the inventory
fn remove_item_system(
    mut query: Query<&mut Inventory>,
    // Additional parameters for the system would be defined here
) {
    // Logic for removing items from the inventory would be implemented here
    // This is a placeholder example of removing the last item from the first inventory found
    for mut inventory in query.iter_mut() {
        inventory.items.pop();
        break; // Only remove from the first inventory for this example
    }
}

// System to use items and apply their effects
fn use_item_system(
    mut query: Query<(&mut Inventory, &mut Equipment)>,
    // Additional parameters for the system would be defined here
) {
    // Logic for using items and applying their effects would be implemented here
    // This is a placeholder example of equipping the first item from the inventory
    for (mut inventory, mut equipment) in query.iter_mut() {
        if let Some(item) = inventory.items.get(0).cloned() {
            match item.item_type {
                ItemType::Hat => equipment.hat = Some(item),
                ItemType::Weapon => equipment.weapon = Some(item),
                ItemType::Armor => equipment.armor = Some(item),
            }
        }
        break; // Only equip to the first character for this example
    }
}
