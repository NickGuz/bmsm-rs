use crate::consts::*;
use crate::new_bms_parser;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;
use bms_rs::lex::command::Key;
use serde_derive::{Deserialize, Serialize};

// #[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Positions {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}
impl Positions {
    /// Checks if a key that corresponds to this direction has been pressed
    pub fn key_just_pressed(&self, input: &ButtonInput<KeyCode>) -> bool {
        let keys = match self {
            Positions::One => [KeyCode::KeyA],
            Positions::Two => [KeyCode::KeyS],
            Positions::Three => [KeyCode::KeyD],
            Positions::Four => [KeyCode::Space],
            Positions::Five => [KeyCode::KeyJ],
            Positions::Six => [KeyCode::KeyK],
            Positions::Seven => [KeyCode::KeyL],
        };

        keys.iter().any(|code| input.just_pressed(*code))
    }

    /// Checks if a key that corresponds to this direction is being pressed
    pub fn key_pressed(&self, input: &ButtonInput<KeyCode>) -> bool {
        let keys = match self {
            Positions::One => [KeyCode::KeyA],
            Positions::Two => [KeyCode::KeyS],
            Positions::Three => [KeyCode::KeyD],
            Positions::Four => [KeyCode::Space],
            Positions::Five => [KeyCode::KeyJ],
            Positions::Six => [KeyCode::KeyK],
            Positions::Seven => [KeyCode::KeyL],
        };

        // println!("is {:#?} pressed?", input);
        keys.iter().any(|code| input.pressed(*code))
    }

    /// Returns the correct x coordinate for a bar with this position
    pub fn x(&self) -> f32 {
        match self {
            Positions::One => -300.,
            Positions::Two => -200.,
            Positions::Three => -100.,
            Positions::Four => 0.,
            Positions::Five => 100.,
            Positions::Six => 200.,
            Positions::Seven => 300.,
        }
    }
}

#[derive(Clone, Debug)]
/// Keeps track of when each note should spawn
pub struct NoteTime {
    pub spawn_time: f64,
    pub position: Positions,
    pub audio_source: Handle<AudioSource>,
}

#[derive(Resource, Debug)]
pub struct SongConfig {
    pub notes: Vec<NoteTime>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Position {}

pub fn load_config(asset_server: Res<AssetServer>) -> SongConfig {
    // test parse file
    let file_path = "[Cres.]endtime/end_time_n.bms";
    let bms = new_bms_parser::new_parse(file_path);
    let bpm = bms.header.bpm.unwrap();
    let notes = bms.notes;
    let num_measures = notes.last_obj_time().unwrap().track.0 as f64; // can panic

    // TODO load sound and add to notetimes?
    let wav_files_map = bms.header.wav_files;

    // this is wrong i think
    let measure_time = (bpm / num_measures) * 0.75;

    let mut notetimes: Vec<NoteTime> = Vec::new();
    for note in notes.all_notes() {
        if let 0..=20 = note.offset.track.0 {
            println!("note: {:#?}", note);
        }

        // load sound file
        let wav_id = note.obj;
        // let wav_file = wav_files_map.get(&wav_id).unwrap().to_str().unwrap();
        let wav_file = wav_files_map.get(&wav_id).clone().unwrap();
        let wav_handle: Handle<AudioSource> = asset_server.load(wav_file.to_owned());

        // determine spawn time based on measure, numerator, denominator -- probably wrong
        // TODO: yeah this doesn't work
        // this assumes that a measure is equal to a second i think, but that's obviously not true
        // need to use bpm and figure this out
        let numerator = note.offset.numerator;
        let denominator = note.offset.denominator;
        let measure = note.offset.track.0;

        // this is also quite stupid
        let key = match note.key {
            bms_rs::lex::command::Key::Key1 => Positions::One,
            bms_rs::lex::command::Key::Key2 => Positions::Two,
            bms_rs::lex::command::Key::Key3 => Positions::Three,
            bms_rs::lex::command::Key::Key4 => Positions::Four,
            bms_rs::lex::command::Key::Key5 => Positions::Five,
            bms_rs::lex::command::Key::Key6 => Positions::Six,
            bms_rs::lex::command::Key::Key7 => Positions::Seven,
            _ => Positions::One,
        };

        // TODO does note_offset also need to depend on bpm? time signature? etc?
        let start_time = measure as f64 * measure_time;
        let note_offset = (numerator as f64 / denominator as f64) * measure_time;
        let spawn_time = (start_time + note_offset) as f64;

        notetimes.push(NoteTime {
            spawn_time,
            position: key,
            audio_source: wav_handle,
        })
    }
    println!("bpm: {}", bpm);
    println!("num_measures: {}", num_measures);
    println!("wav_path_root: {:#?}", bms.header.wav_path_root);

    // println!("NoteTimes: {:#?}", notetimes);
    SongConfig { notes: notetimes }
}
