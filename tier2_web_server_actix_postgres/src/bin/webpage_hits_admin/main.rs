// authentication_database_web_ui/tier2_web_server_actix_postgres/src/bin/webpage_hits_admin/main.rs

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tier2_web_server_actix_postgres as tier2;
use tier2_web_server_actix_postgres::APP_MAIN_ROUTE;

/// the binary executable entry point
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut logger_builder = pretty_env_logger::env_logger::Builder::from_default_env();
    logger_builder.filter(None, log::LevelFilter::Info).init();

    log::info!("Actix web server started on localhost:8080!");
    log::info!("Test it with curl or browser:");
    log::info!("http://localhost:8080/{APP_MAIN_ROUTE}/c1_webpage_hits_mod/c1_webpage_hits_list");

    // connection pool for postgres to reuse connections for better performance
    let db_pool = tier2::deadpool_start_and_check().await;

    // on start get all the input parameters for sql functions.
    // So I can parse string params to a correct rust data type.
    let (all_sql_function_input_params, all_sql_function_input_params_order) =
        tier2::get_for_cache_all_function_input_params(&db_pool).await;

    // I need the view fields and types to construct the where clause
    let sql_view_fields = tier2::get_for_cache_all_view_fields(&db_pool).await;

    // Create web::Data outside of closure HttpServer::new.
    let app_state = actix_web::web::Data::new(tier2::AppState {
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
                if tier2::on_request_received_is_session_cookie_ok(&req) {
                    // add in scope the trait for call()
                    use actix_web::dev::Service;
                    // if it is needed to change the response after the function, then use .map(|res| res) and use futures::FutureExt;  // trait for map()
                    // normal routing and function calling and response returning
                    futures::future::Either::Left(srv.call(req))
                } else {
                    futures::future::Either::Right(futures::future::ready(Ok(
                        tier2::redirect_to_login_page(req),
                    )))
                }
                // endregion: authn pre-process request
            })
            // the route is configured near the implementation code
            .configure(tier2::config_route_main)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await;

    log::info!("");
    log::info!("Actix web server stopped!");
    // return
    http_server_result
}
