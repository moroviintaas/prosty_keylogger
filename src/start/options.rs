use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Options{
    #[arg(short = 's', long="server-address", help = "Adres serwera", default_value = "localhost:8080")]
    pub server_address: String,

}