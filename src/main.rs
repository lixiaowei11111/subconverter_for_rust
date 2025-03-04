mod core;
mod generators;

use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use rustls::{ServerConfig, pki_types::PrivateKeyDer};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::env;

use std::{fs::File, io::BufReader};

use core::request::sub;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let config = load_rustls_config();

    log::info!("starting HTTPS server at https://localhost:8443");

    HttpServer::new(|| App::new().wrap(actix_web::middleware::Logger::default()).service(sub))
        .bind_rustls_0_23("127.0.0.1:11451", config)?
        .workers(2)
        .run()
        .await
}

fn load_rustls_config() -> rustls::ServerConfig {
    dotenv().expect("Could not load .env file");

    rustls::crypto::aws_lc_rs::default_provider().install_default().unwrap();

    let cert_path = env::var("CERT_PATH").expect("CERT_PATH is required");
    let key_path = env::var("KEY_PATH").expect("KEY_PATH is required");

    // init server config builder with safe defaults
    let config = ServerConfig::builder().with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open(cert_path).unwrap());
    let key_file = &mut BufReader::new(File::open(key_path).unwrap());

    // convert files to key/cert objects
    let cert_chain = certs(cert_file).collect::<Result<Vec<_>, _>>().unwrap();
    let mut keys = pkcs8_private_keys(key_file)
        .map(|key| key.map(PrivateKeyDer::Pkcs8))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    // exit if no keys could be parsed
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}
