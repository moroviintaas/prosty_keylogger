use std::path::PathBuf;
use clap::{Parser, ValueEnum};

/*
#[derive(ValueEnum, Debug, Clone)]
pub struct{

}

 */


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ClientArgs{

    #[arg( help = "Assistant address")]//, default_value = "http://127.0.0.1:8080")]
    pub assistant: String,

    #[arg(long = "log-name",  help = "Log file")]
    pub log_filename: Option<PathBuf>,

    #[arg(long = "log-filter",  help = "Log level", default_value = "OFF")]
    pub log_filter: log::LevelFilter,


}