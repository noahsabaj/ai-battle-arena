use bevy::prelude::Entity;
use crate::world::{
    actions::Action,
    hex_grid::HexCoord,
};
use crate::units::{Unit, Team};

/// Read-only snapshot the AI may inspect each turn.
#[derive(Clone)]
pub struct WorldSnapshot {
    pub units: Vec<(Entity, Team, HexCoord)>,
}

/// Behaviour contract for any in-game AI.
pub trait AiController: Send + Sync {
    /// Decide what *one* unit should do this turn.
    fn decide(
        &mut self,
        unit_id: Entity,
        self_state: &Unit,
        world: &WorldSnapshot,
    ) -> Action;
}
