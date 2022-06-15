use clap::Parser;
use serde::Deserialize;
use strum::{EnumIter, EnumString};

/// Another fetch program with variable sized pixel images
#[derive(Debug, Deserialize, Parser)]
#[clap(author)]
pub struct Config {
    /// The maximum width in pixels of the image
    ///
    /// - Must be an integer between 5 and 50
    #[clap(long)]
    pub max_width: Option<u8>,

    /// The minimum alpha value for pixels to be displayed
    ///
    /// - Must be an integer between 0 and 255
    #[clap(long)]
    pub alpha_threshold: Option<u8>,

    /// Whether to show a colon between each info key and value
    #[clap(long)]
    pub show_colons: Option<bool>,

    /// Override the main color
    ///
    /// - Must be an integer between 0 and 7
    ///
    /// - The color for the user@hostname will be this + 1
    #[clap(long)]
    pub color_override: Option<u8>,

    /// Path to a custom image to be used instead of the OS logo
    #[clap(long)]
    pub image_override: Option<String>,

    /// A list of infos to not show
    ///
    /// - See possible values in default config file
    #[clap(long, min_values = 0)]
    pub info_blacklist: Option<Vec<Info>>,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone, Copy, EnumString, EnumIter)]
#[allow(clippy::upper_case_acronyms)]
pub enum Info {
    UserAtHostname,
    OS,
    Host,
    Kernel,
    Uptime,
    Packages,
    Shell,
    Terminal,
    CPU,
    Memory,
    Swap,
    Battery,
    Seperator,
    Colors1,
    Colors2,
}
