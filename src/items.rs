use bevy::prelude::*;

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
        // Systems to manage items, inventory, and equipment would be added here
    }
}
