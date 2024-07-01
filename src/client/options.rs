use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ClientArgs{

    #[arg( help = "Assistant address", default_value = "http://127.0.0.1:8080")]
    pub assistant: String,

    #[arg(long = "no-service")]
    pub no_service: bool

}