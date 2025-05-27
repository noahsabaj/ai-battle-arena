use bevy::prelude::*;
use std::collections::{HashSet, HashMap};
use crate::units::{Unit, HexPosition, Dead, Team};
use crate::ai::{AiController, WorldSnapshot};
use crate::world::actions::Action;
use crate::game::combat_system::{CombatEvent, check_combat};
use crate::config::{GameConfig, SimulationConfig, SimulationMode};
use crate::performance::PerformanceMetrics;
use crate::engine::ReplayRecorder;

#[derive(Resource)]
pub struct TurnState {
    pub turn: u32,
    pub time: f32,
}

#[derive(Resource)]
pub struct GameAI(pub Box<dyn AiController>);

pub struct TurnManagerPlugin;

impl Plugin for TurnManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(TurnState { turn: 0, time: 0.0 })
            .add_systems(Update, game_turn_system.run_if(not(crate::game::victory::game_over)));
    }
}

fn game_turn_system(
    time: Res<Time>,
    mut queries: ParamSet<(
        Query<(Entity, &mut Unit, &mut HexPosition), Without<Dead>>,
        Query<(Entity, &Unit, &HexPosition), Without<Dead>>,
    )>,
    mut turn_state: ResMut<TurnState>,
    mut last_turn: Local<f32>,
    mut turn: Local<u32>,
    mut combat_events: EventWriter<CombatEvent>,
    mut ai: ResMut<GameAI>,
    config: Res<GameConfig>,
    sim_config: Res<SimulationConfig>,
    mut metrics: ResMut<PerformanceMetrics>,
    mut profiler: Option<ResMut<crate::performance::profiler::Profiler>>,
    mut replay_recorder: Option<ResMut<ReplayRecorder>>,
) {
    // Profile turn system
    if let Some(ref mut prof) = profiler {
        prof.start("turn_system");
    }
    
    // Check if it's time for next turn
    let now = time.elapsed_seconds();
    let tick_rate = if sim_config.modes.default == SimulationMode::Headless {
        1.0 / sim_config.headless.timestep_hz as f32
    } else {
        config.game.tick_rate
    };
    
    if now - *last_turn < tick_rate {
        return;
    }
    
    *last_turn = now;
    *turn += 1;
    turn_state.turn = *turn;
    turn_state.time = now;
    metrics.record_tick();
    
    // Only print logs in visual mode
    let should_log = sim_config.modes.default == SimulationMode::Visual;
    
    if should_log {
        println!("\n----------------------------------------");
        println!("[TURN] TURN {} - Time: {:.1}s", *turn, now);
    }
    
    // Create world snapshot for AI
    let world_snapshot = {
        let units_query = queries.p1();
        WorldSnapshot {
            units: units_query.iter().map(|(e, u, p)| (e, u.team, p.coord)).collect(),
        }
    };
    
    // Movement Phase - AI Controlled
    if should_log {
        println!("[MOVE] Movement Phase:");
    }
    
    // Collect current positions and AI decisions
    let (mut occupied, ai_decisions) = {
        let units_query = queries.p1();
        let mut occupied: HashSet<(i32, i32)> = HashSet::new();
        let mut ai_decisions = HashMap::new();
        
        for (entity, unit, pos) in &units_query {
            occupied.insert((pos.coord.q, pos.coord.r));
            let decision = ai.0.decide(entity, unit, &world_snapshot);
            ai_decisions.insert(entity, decision);
        }
        
        (occupied, ai_decisions)
    };
    
    // Apply movements
    let mut moves = Vec::new();
    {
        let mut units_mut = queries.p0();
        for (entity, unit, mut pos) in &mut units_mut {
            let old_coord = pos.coord;
            
            if let Some(action) = ai_decisions.get(&entity) {
                // Record action for replay
                if let Some(ref mut recorder) = replay_recorder {
                    recorder.record_action(entity, unit.team, *action);
                }
                
                match action {
                    Action::Move(dq, dr) => {
                        let new_q = old_coord.q + dq;
                        let new_r = old_coord.r + dr;
                        let new_coord = (new_q, new_r);
                        
                        // Check bounds and collision
                        if new_q.abs() < config.game.map_width/2 && 
                           new_r.abs() < config.game.map_height/2 && 
                           !occupied.contains(&new_coord) {
                            occupied.remove(&(old_coord.q, old_coord.r));
                            occupied.insert(new_coord);
                            pos.coord.q = new_q;
                            pos.coord.r = new_r;
                            moves.push((unit.team, old_coord, pos.coord));
                        }
                    }
                    Action::Stay => {
                        // Unit chose to stay
                    }
                }
            }
        }
    }
    
    // Log movements
    if should_log {
        for (team, old, new) in moves {
            println!("   {} Unit: ({}, {}) -> ({}, {})", 
                team.tag(), old.q, old.r, new.q, new.r);
        }
    }
    
    // Combat Phase
    if should_log {
        println!("\n[COMBAT] Combat Phase:");
    }
    
    let combat_pairs = {
        let units_query = queries.p1();
        check_combat(&units_query)
    };
    
    if combat_pairs.is_empty() {
        if should_log {
            println!("   No combat this turn");
        }
    } else {
        // Log combat and send events
        {
            let units_query = queries.p1();
            for (e1, e2) in &combat_pairs {
                // Each unit damages the other
                combat_events.send(CombatEvent {
                    attacker: *e1,
                    defender: *e2,
                    damage: config.combat.base_damage,
                });
                combat_events.send(CombatEvent {
                    attacker: *e2,
                    defender: *e1,
                    damage: config.combat.base_damage,
                });
                
                // Log combat
                if should_log {
                    if let Ok([(_, u1, p1), (_, u2, p2)]) = units_query.get_many([*e1, *e2]) {
                        println!("   {} unit at ({}, {}) fights {} unit at ({}, {})",
                            u1.team.tag(), p1.coord.q, p1.coord.r,
                            u2.team.tag(), p2.coord.q, p2.coord.r);
                    }
                }
            }
        }
    }
    
    // Status report every 5 turns (only in visual mode)
    if should_log && *turn % 5 == 0 {
        println!("\n[STATUS] Status Report:");
        let units_query = queries.p1();
        let (reds, blues): (Vec<_>, Vec<_>) = units_query.iter()
            .map(|(_, u, p)| (u.team, p.coord, u.health))
            .partition(|(t, _, _)| *t == Team::Red);
        
        println!("   [RED]  {} units", reds.len());
        for (i, (_, c, h)) in reds.iter().enumerate() {
            println!("      {} ({}, {}) {:.0} HP", i + 1, c.q, c.r, h);
        }
        
        println!("   [BLUE] {} units", blues.len());
        for (i, (_, c, h)) in blues.iter().enumerate() {
            println!("      {} ({}, {}) {:.0} HP", i + 1, c.q, c.r, h);
        }
    }
    
    // Record end of turn for replay
    if let Some(ref mut recorder) = replay_recorder {
        recorder.end_turn(*turn, 0); // TODO: Add proper RNG seed
    }
    
    // End profiling
    if let Some(ref mut prof) = profiler {
        prof.end("turn_system");
    }
}


