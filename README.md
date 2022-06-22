# pixfetch
Another fetch program with pixelized images written in Rust

![screenshot with default config](https://raw.githubusercontent.com/RubixDev/pixfetch/master/screenshots/default.png)
![screenshot with custom config](https://raw.githubusercontent.com/RubixDev/pixfetch/master/screenshots/custom.png)

## Configuration
pixfetch can be configured using a config file in `$XDG_CONFIG_HOME/pixfetch/config.toml` or when `$XDG_CONFIG_HOME` is not set in `$HOME/.config/pixfetch/config.toml`. To see all options you can have a look at the [default configuration file](https://github.com/RubixDev/pixfetch/blob/master/src/default_config.toml).

Additionally, all configuration can also be overridden with flags from the command line. To see those options run `pixfetch --help` or `pixfetch -h` for shorter descriptions.

## Installation
### Arch Linux
On Arch Linux pixfetch can be installed through the AUR with a helper like `paru` or `yay`:
```bash
paru -S pixfetch
```

### Cargo
In case you do not need the man page and shell completion scripts you can also install pixfetch through `cargo`:
```bash
cargo install pixfetch
```

### Manual
#### Binary releases
##### 0. Setup
You can set these variables beforehand to be able to just copy below commands (edit the values accordingly):
```bash
version=1.0.0
platform=x86_64-unknown-linux-musl
```

##### 1. Download
Download the latest release for your platform from the [releases page](https://github.com/RubixDev/pixfetch/releases). In case there is no suitable download option for you, please [open an issue](https://github.com/RubixDev/pixfetch/issues/new):
```bash
wget https://github.com/RubixDev/pixfetch/releases/download/v$version/pixfetch-$version-$platform.tar.gz
```

##### 2. Extract the downloaded archive:
```bash
tar -xvf pixfetch-$version-$platform.tar.gz
```
and open the directory:
```bash
cd pixfetch-$version-$platform
```

##### 3. Copy the files to their respective places:
```bash
sudo install -Dm755 pixfetch /usr/bin/pixfetch
sudo install -Dm644 README.md /usr/share/doc/pixfetch/README.md
sudo install -Dm644 LICENSE /usr/share/licenses/pixfetch/LICENSE
sudo install -Dm644 doc/pixfetch.1.gz /usr/share/man/man1/pixfetch.1.gz
sudo install -Dm644 completion/_pixfetch /usr/share/zsh/site-functions/_pixfetch
sudo install -Dm644 completion/pixfetch.bash /usr/share/bash-completion/completions/pixfetch
sudo install -Dm644 completion/pixfetch.fish /usr/share/fish/vendor_completions.d/pixfetch.fish
```

> Note: For bash completion make sure you have [`bash-completion`](https://github.com/scop/bash-completion) installed

#### From source
##### 1. Clone the repository:
```bash
git clone https://github.com/RubixDev/pixfetch.git
```

##### 2. Compile the binary:
```bash
cargo build --release
```

##### 3. Copy the binary, README and LICENSE
```bash
sudo install -Dm755 "${CARGO_TARGET_DIR:-target}/release/pixfetch" /usr/bin/pixfetch
sudo install -Dm644 README.md /usr/share/doc/pixfetch/README.md
sudo install -Dm644 LICENSE /usr/share/licenses/pixfetch/LICENSE
```

##### 4. Locate the output directory:
This is where the generated man page and shell completion scripts are located. The location of that folder was logged while building the binary. Alternatively you can use following command:
```bash
find "${CARGO_TARGET_DIR:-target}/release" -name pixfetch.1 -print0 | xargs -0 ls -t | head -n1 | xargs dirname
```

You can then open that directory using `cd`.

##### 5. Copy the generated file to their respective locations
```bash
gzip pixfetch.1
sudo install -Dm644 pixfetch.1.gz /usr/share/man/man1/pixfetch.1.gz
sudo install -Dm644 _pixfetch /usr/share/zsh/site-functions/_pixfetch
sudo install -Dm644 pixfetch.bash /usr/share/bash-completion/completions/pixfetch
sudo install -Dm644 pixfetch.fish /usr/share/fish/vendor_completions.d/pixfetch.fish
```

> Note: For bash completion make sure you have [`bash-completion`](https://github.com/scop/bash-completion) installed
