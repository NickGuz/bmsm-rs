use crate::consts::*;
use crate::time::ControlledTime;
use crate::types::*;
use crate::user_settings::UserSettings;
use crate::ScoreResource;
// use bevy::audio::*;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bms_rs::lex::command::ObjId;
use ordered_float::OrderedFloat;

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
pub struct GameplayUI;

#[derive(Component)]
struct Bar {
    position: Positions,
    // audio_source: Handle<AudioSource>,
    audio_source_id: ObjId,
}

#[derive(Resource)]
struct SpawnTimer(Timer);

#[derive(Component)]
struct TargetBar;

fn despawn_ui(mut commands: Commands, query: Query<(Entity, &GameplayUI)>) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup_target_bars(mut commands: Commands, materials: Res<BarMaterialResource>) {
    println!("setting up target bars");
    let bar_width = 100.;
    let bar_offset = 400.;

    for n in 1..=7 {
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
            .insert(TargetBar)
            .insert(GameplayUI);
    }
}

fn play_bgms(
    mut commands: Commands,
    mut song_config: ResMut<SongConfig>,
    time: Res<ControlledTime>,
    audio: Res<Audio>,
) {
    // Song starts 3 seconds after start, so we subtract 3 seconds
    // This might be wrong bc of travel time of the note...
    // Notes are spawning after 3 seconds, but they don't play until
    // they are clicked
    let secs = time.seconds_since_startup() - 3.75;
    let secs_last = secs - time.delta_seconds_f64();

    // let mut remove_counter = 0;
    for bgm in &song_config.bgms {
        if secs_last < bgm.spawn_time && bgm.spawn_time <= secs {
            for id in &bgm.audio_source_ids {
                // get handle from map
                let audio_handle = song_config
                    .audio_handles
                    .get(&id)
                    .expect("Could not find bgm audio handle in map");

                // play sound -- do this somehwere else?
                // commands.spawn(AudioBundle {
                //     source: audio_handle.clone(),
                //     settings: PlaybackSettings {
                //         volume: Volume::new(VOLUME),
                //         ..default()
                //     },
                //     ..default()
                // });
                audio.play(audio_handle.clone()).with_volume(VOLUME);
            }
            // TODO this does not work
            // remove_counter += 1;
        }
    }

    // for _ in 0..remove_counter {
    // song_config.bgms.remove(0);
    // }
}

fn spawn_bars(
    mut commands: Commands,
    mut song_config: ResMut<SongConfig>,
    materials: Res<BarMaterialResource>,
    time: Res<ControlledTime>,
    // mut timer: ResMut<SpawnTimer>,
) {
    // We get the current time since startup (secs) and the time since the last iterations (secs_last),
    // this way we check if any bars should spawn in this window

    // Song starts 3 seconds after start, so we subtract 3 seconds
    let secs = time.seconds_since_startup() - 3.;
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
                    audio_source_id: bar.audio_source_id.to_owned(),
                })
                .insert(GameplayUI);
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
fn move_bars(
    time: Res<ControlledTime>,
    mut query: Query<(&mut Transform, &Bar)>,
    settings: Res<UserSettings>,
) {
    for (mut transform, _bar) in query.iter_mut() {
        transform.translation.y -= time.delta_seconds() * settings.scroll_speed;
    }
}

