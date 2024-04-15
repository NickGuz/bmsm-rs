use bms_rs::{
    lex::parse,
    parse::{rng::RngMock, Bms},
};

pub fn new_parse(filename: &str) -> Bms {
    let filename_ = format!("assets/{}", filename);
    let source = std::fs::read_to_string(filename_).expect("filename not found");
    let token_stream = parse(&source).expect("Must be parsed");
    let rng = RngMock([1]);
    let bms = Bms::from_token_stream(&token_stream, rng).expect("must be parsed");
    // println!("{:#?}", bms);

    bms
}
