extern crate dirs;
extern crate toml;

use model::Workspace;
use std;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use toml::de::Error;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub workspaces: Vec<Workspace>,
}

fn get_config_file() -> std::io::Result<File> {
    let home = match dirs::home_dir() {
        Some(path) => path,
        None => panic!("Couldn't find home dir."),
    };
    let path = home.join(".config/slack-rs/config.toml");
    File::open(path)
}

pub fn get_config() -> Result<Config, Error> {
    let input = get_config_file().expect("'~/.config/slack-rs/config.toml' does not exists.");
    let reader = BufReader::new(input);
    let mut buffer = String::new();

    for line in reader.lines() {
        buffer.push_str(&line.unwrap());
        buffer.push_str("\n");
    }

    toml::from_str(&buffer)
}
