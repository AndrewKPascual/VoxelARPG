use bevy::prelude::*;
use bevy::ecs::system::CommandQueue;

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
}

// Define the effects that an item can have on the character
#[derive(Debug, Clone)]
pub struct ItemEffects {
    pub health_bonus: i32,
    pub attack_bonus: i32,
    pub defense_bonus: i32,
}

// Define the inventory to manage and store items
#[derive(Debug, Default, Clone)]
pub struct Inventory {
    pub items: Vec<Item>,
}

// Define the equipment system to allow characters to equip items
#[derive(Debug, Default, Clone)]
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
            .add_system(add_item_system)
            .add_system(remove_item_system)
            .add_system(use_item_system);
    }
}

// System to add items to the inventory
fn add_item_system(
    mut commands: Commands,
    mut query: Query<&mut Inventory>,
    mut command_queue: ResMut<CommandQueue>,
    // Additional parameters for the system would be defined here
) {
    // Logic for adding items to the inventory would be implemented here
    // This is a placeholder example of adding an item to the first inventory found
    for mut inventory in query.iter_mut() {
        let item = Item {
            name: "Mystic Hat".to_string(),
            item_type: ItemType::Hat,
            effects: ItemEffects {
                health_bonus: 5,
                attack_bonus: 2,
                defense_bonus: 3,
            },
        };
        inventory.items.push(item);
        break; // Only add to the first inventory for this example
    }
}

// System to remove items from the inventory
fn remove_item_system(
    mut commands: Commands,
    mut query: Query<&mut Inventory>,
    mut command_queue: ResMut<CommandQueue>,
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
    mut commands: Commands,
    mut query: Query<(&mut Inventory, &mut Equipment)>,
    mut command_queue: ResMut<CommandQueue>,
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
