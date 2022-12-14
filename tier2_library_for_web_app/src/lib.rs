// tier2_library_for_web_app/src/lib.rs

// This module contains all the "libraries" needed to work with the "content" of the web app
// Maybe one day this will be extracted in a stand-alone crate.

#![deny(unused_crate_dependencies)]

pub mod actix_mod;
pub mod app_state_mod;
pub mod deadpool_mod;
pub mod error_mod;
pub mod html_templating_mod;
pub mod postgres_function_mod;
pub mod postgres_mod;
pub mod postgres_select_query_mod;
pub mod postgres_type_mod;
pub mod rust_named_params_for_sql_mod;
pub mod sendgrid_mod;
pub mod server_side_multi_row_mod;
pub mod server_side_single_row_mod;
pub mod sql_params_mod;
pub mod web_params_mod;

pub use app_state_mod::AppState;
