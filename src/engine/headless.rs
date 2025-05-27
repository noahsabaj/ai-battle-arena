use bevy::prelude::*;
use crate::config::SimulationConfig;

pub struct HeadlessPlugin;

impl Plugin for HeadlessPlugin {
    fn build(&self, app: &mut App) {
        let sim_config = SimulationConfig::load().unwrap_or_default();
        
        if sim_config.modes.default == crate::config::SimulationMode::Headless {
            // Configure fixed timestep for headless mode
            let timestep_hz = sim_config.headless.timestep_hz as f64;
            
            // Set fixed timestep
            app.insert_resource(Time::<Fixed>::from_hz(timestep_hz));
            
            info!("Configured HEADLESS mode at {} Hz", sim_config.headless.timestep_hz);
        }
    }
}

/// Marker component for systems that should only run in visual mode
#[derive(Component)]
pub struct VisualOnly;

/// Marker component for systems that should only run in headless mode
#[derive(Component)]
pub struct HeadlessOnly;

/// Run condition for visual-only systems
pub fn in_visual_mode(sim_config: Res<SimulationConfig>) -> bool {
    sim_config.modes.default == crate::config::SimulationMode::Visual
}

/// Run condition for headless-only systems  
pub fn in_headless_mode(sim_config: Res<SimulationConfig>) -> bool {
    sim_config.modes.default == crate::config::SimulationMode::Headless
}
