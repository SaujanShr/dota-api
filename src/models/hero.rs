use std::collections::HashMap;

use super::role::{Position, Role};

pub struct Hero {
    pub id: u16,
    pub name: String,
    pub roles: HashMap<Position, Role>
}
impl Hero {
    pub fn new(id: u16, name: String) -> Self {
        let roles = HashMap::new();

        Self { id, name, roles }
    }
}