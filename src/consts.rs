use bevy::prelude::States;

/// Speed at which a bar moves (this will be dynamic in the future)
pub const BASE_SPEED: f32 = 800.;

/// Y coordinate value at which bars spawn
pub const SPAWN_POSITION: f32 = 300.;

/// Y coordinate value where the bars should be clicked
pub const TARGET_POSITION: f32 = -300.;

/// Margin of error for clicking a note
pub const THRESHOLD: f32 = 100.;

/// This should probably be temporary maybe idk
pub const AUTOPLAY_THRESHOLD: f32 = 1.;

/// Total distance traveled by a note, from spawn to target
pub const DISTANCE: f32 = TARGET_POSITION - SPAWN_POSITION;

/// Temporary global volume level
pub const VOLUME: f64 = 0.0;

/// Temporary autoplay option
pub const AUTOPLAY_ENABLED: bool = true;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MyAppState {
    MainMenu,
    SongSelect,
    InGame,
    Paused,
    Results,
}
