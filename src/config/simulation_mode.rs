use serde::{Deserialize, Serialize};
use std::fs;
use bevy::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct SimulationConfig {
    pub modes: ModeSettings,
    pub visual: VisualModeSettings,
    pub headless: HeadlessModeSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeSettings {
    pub default: SimulationMode,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SimulationMode {
    Visual,
    Headless,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualModeSettings {
    pub enable_rendering: bool,
    pub enable_ui: bool,
    pub vsync: bool,
    pub frame_cap: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadlessModeSettings {
    pub enable_rendering: bool,
    pub enable_ui: bool,
    pub fixed_timestep: bool,
    pub timestep_hz: u32,
}

impl SimulationConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string("configs/simulation_modes.toml")?;
        let config: SimulationConfig = toml::from_str(&config_str)?;
        Ok(config)
    }
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            modes: ModeSettings {
                default: SimulationMode::Visual,
            },
            visual: VisualModeSettings {
                enable_rendering: true,
                enable_ui: true,
                vsync: true,
                frame_cap: 60,
            },
            headless: HeadlessModeSettings {
                enable_rendering: false,
                enable_ui: false,
                fixed_timestep: true,
                timestep_hz: 1000,
            },
        }
    }
}
