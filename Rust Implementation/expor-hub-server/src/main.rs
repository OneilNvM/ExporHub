use std::{fs::File, io::BufReader};

use actix_web::{http::header::{ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN}, middleware, web, App, HttpServer};
use actix_web_lab::{header::StrictTransportSecurity, middleware::RedirectHttps};
use expor_hub_server::{
    initialize_db_pool,
    routes::{
        account::{create_account, create_account_options, login, login_options},
        api::*,
        root::*,
        users::{get_user_by_username, username_options},
    },
    validate_auth,
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
    let auth_mw = actix_web_httpauth::middleware::HttpAuthentication::basic(validate_auth);

    println!("Server running at https://api.exporhub.com:9000");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(mw.clone())
            .service(index)
            .service(
                web::scope("/account")
                    .wrap(
                        middleware::DefaultHeaders::new()
                            .add((ACCESS_CONTROL_ALLOW_ORIGIN, "https://exporhub.com:3000")),
                    )
                    .service(login)
                    .service(login_options)
                    .service(create_account)
                    .service(create_account_options),
            )
            .service(
                web::scope("/api")
                    .wrap(auth_mw.clone())
                    .wrap(
                        middleware::DefaultHeaders::new()
                            .add((ACCESS_CONTROL_ALLOW_ORIGIN, "https://exporhub.com:3000"))
                            .add((ACCESS_CONTROL_ALLOW_CREDENTIALS, "true"))
                            .add((ACCESS_CONTROL_ALLOW_HEADERS, "authorization"))
                            .add((ACCESS_CONTROL_ALLOW_METHODS, "GET, POST, PUT, DELETE"))
                    )
                    .service(
                        web::scope("/user")
                            .wrap(auth_mw.clone())
                            .service(get_user_by_username)
                            .service(username_options),
                    )
                    .service(all_tables)
                    .service(show_users)
                    .service(show_projects)
                    .service(show_images)
                    .service(show_follows)
                    .service(show_favourites)
                    .service(show_comments)
                    .service(show_replies)
                    .service(show_threads)
                    .service(show_comment_likes)
                    .service(show_comment_dislikes)
                    .service(show_reply_likes)
                    .service(show_reply_dislikes),
            )
    })
    .bind_rustls_0_23(("api.exporhub.com", 9000), tls_config)?
    .workers(8)
    .run()
    .await
}
