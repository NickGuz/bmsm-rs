/// Speed at which a bar moves (this will be dynamic in the future)
pub const BASE_SPEED: f32 = 400.;

/// Y coordinate value at which bars spawn
pub const SPAWN_POSITION: f32 = 300.;

/// Y coordinate value where the bars should be clicked
pub const TARGET_POSITION: f32 = -300.;

/// Margin of error for clicking a note
pub const THRESHOLD: f32 = 100.;

/// Total distance traveled by a note, from spawn to target
pub const DISTANCE: f32 = TARGET_POSITION - SPAWN_POSITION;
