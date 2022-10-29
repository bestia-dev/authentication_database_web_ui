// tier2_library_for_web_app/src/postgres_mod.rs

// type alias to make it more concise, precise and readable
/// params are mostly searched by param name
pub type ParamsNameType = HashMap<ParamName, PostgresUdtType>;
/// functions are always searched by function name
pub type SqlFunctionInputParams = HashMap<FunctionName, ParamsNameType>;
/// functions params must be in correct order
pub type SqlFunctionInputParamsOrder = HashMap<FunctionName, Vec<ParamName>>;
/// fields are always searched by field name
pub type FieldsNameType = HashMap<FieldName, PostgresUdtType>;
/// views are always searched by view name
pub type SqlViewFields = HashMap<ViewName, FieldsNameType>;
/// this type is used to give parameters to postgres run command
pub type SqlParamsForPostgres<'a> = Vec<&'a (dyn tokio_postgres::types::ToSql + Sync)>;

// newtypes : forces unambiguous intent
#[derive(Eq, Hash, PartialEq, Clone)]
pub struct FunctionName(pub String);
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct ParamName(pub String);
#[derive(Eq, Hash, PartialEq)]
pub struct ViewName(pub String);
#[derive(Eq, Hash, PartialEq)]
pub struct FieldName(pub String);

use super::{
    error_mod::{file_line_column, LibError},
    postgres_type_mod::PostgresUdtType,
};
use std::collections::HashMap;
use tokio_postgres::error::SqlState;

/// run the query and catch the many different sql errors
#[track_caller]
pub async fn run_sql_select_query_pool(
    db_pool: &deadpool_postgres::Pool,
    query: &str,
    params: &SqlParamsForPostgres<'_>,
) -> Result<Vec<tokio_postgres::Row>, LibError> {
    let postgres_client = super::deadpool_mod::get_postgres_client_from_pool(db_pool).await?;
    postgres_client.query(query, params).await.map_err(|err| {
        /*
        many different sql errors:
        https://github.com/sfackler/rust-postgres/blob/master/tokio-postgres/src/error/sqlstate.rs
        code: SqlState( E42804, ), DATATYPE_MISMATCH
            message: "structure of query does not match function result type",
            detail: Some( "Returned type character varying(100) does not match expected type text in column 2.", ),
            where_: Some( "PL/pgSQL function c1_webpage_hits_insert(int4,text,int4) line 12 at RETURN QUERY", ),
        code: SqlState( E23505, ), SqlState::UNIQUE_VIOLATION
            message: "duplicate key value violates unique constraint \"webpage_uniq_webpage\"",
            detail: Some( "Key (webpage)=(test) already exists.", ),
            where_: Some( "SQL statement \"insert into webpage ( \"id\", webpage)\nvalues (_id, _webpage)\"\nPL/pgSQL function c1_webpage_hits_insert(int4,text,int4) line 6 at SQL statement", ),
        */
        let err_code = err.code().unwrap().clone();
        match err_code {
            SqlState::UNIQUE_VIOLATION =>
            // duplicate key value violates unique constraint
            {
                LibError::QueryError {
                    user_friendly: format!("{}", err),
                    source_error: err,
                    developer_friendly: format!("{:?} {} {:?}", err_code, query, params),
                    source_line_column: format!("{}:{}:{}", file!(), line!(), column!()),
                }
            }
            SqlState::DATATYPE_MISMATCH =>
            // structure of query does not match function result type
            {
                LibError::QueryError {
                    user_friendly: format!("{}", err),
                    source_error: err,
                    developer_friendly: format!("{:?} {} {:?}", err_code, query, params),
                    source_line_column: format!("{}:{}:{}", file!(), line!(), column!()),
                }
            }
            _ => LibError::QueryError {
                user_friendly: format!("{}", err),
                source_error: err,
                developer_friendly: format!("{} {:?}", query, params),
                source_line_column: format!("{}:{}:{}", file!(), line!(), column!()),
            },
        }
    })
}

