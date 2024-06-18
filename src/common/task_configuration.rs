

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaskConfiguration{
    pub smtp_login: String,
    pub smtp_password: String,
    pub mail_from: String,
    pub mail_to: String,
    pub relay: String,
    pub capture_size: u32,
    pub probe_interval_milli: u32,
}

impl Default for TaskConfiguration{
    fn default() -> Self {
        Self{
            smtp_login: "".to_string(),
            smtp_password: "".to_string(),
            mail_from: "".to_string(),
            mail_to: "".to_string(),
            relay: "".to_string(),
            capture_size: 100,
            probe_interval_milli: 100,
        }
    }
}