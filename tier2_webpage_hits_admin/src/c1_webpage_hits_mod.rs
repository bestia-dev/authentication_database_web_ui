// tier2_webpage_hits_admin/src/c1_webpage_hits_mod.rs

// type aliases: for less verbose types and better readability of the code

use tier2_library_for_web_app as T_2;

use T_2::actix_mod::{RequestAndPayload, ResultResponse};
use T_2::server_side_multi_row_mod::ServerSideMultiRow;
use T_2::server_side_single_row_mod::ServerSideSingleRow;

const SCOPE: &'static str = "c1_webpage_hits_mod";

/// scoped actix routing near the implementation code
/// scope is already "/webpage_hits_admin/c1_webpage_hits_mod"
#[rustfmt::skip]
pub fn config_route_webpage_hits(cfg: &mut actix_web::web::ServiceConfig) {
    use actix_web::web::resource;
use actix_web::web::to;
    cfg
        .service(resource("/c1_webpage_hits_list").route(to(c1_webpage_hits_list)))
        .service(resource("/c1_webpage_hits_new").route(to(c1_webpage_hits_new)))
        .service(resource("/c1_webpage_hits_edit").route(to(c1_webpage_hits_edit)))
        .service(resource("/c1_webpage_hits_insert").route(to(c1_webpage_hits_insert)))
        .service(resource("/c1_webpage_hits_show").route(to(c1_webpage_hits_show)))
        .service(resource("/c1_webpage_hits_update").route(to(c1_webpage_hits_update)))
        .service(resource("/c1_webpage_hits_delete").route(to(c1_webpage_hits_delete)))
    ;
}

/// CRUD - read (list all webpages and counts) with simple filter and order_by
#[function_name::named]
pub async fn c1_webpage_hits_list(mut req_payload: RequestAndPayload) -> ResultResponse {
    let mut ssmr = ServerSideMultiRow::new(SCOPE, function_name!(), &mut req_payload).await;
    // The where statement is constructed only for existing parameters, because efficiency.
    ssmr.where_clause = vec![
        "webpage like {f_like_webpage}",
        "hit_counter > {f_gt_hit_counter}",
        "hit_counter < {f_lt_hit_counter}",
    ];

    ssmr.run_sql_and_process_html().await
}

/// UI - new record
#[function_name::named]
pub async fn c1_webpage_hits_new(mut req_payload: RequestAndPayload) -> ResultResponse {
    let mut sssr = ServerSideSingleRow::new(SCOPE, function_name!(), &mut req_payload).await;
    sssr.run_sql_and_process_html().await
}

/// UI - edit record
#[function_name::named]
pub async fn c1_webpage_hits_edit(mut req_payload: RequestAndPayload) -> ResultResponse {
    let mut sssr = ServerSideSingleRow::new(SCOPE, function_name!(), &mut req_payload).await;
    sssr.run_sql_and_process_html().await
}

/// CRUD - create(insert)
#[function_name::named]
pub async fn c1_webpage_hits_insert(mut req_payload: RequestAndPayload) -> ResultResponse {
    let mut sssr = ServerSideSingleRow::new(SCOPE, function_name!(), &mut req_payload).await;
    sssr.run_sql_and_process_html().await
}

/// CRUD - read (show one record)
#[function_name::named]
pub async fn c1_webpage_hits_show(mut req_payload: RequestAndPayload) -> ResultResponse {
    let mut sssr = ServerSideSingleRow::new(SCOPE, function_name!(), &mut req_payload).await;
    sssr.run_sql_and_process_html().await
}

/// CRUD - update
#[function_name::named]
pub async fn c1_webpage_hits_update(mut req_payload: RequestAndPayload) -> ResultResponse {
    let mut sssr = ServerSideSingleRow::new(SCOPE, function_name!(), &mut req_payload).await;
    sssr.run_sql_and_process_html().await
}

/// CRUD - delete
#[function_name::named]
pub async fn c1_webpage_hits_delete(mut req_payload: RequestAndPayload) -> ResultResponse {
    let mut sssr = ServerSideSingleRow::new(SCOPE, function_name!(), &mut req_payload).await;
    sssr.run_sql_and_process_html().await
}
