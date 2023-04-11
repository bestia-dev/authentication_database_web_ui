// tier2_webpage_hits_admin/src/b5_start_page_mod.rs

use tier2_library_for_web_app as T_2;

use T_2::actix_mod::ResultResponse;

const SCOPE: &'static str = "b5_start_page_mod";

/// scoped actix routing near the implementation code
/// scope is already "/webpage_hits_admin/b5_start_page_mod"
#[rustfmt::skip]
pub fn config_route_authn(cfg: &mut actix_web::web::ServiceConfig) {
    use actix_web::web::resource;
use actix_web::web::to;
    cfg
        .service(resource("/b5_start_page").route(to(b5_start_page)))       
    ;
}

#[function_name::named]
pub async fn b5_start_page() -> ResultResponse {
    let body = T_2::html_templating_mod::read_template(SCOPE, function_name!());
    Ok(T_2::actix_mod::return_html_response_no_cache(body))
}
