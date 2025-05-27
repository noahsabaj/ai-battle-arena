use bevy::prelude::*;
use crate::world::HexCoord;
use crate::units::{HexPosition, Dead};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_unit_positions);
    }
}

pub fn update_unit_positions(
    mut query: Query<(&HexPosition, &mut Transform), (Changed<HexPosition>, Without<Dead>)>,
) {
    for (hex_pos, mut transform) in &mut query {
        let world_pos = hex_to_world_pos(hex_pos.coord.q, hex_pos.coord.r);
        transform.translation.x = world_pos.x;
        transform.translation.y = world_pos.y;
    }
}

pub fn hex_to_world_pos(q: i32, r: i32) -> Vec2 {
    const HEX_SIZE: f32 = 30.0;
    Vec2::new(
        HEX_SIZE * (f32::sqrt(3.0) * q as f32 + f32::sqrt(3.0) / 2.0 * r as f32),
        HEX_SIZE * (3.0 / 2.0 * r as f32),
    )
}

pub fn hex_distance(a: HexCoord, b: HexCoord) -> i32 {
    ((a.q - b.q).abs() + (a.q + a.r - b.q - b.r).abs() + (a.r - b.r).abs()) / 2
}
