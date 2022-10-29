// tier2_web_server_actix_postgres/src/a0_library_mod/mod.rs

// This module contains all the "libraries" needed to work with the "content" of the web app
// Maybe one day this will be extracted in a stand-alone crate.

pub mod actix_mod;
pub mod app_state_mod;
pub mod deadpool_mod;
pub mod error_mod;
pub mod html_templating_mod;
pub mod postgres_function_mod;
pub mod postgres_mod;
pub mod postgres_select_query_mod;
pub mod postgres_type_mod;
pub mod server_side_multi_row_mod;
pub mod server_side_single_row_mod;
pub mod sql_params_mod;
pub mod web_params_mod;

pub use app_state_mod::AppState;
