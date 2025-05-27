use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use crate::units::{Unit, HexPosition, Team};
use crate::game::TurnState;
use std::fs;
use std::path::Path;

// Make Team serializable
impl Serialize for Team {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Team::Red => serializer.serialize_str("red"),
            Team::Blue => serializer.serialize_str("blue"),
        }
    }
}

impl<'de> Deserialize<'de> for Team {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "red" => Ok(Team::Red),
            "blue" => Ok(Team::Blue),
            _ => Err(serde::de::Error::custom("invalid team")),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GameSnapshot {
    pub turn: u32,
    pub time: f32,
    pub units: Vec<UnitSnapshot>,
}

#[derive(Serialize, Deserialize)]
pub struct UnitSnapshot {
    pub team: Team,
    pub health: f32,
    pub position: (i32, i32),
}

#[derive(Resource)]
pub struct StateRecorder {
    snapshots: Vec<GameSnapshot>,
    recording: bool,
}

impl StateRecorder {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
            recording: true,
        }
    }
    
    pub fn record_snapshot(
        &mut self,
        turn_state: &TurnState,
        units: Vec<(&Unit, &HexPosition)>,
    ) {
        if !self.recording {
            return;
        }
        
        let snapshot = GameSnapshot {
            turn: turn_state.turn,
            time: turn_state.time,
            units: units.into_iter()
                .map(|(unit, pos)| UnitSnapshot {
                    team: unit.team,
                    health: unit.health,
                    position: (pos.coord.q, pos.coord.r),
                })
                .collect(),
        };
        
        self.snapshots.push(snapshot);
    }
    
    pub fn save_to_file(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(&self.snapshots)?;
        fs::write(path, json)?;
        Ok(())
    }
}

pub struct StateSerializationPlugin;

impl Plugin for StateSerializationPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(StateRecorder::new())
            .add_systems(Update, record_game_state);
    }
}

fn record_game_state(
    turn_state: Res<TurnState>,
    mut recorder: ResMut<StateRecorder>,
    units: Query<(&Unit, &HexPosition), Without<crate::units::Dead>>,
) {
    // Record every 10 turns to avoid huge files
    if turn_state.turn % 10 == 0 {
        let unit_data: Vec<_> = units.iter().collect();
        recorder.record_snapshot(&turn_state, unit_data);
    }
}
