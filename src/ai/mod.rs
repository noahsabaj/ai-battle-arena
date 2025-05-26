pub mod bridge;
pub mod team;
pub mod communication;

pub use team::AITeam;
pub mod controller;
pub use controller::{AiController, WorldSnapshot};

pub mod random;
pub use random::RandomAi;
