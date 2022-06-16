use clap::{builder::ValueHint, Parser};
use serde::Deserialize;
use strum::{EnumIter, EnumString};

/// Another fetch program with variable sized pixel images
#[derive(Debug, Deserialize, Parser)]
#[clap(author)]
pub struct Config {
    /// The maximum width in pixels of the image
    ///
    /// - Must be an integer between 5 and 50
    #[clap(long, value_name = "WIDTH", value_parser = clap::value_parser!(u8).range(5..=50), action)]
    pub max_width: Option<u8>,

    /// The minimum alpha value for pixels to be displayed
    ///
    /// - Must be an integer between 0 and 255
    #[clap(long, value_name = "THRESHOLD", value_parser = clap::value_parser!(u8).range(0..=255), action)]
    pub alpha_threshold: Option<u8>,

    /// Whether to show a colon between each info key and value
    #[clap(long, value_name = "true|false", action)]
    pub show_colons: Option<bool>,

    /// Override the main color
    ///
    /// - Must be an integer between 0 and 7
    ///
    /// - The color for the user@hostname will be this + 1
    #[clap(long, value_name = "COLOR", value_parser = clap::value_parser!(u8).range(0..=7), action)]
    pub color_override: Option<u8>,

    /// Path to a custom image to be used instead of the OS logo
    #[clap(long, value_name = "PATH", value_hint = ValueHint::FilePath, action)]
    pub image_override: Option<String>,

    // TODO: get completion for more than one value
    /// A list of infos to not show
    ///
    /// - Either use the option multiple times, or seperate the items with commas
    #[clap(
        long,
        use_value_delimiter = true,
        require_value_delimiter = true,
        value_name = "INFOS",
        min_values = 0,
        action,
    )]
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

impl clap::ValueEnum for Info {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::UserAtHostname,
            Self::OS,
            Self::Host,
            Self::Kernel,
            Self::Uptime,
            Self::Packages,
            Self::Shell,
            Self::Terminal,
            Self::CPU,
            Self::Memory,
            Self::Swap,
            Self::Battery,
            Self::Seperator,
            Self::Colors1,
            Self::Colors2,
        ]
    }

    fn to_possible_value<'a>(&self) -> Option<clap::PossibleValue<'a>> {
        match self {
            Self::UserAtHostname => Some(clap::PossibleValue::new("UserAtHostname")),
            Self::OS => Some(clap::PossibleValue::new("OS")),
            Self::Host => Some(clap::PossibleValue::new("Host")),
            Self::Kernel => Some(clap::PossibleValue::new("Kernel")),
            Self::Uptime => Some(clap::PossibleValue::new("Uptime")),
            Self::Packages => Some(clap::PossibleValue::new("Packages")),
            Self::Shell => Some(clap::PossibleValue::new("Shell")),
            Self::Terminal => Some(clap::PossibleValue::new("Terminal")),
            Self::CPU => Some(clap::PossibleValue::new("CPU")),
            Self::Memory => Some(clap::PossibleValue::new("Memory")),
            Self::Swap => Some(clap::PossibleValue::new("Swap")),
            Self::Battery => Some(clap::PossibleValue::new("Battery")),
            Self::Seperator => Some(clap::PossibleValue::new("Seperator")),
            Self::Colors1 => Some(clap::PossibleValue::new("Colors1")),
            Self::Colors2 => Some(clap::PossibleValue::new("Colors2")),
        }
    }
}
