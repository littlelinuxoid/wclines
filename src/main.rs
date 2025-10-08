#![allow(dead_code)]
use std::{
    collections::HashMap,
    ffi::OsStr,
    fmt::{Debug, Write},
    fs::{self, DirEntry, File},
    io::Read,
    path::{Path, PathBuf},
};
mod cli;
mod format;
mod table;
use format::Format;

const IO_BUFSIZE: usize = 1024 * 256;
const ELF_SIGNATURE: [u8; 4] = [0x7F, 0x45, 0x4C, 0x46];

struct FileData {
    extension: Format,
    path: PathBuf,

    lines: usize,
}
impl Default for FileData {
    fn default() -> Self {
        Self {
            extension: Format::Other,
            path: ".".into(),
            lines: 0,
        }
    }
}

fn count_lines_recursive<T: AsRef<Path> + Debug>(path: &T) -> Option<HashMap<Format, usize>> {
    let dir_contents = match fs::read_dir(&path) {
        Ok(dir) => dir.map(|item| item.unwrap()),
        Err(ref e) if e.kind() == std::io::ErrorKind::NotADirectory => {
            // this is pure garbage error handling lmfao
            println!("Specified entry is a file, not directory, counting lines in it...");
            let mut file = File::open(&path).unwrap();
            println!("{} lines found", count_lines_in_file(&mut file));
            std::process::exit(0);
        }
        Err(ref e) => {
            eprintln!("[ERROR] {:?}: {e}", path);
            return None;
        }
    };
    // IDEA:
    // Split traversing into threads, probably?
    // use hashmap, because why the fuck not?
    let mut result: HashMap<Format, usize> = HashMap::new();

    for file_desc in dir_contents {
        let mdata = file_desc.metadata().unwrap();
        if mdata.is_file() {
            let file_data = construct_filedata(&file_desc);
            let lines = file_data.lines;
            result
                .entry(file_data.extension)
                .and_modify(|val| *val += lines)
                .or_insert(lines);
        } else if mdata.is_dir() {
            let step = match count_lines_recursive(&file_desc.path()) {
                Some(map) => map,
                None => continue,
            };

            result.extend(step);
        }
    }
    Some(result)
}
fn count_lines_in_file(file: &mut File) -> usize {
    let mut buf: [u8; _] = [0; IO_BUFSIZE + 1];
    let mut lines_counter = 0;
    while let Ok(bytes) = file.read(&mut buf)
        && bytes > 0
    {
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
    lines_counter
}

fn construct_filedata(file: &DirEntry) -> FileData {
    let name = file.path();
    let mut current_file = match File::open(&name) {
        Ok(f) => f,
        Err(ref e) => {
            eprintln!("{}", e);
            return FileData {
                path: name,
                extension: Format::Other,
                lines: 0,
            };
        }
    };
    let lines_counter = count_lines_in_file(&mut current_file);

    // main line counting logic goes here

    FileData {
        extension: get_file_ext(name.extension()),
        path: name,
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
        None => Format::Other,
    }
}
fn count_lines_in_directory<T: AsRef<Path> + Debug>(path: T) -> String {
    let mut answer = String::new();

    let table = match count_lines_recursive(&path) {
        Some(map) => map,
        None => {
            eprintln!("There was an error reading target directory");
            std::process::exit(1)
        }
    };
    for (format, lines) in table {
        let _ = writeln!(answer, "{} files: {lines}", format.match_to_str());
    }

    answer
}
fn main() {
    let args = cli::parseargs();
    println!("{}", count_lines_in_directory(args))
}
