use std::{
    fs::{File, create_dir},
    io::Write,
};


pub fn get_parent_directory() -> String {
    let dir = std::env::current_dir().unwrap_or_else(|e| {
        panic!("couldn't get current dir with error {e}")
    });


    match dir.file_name() {
        Some(v) => {
            v.to_os_string()
                .into_string()
                .unwrap_or_else(|string| {
                    panic!("not utf8. OsString is '{string:?}'")
                })
        }
        None => panic!("no file name"),
    }
}

pub fn create_files(
    path: &str,
    files: Vec<crate::file::FileData>,
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

pub fn create_and_get_directory(name: String) -> String {
    create_dir(name.clone()).unwrap_or_else(|e| {
        panic!("Attempt to create directory '{}' failed with error {}!", name, e);
    });

    name
}

pub fn create_and_get_sub_directory(
    parent: &str,
    sub: &str,
) -> String {
    let new = format!("{}/{}", parent, sub);

    create_and_get_directory(new.clone());

    new
}

#[must_use]
pub fn create_sub_directory_and_files(
    parent: &str,
    sub: &str,
    files: Vec<crate::file::FileData>,
) -> String {
    let path =
        create_and_get_sub_directory(parent, sub);

    create_files(&path, files);

    path
}

#[must_use]
pub fn create_directory_and_files(
    dir: String,
    files: Vec<crate::file::FileData>,
) -> String {
    let path =
        create_and_get_directory(dir.to_string());

    create_files(&path, files);

    path
}



