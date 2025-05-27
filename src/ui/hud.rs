use bevy::prelude::*;
use crate::game::TurnState;
use crate::units::{Unit, Team};

#[derive(Component)]
pub struct TurnText;

#[derive(Component)]
pub struct CountText;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_hud)
            .add_systems(Update, update_hud);
    }
}

fn spawn_hud(mut commands: Commands) {
    // Turn & Time display
    commands.spawn((
        TextBundle::from_section(
            "Turn: 0 | Time: 0.0s",
            TextStyle {
                font_size: 24.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(15.0),
            top: Val::Px(15.0),
            ..default()
        }),
        TurnText,
    ));
    
    // Unit counts
    commands.spawn((
        TextBundle::from_section(
            "Red: 0 • Blue: 0",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(15.0),
            top: Val::Px(50.0),
            ..default()
        }),
        CountText,
    ));
}

fn update_hud(
    turn_state: Res<TurnState>,
    units: Query<&Unit>,
    mut turn_text: Query<&mut Text, (With<TurnText>, Without<CountText>)>,
    mut count_text: Query<&mut Text, (With<CountText>, Without<TurnText>)>,
) {
    // Update turn text
    for mut text in &mut turn_text {
        text.sections[0].value = format!("Turn: {} | Time: {:.1}s", 
            turn_state.turn, turn_state.time);
    }
    
    // Count units
    let (red_count, blue_count) = units.iter()
        .fold((0, 0), |(r, b), unit| {
            if unit.team == Team::Red { (r + 1, b) } else { (r, b + 1) }
        });
    
    // Update count text
    for mut text in &mut count_text {
        text.sections[0].value = format!("Red: {} • Blue: {}", red_count, blue_count);
    }
}
