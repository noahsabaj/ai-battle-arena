use bevy::prelude::*;
use crate::units::{Unit, Dead, Team};

#[derive(Resource)]
pub struct GameOver(pub bool);

pub struct VictoryPlugin;

impl Plugin for VictoryPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GameOver(false))
            .add_systems(Update, check_victory);
    }
}

pub fn check_victory(
    turn_state: Res<crate::game::TurnState>,
    config: Res<crate::config::GameConfig>,
    units: Query<&Unit, Without<Dead>>,
    mut game_over: ResMut<GameOver>,
) {
    if game_over.0 {
        return;
    }
    
    // Check turn limit
    if turn_state.turn >= config.game.max_turns {
        // println!("\n[TIMEOUT] Game ended - turn limit reached!");
        game_over.0 = true;
        return;
    }
    
    let (mut red_count, mut blue_count) = (0, 0);
    
    for unit in &units {
        match unit.team {
            Team::Red => red_count += 1,
            Team::Blue => blue_count += 1,
        }
    }
    
    match (red_count, blue_count) {
        (0, 0) => {
            // println!("\n[DRAW] DRAW! Both teams eliminated!");
            game_over.0 = true;
        }
        (0, _) => {
            // println!("\n[BLUE] BLUE TEAM WINS!");
            game_over.0 = true;
        }
        (_, 0) => {
            // println!("\n[RED]  RED TEAM WINS!");
            game_over.0 = true;
        }
        _ => {}
    }
}

pub fn game_over(game_over: Res<GameOver>) -> bool {
    game_over.0
}


