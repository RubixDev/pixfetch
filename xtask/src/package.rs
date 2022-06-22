use std::{fs, path::PathBuf, process::Command};

use crate::{project_root, Result};

const TARGETS: &[&str] = &[
    "aarch64-linux-android",
    "aarch64-unknown-linux-musl",
    "arm-unknown-linux-musleabihf",
    "i586-unknown-linux-musl",
    "i686-unknown-linux-musl",
    "x86_64-unknown-linux-musl",
];

pub fn main() -> Result<()> {
    let bin = *crate::BIN;
    let version = *crate::VERSION;

    fs::remove_dir_all(project_root().join("bin")).unwrap_or(());

    for target in TARGETS {
        eprintln!("Compiling for target {target}...");

        let status = Command::new("cross")
            .current_dir(project_root())
            .args(&["build", "--release", "--target", target])
            .status()?;
        if !status.success() {
            Err(format!("cargo build for target {target} failed"))?;
        }

        let build_dir = build_dir(target);
        let dest_dir = dest_dir(target);

        fs::copy(build_dir.join(bin), dest_dir.join(bin))?;

        let out_dir = out_dir(target)?;
        fs::copy(
            out_dir.join(format!("{bin}.1")),
            dest_dir.join(format!("doc/{bin}.1")),
        )?;
        Command::new("gzip")
            .arg(dest_dir.join(format!("doc/{bin}.1")))
            .spawn()?;
        fs::copy(
            out_dir.join(format!("_{bin}")),
            dest_dir.join(format!("completion/_{bin}")),
        )?;
        fs::copy(
            out_dir.join(format!("{bin}.bash")),
            dest_dir.join(format!("completion/{bin}.bash")),
        )?;
        fs::copy(
            out_dir.join(format!("{bin}.fish")),
            dest_dir.join(format!("completion/{bin}.fish")),
        )?;
        fs::copy(project_root().join("README.md"), dest_dir.join("README.md"))?;

        Command::new("tar")
            .current_dir(project_root().join("bin"))
            .arg("-czf")
            .arg(format!("{bin}-{version}-{target}.tar.gz"))
            .arg(format!("{bin}-{version}-{target}"))
            .spawn()?;
    }

    Command::new("exa")
        .arg("-lahg")
        .arg("--tree")
        .arg(project_root().join("bin"))
        .spawn()?;
    Ok(())
}

fn build_dir(target: &str) -> PathBuf {
    project_root().join("target").join(target).join("release")
}

fn dest_dir(target: &str) -> PathBuf {
    let (bin, version) = (*crate::BIN, *crate::VERSION);
    let d = project_root()
        .join("bin")
        .join(format!("{bin}-{version}-{target}"));
    fs::create_dir_all(&d).unwrap_or(());
    fs::create_dir_all(d.join("doc")).unwrap_or(());
    fs::create_dir_all(d.join("completion")).unwrap_or(());
    d
}

fn out_dir(target: &str) -> Result<PathBuf> {
    let bin = *crate::BIN;

    let mut manpages = fs::read_dir(
        &project_root()
            .join("target")
            .join(target)
            .join("release/build"),
    )?
    .filter(|dir| {
        dir.as_ref()
            .map(|d| d.path().join(format!("out/{bin}.1")).is_file())
            .unwrap_or(false)
    })
    .map(|dir| dir.unwrap().path().join(format!("out/{bin}.1")))
    .collect::<Vec<_>>();
    manpages.sort_unstable_by_key(|f| f.metadata().unwrap().modified().unwrap());
    Ok(manpages
        .last()
        .ok_or("no man page file found")?
        .parent()
        .unwrap()
        .to_path_buf())
}
