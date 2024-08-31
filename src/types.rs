use crate::consts::*;
use crate::new_bms_parser;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bms_rs::lex::command::Key;
use bms_rs::lex::command::ObjId;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

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
    Scratch,
}
impl Positions {
    /// Checks if a key that corresponds to this direction has been pressed
    pub fn key_just_pressed(&self, input: &ButtonInput<KeyCode>) -> bool {
        let keys: Vec<KeyCode> = match self {
            Positions::One => vec![KeyCode::KeyA],
            Positions::Two => vec![KeyCode::KeyS],
            Positions::Three => vec![KeyCode::KeyD],
            Positions::Four => vec![KeyCode::Space],
            Positions::Five => vec![KeyCode::KeyJ],
            Positions::Six => vec![KeyCode::KeyK],
            Positions::Seven => vec![KeyCode::KeyL],
            Positions::Scratch => vec![KeyCode::ShiftLeft, KeyCode::Semicolon],
        };

        keys.iter().any(|code| input.just_pressed(*code))
    }

    /// Checks if a key that corresponds to this direction is being pressed
    pub fn key_pressed(&self, input: &ButtonInput<KeyCode>) -> bool {
        let keys = match self {
            Positions::One => vec![KeyCode::KeyA],
            Positions::Two => vec![KeyCode::KeyS],
            Positions::Three => vec![KeyCode::KeyD],
            Positions::Four => vec![KeyCode::Space],
            Positions::Five => vec![KeyCode::KeyJ],
            Positions::Six => vec![KeyCode::KeyK],
            Positions::Seven => vec![KeyCode::KeyL],
            Positions::Scratch => vec![KeyCode::ShiftLeft, KeyCode::Semicolon],
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
            Positions::Scratch => -400.,
        }
    }
}

#[derive(Clone, Debug)]
/// Keeps track of when each note should spawn
pub struct NoteTime {
    pub spawn_time: f64,
    pub position: Positions,
    // pub audio_source: Handle<AudioSource>,
    pub audio_source_id: ObjId,
}

#[derive(Clone, Debug)]
pub struct BGM {
    pub spawn_time: f64,
    // pub audio_sources: Vec<Handle<AudioSource>>,
    pub audio_source_ids: Vec<ObjId>,
}

#[derive(Resource, Debug)]
pub struct SongConfig {
    pub notes: Vec<NoteTime>,
    pub bgms: Vec<BGM>,
    pub audio_handles: HashMap<ObjId, Handle<AudioSource>>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Position {}

pub fn load_config(file_path: &str, asset_server: &AssetServer) -> SongConfig {
    // test parse file
    // let file_path = "[Cres.]endtime/end_time_n.bms";
    println!("Loading file_path={}", file_path);
    let bms = new_bms_parser::new_parse(file_path);
    let bpm = bms.header.bpm.unwrap();
    let notes = bms.notes;
    let num_measures = notes.last_obj_time().unwrap().track.0 as f64; // can panic

    // TODO load sound and add to notetimes?
    let wav_files_map = bms.header.wav_files;

    // this is wrong i think
    // TODO if we assume all 4/4, then number of measure * 4 = number of beats
    //      then number of beats / bpm = length of song in seconds
    let num_beats = num_measures * 4.;
    let song_length = (num_beats / bpm) * 60.;
    println!("Num beats: {}", num_beats);
    println!("Song length: {}", song_length);
    // let measure_time = (bpm / num_measures) * 0.75;
    let measure_time = song_length / num_measures;
    println!("Measure length: {}", measure_time);

    let mut audio_handles_map: HashMap<ObjId, Handle<AudioSource>> = HashMap::new();

    let mut wavs_vec: Vec<(&ObjId, &PathBuf)> = wav_files_map.iter().collect();
    wavs_vec.sort_by_key(|&(key, _value)| key);
    wavs_vec.reverse();
    // println!("sorted_wavs: {:#?}", wavs_vec);

    let mut notetimes: Vec<NoteTime> = Vec::new();
    for note in notes.all_notes() {
        // if let 0..=20 = note.offset.track.0 {
        // println!("note: {:#?}", note);
        // }

        // load sound file if not already loaded
        let wav_id = note.obj;
        if !audio_handles_map.contains_key(&wav_id) {
            // let wav_file = wav_files_map.get(&wav_id).unwrap().to_str().unwrap();
            let wav_file = wav_files_map
                .get(&wav_id)
                .clone()
                .expect("Failed to get wav_file from map");
            // let path = format!("songs/{}", file_path);
            let path_buf = PathBuf::from(file_path);
            let parent_path = path_buf
                .parent()
                .to_owned()
                .expect("could not find parent path");
            let mut wav_path = parent_path.join(&wav_file);
            let mut check_path_str = format!("assets/{}", wav_path.clone().to_str().unwrap());
            let mut check_path = PathBuf::from(check_path_str);

            if !check_path.exists() {
                // println!("{:?} does not exist 1", check_path);
                wav_path.set_extension("ogg");
                check_path_str = format!("assets/{}", wav_path.clone().to_str().unwrap());
                check_path = PathBuf::from(check_path_str);

                if !check_path.exists() {
                    println!("ogg {:?} does not exist", check_path);
                }
            }

            let wav_handle: Handle<AudioSource> = asset_server.load(wav_path);
            audio_handles_map.insert(wav_id.to_owned(), wav_handle);
        }

        // determine spawn time based on measure, numerator, denominator -- probably wrong
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
            bms_rs::lex::command::Key::Scratch => Positions::Scratch,
            _ => Positions::One,
        };

        // TODO does note_offset also need to depend on bpm? time signature? etc?
        let start_time = measure as f64 * measure_time;
        let note_offset = (numerator as f64 / denominator as f64) * measure_time;
        let spawn_time = (start_time + note_offset) as f64;

        notetimes.push(NoteTime {
            spawn_time,
            position: key,
            audio_source_id: wav_id,
        });
    }

