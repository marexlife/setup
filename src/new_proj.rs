use std::{
    env::current_dir,
    fs::{File, create_dir},
};

pub fn new_proj(name: &str) {
    create_dir(name).unwrap_or_else(|e| {
        panic!("failed to create project directory with error {e}");
    });

    let files = ["CMakeLists.txt", "clang-tidy"];

    for file in files {
        File::create_new(name)
            .unwrap_or_else(|e| panic!("failed to create '{file}' with error {e}"));
    }
}
