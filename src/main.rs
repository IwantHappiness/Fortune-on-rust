use rand::Rng;
use std::{
    fs::{self, DirEntry, File},
    io::Read,
    path::PathBuf,
};

const DIR: &str = "src/lib";

fn main() {
    let dir_files = match fs::read_dir(&DIR) {
        Err(e) => panic!("Не найдена папка с фразами: {e}"),
        Ok(vec) => vec,
    };

    let dir_files: Vec<DirEntry> = dir_files.map(|i| i.unwrap()).collect();
    let path: Vec<PathBuf> = dir_files.iter().map(|element| element.path()).collect();

    let rand_num = rand::thread_rng().gen_range(0..dir_files.len());

    let mut file = File::open(&path[rand_num]).unwrap();

    let mut s = String::new();

    file.read_to_string(&mut s).unwrap();
    let s: Vec<&str> = s.split('%').collect();
    let rand_num2 = rand::thread_rng().gen_range(0..s.len() - 1);
    println!("{:#}", s[rand_num2]);
}
