// tier2_library_for_web_app/src/rust_named_params_for_sql_mod.rs

use std::collections::HashMap;

use crate::postgres_mod::SqlParamForPostgres;

/// RustNamedParamsForSql are just a key-value collection: HashMap<String, SqlParamForPostgres>
/// The value is a reference to a rust type, that can be converted into a postgres type
#[derive(Debug)]
pub struct RustNamedParamsForSql<'a>(pub HashMap<String, SqlParamForPostgres<'a>>);

impl<'a> RustNamedParamsForSql<'a> {
    pub fn new() -> Self {
        let map: HashMap<String, SqlParamForPostgres> = HashMap::new();
        RustNamedParamsForSql(map)
    }

    /// return self to allow chaining calls
    pub fn insert(
        &mut self,
        name: &str,
        value: SqlParamForPostgres<'a>,
    ) -> &mut RustNamedParamsForSql<'a> {
        self.0.insert(name.to_string(), value);
        self
    }

    pub fn get_cloned(&mut self, name: &str) -> SqlParamForPostgres {
        self.0.get(name).unwrap().clone()
    }
}
