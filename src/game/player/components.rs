use bevy::prelude::*;

use crossterm::style::Color as CrosstermColor;
#[derive(Component)]
pub struct Player {
    name: String,
    ascii_appearance: char,
    ascii_color: CrosstermColor,
}

impl Player {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ascii_appearance: '@',
            ascii_color: CrosstermColor::Red,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn ascii_appearance(&self) -> char {
        self.ascii_appearance
    }

    #[allow(dead_code)]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    #[allow(dead_code)]
    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    pub fn with_ascii_appearance(mut self, appearance: char) -> Self {
        self.ascii_appearance = appearance;
        self
    }

    #[allow(dead_code)]
    pub fn set_ascii_appearance(&mut self, appearance: char) {
        self.ascii_appearance = appearance;
    }

    pub fn ascii_color(&self) -> CrosstermColor {
        self.ascii_color
    }

    pub fn with_ascii_color(mut self, color: CrosstermColor) -> Self {
        self.ascii_color = color;
        self
    }

    #[allow(dead_code)]
    pub fn set_ascii_color(&mut self, color: CrosstermColor) {
        self.ascii_color = color;
    }
}
