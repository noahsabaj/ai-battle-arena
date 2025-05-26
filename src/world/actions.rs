use crate::world::HexCoord;

pub enum Action {
    Move(HexCoord),
    Attack(HexCoord),
    Gather,
}
