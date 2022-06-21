macro_rules! distros {
    ($os:ident; $($tt:tt)*) => {
        match $os {
            Some(distro) => {
                distros!(@IF distro; $($tt)*)
            }
            None => {
                distros!(@IF distro;)
            }
        }
    };
    (@IF $distro:ident; $op:tt $name:literal, $col:literal, $image:literal; $($tt:tt)*) => {
        if distros!(@CONDITION $distro; $op $name) {
            ($col, &include_bytes!(concat!("../logos/", $image, ".png"))[..])
        } else {
            distros!(@IF $distro; $($tt)*)
        }
    };
    (@IF $distro:ident;) => {
        (3, &include_bytes!("../logos/tux.png")[..])
    };
    (@CONDITION $distro:ident; = $name:literal) => {
        $distro == $name
    };
    (@CONDITION $distro:ident; ~ $name:literal) => {
        $distro.contains($name)
    };
}

pub fn get_distro_image(os: Option<String>) -> (u8, &'static [u8]) {
    distros!(
        os;
        ="Arch Linux", 4, "arch";
        ~"Android", 2, "android";
        ~"Debian", 1, "debian";
        ~"Ubuntu", 3, "ubuntu";
        ~"Fedora Linux", 4, "fedora";
        ~"Alpine Linux", 4, "alpine";
        ="EndeavourOS Linux", 4, "endeavour";
    )
}
