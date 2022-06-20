use crate::cli::Info;
#[cfg(not(target_os = "android"))]
use battery::{units::ratio::percent, State};
use chrono::Duration;
#[cfg(target_os = "android")]
use serde::Deserialize;
use std::{env, io::Read, process::Command};
use sysinfo::{
    CpuExt, CpuRefreshKind, Pid, ProcessExt, ProcessRefreshKind, RefreshKind, SystemExt,
};
use systemstat::Platform;

impl Info {
    pub fn get_info(&self, sys: &mut System) -> Option<String> {
        match self {
            Info::UserAtHostname => sys.user_at_hostname(),
            Info::OS => sys.os(),
            Info::Host => sys.host(),
            Info::Kernel => sys.kernel(),
            Info::Uptime => sys.uptime(),
            Info::Packages => sys.packages(),
            Info::Shell => sys.shell(),
            Info::Terminal => sys.terminal(),
            Info::CPU => sys.cpu(),
            Info::Memory => sys.memory(),
            Info::Swap => sys.swap(),
            Info::Battery => sys.battery(),
            Info::Seperator => Some(String::new()),
            Info::Colors1 => Some(sys.colors1()),
            Info::Colors2 => Some(sys.colors2()),
        }
    }
}

#[cfg(target_os = "android")]
#[derive(Deserialize)]
#[allow(dead_code)]
struct BatteryStatus {
    health: String,
    percentage: u8,
    plugged: String,
    status: String,
    temperature: f32,
    current: i32,
}

pub struct System {
    sysinfo: sysinfo::System,
    systemstat: systemstat::System,
}

impl System {
    pub fn new() -> System {
        System {
            sysinfo: sysinfo::System::new_with_specifics(
                RefreshKind::new()
                    .with_cpu(CpuRefreshKind::new())
                    .with_memory(),
            ),
            systemstat: systemstat::System::new(),
        }
    }

    pub fn user_at_hostname(&self) -> Option<String> {
        let user = env::var("USER");
        if let Ok(user) = user {
            return Some(format!("{}@{}", user, self.sysinfo.host_name()?));
        } else {
            return Some(format!(
                "{}@{}",
                String::from_utf8_lossy(&Command::new("id").arg("-un").output().ok()?.stdout)
                    .replace('\n', ""),
                self.sysinfo.host_name()?,
            ));
        }
    }

    pub fn os(&self) -> Option<String> {
        let version = self.sysinfo.os_version();
        if let Some(version) = version {
            if cfg!(target_os = "android") {
                return Some(format!("Android {}", version));
            }
            if !version.contains("rolling") {
                return Some(format!("{} {}", self.sysinfo.name()?, version));
            }
        }
        self.sysinfo.name()
    }

    pub fn host(&self) -> Option<String> {
        if cfg!(target_os = "android") {
            return self.sysinfo.name();
        }

        let name_file = std::fs::File::open("/sys/devices/virtual/dmi/id/product_name").ok();
        let version_file = std::fs::File::open("/sys/devices/virtual/dmi/id/product_version").ok();
        let model_file = std::fs::File::open("/sys/firmware/devicetree/base/model").ok();

        if [&name_file, &version_file, &model_file]
            .iter()
            .all(|f| f.is_none())
        {
            return None;
        }

        let name = if let Some(mut file) = name_file {
            let mut str = String::new();
            file.read_to_string(&mut str).unwrap_or(0);
            str.replace('\n', "")
        } else {
            "".to_string()
        };
        let version = if let Some(mut file) = version_file {
            let mut str = String::new();
            file.read_to_string(&mut str).unwrap_or(0);
            str.replace('\n', "")
        } else {
            "".to_string()
        };
        let model = if let Some(mut file) = model_file {
            let mut str = String::new();
            file.read_to_string(&mut str).unwrap_or(0);
            str.replace('\n', "")
        } else {
            "".to_string()
        };

        let host = format!("{} {} {}", name, version, model);
        let blacklist = [
            "To",
            "to",
            "Be",
            "be",
            "Filled",
            "filled",
            "By",
            "by",
            "O.E.M.",
            "OEM",
            "Not",
            "Applicable",
            "Specified",
            "System",
            "Product",
            "Name",
            "Version",
            "Undefined",
            "Default",
            "string",
            "INVALID",
            "os",
            "Type1ProductConfigId",
        ];
        let mut host_filtered: Vec<String> = vec![];
        for word in host.split(' ') {
            if !blacklist.contains(&word) {
                host_filtered.push(word.to_owned())
            }
        }
        let host_n = host_filtered.join(" ");
        if host_n.is_empty() {
            return Some(
                String::from_utf8_lossy(&Command::new("uname").arg("-m").output().ok()?.stdout)
                    .replace('\n', ""),
            );
        } else {
            Some(host_n)
        }
    }

    pub fn kernel(&self) -> Option<String> {
        self.sysinfo.kernel_version()
    }

    pub fn uptime(&self) -> Option<String> {
        let duration = Duration::from_std(self.systemstat.uptime().ok()?).ok()?;

        let days = duration.num_days();
        let hours = duration.num_hours() - 24 * days;
        let minutes = duration.num_minutes() - 60 * hours - 24 * 60 * days;

        Some(format!(
            "{}{}{}m",
            if days > 0 {
                format!("{}d ", days)
            } else {
                String::new()
            },
            if hours > 0 {
                format!("{}h ", hours)
            } else {
                String::new()
            },
            minutes
        ))
    }

