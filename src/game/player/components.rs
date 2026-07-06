use bevy::prelude::*;
#[derive(Component)]
pub struct Player {
    name: String,
}

impl Player {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}
