use bevy::prelude::*;
use crate::game::GameOver;
use crate::units::{Unit, Team};

#[derive(Component)]
pub struct VictoryBanner;

pub struct OverlayPlugin;

impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, victory_banner_system);
    }
}

fn victory_banner_system(
    game_over: Res<GameOver>,
    units: Query<&Unit>,
    mut spawned: Local<bool>,
    mut commands: Commands,
) {
    if game_over.0 && !*spawned {
        *spawned = true;
        
        let (red_count, blue_count) = units.iter()
            .fold((0, 0), |(r, b), unit| {
                if unit.team == Team::Red { (r + 1, b) } else { (r, b + 1) }
            });
        
        let message = match (red_count, blue_count) {
            (0, 0) => "DRAW!",
            (0, _) => "BLUE TEAM WINS!",
            (_, 0) => "RED TEAM WINS!",
            _ => unreachable!(),
        };
        
        commands.spawn((
            TextBundle::from_section(
                message,
                TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(35.0),
                top: Val::Percent(45.0),
                ..default()
            }),
            VictoryBanner,
        ));
    }
}
