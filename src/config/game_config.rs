use serde::{Deserialize, Serialize};
use std::fs;
use bevy::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct GameConfig {
    pub game: GameSettings,
    pub combat: CombatSettings,
    pub units: UnitSettings,
    pub performance: PerformanceSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSettings {
    pub tick_rate: f32,
    pub map_width: i32,
    pub map_height: i32,
    pub units_per_team: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatSettings {
    pub base_damage: f32,
    pub attack_range: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitSettings {
    pub base_health: f32,
    pub movement_range: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSettings {
    pub target_tps: u32,
    pub enable_metrics: bool,
    pub metrics_interval: f32,
}

impl GameConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string("configs/game_config.toml")?;
        let config: GameConfig = toml::from_str(&config_str)?;
        Ok(config)
    }
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            game: GameSettings {
                tick_rate: 0.5,
                map_width: 20,
                map_height: 15,
                units_per_team: 6,
            },
            combat: CombatSettings {
                base_damage: 35.0,
                attack_range: 1,
            },
            units: UnitSettings {
                base_health: 100.0,
                movement_range: 1,
            },
            performance: PerformanceSettings {
                target_tps: 1000,
                enable_metrics: true,
                metrics_interval: 5.0,
            },
        }
    }
}

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        let config = GameConfig::load().unwrap_or_else(|e| {
            warn!("Failed to load config: {}. Using defaults.", e);
            GameConfig::default()
        });
        
        app.insert_resource(config);
    }
}
