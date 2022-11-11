// tier2_library_for_web_app/src/postgres_function_mod.rs

use crate::postgres_mod::SqlParamsForPostgres;
use crate::rust_named_params_for_sql_mod::RustNamedParamsForSql;

use super::actix_mod::DataAppState;
use super::error_mod::LibError;
use super::postgres_mod::FunctionName;
use super::postgres_type_mod::PostgresValueMultiType as PosType;
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
    sql_params_in_order: Vec<PosType>,
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
        &self,
    ) -> core::result::Result<Vec<tokio_postgres::Row>, LibError> {
        // region: 3. create reference to sql_params_in_order, because postgres_client need this format
        let ref_to_sql_params = SqlParams::ref_to_function_params(&self.sql_params_in_order);
        // endregion
        // 4. run sql function from ref_to_sql_params in order and get single row
        self.run_sql_function_return_multi_row_ref_to_sql_params(ref_to_sql_params)
            .await
    }

    /// run sql function and return multi row (zero or more)
    /// 4. run sql function
    async fn run_sql_function_return_multi_row_ref_to_sql_params<'a>(
        &self,
        ref_to_sql_params: SqlParamsForPostgres<'a>,
    ) -> core::result::Result<Vec<tokio_postgres::Row>, LibError> {
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
        &self,
    ) -> core::result::Result<tokio_postgres::Row, LibError> {
        let multi_row = self.run_sql_function_return_multi_row().await?;
        self.run_sql_function_return_single_row_process(multi_row)
            .await
    }

    /// return exactly single row from function or error
    async fn run_sql_function_return_single_row_ref_to_sql_params<'a>(
        &self,
        ref_to_sql_params: SqlParamsForPostgres<'a>,
    ) -> core::result::Result<tokio_postgres::Row, LibError> {
        let multi_row = self
            .run_sql_function_return_multi_row_ref_to_sql_params(ref_to_sql_params)
            .await?;
        self.run_sql_function_return_single_row_process(multi_row)
            .await
    }

    /// return exactly single row from function or error
    async fn run_sql_function_return_single_row_process(
        &self,
        mut multi_row: Vec<tokio_postgres::Row>,
    ) -> core::result::Result<tokio_postgres::Row, LibError> {
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

    pub async fn run_sql_function_named_params_return_single_row<'a>(
        app_state: DataAppState,
        function_name: &'static str,
        rust_named_params: &'a mut RustNamedParamsForSql<'a>,
    ) -> core::result::Result<tokio_postgres::Row, LibError> {
        let function_name_obj = FunctionName(function_name.to_string());
        let (ref_to_sql_params, placeholders) = get_sql_params_in_order_and_placeholders(
            app_state.clone(),
            function_name,
            rust_named_params,
        );
        let pf = PostgresFunction {
            app_state,
            function_name: function_name_obj,
            sql_params_in_order: vec![],
            placeholders,
        };

        pf.run_sql_function_return_single_row_ref_to_sql_params(ref_to_sql_params)
            .await
    }
}

/// the param order is important to call postgres functions
fn get_sql_params_in_order_and_placeholders<'a>(
    app_state: DataAppState,
    function_name: &'static str,
    rust_named_params: &'a mut RustNamedParamsForSql<'a>,
) -> (SqlParamsForPostgres<'a>, String) {
    let function_name_obj = FunctionName(function_name.to_string());
    let mut sql_params_in_order: SqlParamsForPostgres = vec![];

    let param_name_order = app_state
        .all_sql_function_input_params_order
        .get(&function_name_obj)
        .unwrap();

    // params must be in the correct order
    let mut placeholders = String::new();
    let mut delimiter = String::new();

    for (i, param_name) in param_name_order.iter().enumerate() {
        let pn = param_name.0.clone();
        // I cannot use remove() here, because in a loop it creates multiple mutable reference.
        // I must use clone, but it is not performant.
        sql_params_in_order.push(rust_named_params.0.get(&pn).unwrap().clone());
        // placeholders start with $1, not zero
        placeholders.push_str(&format!("{delimiter}${}", i + 1));
        if delimiter.is_empty() {
            delimiter.push_str(", ");
        }
    }

    (sql_params_in_order, placeholders)
}
