use std::process;

use ansipix::ImageFormat;
use config::expand_path;
use info::Info;

mod config;
mod error;
mod info;

pub use error::Result;

fn main() {
    let sys = info::System::new();
    let config = match config::read_config() {
        Ok(conf) => conf,
        Err(e) => {
            eprintln!("\x1b[1;31mFailed to read config file:\x1b[22m {}\x1b[0m", e);
            process::exit(1);
        }
    };

    let infos: Vec<_> = vec![
        Info::UserAtHostname,
        Info::Os,
        Info::Host,
        Info::Kernel,
        Info::Uptime,
        Info::Packages,
        Info::Shell,
        Info::Terminal,
        Info::Cpu,
        Info::Memory,
        Info::Swap,
        Info::Battery,
        Info::Seperator,
        Info::Colors1,
        Info::Colors2,
    ]
    .iter()
    .filter(|i| !config.info_blacklist.contains(i))
    .copied()
    .collect();
    let infos: Vec<(Info, String)> = infos
        .iter()
        .map(|i| (i, i.get_info(&sys)))
        .filter(|i| i.1.is_some())
        .map(|i| (*i.0, i.1.unwrap()))
        .collect();

    let mut col: u8;
    let img_bytes = match sys.os() {
        Some(distro) => {
            if distro == "Arch Linux" {
                col = 4;
                &include_bytes!("../logos/arch.png")[..]
            } else if distro.contains("Android") {
                col = 2;
                &include_bytes!("../logos/android.png")[..]
            } else if distro.contains("Debian") {
                col = 1;
                &include_bytes!("../logos/debian.png")[..]
            } else {
                col = 3;
                &include_bytes!("../logos/tux.png")[..]
            }
        }
        None => {
            col = 3;
            &include_bytes!("../logos/tux.png")[..]
        }
    };
    if let Some(color_override) = config.color_override {
        col = color_override;
    }
    let img_str = match if let Some(path) = config.image_override {
        let path = expand_path(&path);
        ansipix::of_image_file(
            path,
            (config.max_width.into(), 1000),
            config.alpha_threshold.unwrap_or(50),
            false,
        )
    } else {
        ansipix::of_image_bytes_with_format(
            img_bytes,
            (config.max_width.into(), 1000),
            config.alpha_threshold.unwrap_or(50),
            false,
            ImageFormat::Png,
        )
    } {
        Ok(img) => img,
        Err(e) => {
            eprintln!("\x1b[1mFailed to create image pixel art:\x1b[0m {}", e);
            process::exit(1);
        }
    };
    let img: Vec<&str> = img_str.trim_matches('\n').split('\n').collect();

    for line in 0..(img.len().max(infos.len())) {
        if line < img.len() {
            print!("{}  ", img[line]);
        } else {
            print!("{}  ", " ".repeat(config.max_width.into()));
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
                    format!("{:?}:", infos[line].0),
                    infos[line].1
                );
            }
        }
        println!();
    }
}
