#[derive(Debug, Clone)]
pub struct Resource {
    pub energy: i32,
    pub metal: i32,
    pub knowledge: i32,
}

impl Resource {
    pub fn new() -> Self {
        Self {
            energy: 100,
            metal: 100,
            knowledge: 0,
        }
    }
}
