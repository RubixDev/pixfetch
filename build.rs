use std::{env, fs, path::PathBuf};

use clap::CommandFactory;
use clap_complete::{generate_to, Shell};
use clap_mangen::Man;
use cli::Config;

#[path = "src/cli.rs"]
mod cli;

fn main() -> std::io::Result<()> {
    let outdir = match env::var_os("OUT_DIR") {
        Some(outdir) => outdir,
        None => return Ok(()),
    };
    let mut cmd = Config::command();

    generate_to(Shell::Bash, &mut cmd, "pixfetch", &outdir)?;
    generate_to(Shell::Fish, &mut cmd, "pixfetch", &outdir)?;
    generate_to(Shell::Zsh, &mut cmd, "pixfetch", &outdir)?;

    let man = Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    fs::write(PathBuf::from(&outdir).join("pixfetch.1"), buffer)?;

    println!(
        "cargo:warning=completion files and manpage generated in {:?}",
        outdir
    );

    Ok(())
}
