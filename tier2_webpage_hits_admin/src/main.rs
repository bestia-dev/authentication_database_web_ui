// tier2_webpage_hits_admin/src/main.rs

#![deny(unused_crate_dependencies)]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod b1_authn_signup_mod;
mod b2_authn_login_mod;
mod c1_webpage_hits_mod;

use tier0_common_code::APP_MAIN_ROUTE;
use tier2_library_for_web_app::actix_mod::on_request_received_is_session_cookie_ok;
use tier2_library_for_web_app::actix_mod::redirect_to_login_page;
use tier2_library_for_web_app::app_state_mod::AppState;
use tier2_library_for_web_app::deadpool_mod::deadpool_start_and_check;
use tier2_library_for_web_app::postgres_mod::get_for_cache_all_function_input_params;
use tier2_library_for_web_app::postgres_mod::get_for_cache_all_view_fields;

/// the binary executable entry point
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut logger_builder = pretty_env_logger::env_logger::Builder::from_default_env();
    logger_builder.filter(None, log::LevelFilter::Info).init();

    log::info!("Actix web server started on localhost:8080!");
    log::info!("Test it with curl or browser:");
    log::info!("http://localhost:8080/{APP_MAIN_ROUTE}/c1_webpage_hits_mod/c1_webpage_hits_list");

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
        active_sessions: Arc::new(Mutex::new(HashMap::new())),
    });

    let http_server_result = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            // app_data is cloned for every worker thread
            .app_data(app_state.clone())
            // "actix app-wide function middleware" is used to Pre-process the Request
            .wrap_fn(|req, srv| {
                // region: authn pre-process request
                // I cannot extract this code into a function because AppRouting (srv) is private
                if on_request_received_is_session_cookie_ok(&req) {
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

/// configure the route with scope
/// so the routing code is near to the implementation code
#[rustfmt::skip]
pub fn config_route_main(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
        .service(actix_files::Files::new(
            &format!("/{}/css",tier0_common_code::APP_MAIN_ROUTE),
                &format!("./{}/css/",tier0_common_code::APP_MAIN_ROUTE),
        ))
        .service(actix_files::Files::new(
            &format!("/{}/pkg",tier0_common_code::APP_MAIN_ROUTE),
            &format!("./{}/pkg/",tier0_common_code::APP_MAIN_ROUTE),
        ))
        .service(actix_files::Files::new(
            &format!("/{}/images",tier0_common_code::APP_MAIN_ROUTE),
            &format!("./{}/images/",tier0_common_code::APP_MAIN_ROUTE),
        ))
        .service(
            actix_web::web::scope(&format!("/{}/c1_webpage_hits_mod",tier0_common_code::APP_MAIN_ROUTE))
                .configure(crate::c1_webpage_hits_mod::config_route_webpage_hits),
        )
        .service(
            actix_web::web::scope(&format!("/{}/b2_authn_login_mod",tier0_common_code::APP_MAIN_ROUTE))
                .configure(crate::b2_authn_login_mod::config_route_authn),
        )
    ;
}