    pub fn packages(&self) -> Option<String> {
        let commands = [
            ("pacman", Command::new("pacman").arg("-Qq").output()),
            (
                "dpkg",
                Command::new("dpkg-query")
                    .args(["-f", r".\n", "-W"])
                    .output(),
            ),
            ("bonsai", Command::new("bonsai").arg("list").output()),
            ("pkginfo", Command::new("pkginfo").arg("-i").output()),
            ("rpm", Command::new("rpm").arg("-qa").output()),
            ("xbps-query", Command::new("xbps-query").arg("-l").output()),
            ("apk", Command::new("apk").arg("info").output()),
            (
                "guix",
                Command::new("guix")
                    .args(["package", "--list-installed"])
                    .output(),
            ),
            ("opkg", Command::new("opkg").arg("list-installed").output()),
            ("kiss", Command::new("kiss").arg("l").output()),
            ("cpt-list", Command::new("cpt-list").output()),
            ("pacman-g2", Command::new("pacman-g2").arg("-Q").output()),
            ("lvu", Command::new("lvu").arg("installed").output()),
            ("tce-status", Command::new("tce-status").arg("-i").output()),
            ("pkg_info", Command::new("pkg_info").output()),
            ("pkgin", Command::new("pkgin").arg("list").output()),
            ("sorcery", Command::new("gaze").arg("installed").output()),
            ("alps", Command::new("alps").arg("showinstalled").output()),
            ("butch", Command::new("butch").arg("list").output()),
            (
                "swupd",
                Command::new("swupd")
                    .args(["bundle-list", "--quiet"])
                    .output(),
            ),
            ("pisi", Command::new("pisi").arg("li").output()),
            ("pacstall", Command::new("pacstall").arg("-L").output()),
        ];

        for (pkg_manager, output) in commands.iter() {
            if let Ok(output) = output {
                if output.status.success() {
                    return Some(format!(
                        "{} ({})",
                        String::from_utf8_lossy(&output.stdout)
                            .trim_matches('\n')
                            .split('\n')
                            .count(),
                        pkg_manager,
                    ));
                }
            }
        }
        None
    }

    pub fn shell(&self) -> Option<String> {
        Some(env::var("SHELL").ok()?.rsplit_once('/')?.1.to_owned())
    }

    pub fn terminal(&mut self) -> Option<String> {
        self.sysinfo
            .refresh_processes_specifics(ProcessRefreshKind::new());
        if let Ok(home) = env::var("HOME") {
            if home.contains("termux") {
                return Some(String::from("termux"));
            }
        }

        let process = self.sysinfo.process(Pid::from(std::process::id() as i32))?;
        let shell = self.sysinfo.process(process.parent()?)?;
        let terminal = self.sysinfo.process(shell.parent()?)?;

        let mut name = terminal.name();
        if name == "electron" {
            if let Some(parent1) = terminal.parent().and_then(|pid| self.sysinfo.process(pid)) {
                if parent1.name().contains("code") {
                    name = "vscode";
                } else if let Some(parent2) =
                    parent1.parent().and_then(|pid| self.sysinfo.process(pid))
                {
                    if parent2.name().contains("code") {
                        name = "vscode";
                    }
                }
            }
        }

        Some(name.to_owned())
    }

    pub fn cpu(&self) -> Option<String> {
        Some(self.sysinfo.cpus().iter().next()?.brand().to_string())
    }

    pub fn memory(&self) -> Option<String> {
        Some(format!(
            "{:.2}GB / {:.2}GB",
            (self.sysinfo.used_memory() as f32) / 1024.0 / 1024.0,
            (self.sysinfo.total_memory() as f32) / 1024.0 / 1024.0,
        ))
    }

    pub fn swap(&self) -> Option<String> {
        let total_swap = self.sysinfo.total_swap();
        if total_swap == 0 {
            return None;
        }
        Some(format!(
            "{:.2}GB / {:.2}GB",
            (self.sysinfo.used_swap() as f32) / 1024.0 / 1024.0,
            (self.sysinfo.total_swap() as f32) / 1024.0 / 1024.0,
        ))
    }

    #[cfg(not(target_os = "android"))]
    pub fn battery(&self) -> Option<String> {
        let manager = battery::Manager::new().ok()?;
        let battery = manager.batteries().ok()?.next()?.ok()?;
        Some(format!(
            "{:.0}%{}",
            battery.state_of_charge().get::<percent>(),
            match battery.state() {
                State::Charging => ", charging",
                State::Discharging => ", discharging",
                _ => "",
            }
        ))
    }

    #[cfg(target_os = "android")]
    pub fn battery(&self) -> Option<String> {
        let status = match Command::new("termux-battery-status").output() {
            Ok(output) => output.stdout,
            Err(_) => return None,
        };
        let status: BatteryStatus =
            serde_json::from_str(&String::from_utf8_lossy(&status[..])).ok()?;
        Some(format!(
            "{}%{}",
            status.percentage,
            match status.status.as_str() {
                "CHARGING" => ", charging",
                "DISCHARGING" => ", discharging",
                _ => "",
            }
        ))
    }

    pub fn colors1(&self) -> String {
        (0..8)
            .map(|c| format!("\x1b[4{}m   ", c))
            .collect::<String>()
            + "\x1b[0m"
    }

    pub fn colors2(&self) -> String {
        (8..16)
            .map(|c| format!("\x1b[48;5;{}m   ", c))
            .collect::<String>()
            + "\x1b[0m"
    }
}
