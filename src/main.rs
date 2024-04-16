#![feature(str_split_whitespace_remainder)]

use bevy::audio::AudioPlugin;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};
use std::collections::BTreeMap;

const BLOCK_SIZE: Vec3 = Vec3::new(120.0, 40.0, 0.0);
const BLOCK_SPEED: f32 = 200.0;

const BLOCK_COLOR: Color = Color::rgb(0.0, 0.0, 1.0);
const BACKGROUND_COLOR: Color = Color::rgb(0., 0., 0.);

mod bars;
use bars::BarsPlugin;
mod bms_parser;
mod consts;
mod new_bms_parser;
mod score;
mod types;
mod ui;
use score::ScoreResource;

// fn main() {
//     let file_data = bms_parser::test();

//     // TODO: now how do i go from this data to being able to play
//     // the notes in order

//     // TODO: also need to convert it into ECS architecture so that
//     // bevy can use it

//     println!("Metadata:\n{:#?}", file_data.metadata);
//     println!("Main Data:\n{:#?}", file_data.main_data.get("001"));
// }

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "BMSm".to_string(),
                resolution: WindowResolution::new(1280., 720.),
                // with_scale_factor_override(1.),
                resizable: false,
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .init_resource::<ScoreResource>()
        .add_plugins(BarsPlugin)
        .add_plugins(ui::UIPlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

#[derive(Resource, Debug)]
struct BmsTimer(Timer);

#[derive(Component)]
struct Beat;

fn play_beats(
    mut commands: Commands,
    // audio: Query<&AudioSink, With<MyMusic>>,
    time: Res<Time>,
    mut timer: ResMut<BmsTimer>,
    asset_server: Res<AssetServer>,
    query: Query<&bms_parser::DataFieldMap>,
) {
    // commands.spawn(())
    if timer.0.tick(time.delta()).just_finished() {
        // match audio.get_single() {
        //     Ok(sink) => sink.play(),
        //     Err(bevy::ecs::query::QuerySingleError::NoEntities(_)) => {
        //         println!("Failed to get audio sink, no entities");
        //     }
        //     Err(bevy::ecs::query::QuerySingleError::MultipleEntities(_)) => {
        //         println!("Failed to get audio sink, multiple entities");
        //     }
        // }
        let file_path = "piano_B3.wav";
        commands.spawn(AudioBundle {
            source: asset_server.load(file_path),
            ..default()
        });

        // for item in query.iter() {
        //     println!("{:#?}", item.0);
        // }
        // if let Ok(sink) = audio.get_single() {
        //     sink.play();
        // }
    }
}

// fn iterate_through_blocks(
//     mut commands: Commands,
//     time: Res<Time>,
//     mut timer: ResMut<BmsTimer>,
//     query: Query<&bms_parser::DataFieldMap>,
// ) {
//     // commands.insert_resource(BmsTimer(Timer::from_seconds(10.0, TimerMode::Repeating)));
//     println!("TIMER VAL {:#?}", timer);
//     // let mut timer = BmsTimer(Timer::from_seconds(10.0, TimerMode::Repeating));

//     if timer.0.tick(time.delta()).just_finished() {
//         for item in query.iter() {
//             println!("DATA FIELD MAP");
//             println!("{:#?}", item.0)
//         }
//     }
// }

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Block;

#[derive(Component)]
struct Collider;

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Component)]
struct MyMusic;

fn setup(
    mut commands: Commands,
    // window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let config = types::load_config("songs/[Cres.]endtime/end_time_n.bms", asset_server);

    // Camera
    commands.spawn(Camera2dBundle::default());
    // commands.spawn(UiCameraBundle::default());
    commands.insert_resource(config);
    // .insert_resource(config);

    // test parse file
    // let file_path = "[Cres.]endtime/end_time_n.bms";
    // let bms = new_bms_parser::new_parse(file_path);
    // let notes = bms.notes;

    // for note in notes.all_notes() {
    //     println!("note: {:#?}", note);
    // }

    // Block
    // let window = window_query.get_single().unwrap();
    // let height = window.height();
    // let block_y = height / 2.0; // origin is at center, so divide by 2 to get top

    // commands.spawn((
    //     SpriteBundle {
    //         transform: Transform {
    //             translation: Vec3::new(0.0, block_y, 0.0),
    //             scale: BLOCK_SIZE,
    //             ..default()
    //         },
    //         sprite: Sprite {
    //             color: BLOCK_COLOR,
    //             ..default()
    //         },
    //         ..default()
    //     },
    //     Block,
    //     Collider,
    // ));

    // let file_path = "piano_B3.wav";
    // commands.spawn((
    //     AudioBundle {
    //         source: asset_server.load(file_path),
    //         ..default()
    //     },
    //     MyMusic,
    // ));

    // commands.insert_resource(BmsTimer(Timer::from_seconds(10.0, TimerMode::Repeating)));
}

fn move_block(
    mut query: Query<&mut Transform, With<Block>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    if query.is_empty() {
        return;
    }

    let mut block_transform = query.single_mut();
    let direction = -1.0;

    let mut new_block_position =
        block_transform.translation.y + direction * BLOCK_SPEED * time.delta_seconds();

    // Update the block position
    let window = window_query.get_single().unwrap();
    let height = window.height();

    if new_block_position < -height / 2.0 {
        new_block_position = height / 2.0;
    }

    block_transform.translation.y = new_block_position;
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn hit_block(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Block>>,
    // query: Query<Entity, With<Block>>,
    // time: Res<Time>,
) {
    if keyboard_input.pressed(KeyCode::Slash) {
        // let entity_result = query.get_single();

        // if let Ok(entity_id) = entity_result {
        //     commands.entity(entity_id).despawn();
        // }

        if let Ok(mut transform) = query.get_single_mut() {
            transform.translation.y = 0.0;
        }
    }
}
