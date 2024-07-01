use std::path::PathBuf;

use crate::common::PathFragment;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InstallConfiguration{
    pub installation_base_path: Vec<PathFragment>,
    pub installation_file_name: PathBuf,
    pub server_url: String,
}

impl Default for InstallConfiguration{
    fn default() -> Self {
        Self {
            installation_base_path: vec![PathFragment::Env("HOMEDRIVE".into()),
                                         PathFragment::Env("HOMEPATH".into()),
                                         PathFragment::Raw("Documents\\system\\".into())],
            installation_file_name: PathBuf::from("ptdd_x6.exe"),

            server_url: "http://127.0.0.1:8080".to_string(),
        }

    }
}

impl InstallConfiguration{
    pub fn join_as_exec_path(&self) -> Result<PathBuf, anyhow::Error>{
        let mut p1 = PathFragment::join_slice(&self.installation_base_path)?;
        p1.join(&self.installation_file_name);
        Ok(p1)
    }
}