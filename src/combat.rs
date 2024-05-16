use bevy::prelude::*;

// Define components for combat-related properties
pub struct Health(pub u32);
pub struct Attack(pub u32);
pub struct Defense(pub u32);

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
    mut query: Query<(Entity, &Attack, &mut Health, &Defense)>,
) {
    for (entity, attack, mut health, defense) in query.iter_mut() {
        // Simple attack logic: if attack power is greater than defense, reduce health
        if attack.0 > defense.0 {
            health.0 = health.0.saturating_sub(attack.0 - defense.0);
            println!("Entity {:?} attacks with power {}, health is now {}", entity, attack.0, health.0);
        }
    }
}

// System to handle health updates
fn health_system(
    mut commands: Commands,
    query: Query<(Entity, &mut Health)>,
) {
    for (entity, mut health) in query.iter_mut() {
        // Check if the entity is out of health and remove it
        if health.0 == 0 {
            commands.entity(entity).despawn();
            println!("Entity {:?} has been defeated", entity);
        }
    }
}
