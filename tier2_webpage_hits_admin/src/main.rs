// tier2_webpage_hits_admin/src/main.rs

#![deny(unused_crate_dependencies)]

mod a4_system_mod;
mod b1_authn_signup_mod;
mod b2_authn_login_mod;
mod b5_start_page_mod;
mod c1_webpage_hits_mod;

use tier0_common_code as T_0;
use tier2_library_for_web_app as T_2;
use T_2::error_mod::LibError;

use T_0::APP_MAIN_ROUTE;
use T_2::actix_mod::on_request_received_is_session_cookie_ok;
use T_2::actix_mod::redirect_to_login_page;
use T_2::app_state_mod::AppState;
use T_2::deadpool_mod::deadpool_start_and_check;
use T_2::postgres_mod::get_for_cache_all_function_input_params;
use T_2::postgres_mod::get_for_cache_all_view_fields;

lazy_static::lazy_static! {
    static ref SERVER_PROTOCOL: String = std::env::var("SERVER_PROTOCOL").unwrap();
    static ref SERVER_DOMAIN_AND_PORT: String = std::env::var("SERVER_DOMAIN_AND_PORT").unwrap();
}

/// the binary executable entry point
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut logger_builder = pretty_env_logger::env_logger::Builder::from_default_env();
    logger_builder.filter(None, log::LevelFilter::Info).init();

    match init_env_variables() {
        Ok(_) => (),
        Err(err) => {
            log::error!("{}", err);
            return Ok(());
        }
    }

    log::info!("Actix web server started on {}!", *SERVER_DOMAIN_AND_PORT);
    log::info!("Test it with curl or browser:");
    log::info!(
        "{}://{}/{APP_MAIN_ROUTE}/c1_webpage_hits_mod/c1_webpage_hits_list",
        *SERVER_PROTOCOL,
        *SERVER_DOMAIN_AND_PORT
    );

    // connection pool for postgres to reuse connections for better performance
    let db_pool = deadpool_start_and_check().await;

    // on start get all the input parameters for sql functions.
    // So I can parse string params to a correct rust data type.
    let (all_sql_function_input_params, all_sql_function_input_params_order) =
        get_for_cache_all_function_input_params(&db_pool).await;

    // I need the view fields and types to construct the where clause
    let sql_view_fields = get_for_cache_all_view_fields(&db_pool).await;

    // Create web::Data outside of closure HttpServer::new.
    let app_state = actix_web::web::Data::new(AppState {
        db_pool: db_pool,
        all_sql_function_input_params,
        all_sql_function_input_params_order,
        sql_view_fields,
        active_sessions: std::sync::Arc::new(std::sync::Mutex::new(
            std::collections::HashMap::new(),
        )),
    });

    let http_server_result = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            // app_data is cloned for every worker thread
            .app_data(app_state.clone())
            // "actix app-wide function middleware" is used to Pre-process the Request
            .wrap_fn(|req, srv| {
                // region: authn pre-process request
                // I cannot extract this code into a function because AppRouting (srv) is private
                if do_not_check_session_cookie(&req)
                    || on_request_received_is_session_cookie_ok(&req)
                {
                    // add in scope the trait for call()
                    use actix_web::dev::Service;
                    // if it is needed to change the response after the function, then use .map(|res| res) and use futures::FutureExt;  // trait for map()
                    // normal routing and function calling and response returning
                    futures::future::Either::Left(srv.call(req))
                } else {
                    futures::future::Either::Right(futures::future::ready(Ok(
                        redirect_to_login_page(req),
                    )))
                }
                // endregion: authn pre-process request
            })
            // the route is configured near the implementation code
            .configure(config_route_main)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await;

    log::info!("");
    log::info!("Actix web server stopped!");
    // return
    http_server_result
}

