//! postgres_mod.rs

// type alias to make it more concise, precise and readable
/// params are mostly searched by param name
pub type ParamsNameType = HashMap<ParamName, PostgresInputType>;
/// functions are always searched by function name
pub type SqlFunctionInputParams = HashMap<FunctionName, ParamsNameType>;
/// functions params must be in correct order
pub type SqlFunctionInputParamsOrder = HashMap<FunctionName, Vec<ParamName>>;
/// fields are always searched by field name
pub type FieldsNameType = HashMap<FieldName, PostgresFieldType>;
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

use crate::{
    error_mod::{file_line_column, LibError},
    postgres_type_mod::{PostgresFieldType, PostgresInputType},
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
    let postgres_client = crate::deadpool_mod::get_postgres_client_from_pool(db_pool).await?;
    postgres_client.query(query, params).await.map_err(|err| {
        /*
        many different sql errors:
        https://github.com/sfackler/rust-postgres/blob/master/tokio-postgres/src/error/sqlstate.rs
        code: SqlState( E42804, ), DATATYPE_MISMATCH
            message: "structure of query does not match function result type",
            detail: Some( "Returned type character varying(100) does not match expected type text in column 2.", ),
            where_: Some( "PL/pgSQL function webpage_hits_insert(integer,text,integer) line 12 at RETURN QUERY", ),
        code: SqlState( E23505, ), SqlState::UNIQUE_VIOLATION
            message: "duplicate key value violates unique constraint \"webpage_uniq_webpage\"",
            detail: Some( "Key (webpage)=(test) already exists.", ),
            where_: Some( "SQL statement \"insert into webpage ( \"id\", webpage)\nvalues (_id, _webpage)\"\nPL/pgSQL function webpage_hits_insert(integer,text,integer) line 6 at SQL statement", ),
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

/// Vector of all function input params with data types.
/// Call it once on application start and store the result in a global variable.
/// Postgres input variables can be prefixed with "in_" or just "_". Take it into consideration.
pub async fn get_for_cache_all_function_input_params(
    db_pool: &deadpool_postgres::Pool,
) -> (SqlFunctionInputParams, SqlFunctionInputParamsOrder) {
    let query = "SELECT proname, args_def from a_list_all_function_input_params;";
    let vec_row = run_sql_select_query_pool(db_pool, query, &vec![])
        .await
        .unwrap();
    let mut all_function_input_params: SqlFunctionInputParams = HashMap::new();
    let mut all_function_input_params_order: SqlFunctionInputParamsOrder = HashMap::new();
    for row in vec_row.iter() {
        // newtype
        let function_name = FunctionName(row.get(0));
        //dbg!(&function_name);
        let args_def: String = row.get(1);
        //dbg!(&args_def);
        let mut hm_name_type: ParamsNameType = HashMap::new();
        let mut params_order = vec![];
        if !args_def.is_empty() {
            for name_and_type in args_def.split(", ") {
                let mut spl = name_and_type.split(' ');
                let param_name = ParamName(spl.next().unwrap().to_string());
                // ignore OUT parameters, only input parameters
                if param_name.0 != "OUT" {
                    let arg_type = spl.next().unwrap().to_string();
                    use std::str::FromStr;
                    let arg_type = PostgresInputType::from_str(&arg_type).unwrap();
                    hm_name_type.insert(param_name.clone(), arg_type);
                    params_order.push(param_name);
                }
            }
        }
        //dbg!(&vec_name_type);
        all_function_input_params.insert(function_name.clone(), hm_name_type);
        all_function_input_params_order.insert(function_name, params_order);
    }
    (all_function_input_params, all_function_input_params_order)
}

/// Hashmap of all view fields with data types. I use it to construct the WHERE clause.
/// Call it once on application start and store the result in a global variable.
pub async fn get_for_cache_all_view_fields(db_pool: &deadpool_postgres::Pool) -> SqlViewFields {
    let query = "SELECT relname, attname, typname from a_list_all_view_fields order by relname;";
    let vec_row = run_sql_select_query_pool(db_pool, query, &vec![])
        .await
        .unwrap();

    let mut all_view_fields: SqlViewFields = HashMap::new();
    let mut hm_name_type = HashMap::new();

    let mut old_relname = ViewName(String::new());
    let mut relname: ViewName;
    for row in vec_row.iter() {
        relname = ViewName(row.get(0));
        if relname != old_relname {
            if !old_relname.0.is_empty() {
                //dbg!(&vec_name_type);
                all_view_fields.insert(old_relname, hm_name_type);
                hm_name_type = HashMap::new();
            }
            old_relname = relname;
        }
        dbg!(&row);
        let attname = FieldName(row.get(1));
        let typname: String = row.get(2);
        use std::str::FromStr;
        let arg_type = PostgresFieldType::from_str(&typname).unwrap();
        hm_name_type.insert(attname, arg_type);
    }
    if !old_relname.0.is_empty() {
        //dbg!(&vec_name_type);
        all_view_fields.insert(old_relname, hm_name_type);
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
