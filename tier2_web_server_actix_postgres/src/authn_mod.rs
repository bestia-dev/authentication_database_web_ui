//! authn_mod.rs

// type aliases: for less verbose types and better readability of the code

use crate::actix_mod::DataAppState;
use crate::actix_mod::ResultResponse;
use crate::error_mod::LibError;
use crate::postgres_function_mod::PostgresFunction;
use crate::postgres_mod::get_string_from_row;
use crate::postgres_type_mod::PostgresValueMultiType;
use actix_web::cookie::SameSite;
use actix_web::web::resource;
use actix_web::web::to;

use crate::APP_MAIN_ROUTE;
const SCOPE: &'static str = "authn";

/// scoped actix routing near the implementation code
/// scope is already "/webpage_hits_admin/authn"
#[rustfmt::skip]
pub fn config_route_authn(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
        .service(resource("/authn_login").route(to(authn_login)))
        .service(resource("/authn_login_process_email").route(to(authn_login_process_email)))
        .service(resource("/authn_login_process_hash").route(to(authn_login_process_hash)))
    ;
}

/// Show the input form. I choose the short name because the url looks nice in the address bar.
#[function_name::named]
pub async fn authn_login() -> ResultResponse {
    let body = crate::html_templating_mod::read_template(SCOPE, function_name!());
    crate::actix_mod::return_response_no_cache(body)
}

/// Ajax: receive json in the POST body.
/// finds the salt and return it to the browser as json.
/// Even if it does not find it it returns something, but not an error.
/// I don't want the user to know, that the email is wrong. Because of brute force attacks.
// #[function_name::named]
pub async fn authn_login_process_email(
    app_state: DataAppState,
    data_req: actix_web::web::Json<common_code::DataReqAuthnLoginProcessEmail>,
) -> ResultResponse {
    // TODO: return the result also as json 2022-10-21
    let single_row = call_pg_func_auth_login_show(&data_req.user_email, &app_state).await?;
    let password_hash = get_string_from_row(&single_row, "password_hash")?;
    // extract salt
    let password_hash = password_hash::PasswordHash::new(&password_hash).unwrap();

    let data_resp = common_code::DataRespAuthnLoginProcessEmail {
        salt: password_hash.salt.unwrap().to_string(),
    };
    crate::actix_mod::return_json_from_object(data_resp)
}

/// read data from table authn_login for email_user
async fn call_pg_func_auth_login_show(
    user_email: &str,
    app_state: &DataAppState,
) -> Result<tokio_postgres::Row, LibError> {
    let mut sql_params = crate::sql_params_mod::SqlParams::new();
    sql_params.insert(
        "_user_email",
        PostgresValueMultiType::String(user_email.to_string()),
    );
    let mut pg_func =
        PostgresFunction::new_with_sql_params(&app_state, "authn_login_show", sql_params);
    let single_row = pg_func.run_sql_function_return_single_row().await?;
    Ok(single_row)
}

/// authn_login_process_hash
// #[function_name::named]
pub async fn authn_login_process_hash(
    app_state: DataAppState,
    data_req: actix_web::web::Json<common_code::DataReqAuthnLoginProcessHash>,
) -> ResultResponse {
    // check data_req.hash   in database
    let single_row = call_pg_func_auth_login_show(&data_req.user_email, &app_state).await?;
    let password_hash: String = get_string_from_row(&single_row, "password_hash")?;
    let is_login_success = { password_hash == data_req.hash };
    let data_resp = common_code::DataRespAuthnLoginProcessHash {
        login_success: is_login_success,
    };
    if is_login_success {
        log::info!("The user is authenticated successfully. Returning session_id cookie.");

        // random session_id as UUID
        let uuid = uuid::Uuid::new_v4().to_string();
        // add to app_state active_sessions
        app_state.active_sessions.lock().unwrap().insert(
            uuid.clone(),
            (
                data_req.user_email.clone(),
                crate::error_mod::time_epoch_as_millis(),
            ),
        );
        // create cookie and add to response
        let mut cookie = actix_web::cookie::Cookie::new("session_id", uuid.clone());

        // # About session cookies
        // If 'expires' or 'max-age' is unspecified, the cookie becomes a session cookie.
        // A session finishes when the client shuts down, after which the session cookie is removed.
        // Warning: Many web browsers have a session restore feature that will save all tabs and
        // restore them the next time the browser is used. Session cookies will also be restored,
        // as if the browser was never closed.
        // I will check the session_id expiration on the server, not on the client.

        // very important security setting
        cookie.set_same_site(SameSite::Strict);
        // on the same site, we can have may different applications with different cookies
        cookie.set_path(format!("/{APP_MAIN_ROUTE}/"));
        // this is a session_id cookie that browser sends on every request. Javascript does not need access to it.
        cookie.set_http_only(true);
        // the cookie is sent only over secured https line. Except for localhost.
        cookie.set_secure(true);
        // if successful return new session cookie
        crate::actix_mod::return_json_from_object_with_cookie(data_resp, cookie)
    } else {
        log::warn!("The user authentication failed.");
        crate::actix_mod::return_json_from_object(data_resp)
    }
}
