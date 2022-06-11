use std::path::PathBuf;

// user@hostname
//
// OS
// Host
// kernel
// uptime
// packages
// shell
// terminal
// cpu
// memory
// battery
//
// colors

mod info;

fn main() {
    let sys = info::System::new();

    let info: Vec<(&str, String)> = [
        ("u@h", sys.user_at_hostname()),
        ("OS", sys.os()),
        ("Host", sys.host()),
        ("Kernel", sys.kernel()),
        ("Uptime", sys.uptime()),
        ("Packages", sys.packages()),
        ("Shell", sys.shell()),
        ("Terminal", sys.terminal()),
        ("CPU", sys.cpu()),
        ("Memory", sys.memory()),
        ("Swap", sys.swap()),
        ("Battery", sys.battery()),
        ("", Some(String::new())),
        ("colors", Some(sys.colors1())),
        ("colors", Some(sys.colors2())),
    ]
    .iter()
    .filter(|(_, v)| v != &None)
    .map(|(k, v)| (*k, v.to_owned().unwrap()))
    .collect();

    let col: u8;
    let img_filename = match info.iter().find(|e| e.0 == "OS") {
        Some(distro) => {
            let distro = distro.1.as_str();

            if distro == "Arch Linux" {
                col = 4;
                "logos/arch.png"
            } else if distro.contains("Android") {
                col = 2;
                "logos/android.png"
            } else if distro.contains("Debian") {
                col = 1;
                "logos/debian.png"
            } else {
                col = 3;
                "logos/tux.png"
            }
        }
        None => {
            col = 3;
            "logos/tux.png"
        }
    };

    let img_width = 30;
    let img_str = ansipix::of_image_file(PathBuf::from(img_filename), (img_width, 64), 50, false)
        .expect("error");
    let img: Vec<&str> = img_str.trim_matches('\n').split('\n').collect();

    for line in 0..(img.len().max(info.len())) {
        if line < img.len() {
            print!("{}  ", img[line]);
        } else {
            print!("{}  ", " ".repeat(img_width));
        }

        if line < info.len() {
            if info[line].0 == "u@h" {
                print!("\x1b[1;3{}m{}\x1b[0m", col + 1, info[line].1);
            } else if info[line].0 == "colors" {
                print!("{}", info[line].1)
            } else if !info[line].0.is_empty() {
                print!(
                    "\x1b[1;3{}m{: <9}\x1b[0m {}",
                    col,
                    info[line].0.to_owned() + ":",
                    info[line].1
                );
            }
        }
        println!();
    }
}
