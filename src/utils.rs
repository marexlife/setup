use std::{
    fs::{File, create_dir},
    io::Write,
};


pub fn to_screaming_snake_case(text: String) -> String {
    let mut buf = String::new();
    let mut pervious: Option<char> = None;

    
    for e in text.chars() {
        if e.is_uppercase() {
            if let Some(v) = pervious {
                if v.is_lowercase() {
                    buf.push('_');
                }
            }
            
            buf.push(e);
        } else {
            buf.push(to_upper(e));
        }

        pervious = Some(e);
    }

    fn to_upper(c: char) -> char {
        match c {
            'a' => 'A',
            'b' => 'B',
            'c' => 'C',
            'd' => 'D',
            'e' => 'E',
            'f' => 'F',
            'g' => 'G',
            'h' => 'H',
            'i' => 'I',
            'j' => 'J',
            'k' => 'K',
            'l' => 'L',
            'm' => 'M',
            'n' => 'N',
            'o' => 'O',
            'p' => 'P',
            'q' => 'Q',
            'r' => 'R',
            's' => 'S',
            't' => 'T',
            'u' => 'U',
            'v' => 'V',
            'w' => 'W',
            'x' => 'X',
            'y' => 'Y',
            'z' => 'Z',
            'A' => 'A',
            'B' => 'B',
            'C' => 'C',
            'D' => 'D',
            'E' => 'E',
            'F' => 'F',
            'G' => 'G',
            'H' => 'H',
            'I' => 'I',
            'J' => 'J',
            'K' => 'K',
            'L' => 'L',
            'M' => 'M',
            'N' => 'N',
            'O' => 'O',
            'P' => 'P',
            'Q' => 'Q',
            'R' => 'R',
            'S' => 'S',
            'T' => 'T',
            'U' => 'U',
            'V' => 'V',
            'W' => 'W',
            'X' => 'X',
            'Y' => 'Y',
            'Z' => 'Z',
            '_' => '_',
            '-' => '_',
            _ => '_',
        }
    }

    buf
}

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
    files: Vec<crate::file_data::FileData>,
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
    files: Vec<crate::file_data::FileData>,
) -> String {
    let path =
        create_and_get_sub_directory(parent, sub);

    create_files(&path, files);

    path
}

#[must_use]
pub fn create_directory_and_files(
    dir: String,
    files: Vec<crate::file_data::FileData>,
) -> String {
    let path =
        create_and_get_directory(dir.to_string());

    create_files(&path, files);

    path
}



