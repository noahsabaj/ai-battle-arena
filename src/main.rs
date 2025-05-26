use bevy::prelude::*;
use bevy::app::AppExit;

mod engine;
mod world;
mod ai;
mod utils;

use world::hex_grid::HexGridPlugin;
use world::unit::UnitsPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "AI Battle Arena".to_string(),
                    resolution: (1280., 720.).into(),
                    ..default()
                }),
                ..default()
            }),
            HexGridPlugin,
            UnitsPlugin,  // Add units plugin!
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_input, game_tick))
        .run();
}

fn setup(mut commands: Commands) {
    // Add a 2D camera
    commands.spawn(Camera2dBundle::default());
    
    // UI Text
    commands.spawn((
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
    ));
    
    commands.spawn((
        TextBundle::from_section(
            "Red vs Blue | SPACE: pause | Arrows: pan | R: reset",
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
    ));
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

fn game_tick(time: Res<Time>) {
    // This will eventually run game logic at 1000 TPS
    // For now, just tracks elapsed time
    if time.elapsed_seconds() as i32 % 5 == 0 {
        // Log every 5 seconds
    }
}
