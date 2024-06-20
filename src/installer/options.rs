use clap::{Parser, Subcommand};




#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Options {
    #[arg(short = 's', long="server-address", help = "Adres serwera", default_value = "http://localhost:8080")]
    pub server_address: String,

}