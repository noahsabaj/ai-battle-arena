use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::world::HexCoord;

#[derive(Component)]
pub struct Unit {
    pub team: Team,
    pub unit_type: UnitType,
    pub health: f32,
    pub max_health: f32,
}

#[derive(Component)]
pub struct HexPosition {
    pub coord: HexCoord,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Team {
    Red,
    Blue,
}

#[derive(Clone, Copy, Debug)]
pub enum UnitType {
    Worker,
    Fighter,
    Scout,
}

impl Team {
    pub fn color(&self) -> Color {
        match self {
            Team::Red => Color::rgb(0.8, 0.2, 0.2),
            Team::Blue => Color::rgb(0.2, 0.2, 0.8),
        }
    }
}

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_initial_units)
            .add_systems(Update, (move_units, update_unit_positions));
    }
}

fn spawn_initial_units(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let unit_mesh = meshes.add(Circle::new(15.0));
    
    // Spawn red team
    for i in 0..6 {
        let coord = HexCoord { q: -8 + i * 2, r: -5 };
        spawn_unit(
            &mut commands,
            coord,
            Team::Red,
            UnitType::Fighter,
            unit_mesh.clone(),
            materials.add(ColorMaterial::from(Team::Red.color())),
        );
    }
    
    // Spawn blue team
    for i in 0..6 {
        let coord = HexCoord { q: -8 + i * 2, r: 5 };
        spawn_unit(
            &mut commands,
            coord,
            Team::Blue,
            UnitType::Fighter,
            unit_mesh.clone(),
            materials.add(ColorMaterial::from(Team::Blue.color())),
        );
    }
}

fn spawn_unit(
    commands: &mut Commands,
    coord: HexCoord,
    team: Team,
    unit_type: UnitType,
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
) {
    let world_pos = hex_to_world_pos(coord.q, coord.r);
    
    commands.spawn((
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
    ));
}

fn move_units(
    time: Res<Time>,
    mut units: Query<(&Unit, &mut HexPosition)>,
) {
    // Simple movement - units move occasionally
    for (_unit, mut hex_pos) in units.iter_mut() {
        // Move every 2 seconds
        let current_second = time.elapsed_seconds() as i32;
        if current_second % 2 == 0 {
            // Use a simple counter instead of coord-based randomness to avoid overflow
            let move_counter = (current_second / 2) as usize;
            
            // Random-ish direction based on time
            let directions = [(1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)];
            let dir_index = move_counter % 6;
            let (dq, dr) = directions[dir_index];
            
            // Calculate new position
            let new_q = hex_pos.coord.q + dq;
            let new_r = hex_pos.coord.r + dr;
            
            // Keep units within bounds
            if new_q.abs() < 10 && new_r.abs() < 8 {
                // Only update if we haven't moved this frame already
                // (prevents multiple moves in the same second)
                if hex_pos.coord.q != new_q || hex_pos.coord.r != new_r {
                    hex_pos.coord.q = new_q;
                    hex_pos.coord.r = new_r;
                }
            }
        }
    }
}

fn update_unit_positions(
    mut units: Query<(&HexPosition, &mut Transform), Changed<HexPosition>>,
) {
    for (hex_pos, mut transform) in units.iter_mut() {
        let world_pos = hex_to_world_pos(hex_pos.coord.q, hex_pos.coord.r);
        transform.translation.x = world_pos.x;
        transform.translation.y = world_pos.y;
    }
}

fn hex_to_world_pos(q: i32, r: i32) -> Vec2 {
    const HEX_SIZE: f32 = 30.0;
    let x = HEX_SIZE * (3.0_f32.sqrt() * q as f32 + 3.0_f32.sqrt() / 2.0 * r as f32);
    let y = HEX_SIZE * (3.0 / 2.0 * r as f32);
    Vec2::new(x, y)
}
