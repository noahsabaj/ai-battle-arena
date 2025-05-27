use bevy::prelude::*;
use crate::config::{SimulationConfig, SimulationMode};

/// Marker for systems that should run in all modes
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CoreGameplay;

/// Marker for visual-only systems
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct VisualOnly;

/// Marker for headless-only systems
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct HeadlessOnly;

pub struct UnifiedGamePlugin;

impl Plugin for UnifiedGamePlugin {
    fn build(&self, app: &mut App) {
        let sim_config = app.world.resource::<SimulationConfig>();
        let is_headless = sim_config.modes.default == SimulationMode::Headless;
        
        // Configure core systems that run in all modes
        app.configure_sets(
            Update,
            (
                CoreGameplay,
                VisualOnly.run_if(not(is_headless_mode)),
                HeadlessOnly.run_if(is_headless_mode),
            )
        );
    }
}

pub fn is_headless_mode(sim_config: Res<SimulationConfig>) -> bool {
    sim_config.modes.default == SimulationMode::Headless
}
