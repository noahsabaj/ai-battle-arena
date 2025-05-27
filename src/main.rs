use bevy::prelude::*;
use bevy::app::{AppExit, ScheduleRunnerPlugin};
use std::time::Duration;

mod engine;
mod world;
mod ai;
mod utils;
mod game;
mod units;
mod ui;
mod config;
mod performance;

use world::hex_grid::HexGridPlugin;
use units::{SpawningPlugin, MovementPlugin, HealthPlugin};
use game::{TurnManagerPlugin, CombatPlugin, VictoryPlugin};
use ui::{HudPlugin, OverlayPlugin};
use ai::AiIntegrationPlugin;
use config::{ConfigPlugin, SimulationConfig, SimulationMode, GameConfig};
use performance::{MetricsPlugin, ProfilerPlugin};
use engine::{HeadlessPlugin, StateSerializationPlugin, ReplayPlugin};

fn main() {
    // Check for batch mode
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "batch" {
        let num_games = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(100);
        let parallel = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(4);
        engine::run_batch_games(num_games, parallel);
        return;
    }
    
    // Load simulation config to determine mode
    let sim_config = SimulationConfig::load().unwrap_or_default();
    let is_headless = sim_config.modes.default == SimulationMode::Headless;
    
    let mut app = App::new();
    
    // Configure plugins based on mode
    if is_headless {
        println!("Starting AI Battle Arena in HEADLESS mode");
        
        // For headless, use MinimalPlugins with ScheduleRunner
        let timestep = 1.0 / sim_config.headless.timestep_hz as f64;
        app.add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
                Duration::from_secs_f64(timestep)
            ))
        );
        
        // Add our headless configuration
        app.add_plugins(HeadlessPlugin);
    } else {
        println!("Starting AI Battle Arena in VISUAL mode");
        app.add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "AI Battle Arena".to_string(),
                    resolution: (1280., 720.).into(),
                    ..default()
                }),
                ..default()
            })
        );
    }
    
    // Add core game plugins (always needed)
    app.add_plugins((
        // Configuration
        ConfigPlugin,
        
        // Core game
        HexGridPlugin,
        SpawningPlugin,
        MovementPlugin,
        HealthPlugin,
        TurnManagerPlugin,
        CombatPlugin,
        VictoryPlugin,
        AiIntegrationPlugin,
        
        // Performance monitoring
        MetricsPlugin,
        ProfilerPlugin,
        
        // State management
        StateSerializationPlugin,
        ReplayPlugin,
    ));
    
    // Add visual-only plugins
    if !is_headless {
        app.add_plugins((
            HudPlugin,
            OverlayPlugin,
        ))
        .add_systems(Startup, setup_visual)
        .add_systems(Update, handle_input);
    }
    
    // Insert simulation config as resource
    app.insert_resource(sim_config);
    
    // Add exit system for headless mode
    if is_headless {
        app.add_systems(Update, check_headless_exit);
    }
    
    app.run();
}

fn setup_visual(mut commands: Commands) {
    // Add a 2D camera (only in visual mode)
    commands.spawn(Camera2dBundle::default());
    
    // Main title
    commands.spawn(
        TextBundle::from_section(
            "AI Battle Arena",
            TextStyle {
                font_size: 60.0,
                color: Color::rgb(0.9, 0.9, 0.9),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        }),
    );
    
    // Controls text
    commands.spawn(
        TextBundle::from_section(
            "AI vs AI | SPACE: pause | Arrows: pan | R: reset | ESC: exit",
            TextStyle {
                font_size: 24.0,
                color: Color::rgb(0.7, 0.7, 0.7),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        }),
    );
}

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
    
    if keyboard_input.just_pressed(KeyCode::Space) {
        println!("Game paused/resumed!");
    }
    
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        println!("Reset game!");
    }
    
    // Camera movement
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        let speed = 300.0 * time.delta_seconds();
        
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            camera_transform.translation.x -= speed;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            camera_transform.translation.x += speed;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            camera_transform.translation.y += speed;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            camera_transform.translation.y -= speed;
        }
    }
}

// Exit after game ends in headless mode
fn check_headless_exit(
    game_over: Res<crate::game::GameOver>,
    mut exit: EventWriter<AppExit>,
    mut metrics: ResMut<crate::performance::PerformanceMetrics>,
    state_recorder: Res<crate::engine::StateRecorder>,
    replay_recorder: Res<crate::engine::ReplayRecorder>,
    config: Res<GameConfig>,
    mut checked: Local<bool>,
) {
    if game_over.0 && !*checked {
        *checked = true;
        println!("\nGame ended in headless mode.");
        
        // Create data directory if it doesn't exist
        std::fs::create_dir_all("data/states").ok();
        std::fs::create_dir_all("data/replays").ok();
        
        // Save game state
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let state_filename = format!("game_state_{}.json", timestamp);
        let state_path = std::path::Path::new("data").join("states").join(&state_filename);
        
        match state_recorder.save_to_file(&state_path) {
            Ok(_) => println!("Game state saved to: {}", state_path.display()),
            Err(e) => println!("Failed to save game state: {}", e),
        }
        
        // Save replay
        let replay_filename = format!("game_replay_{}.json", timestamp);
        let replay_path = std::path::Path::new("data").join("replays").join(&replay_filename);
        
        let map_config = engine::replay::ReplayMapConfig {
            width: config.game.map_width,
            height: config.game.map_height,
            units_per_team: config.game.units_per_team,
        };
        
        // Determine outcome
        // TODO: Get actual outcome from game state
        let outcome = Some(engine::replay::GameOutcome::Draw);
        
        match replay_recorder.save_replay(&replay_path, map_config, outcome) {
            Ok(_) => println!("Replay saved to: {}", replay_path.display()),
            Err(e) => println!("Failed to save replay: {}", e),
        }
        
        // Print final performance report
        metrics.print_final_report();
        
        println!("Exiting...");
        exit.send(AppExit);
    }
}
