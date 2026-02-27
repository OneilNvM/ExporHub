use std::{env, fs::File, io::BufReader, time::Duration};

use actix_cors::Cors;
use actix_extensible_rate_limit::{
    backend::{memory::InMemoryBackend, SimpleInputFunctionBuilder},
    RateLimiter,
};
use actix_web::{
    http::header::{
        CACHE_CONTROL, CONTENT_SECURITY_POLICY, X_CONTENT_TYPE_OPTIONS,
    },
    middleware, web, App, HttpServer,
};
use actix_web_lab::{header::StrictTransportSecurity, middleware::RedirectHttps};
use expor_hub_server::{
    db::seeder,
    initialize_db_pool,
    routes::{
        account::*, api::*, comments::get_project_comments, dislikes::*, favourites::*, follows::*,
        images::*, likes::*, projects::*, replies::get_comment_replies, root::*,
        searches::process_search_query, users::*,
    },
    validate_auth,
};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let mut args = env::args();

    if args.len() > 1 {
        args.next();

        let arg = args.next().unwrap();

        if arg == "db-seed" {
            println!("Running seeder...");

            seeder::run().expect("There was an issue inserting data into the database");

            println!("Inserted data into the database");
        }
    }

    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("setting up app from environment");

    let pool = initialize_db_pool();

    let mut certs_file = BufReader::new(
        File::open("../../website-frontend/exporhub-app/certificates/exporhub.crt").unwrap(),
    );
    let mut key_file = BufReader::new(
        File::open("../../website-frontend/exporhub-app/certificates/private.key").unwrap(),
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
    let _auth_mw = actix_web_httpauth::middleware::HttpAuthentication::basic(validate_auth);

    let backend = InMemoryBackend::builder().build();

    println!("Server running at https://api.exporhub.com:9000");

    HttpServer::new(move || {
        let input = SimpleInputFunctionBuilder::new(Duration::from_secs(20), 1000).real_ip_key().build();
        let rate_limit_mw = RateLimiter::builder(backend.clone(), input).add_headers().build();
        let cors = Cors::default().allowed_origin("https://test.exporhub.com:3000").allowed_headers(vec!["authorization", "content-type"]).allowed_methods(vec!["GET", "POST", "OPTIONS"]).supports_credentials();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rate_limit_mw)
            .wrap(mw.clone())
            .wrap(cors)
            .wrap(middleware::DefaultHeaders::new().add((CACHE_CONTROL, "max-age=1200, no-cache, public")))
            .wrap(middleware::DefaultHeaders::new().add((X_CONTENT_TYPE_OPTIONS, "nosniff")))
            .wrap(middleware::DefaultHeaders::new().add((CONTENT_SECURITY_POLICY, "default-src 'none'; script-src 'self'; connect-src 'self'; img-src 'self'; style-src 'self'; frame-ancestors 'self'; form-action 'self';")))
            .service(index)
            .service(
                web::scope("/account")
                    .service(login)
                    .service(create_account)
            )
            .service(
                web::scope("/api")
                    .wrap(_auth_mw.clone())
                    .service(
                        web::scope("/user")
                            .service(get_user_by_id)
                            .service(get_user_by_username)
                            .service(get_user_by_email)
                    )
                    .service(
                        web::scope("/project")
                            .service(get_project_by_id)
                            .service(get_projects_by_user_id)
                            .service(get_projects_by_date_updated)
                            .service(get_num_of_projects_by_user)
                            .service(get_project_by_name)
                            .service(create_project)
                    )
                    .service(
                        web::scope("/favourite")
                            .service(create_new_favourite)
                            .service(unfavourite_project)
                            .service(get_favourite_by_ids)
                            .service(get_favourites_by_user_id),
                    )
                    .service(
                        web::scope("/follow")
                            .service(get_user_followings)
                            .service(unfollow)
                            .service(new_follow)
                            .service(get_follow_by_ids),
                    )
                    .service(
                        web::scope("/image")
                            .service(get_profile_image)
                            .service(get_project_images)
                            .service(upload_profile_image)
                    )
                    .service(web::scope("/comment").service(get_project_comments))
                    .service(web::scope("/reply").service(get_comment_replies))
                    .service(
                        web::scope("like")
                            .service(get_user_likes)
                            .service(get_comment_likes)
                            .service(get_reply_likes)
                    )
                    .service(
                        web::scope("dislike")
                            .service(get_user_dislikes)
                            .service(get_comment_dislikes)
                            .service(get_reply_dislikes)
                    )
                    .service(web::scope("/search").service(process_search_query))
                    .service(all_tables)
                    .service(show_users)
                    .service(show_projects)
                    .service(show_images)
                    .service(show_follows)
                    .service(show_favourites)
                    .service(show_comments)
                    .service(show_replies)
                    .service(show_threads)
                    .service(show_likes)
                    .service(show_dislikes)
                    .service(show_comment_likes)
                    .service(show_comment_dislikes)
                    .service(show_reply_likes)
                    .service(show_reply_dislikes),
            )
    })
    .bind_rustls_0_23(("api.exporhub.com", 9000), tls_config)?
    .workers(12)
    .run()
    .await
}
