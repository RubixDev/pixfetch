use clap::Parser;
use serde::Deserialize;
use std::{
    env,
    fs::{self, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
    process,
};

use crate::{error::Error, info::Info};

/// Another fetch program with variable sized pixel images
#[derive(Debug, Deserialize, Parser, Default)]
#[clap(author)]
pub struct Config {
    /// The maximum width in pixels of the image
    ///
    /// - Must be an integer between 5 and 50
    #[clap(long)]
    pub max_width: Option<u8>,

    /// The minimum alpha value for pixels to be displayed
    ///
    /// - Must be an integer between 0 and 255
    #[clap(long)]
    pub alpha_threshold: Option<u8>,

    /// Override the main color
    ///
    /// - Must be an integer between 0 and 7
    ///
    /// - The color for the user@hostname will be this + 1
    #[clap(long)]
    pub color_override: Option<u8>,

    /// Path to a custom image to be used instead of the OS logo
    #[clap(long)]
    pub image_override: Option<String>,

    /// A list of infos to not show
    ///
    /// - See possible values in default config file
    #[clap(long, min_values = 0)]
    pub info_blacklist: Option<Vec<Info>>,
}

impl Config {
    pub fn validated(self) -> crate::Result<Self> {
        if let Some(width) = &self.max_width {
            if !(5..=50).contains(width) {
                return Err(Error::InvalidConfig(format!(
                    "The specified max_width `{}` is not between 5 and 50",
                    width,
                )));
            }
        }
        if let Some(color) = &self.color_override {
            if !(0..=7).contains(color) {
                return Err(Error::InvalidConfig(format!(
                    "The specified color `{}` is not between 0 and 7",
                    color,
                )));
            }
        }
        if let Some(path) = &self.image_override {
            if !expand_path(path).is_file() {
                return Err(Error::InvalidConfig(format!(
                    "The specified image is not a file: `{}`",
                    path,
                )));
            }
        }
        Ok(self)
    }
}

pub fn expand_path(path: &str) -> PathBuf {
    if path.starts_with("~/") {
        match env::var("HOME") {
            Ok(home) => PathBuf::from(home + &path[1..]),
            Err(_) => {
                eprintln!("\x1b[31mFailed to determine HOME directory, please specify the full path in your config file.\x1b[0m");
                process::exit(1);
            }
        }
    } else {
        PathBuf::from(path)
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

                return Ok(Config::default());
            }
            _ => return Err(e.into()),
        },
    };
    file.read_to_string(&mut buf)?;

    Ok(toml::from_str::<Config>(&buf)?)
}
