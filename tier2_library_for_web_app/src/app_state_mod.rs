// tier2_library_for_web_app/src/app_state_mod.rs

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use super::postgres_mod::{SqlFunctionInputParams, SqlFunctionInputParamsOrder, SqlViewFields};

/// This struct represents state
/// Every function can extract this simply with an input parameter
/// fun (app_state: actix_web::web::Data<AppState>)
pub struct AppState {
    pub db_pool: deadpool_postgres::Pool,
    pub all_sql_function_input_params: SqlFunctionInputParams,
    pub all_sql_function_input_params_order: SqlFunctionInputParamsOrder,
    pub sql_view_fields: SqlViewFields,
    /// mutable HashMap. First string is session_id,
    /// the tuple has string user_email and u128 unix epoch time for last_access_time.
    pub active_sessions: Arc<Mutex<HashMap<String, (String, u128)>>>,
}
