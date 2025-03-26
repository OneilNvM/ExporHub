use std::{fs::File, io::BufReader};

use actix_web::{web, App, HttpServer};
use actix_web_lab::{header::StrictTransportSecurity, middleware::RedirectHttps};
use expor_hub_server::{
    initialize_db_pool,
    routes::{
        account::{login, login_options},
        root::*,
    },
};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("setting up app from environment");

    let pool = initialize_db_pool();

    let mut certs_file = BufReader::new(
        File::open("../../Website Frontend/exporhub-app/certificates/exporhub.crt").unwrap(),
    );
    let mut key_file = BufReader::new(
        File::open("../../Website Frontend/exporhub-app/certificates/private.key").unwrap(),
    );

    let tls_certs = rustls_pemfile::certs(&mut certs_file)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let tls_key = rustls_pemfile::ec_private_keys(&mut key_file)
        .next()
        .unwrap()
        .unwrap();
    let tls_config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(tls_certs, rustls::pki_types::PrivateKeyDer::Sec1(tls_key))
        .unwrap();

    let mw = RedirectHttps::with_hsts(StrictTransportSecurity::default().include_subdomains());

    println!("Server running at https://api.exporhub.com:9000");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(mw.clone())
            .service(web::scope("/account").service(login).service(login_options))
            .service(index)
    })
    .bind_rustls_0_23(("api.exporhub.com", 9000), tls_config)?
    .workers(8)
    .run()
    .await
}
