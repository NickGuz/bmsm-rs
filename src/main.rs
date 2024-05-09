#![feature(str_split_whitespace_remainder)]
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};
use bevy_kira_audio::prelude::*;
use std::collections::BTreeMap;

const BLOCK_SIZE: Vec3 = Vec3::new(120.0, 40.0, 0.0);
const BLOCK_SPEED: f32 = 200.0;

const BLOCK_COLOR: Color = Color::rgb(0.0, 0.0, 1.0);
const BACKGROUND_COLOR: Color = Color::rgb(0., 0., 0.);

mod bars;
use bars::BarsPlugin;
mod bms_parser;
mod consts;
mod menu;
mod new_bms_parser;
mod results;
mod score;
mod time;
mod types;
mod ui;
mod user_settings;
use consts::MyAppState;
use menu::MenuPlugin;
use results::ResultsPlugin;
use score::ScoreResource;
use time::TimePlugin;
use user_settings::UserSettings;

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
        .add_plugins(AudioPlugin)
        .insert_state(MyAppState::SongSelect)
        .init_resource::<ScoreResource>()
        .init_resource::<UserSettings>()
        .add_plugins(MenuPlugin {
            state: MyAppState::SongSelect,
        })
        .add_plugins(BarsPlugin {
            state: MyAppState::InGame,
        })
        .add_plugins(ui::UIPlugin {
            state: MyAppState::InGame,
        })
        .add_plugins(ResultsPlugin {
            state: MyAppState::Results,
        })
        .add_plugins(TimePlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        //.add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    // window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let config = types::load_config("songs/[Cres.]endtime/end_time_n.bms", &asset_server);

    // Camera
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(config);
}
