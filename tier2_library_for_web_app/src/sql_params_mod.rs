// tier2_library_for_web_app/src/sql_params_mod.rs

use std::collections::HashMap;

use crate::postgres_type_mod::PostgresUdtType;

use super::actix_mod::DataAppState;
use super::postgres_mod::{FunctionName, SqlParamsForPostgres};
use super::postgres_type_mod::PostgresValueMultiType as PosType;
use super::web_params_mod::WebParams;

/// SqlParams are just a key-value collection: HashMap(String,PostgresValue)
#[derive(Debug)]
pub struct SqlParams(pub HashMap<String, PosType>);

impl SqlParams {
    pub fn new() -> Self {
        let map: HashMap<String, PosType> = HashMap::new();
        SqlParams(map)
    }
    /// get sql_params for sql function with correct data types from web_params
    /// This is a HashMap with no order.    
    pub fn from_web_params(
        app_state: DataAppState,
        function_name: &FunctionName,
        web_params: &WebParams,
    ) -> Self {
        let mut sql_params = SqlParams::new();

        let name_type = app_state
            .all_sql_function_input_params
            .get(&function_name)
            .unwrap();

        for (param_name, param_input_type) in name_type.iter() {
            let name_wo_prefix = param_name
                .0
                .trim_start_matches("_")
                .trim_start_matches("in_");
            // dbg!(&name);

            // dbg!(param_input_type.as_ref());
            match param_input_type {
                PostgresUdtType::Name | &PostgresUdtType::Text | &PostgresUdtType::Varchar => {
                    sql_params.insert(
                        &param_name.0,
                        PosType::String(web_params.get_str(name_wo_prefix).unwrap().to_string()),
                    );
                }
                PostgresUdtType::Int4 => {
                    sql_params.insert(
                        &param_name.0,
                        PosType::I32(web_params.get_i32(name_wo_prefix).unwrap()),
                    );
                }
                PostgresUdtType::Bool => {
                    sql_params.insert(
                        &param_name.0,
                        PosType::Bool(web_params.get_bool(name_wo_prefix).unwrap()),
                    );
                }
            }
        }
        sql_params
    }

    pub fn insert(&mut self, name: &str, value: PosType) {
        self.0.insert(name.to_string(), value);
    }

    /// returns a reference to an existing vector (params in order and in correct data type)
    /// because this is the format expected by the postgres client library
    pub fn ref_to_function_params(sql_params_in_order: &Vec<PosType>) -> SqlParamsForPostgres {
        let mut ref_to_sql_params: SqlParamsForPostgres = vec![];

        for x in sql_params_in_order.iter() {
            match x {
                PosType::String(xx) => ref_to_sql_params.push(xx),
                PosType::I32(xx) => ref_to_sql_params.push(xx),
                PosType::Bool(xx) => ref_to_sql_params.push(xx),
            }
        }
        ref_to_sql_params
    }

    /// the param order is important to call postgres functions
    pub fn get_sql_params_in_order_and_placeholders(
        &self,
        app_state: DataAppState,
        function_name: &FunctionName,
    ) -> (Vec<PosType>, String) {
        let mut sql_params_in_order: Vec<PosType> = vec![];

        let param_name_order = app_state
            .all_sql_function_input_params_order
            .get(&function_name)
            .unwrap();

        // params must be in the correct order
        let mut placeholders = String::new();
        let mut delimiter = String::new();
        for (i, param_name) in param_name_order.iter().enumerate() {
            let x = self.0.get(&param_name.0).unwrap();
            sql_params_in_order.push(x.clone());
            // placeholders start with $1, not zero
            placeholders.push_str(&format!("{delimiter}${}", i + 1));
            if delimiter.is_empty() {
                delimiter.push_str(", ");
            }
        }
        (sql_params_in_order, placeholders)
    }
}
