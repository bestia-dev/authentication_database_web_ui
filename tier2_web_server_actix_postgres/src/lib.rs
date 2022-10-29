// tier2_web_server_actix_postgres/src/lib.rs

mod b1_authn_signup_mod;
mod b2_authn_login_mod;
mod c1_webpage_hits_mod;

pub const APP_MAIN_ROUTE: &'static str = "webpage_hits_admin";

pub use tier2_library_for_web_app::actix_mod::on_request_received_is_session_cookie_ok;
pub use tier2_library_for_web_app::actix_mod::redirect_to_login_page;
pub use tier2_library_for_web_app::app_state_mod::AppState;
pub use tier2_library_for_web_app::deadpool_mod::deadpool_start_and_check;
pub use tier2_library_for_web_app::postgres_mod::get_for_cache_all_function_input_params;
pub use tier2_library_for_web_app::postgres_mod::get_for_cache_all_view_fields;

/// configure the route with scope
/// so the routing code is near to the implementation code
#[rustfmt::skip]
pub fn config_route_main(cfg: &mut actix_web::web::ServiceConfig) {
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
            actix_web::web::scope(&format!("/{APP_MAIN_ROUTE}/c1_webpage_hits_mod"))
                .configure(crate::c1_webpage_hits_mod::config_route_webpage_hits),
        )
        .service(
            actix_web::web::scope(&format!("/{APP_MAIN_ROUTE}/b2_authn_login_mod"))
                .configure(crate::b2_authn_login_mod::config_route_authn),
        )
    ;
}
