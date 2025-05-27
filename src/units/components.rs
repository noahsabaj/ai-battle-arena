use bevy::prelude::*;
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

#[derive(Component)]
pub struct Dead;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Team {
    Red,
    Blue,
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

    pub fn tag(&self) -> &'static str {
        match self {
            Team::Red => "[RED]",
            Team::Blue => "[BLUE]",
        }
    }
}
