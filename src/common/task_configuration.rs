//use crate::common::path::PathFragment;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MailConfiguration{
    pub smtp_login: String,
    pub smtp_password: String,
    pub mail_from: String,
    pub mail_to: String,
    pub relay: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ReportConfig{
    Mail(MailConfiguration)
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaskConfiguration{
    pub id: u64,
    pub report_config: ReportConfig,

    pub capture_size: u32,
    pub probe_interval_milli: u32,
    //pub installation_path: Vec<PathFragment>,
}

impl Default for TaskConfiguration{
    fn default() -> Self {
        Self{
            id: 0,
            report_config: ReportConfig::Mail(MailConfiguration{
                smtp_login: "".to_string(),
                smtp_password: "".to_string(),
                mail_from: "".to_string(),
                mail_to: "".to_string(),
                relay: "".to_string(),
            }),

            capture_size: 100,
            probe_interval_milli: 100,
            /*installation_path: vec![PathFragment::Env("HOMEDRIVE".into()),
                                    PathFragment::Env("HOMEPATH".into()),
                                    PathFragment::Raw("Documents/system/ptdd_x6".into())],

             */
        }
    }
}