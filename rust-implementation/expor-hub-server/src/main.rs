use std::{env, fs::File, io::BufReader};

use actix_web::{http::header::ACCESS_CONTROL_ALLOW_ORIGIN, middleware, web, App, HttpServer};
use actix_web_lab::{header::StrictTransportSecurity, middleware::RedirectHttps};
use expor_hub_server::{
    db::seeder,
    initialize_db_pool,
    routes::{
        account::{create_account, create_account_options, login, login_options},
        api::*,
        favourites::{
            create_new_favourite, get_favourite_by_ids, get_favourites_by_user_id,
            unfavourite_project,
        },
        follows::{get_follow_by_ids, get_user_followings, new_follow, unfollow},
        projects::{
            get_num_of_projects_by_user, get_project_by_id, get_projects_by_date_updated,
            get_projects_by_user_id,
        },
        root::*,
        searches::process_search_query,
        users::{
            email_options, get_user_by_email, get_user_by_id, get_user_by_username,
            user_id_options, username_options,
        },
    },
    validate_auth,
};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let mut args = env::args().into_iter();

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
                    .wrap(
                        middleware::DefaultHeaders::new()
                            .add((ACCESS_CONTROL_ALLOW_ORIGIN, "https://exporhub.com:3000")),
                    )
                    .service(
                        web::scope("/user")
                            .service(get_user_by_id)
                            .service(get_user_by_username)
                            .service(get_user_by_email)
                            .service(user_id_options)
                            .service(username_options)
                            .service(email_options),
                    )
                    .service(
                        web::scope("/project")
                            .service(get_project_by_id)
                            .service(get_projects_by_user_id)
                            .service(get_projects_by_date_updated)
                            .service(get_num_of_projects_by_user),
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
