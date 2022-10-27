// server_side_multi_row.rs

// Structs and methods for server side rendering in web server functions
// for multi row: (list):
// 1. parse web data: strings coming from the browser in path, query and form
// 2. these are filter data for where and ORDER_BY from the parameters
// 3. cast the web data to call the sql statement with correct data types
// 4. retrieve sql data as vector of rows
// 5. read html template (presentation) from disk or cache
// 6. extract the <!--row_start--> and <!--row_end--> fragment, that will be repeated
// 7. mix presentation and data, because this is server-side rendering
// 8. return a response with no cache (because data in database can change fast)

// TODO: dynamically construct a where clause only for the used filters for efficiency 2022-10-21
// TODO: dynamically construct the fields list only for fields used in the html (for efficiency) 2022-10-21

use lazy_static::lazy_static;
use regex::Regex;

use crate::actix_mod::{DataAppState, RequestAndPayload, ResultResponse};
use crate::postgres_mod::{FieldName, SqlParamsForPostgres, ViewName};
use crate::postgres_type_mod::PostgresValueMultiType;
use crate::sql_params_mod::SqlParams;
use crate::web_params_mod::WebParams;

lazy_static! {
    static ref RGX_01: Regex = Regex::new(r###"\{(.+?)}"###).unwrap();
}

/// the main ServerSideMultiRow object (struct with implementation)
pub struct ServerSideMultiRow {
    app_state: DataAppState,
    scope: &'static str,
    view_name: ViewName,
    web_params: WebParams,
    sql_params_in_order: Vec<PostgresValueMultiType>,
    pub where_clause: Vec<&'static str>,
    sql_where: String,
    sql_order_by: String,
}

impl ServerSideMultiRow {
    /// constructor for the server side rendering object
    /// It takes all the data needed to execute the function
    #[track_caller]
    pub async fn new(
        scope: &'static str,
        view_name: &'static str,
        req_payload: &mut RequestAndPayload,
    ) -> ServerSideMultiRow {
        // region: 1. parse web data: strings coming from the browser in path, query and form
        let web_params = req_payload.web_params().await;
        let app_state = req_payload.app_state().await;
        // endregion

        ServerSideMultiRow {
            app_state,
            scope,
            view_name: ViewName(view_name.to_string()),
            web_params,
            sql_params_in_order: vec![],
            where_clause: vec![],
            sql_where: String::new(),
            sql_order_by: String::new(),
        }
    }

    /// typical steps for a web app function for multi Row sql statement
    /// These steps can be called separately if some customization is needed
    pub async fn run_sql_and_process_html(&mut self) -> ResultResponse {
        // region: 2. find out the filters from the parameters
        self.prepare_filter_params();
        // endregion

        // region: 3. cast the web data to call the function with correct data types
        let sql_params = SqlParams::ref_to_function_params(&self.sql_params_in_order);
        // endregion

        // region: 4. retrieve sql data as vector of rows
        let multi_row = self.run_sql_multi_row_statement(sql_params).await;
        // endregion

        // region: 5. read html template (presentation) from disk or cache
        let mut body = crate::html_templating_mod::read_template(self.scope, &self.view_name.0);
        // endregion

        // region: 6. extract the fragment from <!--row_start--> to <!--row_end-->. It will be repeated for every Row.
        let row_start_outer = body.find("<!--row_start-->").unwrap();
        let row_start_inner = row_start_outer + "<!--row_start-->".len();
        let row_end_inner = body.find("<!--row_end-->").unwrap();
        let row_end_outer = row_end_inner + "<!--row_end-->".len();
        let fragment_for_single_row = &body[row_start_inner..row_end_inner];
        // endregion

        // region: 7. mix presentation and data, because this is server-side rendering
        let mut replaced_with_multi_row = String::new();
        for single_row in multi_row {
            let replaced_fragment =
                crate::html_templating_mod::template_replace_fields_from_single_row(
                    fragment_for_single_row,
                    single_row,
                );
            replaced_with_multi_row.push_str(&replaced_fragment);
        }

        body.replace_range(row_start_outer..row_end_outer, &replaced_with_multi_row);

        // replace the filter fields from the input web_params
        // if there are not input web_params then find and replace with empty
        for (name, value) in self.web_params.0.iter() {
            let from = format!("{{{}}}", name);
            body = body.replace(&from, value);
        }
        // endregion

        // region: 8. return a response with no cache (because data in database can change fast)
        Ok(crate::actix_mod::return_html_response_no_cache(body))
        // endregion
    }

    /// prepares where clause and params inside struct field sql_where and sql_params
    /// filter: f_like_webpage
    /// if starts with f_like_ use the like operator
    pub fn prepare_filter_params(&mut self) {
        let mut placeholder = 1;
        let mut where_inter_word = "where ";

        for single_line_of_where in self.where_clause.iter() {
            // find all variables in {} using regex
            for m in RGX_01.captures_iter(single_line_of_where) {
                // every group captured inside the match
                for i in 1..m.len() {
                    let param_name = m.get(i).unwrap().as_str();
                    // dbg!(param_name);
                    // check if param exists in web_params
                    if let Some(param_from_web) =
                        self.web_params.0.iter().find(|&x| x.0 == param_name)
                    {
                        // the first inter_word is where, later is and
                        self.sql_where.push_str(where_inter_word);
                        if where_inter_word != " and " {
                            where_inter_word = " and ";
                        }

                        let placeholder_str = format!("${placeholder}");
                        placeholder += 1;

                        let param_name_placeholder = format!("{{{}}}", param_name);
                        let where_clause =
                            single_line_of_where.replace(&param_name_placeholder, &placeholder_str);
                        self.sql_where.push_str(&where_clause);
                        self.sql_params_in_order
                            .push(PostgresValueMultiType::String(param_from_web.1.to_string()));
                    }
                }
            }
        }

        // region: order by
        let view_field_type = self.app_state.sql_view_fields.get(&self.view_name).unwrap();

        for (name, value) in self.web_params.0.iter() {
            if !value.is_empty() {
                if name == "f_order_by" {
                    let field_name = FieldName(value.to_string());
                    let _field_type = view_field_type.get(&field_name).unwrap();

                    self.sql_order_by
                        .push_str(&format!("ORDER BY {}", field_name.0));
                } else if name == "f_order_by_direction" {
                    let direction = value.to_string();
                    if direction.to_lowercase() == "desc" {
                        self.sql_order_by.push_str(" DESC ");
                    }
                }
            }
        }
        // endregion: order by

        // dbg!(&self.sql_where);
        // dbg!(&self.sql_params);
        // dbg!(&self.sql_order_by);
    }

    /// run sql multi row statement
    pub async fn run_sql_multi_row_statement(
        &self,
        sql_params: SqlParamsForPostgres<'_>,
    ) -> Vec<tokio_postgres::Row> {
        let postgres_client =
            crate::deadpool_mod::get_postgres_client_from_pool(&self.app_state.db_pool)
                .await
                .unwrap();

        let query = format!(
            "select * from {} {} {};",
            self.view_name.0, self.sql_where, self.sql_order_by
        );
        // dbg!(&query);
        // TODO: convert sql errors in a single place 2022-10-21
        let row_set = postgres_client.query(&query, &sql_params).await.unwrap();

        row_set
    }
}
