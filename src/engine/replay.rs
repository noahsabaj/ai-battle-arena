use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;
use crate::world::actions::Action;
use crate::units::Team;

#[derive(Serialize, Deserialize, Clone)]
pub struct ReplayFrame {
    pub turn: u32,
    pub actions: Vec<ReplayAction>,
    pub rng_seed: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ReplayAction {
    pub entity_id: u32,  // We'll use a stable ID instead of Entity
    pub team: Team,
    pub action: Action,
}

#[derive(Serialize, Deserialize)]
pub struct ReplayFile {
    pub initial_seed: u64,
    pub map_config: ReplayMapConfig,
    pub frames: Vec<ReplayFrame>,
    pub final_outcome: Option<GameOutcome>,
}

#[derive(Serialize, Deserialize)]
pub struct ReplayMapConfig {
    pub width: i32,
    pub height: i32,
    pub units_per_team: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum GameOutcome {
    RedWins,
    BlueWins,
    Draw,
}

#[derive(Resource)]
pub struct ReplayRecorder {
    initial_seed: u64,
    frames: Vec<ReplayFrame>,
    current_frame: Vec<ReplayAction>,
    recording: bool,
    entity_id_map: std::collections::HashMap<Entity, u32>,
    next_id: u32,
}

impl ReplayRecorder {
    pub fn new(seed: u64) -> Self {
        Self {
            initial_seed: seed,
            frames: Vec::new(),
            current_frame: Vec::new(),
            recording: true,
            entity_id_map: std::collections::HashMap::new(),
            next_id: 0,
        }
    }
    
    pub fn get_or_assign_id(&mut self, entity: Entity) -> u32 {
        *self.entity_id_map.entry(entity).or_insert_with(|| {
            let id = self.next_id;
            self.next_id += 1;
            id
        })
    }
    
    pub fn record_action(&mut self, entity: Entity, team: Team, action: Action) {
        if !self.recording {
            return;
        }
        
        let entity_id = self.get_or_assign_id(entity);
        self.current_frame.push(ReplayAction {
            entity_id,
            team,
            action,
        });
    }
    
    pub fn end_turn(&mut self, turn: u32, rng_seed: u64) {
        if !self.recording || self.current_frame.is_empty() {
            return;
        }
        
        self.frames.push(ReplayFrame {
            turn,
            actions: std::mem::take(&mut self.current_frame),
            rng_seed,
        });
    }
    
    pub fn save_replay(
        &self,
        path: &Path,
        map_config: ReplayMapConfig,
        outcome: Option<GameOutcome>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let replay = ReplayFile {
            initial_seed: self.initial_seed,
            map_config,
            frames: self.frames.clone(),
            final_outcome: outcome,
        };
        
        let json = serde_json::to_string_pretty(&replay)?;
        fs::write(path, json)?;
        Ok(())
    }
}

#[derive(Resource)]
pub struct ReplayPlayer {
    replay: ReplayFile,
    current_frame_index: usize,
    playing: bool,
}

impl ReplayPlayer {
    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let json = fs::read_to_string(path)?;
        let replay: ReplayFile = serde_json::from_str(&json)?;
        
        Ok(Self {
            replay,
            current_frame_index: 0,
            playing: false,
        })
    }
    
    pub fn get_next_frame(&mut self) -> Option<&ReplayFrame> {
        if self.current_frame_index < self.replay.frames.len() {
            let frame = &self.replay.frames[self.current_frame_index];
            self.current_frame_index += 1;
            Some(frame)
        } else {
            None
        }
    }
    
    pub fn reset(&mut self) {
        self.current_frame_index = 0;
    }
}

pub struct ReplayPlugin;

impl Plugin for ReplayPlugin {
    fn build(&self, app: &mut App) {
        // Initialize with a random seed for now
        let seed = chrono::Local::now().timestamp() as u64;
        app.insert_resource(ReplayRecorder::new(seed));
    }
}
