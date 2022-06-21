use std::{
    collections::hash_map::DefaultHasher,
    env,
    fs::{self, File},
    hash::{Hash, Hasher},
    io::{Read, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::{cli::Config, DEFAULT_ALIASING, DEFAULT_ALPHA_THRESHOLD, DEFAULT_MAX_WIDTH};

#[derive(Deserialize, Serialize)]
pub struct Cache {
    image_hash: i64,
    max_width: u8,
    alpha_threshold: u8,
    aliasing: bool,
    pub image: String,
}

#[inline]
fn cache_path() -> Option<String> {
    if let Ok(xdg_home) = env::var("XDG_CACHE_HOME") {
        Some(format!("{}/pixfetch/cache.toml", xdg_home))
    } else if let Ok(home) = env::var("HOME") {
        Some(format!("{}/.cache/pixfetch/cache.toml", home))
    } else {
        None
    }
}

pub fn read_cache(config: &Config, image: &[u8]) -> Option<Cache> {
    if config.skip_cache.unwrap_or(false) {
        return None;
    }

    let path = cache_path()?;

    let mut buf = String::new();
    let mut file = File::open(&path).ok()?;
    file.read_to_string(&mut buf).ok()?;

    let c = toml::from_str::<Cache>(&buf).ok()?;
    if c.max_width != config.max_width.unwrap_or(DEFAULT_MAX_WIDTH)
        || c.alpha_threshold != config.alpha_threshold.unwrap_or(DEFAULT_ALPHA_THRESHOLD)
        || c.aliasing != config.aliasing.unwrap_or(DEFAULT_ALIASING)
    {
        return None;
    }

    let mut s = DefaultHasher::new();
    image.hash(&mut s);
    let a = s.finish();
    if a != c.image_hash as u64 {
        return None;
    }
    Some(c)
}

pub fn write_cache(config: &Config, image: &[u8], image_str: String) -> Option<()> {
    if config.skip_cache.unwrap_or(false) {
        return None;
    }

    let path = cache_path()?;

    let mut s = DefaultHasher::new();
    image.hash(&mut s);
    let c = Cache {
        image_hash: s.finish() as i64,
        max_width: config.max_width.unwrap_or(DEFAULT_MAX_WIDTH),
        alpha_threshold: config.alpha_threshold.unwrap_or(DEFAULT_ALPHA_THRESHOLD),
        aliasing: config.aliasing.unwrap_or(DEFAULT_ALIASING),
        image: image_str,
    };

    fs::create_dir_all(Path::new(&path).parent().unwrap()).ok()?;
    let mut file = File::create(path).ok()?;
    file.write_all(toml::to_string(&c).ok()?.as_bytes()).ok()?;
    Some(())
}
