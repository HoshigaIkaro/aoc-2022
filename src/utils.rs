pub fn load_input(day: u8) -> String {
    let path = format!("{}/inputs/day_{day}.txt", env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(path).unwrap()
}