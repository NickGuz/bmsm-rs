use bevy::prelude::*;

#[derive(Resource)]
pub struct UserSettings {
    pub scroll_speed: f32,
    pub autoplay_enabled: bool,
}
impl Default for UserSettings {
    fn default() -> Self {
        Self {
            scroll_speed: 800.,
            autoplay_enabled: true,
        }
    }
}
