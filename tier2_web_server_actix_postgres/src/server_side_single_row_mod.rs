// server_side_single_row.rs

// Structs and methods for server side rendering in web server functions
// for single row: (new, insert, show, edit, update, delete):
// 1. prepare web_params from strings coming from the browser in path, query and form
// 2. prepare PostgresFunction with correct data types from web_params
// 3. run sql function and get single row
// 4. read html template (presentation) from disk or cache
// 5. mix presentation and data, because this is server-side rendering
// 6. return a response with no cache (because data in database can change fast)

use crate::actix_mod::{DataAppState, ResultResponse, WebForm, WebQuery};
use crate::postgres_function_mod::PostgresFunction;
use crate::postgres_mod::FunctionName;
use crate::web_params_mod::WebParams;

/// the main ServerSideSingleRow object (struct with implementation)
pub struct ServerSideSingleRow<'a> {
    scope: &'a str,
    function_name: FunctionName,
    /// process params and runs sql function
    postgres_function: PostgresFunction<'a>,
}

impl<'a> ServerSideSingleRow<'a> {
    /// Constructor from actix extractors: query and form    
    #[track_caller]
    pub fn new_with_query_and_form(
        app_state: &'a DataAppState,
        scope: &'a str,
        function_name: &'static str,
        query: &'a WebQuery,
        form: &'a Option<WebForm>,
    ) -> ServerSideSingleRow<'a> {
        // 1. prepare web_params from strings coming from the browser in path, query and form
        let web_params = WebParams::from_actix(query, form);
        Self::new_with_web_params(app_state, scope, function_name, web_params)
    }

    /// constructor from web_params    
    #[track_caller]
    pub fn new_with_web_params(
        app_state: &'a DataAppState,
        scope: &'a str,
        function_name: &'static str,
        web_params: WebParams,
    ) -> ServerSideSingleRow<'a> {
        // 2. prepare PostgresFunction with correct data types from web_params
        let postgres_function =
            PostgresFunction::new_with_web_params(app_state, function_name, web_params);

        ServerSideSingleRow {
            scope,
            function_name: FunctionName(function_name.to_string()),
            postgres_function,
        }
    }

    /// typical steps for a web app function for single Row sql function
    /// These steps can be called separately if some customization is needed
    pub async fn run_sql_from_web_params_and_process_html(&mut self) -> ResultResponse {
        // region: 3. run sql function and get single row
        let single_row = self
            .postgres_function
            .run_sql_function_return_single_row()
            .await?;
        // endregion

        // region: 4. read html template (presentation) from disk or cache
        let body = crate::html_templating_mod::read_template(self.scope, &self.function_name.0);
        // endregion

        // region: 5. mix presentation and data, because this is server-side rendering
        let body =
            crate::html_templating_mod::template_replace_fields_from_single_row(&body, single_row);
        // endregion

        // region: 6. return a response with no cache (because data in database can change fast)
        crate::actix_mod::return_response_no_cache(body)
        // endregion
    }
}
