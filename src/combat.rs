use bevy::prelude::*;
use bevy::math::Vec3;
use rand::Rng; // Assuming rand is in the dependencies

// Define components for combat-related properties
#[derive(Component)]
pub struct Health(pub u32);
#[derive(Component)]
pub struct Attack(pub u32);
#[derive(Component)]
pub struct Defense(pub u32);

// Additional components for AI
#[derive(Component)]
pub struct Enemy;
#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct Velocity(pub Vec3);

// Components for item effects
#[derive(Component)]
pub struct ItemEffects {
    pub health_bonus: i32,
    pub attack_bonus: i32,
    pub defense_bonus: i32,
}

// Plugin to set up combat systems
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(attack_system)
            .add_systems(health_system)
            .add_systems(enemy_ai_system);
    }
}

// System to handle attacks
fn attack_system(
    mut commands: Commands,
    _time: Res<Time>,
    mut query: Query<(Entity, &Attack, &mut Health, &Defense, Option<&ItemEffects>)>,
) {
    for (entity, attack, mut health, defense, item_effects) in query.iter_mut() {
        // Calculate total attack power with item effects
        let total_attack = attack.0 + item_effects.map_or(0, |effects| effects.attack_bonus as u32);

        // Simple attack logic: if total attack power is greater than defense, reduce health
        if total_attack > defense.0 {
            health.0 = health.0.saturating_sub(total_attack - defense.0);
            println!("Entity {:?} attacks with power {}, health is now {}", entity, total_attack, health.0);
        }
    }
}

// System to handle health updates
fn health_system(
    mut commands: Commands,
    _time: Res<Time>,
    mut query: Query<(Entity, &mut Health, Option<&ItemEffects>)>,
) {
    for (entity, mut health, item_effects) in query.iter_mut() {
        // Apply health bonus from item effects
        let health_bonus = item_effects.map_or(0, |effects| effects.health_bonus as u32);
        health.0 += health_bonus;

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
    time: Res<Time>,
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
            transform.translation += velocity.0 * time.delta_seconds(); // Use delta_seconds for time step
            println!("Enemy {:?} moves to {:?}", entity, transform.translation);
        }
    }
}
