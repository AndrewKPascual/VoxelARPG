use bevy::prelude::*;
use bevy::math::Vec3;
use rand::Rng; // Assuming rand is in the dependencies

// Define components for combat-related properties
pub struct Health(pub u32);
pub struct Attack(pub u32);
pub struct Defense(pub u32);

// Additional components for AI
pub struct Enemy;
pub struct Player;
pub struct Velocity(pub Vec3);

// Plugin to set up combat systems
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(attack_system)
            .add_system(health_system)
            .add_system(enemy_ai_system);
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

// System to handle enemy AI
fn enemy_ai_system(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &Enemy, &mut Transform, &mut Velocity)>,
    player_query: Query<(&Player, &Transform)>,
) {
    if let Some((_, player_transform)) = player_query.iter().next() {
        for (entity, _, mut transform, mut velocity) in enemy_query.iter_mut() {
            let mut rng = rand::thread_rng();
            let player_position = player_transform.translation;
            let direction_to_player = player_position - transform.translation;
            let distance_to_player = direction_to_player.length();

            // Simple AI: Move towards the player if they are within a certain range
            if distance_to_player < 10.0 {
                velocity.0 = direction_to_player.normalize() * rng.gen_range(0.5..1.5);
            } else {
                // Idle or random movement
                velocity.0 = Vec3::new(rng.gen_range(-1.0..1.0), 0.0, rng.gen_range(-1.0..1.0)).normalize() * rng.gen_range(0.5..1.5);
            }

            // Update the enemy's position
            transform.translation += velocity.0 * TIME_STEP; // TIME_STEP should be the time elapsed since the last update
            println!("Enemy {:?} moves to {:?}", entity, transform.translation);
        }
    }
}
