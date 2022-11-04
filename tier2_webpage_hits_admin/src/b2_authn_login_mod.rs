// tier2_webpage_hits_admin/src/b2_authn_login_mod.rs

use actix_web::web::resource;
use actix_web::web::to;
use tier0_common_code::APP_MAIN_ROUTE;
use tier2_library_for_web_app::actix_mod::DataAppState;
use tier2_library_for_web_app::actix_mod::ResultResponse;
use tier2_library_for_web_app::error_mod::LibError;
use tier2_library_for_web_app::postgres_function_mod::PostgresFunction;
use tier2_library_for_web_app::postgres_mod::get_string_from_row;
use tier2_library_for_web_app::postgres_type_mod::PostgresValueMultiType;

const SCOPE: &'static str = "b2_authn_login_mod";

/// scoped actix routing near the implementation code
/// scope is already "/webpage_hits_admin/b2_authn_login_mod"
#[rustfmt::skip]
pub fn config_route_authn(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
        .service(resource("/b2_authn_login").route(to(b2_authn_login)))
        .service(resource("/b2_authn_login_process_email").route(to(b2_authn_login_process_email)))
        .service(resource("/b2_authn_login_process_hash").route(to(b2_authn_login_process_hash)))
    ;
}

/// Show the input form. I choose the short name because the url looks nice in the address bar.
#[function_name::named]
pub async fn b2_authn_login() -> ResultResponse {
    let body =
        tier2_library_for_web_app::html_templating_mod::read_template(SCOPE, function_name!());
    Ok(tier2_library_for_web_app::actix_mod::return_html_response_no_cache(body))
}

/// read data from postgres database table b2_authn_login for email_user
async fn call_pg_func_auth_login_show(
    user_email: &str,
    app_state: DataAppState,
) -> Result<tokio_postgres::Row, LibError> {
    let mut sql_params = tier2_library_for_web_app::sql_params_mod::SqlParams::new();
    sql_params.insert(
        "_user_email",
        PostgresValueMultiType::String(user_email.to_string()),
    );
    let mut pg_func =
        PostgresFunction::new_with_sql_params(app_state, "b2_authn_login_show", sql_params);
    let single_row = pg_func.run_sql_function_return_single_row().await?;
    Ok(single_row)
}

/// Ajax: receive json in the POST body.
/// Finds the salt from database and return it to the browser as json.
/// I don't want the client to know, that the email is wrong. Because of brute force attacks.
/// If email does not exist return a random salt.
pub async fn b2_authn_login_process_email(
    app_state: DataAppState,
    data_req: actix_web::web::Json<tier0_common_code::DataReqAuthnLoginProcessEmail>,
) -> ResultResponse {
    let salt = match call_pg_func_auth_login_show(&data_req.user_email, app_state).await {
        Err(_err) => {
            // return a random salt, so the client cannot know that the email does not exist in the database. Never trust the client.
            let uuid = uuid::Uuid::new_v4().to_string();
            let salt = uuid.as_str().to_string();
            salt
        }
        Ok(single_row) => {
            let password_hash = get_string_from_row(&single_row, "password_hash")?;
            // extract salt
            let password_hash = password_hash::PasswordHash::new(&password_hash)
                .map_err(|_| tier2_library_for_web_app::error_mod::LibError::PasswordHash)?;

            let salt = password_hash
                .salt
                // ok_or from option to error
                .ok_or(LibError::PasswordHash)?
                .to_string();
            salt
        }
    };

    let data_resp = tier0_common_code::DataRespAuthnLoginProcessEmail { salt };
    tier2_library_for_web_app::actix_mod::return_json_resp_from_object(data_resp)
}

/// b2_authn_login_process_hash
pub async fn b2_authn_login_process_hash(
    app_state: DataAppState,
    data_req: actix_web::web::Json<tier0_common_code::DataReqAuthnLoginProcessHash>,
) -> ResultResponse {
    // check data_req.hash   in database
    let single_row = call_pg_func_auth_login_show(&data_req.user_email, app_state.clone()).await?;
    let password_hash: String = get_string_from_row(&single_row, "password_hash")?;
    let is_login_success = { password_hash == data_req.hash };
    if !is_login_success {
        Err(LibError::AuthenticationFailed.into())
    } else {
        log::info!("The user is authenticated successfully. Returning session_id cookie.");

        // region: add random session_id as UUID into app_state active_sessions
        let uuid = uuid::Uuid::new_v4().to_string();
        app_state
            .active_sessions
            .lock()
            .map_err(|_err| LibError::MutexError)?
            .insert(
                uuid.clone(),
                (
                    data_req.user_email.clone(),
                    tier2_library_for_web_app::error_mod::time_epoch_as_millis(),
                ),
            );
        // endregion: add random session_id as UUID into app_state active_sessions

        // region: create cookie to add to response
        let mut cookie = actix_web::cookie::Cookie::new("session_id", uuid.clone());

        // # About session cookies
        // If 'expires' or 'max-age' is unspecified, the cookie becomes a session cookie.
        // A session finishes when the client shuts down, after which the session cookie is removed.
        // Warning: Many web browsers have a session restore feature that will save all tabs and
        // restore them the next time the browser is used. Session cookies will also be restored,
        // as if the browser was never closed.
        // I will check the session_id expiration on the server, not on the client.

        // very important security setting
        cookie.set_same_site(actix_web::cookie::SameSite::Strict);
        // on the same site, we can have may different applications with different cookies
        cookie.set_path(format!("/{APP_MAIN_ROUTE}/"));
        // this is a session_id cookie that browser sends on every request. Javascript does not need access to it.
        cookie.set_http_only(true);
        // the cookie is sent only over secured https line. Except for localhost.
        cookie.set_secure(true);
        // endregion: create cookie to add to response

        // if successful return response with new session cookie
        let data_resp = tier0_common_code::DataRespAuthnLoginProcessHash {
            login_success: is_login_success,
        };
        tier2_library_for_web_app::actix_mod::return_json_resp_from_object_with_cookie(
            data_resp, cookie,
        )
    }
}
