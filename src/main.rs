#![allow(dead_code)]
use std::{
    collections::HashMap,
    ffi::OsStr,
    fmt::{Debug, Write as write},
    fs::{self, DirEntry, File},
    io::Read,
    path::Path,
};
use wcl_proc_macros::Matcher;
mod table;

const IO_BUFSIZE: usize = 1024 * 256;
const ELF_SIGNATURE: [u8; 4] = [0x7F, 0x45, 0x4C, 0x46];

#[derive(Debug, Eq, PartialEq, Hash, Matcher)]
enum Format {
    #[file_format("rs")]
    Rust,
    B,
    C,
    D,
    #[output("Non-Code Text")]
    Txt,
    Java,
    #[file_format("rb")]
    Ruby,
    Json,
    #[output("C++")]
    Cpp,
    #[file_format("cs")]
    #[output("C#")]
    CSharp,
    #[file_format("hs")]
    Haskell,
    #[output("JavaScript")]
    Js,
    Ada,
    Svelte,
    #[file_format("s")]
    Assembly,
    Dart,
    #[file_format("py")]
    Python,
    #[output("TypeScript")]
    Ts,
    Other,
    #[error_attr]
    ErrorOccured {
        message: String,
        file: String,
    },
}

#[derive(Debug)]
struct FileData {
    extension: Format,
    lines: usize,
}

fn count_lines_in_directory<T: AsRef<Path> + Debug>(path: T) -> String {
    let dir_contents = match fs::read_dir(&path) {
        Ok(dir) => dir.map(|item| item.unwrap()),
        Err(ref e) => {
            eprintln!("[ERROR] {:?}: {e}", path);
            return "".to_string();
        }
    };
    let dir_contents = dir_contents.filter(|item| item.metadata().unwrap().is_file());
    // use hashmap, because why the fuck not?
    let mut result: HashMap<Format, usize> = HashMap::new();

    for file_desc in dir_contents {
        let file_data = count_lines_in_file(&file_desc);
        let lines = file_data.lines;
        result
            .entry(file_data.extension)
            .and_modify(|val| *val += lines)
            .or_insert(lines);
    }
    process_hashtable(&result)
}

fn count_lines_in_file(file: &DirEntry) -> FileData {
    let mut buf: [u8; _] = [0; IO_BUFSIZE + 1];
    let name = file.path();
    let mut current_file = match File::open(&name) {
        Ok(f) => f,
        Err(ref e) => {
            eprintln!("{}", e);
            return FileData {
                extension: Format::ErrorOccured {
                    message: e.to_string(),
                    file: name.to_str().unwrap().to_string(),
                },
                lines: 0,
            };
        }
    };

    // main line counting logic goes here
    let mut lines_counter = 0;
    while current_file.read(&mut buf).unwrap() > 0 {
        // try to skip ELF executables as it makes no sense to count lines in binary files
        // whatsoever.
        // This approach is dumb and barely extendible
        // TODO: think of a way to skip most of well-known binary files
        if buf[0..4] == ELF_SIGNATURE {
            break;
        }
        for character in buf {
            match &character {
                b'\n' => lines_counter += 1,
                _ => continue,
            }
        }
    }

    // println!("{lines_counter} lines in file {}", name.to_str().unwrap());

    FileData {
        extension: get_file_ext(file.path().extension()),
        lines: lines_counter,
    }
}

fn get_file_ext(ext: Option<&OsStr>) -> Format {
    match ext {
        Some(val) => val
            .to_owned()
            .into_string()
            .unwrap()
            .parse::<Format>()
            .unwrap(),
        // TODO: Binary processing logic goes here
        None => Format::Other,
    }
}
fn process_hashtable(table: &HashMap<Format, usize>) -> String {
    let mut answer = String::new();

    for (format, lines) in table {
        let _ = writeln!(answer, "{} files: {lines}", format.match_to_str());
    }

    answer
}
fn main() {
    let a = count_lines_in_directory("test");
    let err = count_lines_in_directory("Laksjdlakjs");
    println!("{a}");
    println!("{err}");
}
