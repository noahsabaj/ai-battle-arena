use rand::{seq::SliceRandom, Rng, SeedableRng, rngs::SmallRng};
use bevy::prelude::Entity;

use crate::world::{actions::Action, hex_grid::HexCoord, unit::Team};
use super::{AiController, WorldSnapshot};

/// Very simple AI: 50 % chance to stay, otherwise pick a random axial neighbour.
pub struct RandomAi {
    rng: SmallRng,
}
impl RandomAi {
    pub fn new() -> Self { Self { rng: SmallRng::from_entropy() } }
}
impl Default for RandomAi { fn default() -> Self { Self::new() } }

impl AiController for RandomAi {
    fn decide(
        &mut self,
        _unit_id: Entity,
        _self_state: &crate::world::unit::Unit,
        world: &WorldSnapshot,
    ) -> Action {
        // get *this* unit’s position
        let (_, _, my_hex) = world.units
            .iter()
            .find(|(e, _, _)| *e == _unit_id)
            .copied()
            .unwrap();

        // 50 % chance to stay
        if self.rng.gen_bool(0.5) {
            return Action::Stay;
        }

        // random neighbour in axial coords
        const DIRS: [(i32,i32);6] = [(1,0),(1,-1),(0,-1),(-1,0),(-1,1),(0,1)];
        let (dq,dr) = *DIRS.choose(&mut self.rng).unwrap();
        Action::Move(dq,dr)
    }
}





