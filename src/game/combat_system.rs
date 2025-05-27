use bevy::prelude::*;
use crate::units::{Unit, HexPosition, Dead};
use crate::units::movement::hex_distance;

#[derive(Event)]
pub struct CombatEvent {
    pub attacker: Entity,
    pub defender: Entity,
    pub damage: f32,
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CombatEvent>()
            .add_systems(Update, resolve_combat);
    }
}

pub fn check_combat(
    units: &Query<(Entity, &Unit, &HexPosition), Without<Dead>>,
) -> Vec<(Entity, Entity)> {
    let mut combat_pairs = Vec::new();
    let snapshot: Vec<_> = units.iter().collect();
    
    for i in 0..snapshot.len() {
        for j in (i + 1)..snapshot.len() {
            let (e1, u1, p1) = snapshot[i];
            let (e2, u2, p2) = snapshot[j];
            
            if u1.team != u2.team && hex_distance(p1.coord, p2.coord) <= 1 {
                combat_pairs.push((e1, e2));
            }
        }
    }
    
    combat_pairs
}

fn resolve_combat(
    mut commands: Commands,
    mut combat_events: EventReader<CombatEvent>,
    mut units: Query<&mut Unit>,
) {
    for event in combat_events.read() {
        // Apply damage to defender
        if let Ok(mut defender_unit) = units.get_mut(event.defender) {
            defender_unit.health -= event.damage;
            
            if defender_unit.health <= 0.0 {
                commands.entity(event.defender).insert(Dead);
                println!("     {} {:?} Fighter defeated!", 
                    defender_unit.team.tag(), defender_unit.team);
            }
        }
    }
}
