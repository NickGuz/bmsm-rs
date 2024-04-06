use crate::consts::*;
use crate::types::*;
use crate::ScoreResource;
use bevy::prelude::*;
use bms_rs::lex::command::ObjId;

/// Keeps the textures and materials for Bars
#[derive(Resource)]
struct BarMaterialResource {
    blue_texture: Handle<Image>,
    white_texture: Handle<Image>,
    border_texture: Handle<Image>,
}

impl FromWorld for BarMaterialResource {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let blue_texture = asset_server.load("jakads/mania-note2.png");
        let white_texture = asset_server.load("jakads/mania-note1.png");
        let border_texture = asset_server.load("jakads/mania-noteS.png");
        BarMaterialResource {
            blue_texture,
            white_texture,
            border_texture,
        }
    }
}

#[derive(Component)]
struct Bar {
    position: Positions,
    audio_source: Handle<AudioSource>,
}

#[derive(Resource)]
struct SpawnTimer(Timer);

#[derive(Component)]
struct TargetBar;

fn setup_target_bars(mut commands: Commands, materials: Res<BarMaterialResource>) {
    let bar_width = 100.;
    let bar_offset = 400.;

    for n in 1..7 {
        let transform = Transform::from_translation(Vec3::new(
            n as f32 * bar_width - bar_offset,
            TARGET_POSITION,
            1.,
        ));

        commands
            .spawn(SpriteBundle {
                texture: materials.border_texture.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(bar_width, 30.)),
                    ..default()
                },
                transform,
                ..default()
            })
            .insert(TargetBar);
    }
}

fn spawn_bars(
    mut commands: Commands,
    mut song_config: ResMut<SongConfig>,
    materials: Res<BarMaterialResource>,
    time: Res<Time>,
    // mut timer: ResMut<SpawnTimer>,
) {
    // We get the current time since startup (secs) and the time since the last iterations (secs_last),
    // this way we check if any bars should spawn in this window

    // Song starts 3 seconds after start, so we subtract 3 seconds
    let secs = time.elapsed_seconds_f64() - 3.;
    let secs_last = secs - time.delta_seconds_f64();

    // Counter of how many bars we need to spawn and remove from the list
    let mut remove_counter = 0;
    for bar in &song_config.notes {
        // List is ordered, so we can just check until an item fails
        // Check if bar should be spawned at any point between last frame and this frame
        // println!(
        //     "secs_last={}, spawn_time={}, secs={}",
        //     secs_last, bar.spawn_time, secs
        // );
        if secs_last < bar.spawn_time && bar.spawn_time <= secs {
            remove_counter += 1;

            // Get the correct material according to position
            let material = match bar.position {
                Positions::One | Positions::Three | Positions::Five | Positions::Seven => {
                    materials.blue_texture.clone()
                }
                _ => materials.white_texture.clone(),
            };

            let bar_width = 100.;
            // let bar_x_pos = bar.position.x() as f32 * bar_width - 400.;
            let bar_x_pos = bar.position.x();

            // let transform = Transform::from_translation(Vec3::new(-400., SPAWN_POSITION, 1.));
            let transform = Transform::from_translation(Vec3::new(bar_x_pos, SPAWN_POSITION, 1.));
            commands
                .spawn(SpriteBundle {
                    texture: material,
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(bar_width, 30.)),
                        ..default()
                    },
                    transform,
                    ..default()
                })
                .insert(Bar {
                    position: bar.position,
                    audio_source: bar.audio_source.to_owned(),
                });
        } else {
            break;
        }
    }

    // Remove the bars we have spawned from the list (should prob just use a stack or something)
    for _ in 0..remove_counter {
        song_config.notes.remove(0);
    }
}

/// Moves the bars downward
fn move_bars(time: Res<Time>, mut query: Query<(&mut Transform, &Bar)>) {
    for (mut transform, _bar) in query.iter_mut() {
        transform.translation.y -= time.delta_seconds() * BASE_SPEED;
    }
}

/// Despawns bars when they reach the end if the correct button is clicked
fn despawn_bars(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Bar)>,
    // mut song_config: ResMut<SongConfig>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut score: ResMut<ScoreResource>,
) {
    for (entity, transform, bar) in query.iter() {
        let pos = transform.translation.y;

        // Check if bar is inside clicking threshold
        if (TARGET_POSITION - THRESHOLD..=TARGET_POSITION + THRESHOLD).contains(&pos)
            && bar.position.key_just_pressed(&keyboard_input)
        {
            commands.entity(entity).despawn();

            // play sound -- do this somehwere else?
            commands.spawn(AudioBundle {
                source: bar.audio_source.to_owned(),
                ..default()
            });

            let _points = score.increase_correct(TARGET_POSITION - pos);
        }

        // Despawn bar after they leave the screen
        if pos <= 2. * TARGET_POSITION {
            commands.entity(entity).despawn();

            score.increase_fails();
        }
    }
}

pub struct BarsPlugin;
impl Plugin for BarsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BarMaterialResource>();
        app.insert_resource(SpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));
        app.add_systems(Startup, setup_target_bars);
        app.add_systems(Update, spawn_bars);
        app.add_systems(Update, move_bars);
        app.add_systems(Update, despawn_bars);
    }
}
