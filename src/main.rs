use rand::random_range;
use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
    process,
};

const DIR: &str = "assets";
const SEPPARTOR: &str = "%";

fn main() {
    let dir_files: Vec<PathBuf> = match fs::read_dir(DIR) {
        Err(_) => {
            eprintln!("Не найдена папка с фразами.");
            process::exit(1);
        }
        Ok(vec) => vec.flatten().filter_map(|e| Some(e.path())).collect(),
    };

    let rand_num = random_range(0..dir_files.len());

    if let Ok(mut file) = File::open(&dir_files[rand_num]) {
        let mut s = String::new();

        if let Ok(_) = file.read_to_string(&mut s) {
            let s: Vec<&str> = s.split(SEPPARTOR).collect();
            let rand_num = random_range(0..s.len() - 1);

            println!("{:#}", s[rand_num]);
        } else {
            eprintln!("Не удалось прочитать файл.");
            process::exit(1);
        }
    } else {
        eprintln!("Ну удалось открыть файл.");
        process::exit(1);
    };
}
