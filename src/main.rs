use std::{fs, env, path};

use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use home::home_dir;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub trashdir: String,
    pub deletion: u64,
}

impl Config {
    fn config_path() -> PathBuf {
        let mut path = home_dir().expect("Could not find home directory");
        path.push(".rumrc.toml");
        path
    }

    pub fn open() -> Self {
        let path = Self::config_path();

        if path.exists() {
            let content = fs::read_to_string(&path)
                .expect("Failed to read config file");
            toml::from_str(&content)
                .expect("Failed to parse config file")
        } else {
            let trashdir = home_dir()
                .map(|mut p| {
                    p.push(".rumtrash");
                    p.to_string_lossy().into_owned()
                })
                .expect("Could not determine home directory");
            fs::create_dir_all(path::Path::new(&trashdir)).unwrap();
            let default = Self {
                trashdir,
                deletion: 0,
            };
            default.save();
            default
        }
    }

    pub fn save(&self) {
        let path = Self::config_path();
        let toml_str = toml::to_string_pretty(self)
            .expect("Failed to serialize config");
        fs::write(path, toml_str)
            .expect("Failed to write config file");
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 0 {
        println!("Nothin' to remove boy.");
        return;
    }
    let mut conf = Config::open();
    println!("rumoving... You can find the remains at {}", conf.trashdir);
    conf.deletion += 1;
    fs::create_dir_all(get_trash_path("".to_string(), &conf)).unwrap();
    
    for arg in args {
        remove(arg, &conf);
    }
    conf.save();
}

fn remove(file: String, conf: &Config) {
    let fpath = path::Path::new(&file).canonicalize().unwrap();
    if !fpath.exists() {
        eprintln!("Error: {} does not exist.", file);
        std::process::exit(1);
    }
    println!("{}",
        get_trash_path(fpath.file_name().unwrap()
            .to_string_lossy().to_string(), conf));
    fs::rename(file, 
        get_trash_path(fpath.file_name().unwrap()
            .to_string_lossy().to_string(), conf))
        .unwrap();
}

fn get_trash_path(of: String, config: &Config) -> String {
    return format!("{}/{}/{}", config.trashdir, config.deletion, of);
}
