use anyhow::Result;

pub struct GameLoop {
    tick_rate: u32,
}

impl GameLoop {
    pub fn new(tick_rate: u32) -> Self {
        Self { tick_rate }
    }
}
