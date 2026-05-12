#![allow(dead_code)]
use std::{
    collections::HashMap,
    ffi::OsStr,
    fmt::Write,
    fs::{self, DirEntry, File},
    io::Read,
    path::{Path, PathBuf},
    process::ExitCode,
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

fn count_lines_recursive<T: AsRef<Path>>(path: &T) -> Option<HashMap<Format, usize>> {
    let dir_contents = match fs::read_dir(path) {
        Ok(dir) => dir.map(|item| item.unwrap()),

        Err(ref e) => {
            match e.kind() {
                std::io::ErrorKind::NotADirectory => {
                    // this is pure garbage error handling lmfao
                    println!("Specified entry is a file, not directory, counting lines in it...");
                    let mut file = File::open(path).unwrap();
                    println!("{} lines found", count_lines_in_file(&mut file));
                    return None
                },

                std::io::ErrorKind::NotFound => {
                    eprintln!("Specified entry does not exist.");
                    return None
                }

                std::io::ErrorKind::PermissionDenied => {
                    eprintln!("This user does not have permissions to access the entry, aborting...");
                    return None
                }
                _ => {
                    eprintln!("This Error Should Not occur!");
                    return None;
                }

            }

        }

    };

    // NOTE:
    // Split traversing into threads, probably?
    // This is a good idea if the directory is somewhat large, otherwise creating threads becomes a
    // bottleneck.
    let mut result: HashMap<Format, usize> = HashMap::new();
    // use hashmap, because why the fuck not?

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

fn count_lines_in_file(f: &mut File) -> usize {
    let mut buf: [u8; _] = [0; IO_BUFSIZE + 1];
    let mut counter = 0;
    while let Ok(bytes) = f.read(&mut buf[..])
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
            if character == b'\n' {
                counter += 1
            }
        }
    }
    counter
}

fn construct_filedata(f: &DirEntry) -> FileData {
    let name = f.path();
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

    // main line counting logic goes here
    let lines_counter = count_lines_in_file(&mut current_file);

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

fn count_lines_in_directory<T: AsRef<Path>>(path: T) -> Result<(), std::io::Error> {
    let mut answer = String::new();

    let table = match count_lines_recursive(&path) {
        Some(map) => map,
        None => {
            eprintln!("There was an error reading target directory");
            return Err(std::io::Error::other("Unexpected Error Occured"));
        }
    };

    for (format, lines) in table {
        let _ = writeln!(answer, "{} files: {lines}", format.match_to_str());
    }

    println!("{}", answer);
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let args = cli::parseargs();
    count_lines_in_directory(args)
}
