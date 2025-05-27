use bevy::prelude::*;
use bevy::ecs::query::QueryIter;
use std::collections::HashMap;
use crate::world::HexCoord;

/// Spatial index for fast proximity queries
#[derive(Resource, Default)]
pub struct SpatialIndex {
    grid: HashMap<(i32, i32), Vec<Entity>>,
}

impl SpatialIndex {
    pub fn clear(&mut self) {
        self.grid.clear();
    }
    
    pub fn insert(&mut self, coord: HexCoord, entity: Entity) {
        self.grid.entry((coord.q, coord.r))
            .or_insert_with(Vec::new)
            .push(entity);
    }
    
    pub fn get_neighbors(&self, coord: HexCoord, range: i32) -> Vec<Entity> {
        let mut neighbors = Vec::new();
        
        for q in -range..=range {
            for r in -range..=range {
                let check_q = coord.q + q;
                let check_r = coord.r + r;
                
                if let Some(entities) = self.grid.get(&(check_q, check_r)) {
                    neighbors.extend(entities);
                }
            }
        }
        
        neighbors
    }
}

pub struct SpatialIndexPlugin;

impl Plugin for SpatialIndexPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SpatialIndex>()
            .add_systems(PreUpdate, update_spatial_index);
    }
}

fn update_spatial_index(
    mut index: ResMut<SpatialIndex>,
    units: Query<(Entity, &crate::units::HexPosition), Without<crate::units::Dead>>,
) {
    index.clear();
    for (entity, pos) in &units {
        index.insert(pos.coord, entity);
    }
}
