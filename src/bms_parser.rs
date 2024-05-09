use bevy::prelude::*;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const BLOCK_SIZE: Vec3 = Vec3::new(120.0, 40.0, 0.0);
const BLOCK_SPEED: f32 = 200.0;

const BLOCK_COLOR: Color = Color::rgb(0.0, 0.0, 1.0);
const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Component, Debug)]
pub struct DataField {
    pub track: String,
    pub channel: String,
    pub message: String,
}

#[derive(Component, Debug)]
pub struct DataFieldMap(pub BTreeMap<String, Vec<DataField>>);

#[derive(Component, Debug)]
pub struct FileData {
    pub metadata: HashMap<String, String>,
    pub wav_data: HashMap<String, String>,
    pub bmp_data: HashMap<String, String>,
    pub main_data: BTreeMap<String, Vec<DataField>>,
}

pub fn parse(mut commands: Commands) {
    let file_path = "[Cres.]endtime/end_time_n.bms";
    let mut metadata: HashMap<String, String> = HashMap::new();
    let mut wav_data: HashMap<String, String> = HashMap::new();
    let mut bmp_data: HashMap<String, String> = HashMap::new();
    let mut main_data: Vec<DataField> = Vec::new();
    let mut main_data_map: BTreeMap<String, Vec<DataField>> = BTreeMap::new();

    let mut header_done = false;

    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            // println!("{}", line);
            if line.contains("HEADER FIELD") {
                println!("Processing header...");
            }

            if line.contains("MAIN DATA FIELD") {
                println!("Header done");
                header_done = true;
                // break;
            }

            if !line.starts_with("#") {
                continue;
            }

            // Process header
            if !header_done {
                let (command_name, command_value) = parse_header_line(line);

                if command_name.starts_with("WAV") {
                    wav_data.insert(command_name.to_string(), command_value.to_string());
                } else if command_name.starts_with("BMP") {
                    bmp_data.insert(command_name.to_string(), command_value.to_string());
                } else {
                    metadata.insert(command_name.to_string(), command_value.to_string());
                }

                continue;
            }

            // Process main data field
            // let mut split = line.split(':');
            let track_number = line.get(1..4).expect("Failed to parse track number");
            let channel_number = line.get(4..6).expect("Failed to parse channel number");
            let message = line.split(':').nth(1).expect("Failed to parse message");

            // main_data.push(DataField {
            //     track: track_number.to_string(),
            //     channel: channel_number.to_string(),
            //     message: message.to_string(),
            // });

            let d = DataField {
                track: track_number.to_string(),
                channel: channel_number.to_string(),
                message: message.to_string(),
            };

            if main_data_map.contains_key(track_number) {
                main_data_map
                    .get_mut(track_number)
                    .expect("expected track_number vec in data map")
                    .push(d);
            } else {
                let v = vec![d];
                main_data_map.insert(track_number.to_string(), v);
            }
        } // for line in lines
    }

    commands.spawn(DataFieldMap(main_data_map));

    println!("Metadata:\n{:#?}", metadata);
    // println!("Main Data:\n{:#?}", main_data_map.get("001"));

    let block_y = 0.0;

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
    //     crate::Block,
    //     crate::Collider,
    // ));

    // FileData {
    //     metadata,
    //     wav_data,
    //     bmp_data,
    //     main_data: main_data_map,
    // }
}

fn parse_header_line(line: String) -> (String, String) {
    let mut split = line.split_whitespace();
    let command_name = split
        .nth(0)
        .unwrap()
        .get(1..)
        .expect("Expected command name to not be empty");

    let command_value = split.remainder().unwrap_or("");

    (command_name.to_string(), command_value.to_string())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
