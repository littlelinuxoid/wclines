#![allow(dead_code)]
use std::{
    fs::{self, DirEntry, File, FileType},
    os::linux::fs::MetadataExt,
    path::Path,
};

enum SupportedFormat {
    RustFile,
    TextFile,
    CFile,
}

fn main() -> Result<(), std::io::Error> {
    let dir_contents = fs::read_dir(".")?.map(|item| item.unwrap());
    let dir_contents = dir_contents.filter(|item| item.metadata().unwrap().is_file());
    for entry in dir_contents.map(|dir_entry| get_file_extension(dir_entry)) {
        match entry.as_str() {
            "txt" => {
                println!("A txt file!")
            }
            "rs" => {
                println!("A Rust File!")
            }
            &_ => println!("Cringe happened!"),
        }
    }

    Ok(())
}
fn get_file_size(f: DirEntry) -> u64 {
    f.metadata().unwrap().len()
}
fn get_file_extension(f: DirEntry) -> String {
    let p = f.path();
    let os_ext = p.extension().unwrap();
    os_ext.to_str().unwrap().to_owned()
}
