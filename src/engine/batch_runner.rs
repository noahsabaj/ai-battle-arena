use bevy::prelude::*;
use bevy::app::{AppExit, ScheduleRunnerPlugin};
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use std::sync::mpsc::channel;
use crate::config::{GameConfig, SimulationConfig, SimulationMode};

#[derive(Clone)]
pub struct BatchGameConfig {
    pub game_config: GameConfig,
    pub sim_config: SimulationConfig,
    pub num_games: usize,
    pub parallel_games: usize,
}

pub struct GameResult {
    pub game_id: usize,
    pub winner: GameOutcome,
    pub total_turns: u32,
    pub duration_secs: f64,
    pub final_tps: f64,
}

#[derive(Clone, Debug)]
pub enum GameOutcome {
    RedWins,
    BlueWins,
    Draw,
}

pub struct BatchRunner {
    config: BatchGameConfig,
}

impl BatchRunner {
    pub fn new(config: BatchGameConfig) -> Self {
        Self { config }
    }
    
    pub fn run_batch(&self) -> Vec<GameResult> {
        let mut results = Vec::new();
        let (tx, rx) = channel();
        let mut handles = Vec::new();
        
        // Create thread pool
        let num_threads = self.config.parallel_games.min(self.config.num_games);
        let games_per_thread = self.config.num_games / num_threads;
        let remainder = self.config.num_games % num_threads;
        
        println!("Starting batch run: {} games across {} threads", 
            self.config.num_games, num_threads);
        
        for thread_id in 0..num_threads {
            let sender = tx.clone();
            let config = self.config.clone();
            
            let thread_games = if thread_id < remainder {
                games_per_thread + 1
            } else {
                games_per_thread
            };
            
            let game_start_id = thread_id * games_per_thread + thread_id.min(remainder);
            
            let handle = thread::spawn(move || {
                for i in 0..thread_games {
                    let game_id = game_start_id + i;
                    println!("Thread {}: Starting game {}", thread_id, game_id);
                    let result = run_single_game(game_id, &config);
                    println!("Thread {}: Completed game {} - {:?}", thread_id, game_id, result.winner);
                    sender.send(result).ok();
                }
            });
            
            handles.push(handle);
        }
        
        // Drop our sender so receiver knows when all threads are done
        drop(tx);
        
        // Collect results as they come in
        let mut completed = 0;
        while let Ok(result) = rx.recv() {
            completed += 1;
            println!("Progress: {}/{} games completed", completed, self.config.num_games);
            results.push(result);
        }
        
        // Wait for all threads
        for handle in handles {
            handle.join().ok();
        }
        
        results
    }
}

fn run_single_game(game_id: usize, config: &BatchGameConfig) -> GameResult {
    let start_time = Instant::now();
    
    // Create a minimal Bevy app for headless simulation
    let mut app = App::new();
    
    // Set up headless mode
    let timestep = 1.0 / config.sim_config.headless.timestep_hz as f64;
    app.add_plugins(
        MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_secs_f64(timestep)
        ))
    );
    
    // Add game plugins
    app.add_plugins((
        crate::config::ConfigPlugin,
        crate::engine::HeadlessPlugin,
        crate::world::hex_grid::HexGridPlugin,
        crate::units::SpawningPlugin,
        crate::units::MovementPlugin,
        crate::units::HealthPlugin,
        crate::game::TurnManagerPlugin,
        crate::game::CombatPlugin,
        crate::game::VictoryPlugin,
        crate::ai::AiIntegrationPlugin,
        crate::performance::MetricsPlugin,
    ));
    
    // Insert configs
    app.insert_resource(config.sim_config.clone());
    app.insert_resource(config.game_config.clone());
    
    // Add game end detection
    app.insert_resource(BatchGameResult {
        game_id,
        outcome: None,
        turns: 0,
        completed: false,
    });
    app.insert_resource(config.game_config.clone());
    
    
    
    app.add_systems(Update, check_game_end);
    
    // Run until game ends
    app.run();
    
    // Extract results
    let batch_result = app.world.resource::<BatchGameResult>();
    let duration = start_time.elapsed().as_secs_f64();
    let final_tps = batch_result.turns as f64 / duration;
    
    GameResult {
        game_id,
        winner: batch_result.outcome.clone().unwrap_or(GameOutcome::Draw),
        total_turns: batch_result.turns,
        duration_secs: duration,
        final_tps,
    }
}

