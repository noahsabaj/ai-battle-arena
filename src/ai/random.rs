use rand::{seq::SliceRandom, Rng, SeedableRng, rngs::SmallRng};
use bevy::prelude::Entity;

use crate::world::actions::Action;
use crate::units::Unit;
use super::{AiController, WorldSnapshot};

/// Improved AI: Moves toward nearest enemy, with some randomness
pub struct RandomAi {
    rng: SmallRng,
}

impl RandomAi {
    pub fn new() -> Self { 
        Self { rng: SmallRng::from_entropy() } 
    }
}

impl Default for RandomAi { 
    fn default() -> Self { 
        Self::new() 
    }
}

impl AiController for RandomAi {
    fn decide(
        &mut self,
        unit_id: Entity,
        self_state: &Unit,
        world: &WorldSnapshot,
    ) -> Action {
        // Get this unit's position
        let (_, _, my_pos) = world.units
            .iter()
            .find(|(e, _, _)| *e == unit_id)
            .copied()
            .unwrap();

        // Find nearest enemy
        let enemies: Vec<_> = world.units
            .iter()
            .filter(|(_, team, _)| *team != self_state.team)
            .map(|(_, _, pos)| *pos)
            .collect();

        if enemies.is_empty() {
            return Action::Stay;
        }

        // Find closest enemy
        let nearest_enemy = enemies
            .iter()
            .min_by_key(|enemy_pos| {
                let dq = enemy_pos.q - my_pos.q;
                let dr = enemy_pos.r - my_pos.r;
                dq.abs() + dr.abs() + (dq + dr).abs()
            })
            .unwrap();

        // 30% chance to move randomly (exploration)
        if self.rng.gen_bool(0.3) {
            const DIRS: [(i32,i32);6] = [(1,0),(1,-1),(0,-1),(-1,0),(-1,1),(0,1)];
            let (dq,dr) = *DIRS.choose(&mut self.rng).unwrap();
            return Action::Move(dq,dr);
        }

        // Move toward nearest enemy
        let _dq = (nearest_enemy.q - my_pos.q).signum();
        let _dr = (nearest_enemy.r - my_pos.r).signum();

        // Valid hex moves
        const DIRS: [(i32,i32);6] = [(1,0),(1,-1),(0,-1),(-1,0),(-1,1),(0,1)];
        
        // Find move that gets us closest to target
        let best_move = DIRS
            .iter()
            .min_by_key(|(move_dq, move_dr)| {
                let new_q = my_pos.q + move_dq;
                let new_r = my_pos.r + move_dr;
                let dist_q = nearest_enemy.q - new_q;
                let dist_r = nearest_enemy.r - new_r;
                dist_q.abs() + dist_r.abs() + (dist_q + dist_r).abs()
            })
            .unwrap();

        Action::Move(best_move.0, best_move.1)
    }
}