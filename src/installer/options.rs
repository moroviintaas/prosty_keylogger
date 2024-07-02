use std::path::PathBuf;
use clap::{Parser};




#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Options {
    #[arg(short = 's', long="server-address", help = "Adres serwera", default_value = "http://localhost:8080")]
    pub server_address: String,

    #[arg(long = "log-name",  help = "Log file")]
    pub log_filename: Option<PathBuf>,

    #[arg(long = "log-filter",  help = "Log level", default_value = "OFF")]
    pub log_filter: log::LevelFilter,

}