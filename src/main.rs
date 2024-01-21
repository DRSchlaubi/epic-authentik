use std::collections::HashMap;
use std::env::var;
use std::io::Result;

use actix_web::web::Form;
use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use log::info;
use reqwest::Client;

#[post("oauth/token")]
async fn token(req_body: Form<HashMap<String, String>>) -> impl Responder {
    lazy_static! {
        static ref CLIENT: Client = Client::new();
        #[derive(Debug)]
        static ref DEPLOYMENT_ID: String = var("DEPLOYMENT_ID").unwrap();
        #[derive(Debug)]
        static ref CLIENT_ID: String = var("CLIENT_ID").unwrap();
        #[derive(Debug)]
        static ref CLIENT_SECRET: String = var("CLIENT_SECRET").unwrap();
    }

    let mut modified_data = req_body.clone();
    modified_data.insert("deployment_id".to_string(), DEPLOYMENT_ID.to_string());

    let response = CLIENT
        .post("https://api.epicgames.dev/epic/oauth/v2/token")
        .basic_auth(CLIENT_ID.to_string(), Some(CLIENT_SECRET.to_string()))
        .form(&modified_data)
        .send()
        .await
        .unwrap();
    let status_code = response.status();
    info!("Got Epic response code: {}", status_code);
    let bytes = response.bytes().await.unwrap();

    HttpResponse::build(status_code).body(bytes)
}

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| App::new().service(token))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
