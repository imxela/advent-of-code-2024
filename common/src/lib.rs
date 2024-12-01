#![allow(clippy::cargo_common_metadata)]

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn read_input() -> String {
    let input_content: String =
        std::fs::read_to_string(std::env::current_dir().unwrap().join("res/input"))
            .expect("failed to read AoC input file!");

    input_content
}
