#[derive(Debug, Clone)]
pub struct AITeam {
    pub id: u32,
    pub name: String,
    pub ai_count: usize,
}

impl AITeam {
    pub fn new(id: u32, name: String, ai_count: usize) -> Self {
        Self { id, name, ai_count }
    }
}