fn init_env_variables() -> Result<(), LibError> {
    dotenv::dotenv().map_err(|_err| LibError::EnvVarError {
        user_friendly: String::from("dotenv::dotenv()"),
    })?;

    // on start check if all env variables are available
    // config from .env file
    if std::env::vars().find(|var| var.0 == "PG.USER") == None {
        return Err(LibError::EnvVarError {
            user_friendly: String::from("PG.USER"),
        });
    }
    if std::env::vars().find(|var| var.0 == "PG.HOST") == None {
        return Err(LibError::EnvVarError {
            user_friendly: String::from("PG.HOST"),
        });
    }
    if std::env::vars().find(|var| var.0 == "PG.PORT") == None {
        return Err(LibError::EnvVarError {
            user_friendly: String::from("PG.PORT"),
        });
    }
    if std::env::vars().find(|var| var.0 == "PG.DBNAME") == None {
        return Err(LibError::EnvVarError {
            user_friendly: String::from("PG.DBNAME"),
        });
    }
    if std::env::vars().find(|var| var.0 == "PG.POOL.MAX_SIZE") == None {
        return Err(LibError::EnvVarError {
            user_friendly: String::from("PG.POOL.MAX_SIZE"),
        });
    }
    if std::env::vars().find(|var| var.0 == "RUST_LOG") == None {
        return Err(LibError::EnvVarError {
            user_friendly: String::from("RUST_LOG"),
        });
    }
    if std::env::vars().find(|var| var.0 == "SERVER_PROTOCOL") == None {
        return Err(LibError::EnvVarError {
            user_friendly: String::from("SERVER_PROTOCOL"),
        });
    }
    if std::env::vars().find(|var| var.0 == "SERVER_DOMAIN_AND_PORT") == None {
        return Err(LibError::EnvVarError {
            user_friendly: String::from("SERVER_DOMAIN_AND_PORT"),
        });
    }

    // SECRETS in env variables must be encrypted with the master_key
    if std::env::vars().find(|var| var.0 == "MASTER_KEY") == None {
        return Err(LibError::EnvVarError {
            user_friendly: String::from("MASTER_KEY"),
        });
    }
    if std::env::vars().find(|var| var.0 == "SENDGRID_API_KEY") == None {
        return Err(LibError::EnvVarError {
            user_friendly: String::from("SENDGRID_API_KEY"),
        });
    }
    return Ok(());
}

/// paths that don't need to check the session cookie
fn do_not_check_session_cookie(req: &actix_web::dev::ServiceRequest) -> bool {
    req.path()
        .starts_with(&format!("/{APP_MAIN_ROUTE}/b1_authn_signup_mod/"))
        || req
            .path()
            .starts_with(&format!("/{APP_MAIN_ROUTE}/b2_authn_login_mod/"))
        || req
            .path()
            .starts_with(&format!("/{APP_MAIN_ROUTE}/a4_system_mod/"))
        || req.path().starts_with(&format!("/{APP_MAIN_ROUTE}/css/"))
        || req.path().starts_with(&format!("/{APP_MAIN_ROUTE}/pkg/"))
        || req
            .path()
            .starts_with(&format!("/{APP_MAIN_ROUTE}/images/"))
}

/// configure the route with scope
/// so the routing code is near to the implementation code
#[rustfmt::skip]
fn config_route_main(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
        .service(actix_files::Files::new(
            &format!("/{APP_MAIN_ROUTE}/css"),
                &format!("./{APP_MAIN_ROUTE}/css/"),
        ))
        .service(actix_files::Files::new(
            &format!("/{APP_MAIN_ROUTE}/pkg"),
            &format!("./{APP_MAIN_ROUTE}/pkg/"),
        ))
        .service(actix_files::Files::new(
            &format!("/{APP_MAIN_ROUTE}/images"),
            &format!("./{APP_MAIN_ROUTE}/images/"),
        ))
        .service(
            actix_web::web::scope(&format!("/{APP_MAIN_ROUTE}/a4_system_mod"))
                .configure(crate::a4_system_mod::config_route_authn),
        )
        .service(
            actix_web::web::scope(&format!("/{APP_MAIN_ROUTE}/b1_authn_signup_mod"))
                .configure(crate::b1_authn_signup_mod::config_route_authn),
        )
        .service(
            actix_web::web::scope(&format!("/{APP_MAIN_ROUTE}/b2_authn_login_mod"))
                .configure(crate::b2_authn_login_mod::config_route_authn),
        )
        .service(
            actix_web::web::scope(&format!("/{APP_MAIN_ROUTE}/b5_start_page_mod"))
                .configure(crate::b5_start_page_mod::config_route_authn),
        )
        .service(
            actix_web::web::scope(&format!("/{APP_MAIN_ROUTE}/c1_webpage_hits_mod"))
                .configure(crate::c1_webpage_hits_mod::config_route_webpage_hits),
        )
    ;
}