/// Despawns bars when they reach the end if the correct button is clicked
fn despawn_bars(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Bar)>,
    mut song_config: ResMut<SongConfig>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut score: ResMut<ScoreResource>,
    audio: Res<Audio>,
    settings: Res<UserSettings>,
) {
    let mut notes_in_threshold: Vec<(Entity, f32, &Bar)> = Vec::new();

    for (entity, transform, bar) in query.iter() {
        let pos = transform.translation.y;

        // Check if bar is inside clicking threshold
        if (TARGET_POSITION - THRESHOLD..=TARGET_POSITION + THRESHOLD).contains(&pos)
            && (bar.position.key_just_pressed(&keyboard_input) || settings.autoplay_enabled)
        {
            // TODO this is stupid -- maybe set threshold to AUTOPLAY_THRESHOLD earlier if AUTOPLAY
            // is enabled or something
            if settings.autoplay_enabled {
                if (TARGET_POSITION - AUTOPLAY_THRESHOLD..=TARGET_POSITION + AUTOPLAY_THRESHOLD)
                    .contains(&pos)
                {
                    notes_in_threshold.push((entity, pos, bar));
                }
            } else {
                // before despawning, check if first in lane in threshold?
                // using lowest y pos for now -- might be bad?
                notes_in_threshold.push((entity, pos, bar));
            }
        }

        // Despawn bar after they leave the screen
        if pos <= 2. * TARGET_POSITION {
            commands.entity(entity).despawn();

            score.increase_fails();
        }
    }

    // TODO need to account for notes in the same position (chords)
    let min_note = notes_in_threshold.iter().min_by_key(|x| OrderedFloat(x.1));

    // notes_in_threshold.sort_by_key(|x| OrderedFloat(x.1));
    // let first_note = notes_in_threshold.first();

    let mut notes_to_play: Vec<Option<&(Entity, f32, &Bar)>> = Vec::new();

    for note in notes_in_threshold.iter() {
        if note.1 == min_note.expect("no min note").1 {
            notes_to_play.push(Some(note));
        }
    }

    for note in notes_to_play {
        match note {
            Some((entity, pos, bar)) => {
                commands.entity(*entity).despawn();

                // get audio handle
                let audio_handle = song_config
                    .audio_handles
                    .get(&bar.audio_source_id)
                    .expect("Audio source ID not found in map");

                audio.play(audio_handle.clone()).with_volume(VOLUME);

                let _points = score.increase_correct(TARGET_POSITION - pos);

                let msval = calculate_milliseconds_from_target(
                    *pos,
                    TARGET_POSITION,
                    settings.scroll_speed,
                );

                // using mostly IIDX timings for now
                if msval <= 16.67 {
                    score.pgreats += 1;
                } else if msval <= 33.33 {
                    score.greats += 1;
                } else if msval <= 116.67 {
                    score.goods += 1;
                } else if msval <= 250.0 {
                    score.bads += 1;
                } else if msval <= 1000.0 {
                    score.poors += 1;
                } else {
                    println!("MISS");
                }
            }
            None => {}
        };
    }
}

fn calculate_milliseconds_from_target(pos: f32, target_pos: f32, note_speed: f32) -> f32 {
    let distance = (target_pos - pos).abs();
    let time_in_seconds = distance / note_speed;
    time_in_seconds * 1000.0
}

fn show_results_on_finished(
    song_config: Res<SongConfig>,
    mut next_state: ResMut<NextState<MyAppState>>,
) {
    // println!(
    //     "noteslen: {}, bgmslen: {}",
    //     song_config.notes.len(),
    //     song_config.bgms.len()
    // );

    // TODO for some reason we have leftover bgms, so just ignoring that for now
    if song_config.notes.is_empty() {
        //&& song_config.bgms.is_empty() {
        next_state.set(MyAppState::Results);
        println!("Switching to Results state");
    }
}

// TODO this seems to cause notes to randomly stop playing
fn debug_goto_results(
    mut song_config: ResMut<SongConfig>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        song_config.bgms.clear();
        song_config.notes.clear();
    }
}

pub struct BarsPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for BarsPlugin<S> {
    fn build(&self, app: &mut App) {
        app.init_resource::<BarMaterialResource>();
        app.insert_resource(SpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));
        // app.add_systems(
        //     Startup,
        //     setup_target_bars.run_if(in_state(self.state.clone())),
        // );
        app.add_systems(OnEnter(self.state.clone()), setup_target_bars);
        app.add_systems(
            Update,
            (
                spawn_bars,
                move_bars,
                despawn_bars,
                play_bgms,
                show_results_on_finished,
                // debug_goto_results,
            )
                .run_if(in_state(self.state.clone())),
        );
        app.add_systems(OnExit(self.state.clone()), despawn_ui);
    }
}
