use bevy::prelude::*;
use bevy::sprite::Sprite;
use crate::units::{Unit, Dead};

#[derive(Component)]
pub struct HealthBar;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_health_bars,
            mark_dead_units,
            remove_dead_units.after(mark_dead_units),
        ));
    }
}

pub fn mark_dead_units(
    mut commands: Commands,
    units: Query<(Entity, &Unit), Without<Dead>>,
) {
    for (entity, unit) in &units {
        if unit.health <= 0.0 {
            commands.entity(entity).insert(Dead);
        }
    }
}

pub fn update_health_bars(
    mut bars: Query<(&Parent, &mut Sprite), With<HealthBar>>,
    units: Query<&Unit>,
) {
    for (parent, mut sprite) in &mut bars {
        if let Ok(unit) = units.get(parent.get()) {
            let ratio = (unit.health / unit.max_health).clamp(0.0, 1.0);
            
            // Scale width
            if let Some(size) = sprite.custom_size.as_mut() {
                size.x = 60.0 * ratio;
            }
            
            // Color gradient
            sprite.color = if ratio > 0.5 {
                Color::GREEN
            } else if ratio > 0.25 {
                Color::rgb(1.0, 0.9, 0.0)
            } else {
                Color::RED
            };
        }
    }
}

pub fn remove_dead_units(
    mut commands: Commands,
    dead_query: Query<Entity, With<Dead>>,
) {
    for entity in &dead_query {
        commands.entity(entity).despawn_recursive();
    }
}
