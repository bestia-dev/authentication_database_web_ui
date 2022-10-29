// tier2_web_server_actix_postgres/src/lib.rs

pub mod a0_library_mod;
mod b1_authn_signup_mod;
mod b2_authn_login_mod;
mod c1_webpage_hits_mod;

pub const APP_MAIN_ROUTE: &'static str = "webpage_hits_admin";

pub use a0_library_mod::actix_mod::on_request_received_is_session_cookie_ok;
pub use a0_library_mod::actix_mod::redirect_to_login_page;
pub use a0_library_mod::app_state_mod::AppState;
pub use a0_library_mod::deadpool_mod::deadpool_start_and_check;
pub use a0_library_mod::postgres_mod::get_for_cache_all_function_input_params;
pub use a0_library_mod::postgres_mod::get_for_cache_all_view_fields;

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
