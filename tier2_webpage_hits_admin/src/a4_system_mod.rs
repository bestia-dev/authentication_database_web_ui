// tier2_webpage_hits_admin/src/a4_system_mod.rs

use tier2_library_for_web_app as T_2;
use T_2::actix_mod::ResultResponse;

const SCOPE: &'static str = "a4_system_mod";

/// scoped actix routing near the implementation code
/// scope is already "/webpage_hits_admin/a4_system_mod"
#[rustfmt::skip]
pub fn config_route_authn(cfg: &mut actix_web::web::ServiceConfig) {
    use actix_web::web::resource;
    use actix_web::web::to;
    cfg
        .service(resource("/a4_string_encrypt_decrypt").route(to(a4_string_encrypt_decrypt)))
    ;
}

/// Show the form. All other processing is in WASM.
#[function_name::named]
pub async fn a4_string_encrypt_decrypt() -> ResultResponse {
    let body = T_2::html_templating_mod::read_template(SCOPE, function_name!());
    Ok(T_2::actix_mod::return_html_response_no_cache(body))
}