#[derive(Resource)]
struct BatchGameResult {
    game_id: usize,
    outcome: Option<GameOutcome>,
    turns: u32,
    completed: bool,
}

fn check_game_end(
    game_over: Res<crate::game::GameOver>,
    turn_state: Res<crate::game::TurnState>,
    units: Query<&crate::units::Unit>,
    mut result: ResMut<BatchGameResult>,
    mut exit: EventWriter<AppExit>,
) {
    if !result.completed {
        result.turns = turn_state.turn;
        
        if game_over.0 {
            result.completed = true;
            
            // Count units to determine winner
            let (red, blue) = units.iter().fold((0, 0), |(r, b), unit| {
                match unit.team {
                    crate::units::Team::Red => (r + 1, b),
                    crate::units::Team::Blue => (r, b + 1),
                }
            });
            
            result.outcome = Some(match (red, blue) {
                (0, 0) => GameOutcome::Draw,
                (0, _) => GameOutcome::BlueWins,
                (_, 0) => GameOutcome::RedWins,
                _ => GameOutcome::Draw,
            });
            
            exit.send(AppExit);
        }
    }
}

pub struct BatchRunnerPlugin;

impl Plugin for BatchRunnerPlugin {
    fn build(&self, _app: &mut App) {
        // This plugin is mainly for organizing batch functionality
    }
}

// Command line interface for batch running
pub fn run_batch_games(num_games: usize, parallel: usize) {
    println!("AI Battle Arena - Batch Mode");
    println!("===========================");
    
    // Load configs
    let game_config = GameConfig::load().unwrap_or_default();
    let mut sim_config = SimulationConfig::load().unwrap_or_default();
    sim_config.modes.default = SimulationMode::Headless; // Force headless
    
    let config = BatchGameConfig {
        game_config,
        sim_config,
        num_games,
        parallel_games: parallel,
    };
    
    let runner = BatchRunner::new(config);
    let start = Instant::now();
    let results = runner.run_batch();
    let total_duration = start.elapsed();
    
    // Print summary
    let mut red_wins = 0;
    let mut blue_wins = 0;
    let mut draws = 0;
    let mut total_tps = 0.0;
    let mut total_turns = 0;
    
    for result in &results {
        match result.winner {
            GameOutcome::RedWins => red_wins += 1,
            GameOutcome::BlueWins => blue_wins += 1,
            GameOutcome::Draw => draws += 1,
        }
        total_tps += result.final_tps;
        total_turns += result.total_turns;
    }
    
    println!("\n========== BATCH RESULTS ==========");
    println!("Total Games: {}", num_games);
    println!("Total Duration: {:.2}s", total_duration.as_secs_f64());
    println!("Games/Second: {:.2}", num_games as f64 / total_duration.as_secs_f64());
    println!();
    println!("Red Wins: {} ({:.1}%)", red_wins, red_wins as f64 / num_games as f64 * 100.0);
    println!("Blue Wins: {} ({:.1}%)", blue_wins, blue_wins as f64 / num_games as f64 * 100.0);
    println!("Draws: {} ({:.1}%)", draws, draws as f64 / num_games as f64 * 100.0);
    println!();
    println!("Average TPS: {:.2}", total_tps / results.len() as f64);
    println!("Average Turns/Game: {:.1}", total_turns as f64 / results.len() as f64);
    println!("Total Turns Simulated: {}", total_turns);
    println!("===================================");
}




