use clap::{Args, Parser};

#[derive(Args)]
pub struct ImageArg {
    pub url: String,
    #[arg(short, long, default_value_t = 1)]
    pub concurrent: u16,

    #[arg(short, long, default_value = ".")]
    pub path: String,
}