use serde::Deserialize;
use std::{
    env,
    fs::{self, File},
    io::{self, Read, Write},
    path::Path,
};

use crate::info::Info;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub max_width: u8,
    pub alpha_threshold: Option<u8>,
    pub color_override: Option<u8>,
    pub image_override: Option<String>,
    #[serde(default = "Vec::new")]
    pub info_blacklist: Vec<Info>,
}

impl Config {
    fn validated(self) -> crate::Result<Self> {
        // TODO: validate
        Ok(self)
    }
}

pub fn read_config() -> crate::Result<Config> {
    let path = if let Ok(xdg_home) = env::var("XDG_CONFIG_HOME") {
        format!("{}/pixfetch/config.toml", xdg_home)
    } else if let Ok(home) = env::var("HOME") {
        format!("{}/.config/pixfetch/config.toml", home)
    } else {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Neither an XDG_CONFIG_HOME nor a HOME environment variable is set",
        )
        .into());
    };

    let mut buf = String::new();
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                println!(
                    "\x1b[33mNo config file found, creating a default in `{}`...\x1b[0m",
                    path
                );
                fs::create_dir_all(Path::new(&path).parent().unwrap())?;
                let mut file = File::create(path)?;
                file.write_all(include_bytes!("default_config.toml"))?;

                return Ok(Config {
                    max_width: 30,
                    alpha_threshold: None,
                    color_override: None,
                    image_override: None,
                    info_blacklist: vec![],
                });
            }
            _ => return Err(e.into()),
        },
    };
    file.read_to_string(&mut buf)?;

    toml::from_str::<Config>(&buf)?.validated()
}
