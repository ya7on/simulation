use clap::Parser;
use lazy_static::lazy_static;

#[derive(Parser)]
#[clap(author, version, about, next_line_help = true)]
pub struct Config {
    /// TODO window_width
    #[clap(long = "window-width", env = "WINDOW_WIDTH", default_value = "800")]
    pub window_width: u32,

    /// TODO window_height
    #[clap(long = "window-height", env = "WINDOW_HEIGHT", default_value = "600")]
    pub window_height: u32,

    /// TODO field_width
    #[clap(long = "field-width", env = "FIELD_WIDTH", default_value = "160")]
    pub field_width: usize,

    /// TODO field_height
    #[clap(long = "field-height", env = "FIELD_HEIGHT", default_value = "120")]
    pub field_height: usize,
}

pub fn get_conf() -> &'static Config {
    lazy_static! {
        static ref CONFIG: Config = Config::parse();
    };
    &CONFIG
}
