use std::{
    fs::{File, create_dir},
    io::Write,
};

pub fn get_parent_directory() -> String {
    for (i, mut arg) in std::env::args().enumerate() {
        if i == 0 {
            let mut pos_from_last = 0;

            for (i, e)in arg.chars().enumerate() {
                if e == '/' || e == '\\' {
                    pos_from_last = i;
                }
            }

            return arg.split_off(pos_from_last);
        }
    }

    panic!("No First argument as program, something wrong with your OS?")
}

pub fn create_files(
    path: &str,
    files: Vec<crate::file::File>,
) {
    eprintln!("path: {}", path);

    for file in files {
        File::create(format!("{}/{}", path, file.get_name()))
            .unwrap_or_else(|e| 
                panic!("failed to create '{}' in path {} with error {}", file.get_name(), path, e)
            )
            .write_all(file.get_contents().as_bytes())
            .unwrap_or_else(|e| 
                panic!("failed to write contents in file '{}' with error {}", file.get_name(), e)
            );
    }
}

pub fn create_directory(name: String) -> String {
    create_dir(name.clone()).unwrap_or_else(|e| {
        panic!("Attempt to create directory '{}' failed with error {}!", name, e);
    });

    name
}

pub fn create_and_get_directory(
    parent: &str,
    sub: &str,
) -> String {
    let new = format!("{}/{}", parent, sub);

    create_directory(new.clone());

    new
}

#[must_use]
pub fn create_directory_and_files(
    parent: &str,
    sub: &str,
    files: Vec<crate::file::File>,
) -> String {
    let path =
        create_and_get_directory(parent, sub);

    create_files(&path, files);

    path
}