/// Vector of all function input params with "udt" data type names.
/// Call it once on application start and store the result in a global variable.
/// Postgres input variables can be prefixed with "in_" or just "_". Take it into consideration.
pub async fn get_for_cache_all_function_input_params(
    db_pool: &deadpool_postgres::Pool,
) -> (SqlFunctionInputParams, SqlFunctionInputParamsOrder) {
    let query = "
    select routine_name, parameter_name, udt_name 
    from a1_list_all_function_input_params 
    order by routine_name, ordinal_position;
    ";
    let vec_row = run_sql_select_query_pool(db_pool, query, &vec![])
        .await
        .unwrap();

    let mut all_function_input_params: SqlFunctionInputParams = HashMap::new();
    let mut all_function_input_params_order: SqlFunctionInputParamsOrder = HashMap::new();
    let mut old_function_name = FunctionName(String::new());
    let mut function_name: FunctionName;
    let mut hm_name_type: ParamsNameType = HashMap::new();
    let mut params_order = vec![];
    for row in vec_row.iter() {
        function_name = FunctionName(row.get(0));
        if function_name != old_function_name {
            if !old_function_name.0.is_empty() {
                all_function_input_params.insert(old_function_name.clone(), hm_name_type);
                all_function_input_params_order.insert(old_function_name, params_order);
            }
            old_function_name = function_name;
            hm_name_type = HashMap::new();
            params_order = vec![];
        }
        let param_name = ParamName(row.get(1));
        use std::str::FromStr;
        let udt_name = row.get(2);
        //dbg!(&udt_name);
        let arg_type = PostgresUdtType::from_str(udt_name).unwrap();
        hm_name_type.insert(param_name.clone(), arg_type);
        params_order.push(param_name);
        //dbg!(&vec_name_type);
    }
    if !old_function_name.0.is_empty() {
        all_function_input_params.insert(old_function_name.clone(), hm_name_type);
        all_function_input_params_order.insert(old_function_name, params_order);
    }

    (all_function_input_params, all_function_input_params_order)
}

/// Hashmap of all view fields with data types. I use it to construct the where clause.
/// Call it once on application start and store the result in a global variable.
pub async fn get_for_cache_all_view_fields(db_pool: &deadpool_postgres::Pool) -> SqlViewFields {
    let query = "
        select view_name, column_name, udt_name 
        from a1_list_all_view_fields 
        order by view_name;
        ";
    let vec_row = run_sql_select_query_pool(db_pool, query, &vec![])
        .await
        .unwrap();

    let mut all_view_fields: SqlViewFields = HashMap::new();
    let mut hm_name_type = HashMap::new();

    let mut old_view_name = ViewName(String::new());
    let mut view_name: ViewName;
    for row in vec_row.iter() {
        view_name = ViewName(row.get(0));
        if view_name != old_view_name {
            if !old_view_name.0.is_empty() {
                //dbg!(&vec_name_type);
                all_view_fields.insert(old_view_name, hm_name_type);
                hm_name_type = HashMap::new();
            }
            old_view_name = view_name;
        }
        //dbg!(&row);
        let column_name = FieldName(row.get(1));
        let udt_name: String = row.get(2);
        //dbg!(&udt_name);
        use std::str::FromStr;
        let arg_type = PostgresUdtType::from_str(&udt_name).unwrap();
        hm_name_type.insert(column_name, arg_type);
    }
    if !old_view_name.0.is_empty() {
        //dbg!(&vec_name_type);
        all_view_fields.insert(old_view_name, hm_name_type);
    }
    // dbg!(&view_fields);
    all_view_fields
}

pub fn get_string_from_row(
    single_row: &tokio_postgres::Row,
    colum_name: &str,
) -> Result<String, LibError> {
    single_row
        .try_get::<&str, String>(colum_name)
        .map_err(|err| LibError::RowTryGet {
            user_friendly: "password_hash".to_string(),
            developer_friendly: format!("{err} \n  row:{:?}", single_row),
            source_line_column: file_line_column(&std::panic::Location::caller()),
        })
}
