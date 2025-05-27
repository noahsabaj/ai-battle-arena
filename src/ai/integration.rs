use bevy::prelude::*;
use crate::game::turn_manager::GameAI;
use crate::ai::RandomAi;

pub struct AiIntegrationPlugin;

impl Plugin for AiIntegrationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ai);
    }
}

fn setup_ai(mut commands: Commands) {
    // Initialize with RandomAI for now
    let ai = RandomAi::new();
    commands.insert_resource(GameAI(Box::new(ai)));
}