    let bgms = notes.bgms(); // 'notes' was moved earlier
                             // let mut bgms: Vec<BGM> = Vec::new();
    let mut bgms_config_list: Vec<BGM> = Vec::new();
    for bgm in bgms {
        let time = bgm.0;
        let obj_ids = bgm.1;

        // Determine spawn time
        // TODO spawn timing is definitely off
        //      if there are blank notes in the measures, we can determine time signature
        let measure = time.track.0;
        let start_time = measure as f64 * measure_time;
        let note_offset = (time.numerator as f64 / time.denominator as f64) * measure_time;
        let spawn_time = (start_time + note_offset) as f64;

        for id in obj_ids {
            if !audio_handles_map.contains_key(&id) {
                // println!("objid {:#?}", &id);
                // let default = PathBuf::from(r"bass_A#1.wav");
                let wav_file = wav_files_map.get(&id).clone().unwrap(); //_or(&default);
                let path_buf = PathBuf::from(file_path);
                let parent_path = path_buf
                    .parent()
                    .to_owned()
                    .expect("could not find parent path");
                let mut wav_path = parent_path.join(&wav_file);
                let mut check_path_str = format!("assets/{}", wav_path.clone().to_str().unwrap());
                let mut check_path = PathBuf::from(check_path_str);

                if !check_path.exists() {
                    // println!("{:?} does not exist 2", check_path);
                    wav_path.set_extension("ogg");
                    check_path_str = format!("assets/{}", wav_path.clone().to_str().unwrap());
                    check_path = PathBuf::from(check_path_str);

                    if !check_path.exists() {
                        println!("ogg {:?} does not exist", check_path);
                    }
                }

                let wav_handle: Handle<AudioSource> = asset_server.load(wav_path);
                audio_handles_map.insert(id.to_owned(), wav_handle);
            }
        }

        bgms_config_list.push(BGM {
            spawn_time,
            audio_source_ids: obj_ids.clone(),
        });
    }

    println!("bpm: {}", bpm);
    println!("num_measures: {}", num_measures);
    println!("wav_path_root: {:#?}", bms.header.wav_path_root);

    // println!("NoteTimes: {:#?}", notetimes);
    SongConfig {
        notes: notetimes,
        bgms: bgms_config_list,
        audio_handles: audio_handles_map,
    }
}
