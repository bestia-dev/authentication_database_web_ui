// authentication_database_web_ui/tier2_web_server_actix_postgres/src/lib.rs

mod actix_mod;
mod app_state_mod;
mod authn_mod;
mod deadpool_mod;
mod error_mod;
mod html_templating_mod;
mod postgres_function_mod;
mod postgres_mod;
mod postgres_select_query_mod;
mod postgres_type_mod;
mod server_side_multi_row_mod;
mod server_side_single_row_mod;
mod sql_params_mod;
mod web_params_mod;
mod webpage_hits_mod;

pub const APP_MAIN_ROUTE: &'static str = "webpage_hits_admin";

pub use actix_mod::config_route_main;
pub use actix_mod::on_request_received_is_session_cookie_ok;
pub use actix_mod::redirect_to_login_page;
pub use app_state_mod::AppState;
pub use deadpool_mod::deadpool_start_and_check;
pub use postgres_mod::get_for_cache_all_function_input_params;
pub use postgres_mod::get_for_cache_all_view_fields;
