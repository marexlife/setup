use std::{
    fs::{File, create_dir},
    io::Write,
};

pub fn to_snake_case(text: &str) -> String {
    let mut previous: Option<char> = None;
    let mut buf = String::new();

    for e in text.chars() {
        let mut previous_was_underscore = false;

        if let Some(previous) = previous {
            if previous.is_uppercase() {
                previous_was_underscore = true;
            }
        }

        if e.is_uppercase() && !previous_was_underscore {
            buf.push('_');
            buf.push(to_lower(e));

            previous = Some(e);
            continue;
        }

        buf.push(to_lower(e));

        previous = Some(e);
    }

    buf
}

pub fn to_pascal_case(text: &str) -> String {
    let mut previous: Option<char> = None;
    let mut buf = String::new();

    for (i, e) in text.chars().enumerate() {
        let c: Option<char>  = match e {
            _ if i == 0 => Some(to_upper(e)),
            '_' => None,
            e => {
                let f = || {
                    if let Some(previous) = previous {
                        if previous == '_' {
                            return Some(to_upper(e));
                        }
                    } 
    
                    Some(to_lower(e))
                };

                f()
            },
        };

        
        if let Some(c) = c {
            buf.push(c);
        }
        
        previous = Some(e);
    }

    buf
}

pub fn to_screaming_snake_case(text: &str) -> String {
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

fn to_lower(c: char) -> char {
    match c {
        'A' => 'a',
        'B' => 'b',
        'C' => 'c',
        'D' => 'd',
        'E' => 'e',
        'F' => 'f',
        'G' => 'g',
        'H' => 'h',
        'I' => 'i',
        'J' => 'j',
        'K' => 'k',
        'L' => 'l',
        'M' => 'm',
        'N' => 'n',
        'O' => 'o',
        'P' => 'p',
        'Q' => 'q',
        'R' => 'r',
        'S' => 's',
        'T' => 't',
        'U' => 'u',
        'V' => 'v',
        'W' => 'w',
        'X' => 'x',
        'Y' => 'y',
        'Z' => 'z',
        'a' => 'a',
        'b' => 'b',
        'c' => 'c',
        'd' => 'd',
        'e' => 'e',
        'f' => 'f',
        'g' => 'g',
        'h' => 'h',
        'i' => 'i',
        'j' => 'j',
        'k' => 'k',
        'l' => 'l',
        'm' => 'm',
        'n' => 'n',
        'o' => 'o',
        'p' => 'p',
        'q' => 'q',
        'r' => 'r',
        's' => 's',
        't' => 't',
        'u' => 'u',
        'v' => 'v',
        'w' => 'w',
        'x' => 'x',
        'y' => 'y',
        'z' => 'z',
        '_' => '_',
        '-' => '_',
        _ => '_',
    }
}