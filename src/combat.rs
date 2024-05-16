use bevy::prelude::*;

// Define components for combat-related properties
pub struct Health(pub u32);
pub struct Attack(pub u32);

// Plugin to set up combat systems
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(attack_system)
            .add_system(health_system);
    }
}

// System to handle attacks
fn attack_system(
    mut commands: Commands,
    query: Query<(Entity, &Attack, &Transform), With<Health>>,
) {
    for (entity, attack, transform) in query.iter() {
        // Placeholder logic for attacking
        println!("Entity {:?} attacks with power {}", entity, attack.0);
    }
}

// System to handle health updates
fn health_system(
    mut commands: Commands,
    query: Query<(Entity, &mut Health)>,
) {
    for (entity, mut health) in query.iter_mut() {
        // Placeholder logic for health reduction
        if health.0 > 0 {
            health.0 -= 1;
            println!("Entity {:?} takes damage, health is now {}", entity, health.0);
        } else {
            // Entity is out of health, remove it
            commands.entity(entity).despawn();
            println!("Entity {:?} has been defeated", entity);
        }
    }
}
