use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Sprite, SpriteBundle};
use crate::world::HexCoord;
use crate::units::{Unit, HexPosition, Team, UnitType};
use crate::units::health::HealthBar;
use crate::units::movement::hex_to_world_pos;
use crate::config::{SimulationConfig, SimulationMode, GameConfig};

pub struct SpawningPlugin;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_initial_units);
    }
}

fn spawn_initial_units(
    mut commands: Commands,
    mut meshes: Option<ResMut<Assets<Mesh>>>,
    mut materials: Option<ResMut<Assets<ColorMaterial>>>,
    sim_config: Res<SimulationConfig>,
    game_config: Res<GameConfig>,
) {
    let is_visual = sim_config.modes.default == SimulationMode::Visual;
    let units_per_team = game_config.game.units_per_team;
    
    if sim_config.modes.default == SimulationMode::Visual { 
        println!("[START] GAME START - Spawning {} units per team!", units_per_team); 
        println!("----------------------------------------"); 
    }
    
    // Spawn Red team
    for i in 0..units_per_team {
        let coord = HexCoord { 
            q: -8 + (i as i32 % 8) * 2, 
            r: -5 - (i as i32 / 8) 
        };
        if sim_config.modes.default == SimulationMode::Visual { 
            println!("[RED]  Spawning RED  Fighter {} at ({}, {})", i + 1, coord.q, coord.r); 
        }
        
        if is_visual {
            // Visual mode with meshes
            if let (Some(ref mut meshes), Some(ref mut materials)) = (&mut meshes, &mut materials) {
                let unit_mesh = meshes.add(Circle::new(15.0));
                spawn_visual_unit(
                    &mut commands,
                    coord,
                    Team::Red,
                    UnitType::Fighter,
                    unit_mesh,
                    materials.add(ColorMaterial::from(Team::Red.color())),
                );
            }
        } else {
            // Headless mode without visuals
            spawn_headless_unit(
                &mut commands,
                coord,
                Team::Red,
                UnitType::Fighter,
            );
        }
    }
    
    // Spawn Blue team
    for i in 0..units_per_team {
        let coord = HexCoord { 
            q: -8 + (i as i32 % 8) * 2, 
            r: 5 + (i as i32 / 8) 
        };
        if sim_config.modes.default == SimulationMode::Visual { 
            println!("[BLUE] Spawning BLUE Fighter {} at ({}, {})", i + 1, coord.q, coord.r); 
        }
        
        if is_visual {
            // Visual mode with meshes
            if let (Some(ref mut meshes), Some(ref mut materials)) = (&mut meshes, &mut materials) {
                let unit_mesh = meshes.add(Circle::new(15.0));
                spawn_visual_unit(
                    &mut commands,
                    coord,
                    Team::Blue,
                    UnitType::Fighter,
                    unit_mesh,
                    materials.add(ColorMaterial::from(Team::Blue.color())),
                );
            }
        } else {
            // Headless mode without visuals
            spawn_headless_unit(
                &mut commands,
                coord,
                Team::Blue,
                UnitType::Fighter,
            );
        }
    }
    
    if sim_config.modes.default == SimulationMode::Visual { 
        println!("----------------------------------------"); 
    }
}

fn spawn_visual_unit(
    commands: &mut Commands,
    coord: HexCoord,
    team: Team,
    unit_type: UnitType,
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
) {
    let world_pos = hex_to_world_pos(coord.q, coord.r);
    
    let unit_entity = commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh.into(),
            material,
            transform: Transform::from_translation(Vec3::new(world_pos.x, world_pos.y, 1.0)),
            ..default()
        },
        Unit {
            team,
            unit_type,
            health: 100.0,
            max_health: 100.0,
        },
        HexPosition { coord },
    )).id();
    
    // Spawn health bar as child
    let health_bar = commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(60.0, 8.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 28.0, 3.0)),
            ..default()
        },
        HealthBar,
    )).id();
    
    commands.entity(unit_entity).add_child(health_bar);
}

fn spawn_headless_unit(
    commands: &mut Commands,
    coord: HexCoord,
    team: Team,
    unit_type: UnitType,
) {
    // In headless mode, just spawn the unit data without visuals
    commands.spawn((
        Unit {
            team,
            unit_type,
            health: 100.0,
            max_health: 100.0,
        },
        HexPosition { coord },
        // Add a transform even in headless for spatial queries
        Transform::default(),
    ));
}
