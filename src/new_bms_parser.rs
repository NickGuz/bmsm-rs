use bms_rs::{
    lex::parse,
    parse::{rng::RngMock, Bms},
};
use encoding_rs::SHIFT_JIS;

pub fn new_parse(filename: &str) -> Bms {
    let filename_ = format!("assets/{}", filename);
    let data = std::fs::read(&filename_).expect("Filename not found");

    // First try to decode as UTF-8
    let source = match std::str::from_utf8(&data) {
        Ok(v) => v.to_owned(),
        Err(_) => {
            // If UTF-8 fails, try SHIFT-JIS
            let (cow, _encoding_used, _had_errors) = SHIFT_JIS.decode(&data);
            cow.into_owned()
        }
    };

    // let source = std::fs::read_to_string(filename_).expect("filename not found");
    let token_stream = parse(&source).expect("Must be parsed");
    let rng = RngMock([1]);
    let bms = Bms::from_token_stream(&token_stream, rng).expect("must be parsed");
    // println!("{:#?}", bms);

    bms
}
