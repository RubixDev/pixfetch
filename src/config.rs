use std::{
    env,
    fs::{self, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
    process,
};

use crate::{
    cli::{Config, Info},
    error::Error,
};

impl Default for Config {
    fn default() -> Self {
        Self {
            max_width: None,
            alpha_threshold: None,
            show_colons: None,
            skip_cache: None,
            aliasing: None,
            gap: None,
            image_override: None,
            color_override: None,
            info_whitelist: None,
            info_blacklist: Some(vec![Info::Terminal]),
        }
    }
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
                    "The specified image path does not point to a file: `{}`",
                    path,
                )));
            }
        }
        if let Some(gap) = &self.gap {
            if !(0..=10).contains(gap) {
                return Err(Error::InvalidConfig(format!(
                    "The specified gap `{gap}` is not between 0 and 10"
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
