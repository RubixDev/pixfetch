#![doc = include_str!("../README.md")]

use std::{fs::File, io::Read, process};

use ansipix::FilterType;
use cache::{read_cache, write_cache};
use clap::Parser;
use cli::{Config, Info};
use config::expand_path;

mod cache;
mod cli;
mod config;
mod distro;
mod error;
mod info;

pub use error::Result;
use strum::IntoEnumIterator;

pub const DEFAULT_MAX_WIDTH: u8 = 30;
pub const DEFAULT_ALPHA_THRESHOLD: u8 = 50;
pub const DEFAULT_ALIASING: bool = false;
pub const DEFAULT_SKIP_CACHE: bool = false;
pub const DEFAULT_SHOW_COLONS: bool = true;
pub const DEFAULT_GAP: u8 = 2;

fn main() {
    let mut sys = info::System::new();
    let flags = Config::parse();
    let config = match config::read_config() {
        Ok(conf) => match (Config {
            max_width: flags.max_width.or(conf.max_width),
            alpha_threshold: flags.alpha_threshold.or(conf.alpha_threshold),
            show_colons: flags.show_colons.or(conf.show_colons),
            skip_cache: flags.skip_cache.or(conf.skip_cache),
            aliasing: flags.aliasing.or(conf.aliasing),
            gap: flags.gap.or(conf.gap),
            color_override: flags.color_override.or(conf.color_override),
            image_override: flags.image_override.or(conf.image_override),
            info_whitelist: flags.info_whitelist.or(conf.info_whitelist),
            info_blacklist: flags.info_blacklist.or(conf.info_blacklist),
        }
        .validated())
        {
            Ok(config) => config,
            Err(e) => {
                eprintln!("\x1b[1;31mYour configuration (flags and/or config file) is invalid:\x1b[22m {}\x1b[0m", e);
                process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("\x1b[1;31mFailed to read config file:\x1b[22m {}\x1b[0m", e);
            process::exit(1);
        }
    };

    let infos: Vec<_> = Info::iter()
        .filter(|i| {
            if let Some(whitelist) = &config.info_whitelist {
                whitelist.contains(i)
            } else {
                true
            }
        })
        .filter(|i| {
            if let Some(blacklist) = &config.info_blacklist {
                !blacklist.contains(i)
            } else {
                true
            }
        })
        .collect();
    let infos: Vec<(Info, String)> = infos
        .iter()
        .map(|i| (i, i.get_info(&mut sys)))
        .filter(|i| i.1.is_some())
        .map(|i| (*i.0, i.1.unwrap()))
        .collect();

    let (mut col, img_bytes) = distro::get_distro_image(sys.os());
    let mut buf = vec![];
    let img_bytes = if let Some(path) = &config.image_override {
        let mut file = match File::open(expand_path(path)) {
            Ok(file) => file,
            Err(e) => {
                eprintln!(
                    "\x1b[1;31mCould not open custom image:\x1b[22m {}\x1b[0m",
                    e
                );
                process::exit(1);
            }
        };
        file.read_to_end(&mut buf).unwrap_or_else(|e| {
            eprintln!(
                "\x1b[1;31mCould not read custom image:\x1b[22m {}\x1b[0m",
                e
            );
            process::exit(1);
        });
        &buf[..]
    } else {
        img_bytes
    };

    let max_width = config.max_width.unwrap_or(DEFAULT_MAX_WIDTH).into();
    let (img_str, used_cache) = if let Some(cache) = read_cache(&config, img_bytes) {
        (cache.image, true)
    } else {
        match ansipix::of_image_bytes_with_filter(
            img_bytes,
            (max_width, 1000),
            config.alpha_threshold.unwrap_or(DEFAULT_ALPHA_THRESHOLD),
            false,
            if config.aliasing.unwrap_or(DEFAULT_ALIASING) {
                FilterType::CatmullRom
            } else {
                FilterType::Nearest
            },
        ) {
            Ok(img) => (img, false),
            Err(e) => {
                eprintln!(
                    "\x1b[1;31mFailed to create image pixel art:\x1b[22m {}\x1b[0m",
                    e
                );
                process::exit(1);
            }
        }
    };

    if let Some(color_override) = config.color_override {
        col = color_override;
    }
    let img: Vec<&str> = img_str.trim_matches('\n').split('\n').collect();
    let gap = " ".repeat(config.gap.unwrap_or(DEFAULT_GAP).into());

    for line in 0..(img.len().max(infos.len())) {
        if line < img.len() {
            print!("{gap}{}{gap}", img[line]);
        } else {
            print!("{gap}{}{gap}", " ".repeat(max_width));
        }

        if line < infos.len() {
            #[allow(clippy::format_in_format_args)]
            if infos[line].0 == Info::UserAtHostname {
                print!("\x1b[1;3{}m{}\x1b[0m", (col + 1) % 8, infos[line].1);
            } else if infos[line].0 == Info::Colors1 || infos[line].0 == Info::Colors2 {
                print!("{}", infos[line].1)
            } else if infos[line].0 != Info::Seperator {
                print!(
                    "\x1b[1;3{}m{: <9}\x1b[0m {}",
                    col,
                    format!(
                        "{:?}{}",
                        infos[line].0,
                        if config.show_colons.unwrap_or(DEFAULT_SHOW_COLONS) {
                            ":"
                        } else {
                            ""
                        }
                    ),
                    infos[line].1
                );
            }
        }
        println!();
    }

    if !used_cache {
        write_cache(&config, img_bytes, img_str);
    }
}
