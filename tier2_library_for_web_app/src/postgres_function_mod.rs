// tier2_library_for_web_app/src/postgres_function_mod.rs

use super::actix_mod::DataAppState;
use super::error_mod::LibError;
use super::postgres_mod::FunctionName;
use super::postgres_type_mod::PostgresValueMultiType;
use super::sql_params_mod::SqlParams;
use super::web_params_mod::WebParams;

// Struct with methods (object) for calling postgres functions
// 1. prepare sql_params for sql function with correct data types from web_params
// 2. prepare sql_params_in_order and placeholders from sql_params
// 3. create reference to sql_params_in_order, because postgres_client need this format
// 4. run sql function from sql_params in order and get single row

/// object for calling Postgres functions
pub struct PostgresFunction {
    /// contains the connection pool and cached lists of functions and parameters
    app_state: DataAppState,
    function_name: FunctionName,
    /// this params are in correct order for calling the sql function
    sql_params_in_order: Vec<PostgresValueMultiType>,
    /// placeholders for calling sql function with postgres_client
    placeholders: String,
}

impl PostgresFunction {
    /// constructor with web_params
    #[track_caller]
    pub fn new_with_web_params(
        app_state: DataAppState,
        function_name: &'static str,
        web_params: WebParams,
    ) -> PostgresFunction {
        let function_name_obj = FunctionName(function_name.to_string());
        // region: 1. prepare sql_params for sql function with correct data types from web_params
        let sql_params =
            SqlParams::from_web_params(app_state.clone(), &function_name_obj, &web_params);
        // endregion

        Self::new_with_sql_params(app_state, function_name, sql_params)
    }

    /// constructor from sql_params
    #[track_caller]
    pub fn new_with_sql_params(
        app_state: DataAppState,
        function_name: &'static str,
        sql_params: SqlParams,
    ) -> PostgresFunction {
        let function_name_obj = FunctionName(function_name.to_string());
        // region: 2. prepare sql_params_in_order and placeholders from sql_params
        let (sql_params_in_order, placeholders) = sql_params
            .get_sql_params_in_order_and_placeholders(app_state.clone(), &function_name_obj);
        // endregion

        PostgresFunction {
            app_state,
            function_name: function_name_obj,
            sql_params_in_order,
            placeholders,
        }
    }

    /// run sql function and return multi row (zero or more)
    /// 3. and 4. create reference to sql_params_in_order and run sql function
    pub async fn run_sql_function_return_multi_row(
        &mut self,
    ) -> core::result::Result<Vec<tokio_postgres::Row>, LibError> {
        // region: 3. create reference to sql_params_in_order, because postgres_client need this format
        let ref_to_sql_params = SqlParams::ref_to_function_params(&self.sql_params_in_order);
        // endregion
        // 4. run sql function from ref_to_sql_params in order and get single row
        let query = format!(
            "select * from {}({});",
            self.function_name.0, self.placeholders
        );

        // This function will return Result with LibError
        let multi_row = super::postgres_mod::run_sql_select_query_pool(
            &self.app_state.db_pool,
            &query,
            &ref_to_sql_params,
        )
        .await?;

        // endregion
        Ok(multi_row)
    }

    /// return exactly single row from function or error
    pub async fn run_sql_function_return_single_row(
        &mut self,
    ) -> core::result::Result<tokio_postgres::Row, LibError> {
        let mut multi_row = self.run_sql_function_return_multi_row().await?;
        if multi_row.len() == 0 {
            Err(LibError::QueryReturnZeroRow {
                developer_friendly: format!(
                    "{} {:?}",
                    self.function_name.0, self.sql_params_in_order
                ),
                source_line_column: format!("{}:{}:{}", file!(), line!(), column!()),
            })
        } else if multi_row.len() > 1 {
            Err(LibError::QueryReturnMoreThanOneRow {
                developer_friendly: format!(
                    "{} {:?}",
                    self.function_name.0, self.sql_params_in_order
                ),
                source_line_column: format!("{}:{}:{}", file!(), line!(), column!()),
            })
        } else {
            let single_row = multi_row.swap_remove(0);
            Ok(single_row)
        }
    }
}
