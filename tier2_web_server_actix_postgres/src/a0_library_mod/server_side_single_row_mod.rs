// tier2_web_server_actix_postgres/src/a0_library_mod/server_side_single_row_mod.rs

// Structs and methods for server side rendering in web server functions
// for single row: (new, insert, show, edit, update, delete):
// 1. prepare web_params from strings coming from the browser in path, query and form
// 2. prepare PostgresFunction with correct data types from web_params
// 3. run sql function and get single row
// 4. read html template (presentation) from disk or cache
// 5. mix presentation and data, because this is server-side rendering
// 6. return a response with no cache (because data in database can change fast)

use super::actix_mod::{RequestAndPayload, ResultResponse};
use super::postgres_function_mod::PostgresFunction;
use super::postgres_mod::FunctionName;

/// the main ServerSideSingleRow object (struct with implementation)
pub struct ServerSideSingleRow<'a> {
    scope: &'a str,
    function_name: FunctionName,
    /// process params and runs sql function
    postgres_function: PostgresFunction,
}

impl<'a> ServerSideSingleRow<'a> {
    #[track_caller]
    pub async fn new(
        scope: &'static str,
        view_name: &'static str,
        req_payload: &mut RequestAndPayload,
    ) -> ServerSideSingleRow<'a> {
        // region: 1. parse web data: strings coming from the browser in path, query and form
        let web_params = req_payload.web_params().await;
        let app_state = req_payload.app_state().await;
        // endregion

        // 2. prepare PostgresFunction with correct data types from web_params
        let postgres_function =
            PostgresFunction::new_with_web_params(app_state, view_name, web_params);

        ServerSideSingleRow {
            scope,
            function_name: FunctionName(view_name.to_string()),
            postgres_function,
        }
    }

    /// typical steps for a web app function for single Row sql function
    /// These steps can be called separately if some customization is needed
    pub async fn run_sql_and_process_html(&mut self) -> ResultResponse {
        // region: 3. run sql function and get single row
        let single_row = self
            .postgres_function
            .run_sql_function_return_single_row()
            .await?;
        // endregion

        // region: 4. read html template (presentation) from disk or cache
        let body = super::html_templating_mod::read_template(self.scope, &self.function_name.0);
        // endregion

        // region: 5. mix presentation and data, because this is server-side rendering
        let body =
            super::html_templating_mod::template_replace_fields_from_single_row(&body, single_row);
        // endregion

        // region: 6. return a response with no cache (because data in database can change fast)
        Ok(super::actix_mod::return_html_response_no_cache(body))
        // endregion
    }
}
