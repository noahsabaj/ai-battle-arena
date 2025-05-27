use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::render::render_asset::RenderAssetUsages;
use crate::config::{SimulationConfig, SimulationMode};

const HEX_SIZE: f32 = 30.0;
const GRID_WIDTH: i32 = 20;
const GRID_HEIGHT: i32 = 15;

#[derive(Component, Clone, Copy, Debug)]
pub struct HexCoord {
    pub q: i32,  // Axial coordinates
    pub r: i32,
}

#[derive(Component)]
pub struct HexTile {
    pub coord: HexCoord,
}

pub struct HexGrid {
    pub width: i32,
    pub height: i32,
}

pub struct HexGridPlugin;

impl Plugin for HexGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_hex_grid);
    }
}

fn spawn_hex_grid(
    mut commands: Commands,
    meshes: Option<ResMut<Assets<Mesh>>>,
    materials: Option<ResMut<Assets<ColorMaterial>>>,
    sim_config: Res<SimulationConfig>,
) {
    // Only spawn visual hex grid in visual mode
    if sim_config.modes.default != SimulationMode::Visual {
        return;
    }
    
    // Check if we have rendering resources
    if let (Some(mut meshes), Some(mut materials)) = (meshes, materials) {
        // Create hex mesh
        let hex_mesh = create_hex_mesh(HEX_SIZE);
        let mesh_handle = meshes.add(hex_mesh);
        
        // Spawn hexagons
        for q in -GRID_WIDTH/2..GRID_WIDTH/2 {
            for r in -GRID_HEIGHT/2..GRID_HEIGHT/2 {
                let pos = hex_to_world_pos(q, r);
                
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: mesh_handle.clone().into(),
                        material: materials.add(ColorMaterial::from(Color::rgb(0.2, 0.3, 0.4))),
                        transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0)),
                        ..default()
                    },
                    HexTile { 
                        coord: HexCoord { q, r } 
                    },
                ));
            }
        }
    }
}

fn hex_to_world_pos(q: i32, r: i32) -> Vec2 {
    let x = HEX_SIZE * (3.0_f32.sqrt() * q as f32 + 3.0_f32.sqrt() / 2.0 * r as f32);
    let y = HEX_SIZE * (3.0 / 2.0 * r as f32);
    Vec2::new(x, y)
}

fn create_hex_mesh(size: f32) -> Mesh {
    let mut mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD
    );
    
    // Create vertices for hexagon
    let mut vertices = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    
    // Center vertex
    vertices.push([0.0, 0.0, 0.0]);
    normals.push([0.0, 0.0, 1.0]);
    uvs.push([0.5, 0.5]);
    
    // Hex vertices
    for i in 0..6 {
        let angle = std::f32::consts::PI / 3.0 * i as f32;
        let x = size * angle.cos();
        let y = size * angle.sin();
        vertices.push([x, y, 0.0]);
        normals.push([0.0, 0.0, 1.0]);
        uvs.push([(x / size + 1.0) / 2.0, (y / size + 1.0) / 2.0]);
    }
    
    // Create triangles
    let mut indices = Vec::new();
    for i in 0..6 {
        indices.push(0);
        indices.push(i + 1);
        indices.push((i % 6) + 2);
    }
    
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));
    
    mesh
}
