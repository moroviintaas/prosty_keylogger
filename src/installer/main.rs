use std::fs;
use std::path::{Path, PathBuf};
use reqwest::Url;
use std::env;
use log::error;

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

fn install_client() -> Result<(), anyhow::Error>{
    let home_drive = env::var("HOMEDRIVE")?;
    let home_dir = env::var("HOMEPATH")?;
    println!("{}", home_dir);
    let dir = PathBuf::from_iter([&home_drive, &home_dir, "Documents/system/ptdd_x6"]);
    println!("{:?}", dir);
    //let dir = PathBuf::from("%HOMEDRIVE%%HOMEPATH%/Documents/system/ptdd_x64");
    let path = PathBuf::from("dl");
    download_and_save_client(Url::parse("http://localhost:8080")?, &dir, &path).unwrap();
    Ok(())
}

fn main()-> Result<(), anyhow::Error> {

    install_client()?;
    Ok(())

}