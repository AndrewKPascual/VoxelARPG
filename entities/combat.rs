use bevy::prelude::*;
use bevy::ecs::schedule::ShouldRun;

// Components for health and attack capabilities
#[derive(Component)]
pub struct Health {
    pub current: i32, // Current health of the entity
    pub max: i32,     // Maximum health of the entity
}

#[derive(Component)]
pub struct Attack {
    pub damage: i32,  // Damage the entity can inflict
}

// Component to mark entities as enemies for AI targeting
#[derive(Component)]
pub struct Enemy;

// Plugin to set up combat-related systems and resources
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(attack_system)
            .add_system(health_system)
            .add_system(enemy_ai_system);
    }
}

// System to handle attacks between entities
fn attack_system(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Attack)>,
    target_query: Query<(Entity, &Transform, &mut Health)>
) {
    for (attacker_entity, attacker_transform, attacker) in query.iter() {
        for (target_entity, target_transform, mut target_health) in target_query.iter_mut() {
            // Check if the target is within attack range, for simplicity, we'll assume all entities have a range of 1.0
            if attacker_transform.translation.distance(target_transform.translation) < 1.0 {
                // Apply damage to the target's health
                target_health.current -= attacker.damage;

                // If the target's health falls below zero, despawn it
                if target_health.current <= 0 {
                    commands.entity(target_entity).despawn();
                }
            }
        }
    }
}

// System to update health, could include regeneration or other effects
fn health_system(
    mut query: Query<&mut Health>
) {
    for mut health in query.iter_mut() {
        // For now, we'll just clamp the health to the max value
        health.current = health.current.min(health.max);
    }
}

// System for enemy AI behavior
fn enemy_ai_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &Enemy, &Attack)>,
    player_query: Query<&Transform, With<Player>>,
) {
    for (enemy_entity, mut enemy_transform, enemy, attack) in query.iter_mut() {
        if let Some(player_transform) = player_query.iter().next() {
            // Move towards the player
            let direction = (player_transform.translation - enemy_transform.translation).normalize();
            enemy_transform.translation += direction * time.delta_seconds() * 2.0; // Enemy speed

            // Check if the enemy is close enough to attack the player
            if enemy_transform.translation.distance(player_transform.translation) < 1.0 {
                // Perform attack
                commands.entity(enemy_entity).insert(Attack { damage: attack.damage });
            }
        }
    }
}

// Conditionally run systems based on whether the player exists
fn run_if_player_exists(
    player_query: Query<&Player>,
) -> ShouldRun {
    if player_query.iter().next().is_some() {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}
