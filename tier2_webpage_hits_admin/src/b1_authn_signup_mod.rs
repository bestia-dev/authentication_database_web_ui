// tier2_webpage_hits_admin/src/b1_authn_signup_mod.rs

use T_2::error_mod::LibError;
use T_2::postgres_function_mod::PostgresFunction as P_Func;
use tier0_common_code as T_0;
use tier2_library_for_web_app as T_2;

//use T_0::APP_MAIN_ROUTE;
use T_2::actix_mod::DataAppState;
use T_2::actix_mod::ResultResponse;
use T_2::server_side_single_row_mod::ServerSideSingleRow;
//use T_2::error_mod::LibError;
//use T_2::postgres_mod::get_string_from_row;

const SCOPE: &'static str = "b1_authn_signup_mod";

/// scoped actix routing near the implementation code
/// scope is already "/webpage_hits_admin/b1_authn_signup_mod"
#[rustfmt::skip]
pub fn config_route_authn(cfg: &mut actix_web::web::ServiceConfig) {
    use actix_web::web::resource;
    use actix_web::web::to;
    cfg
        .service(resource("/b1_authn_signup").route(to(b1_authn_signup)))
        .service(resource("/b1_authn_signup_process_email").route(to(b1_authn_signup_process_email)))
        .service(resource("/b1_authn_signup_insert").route(to(b1_authn_signup_insert)))
        .service(resource("/b1_authn_signup_email_verification").route(to(b1_authn_signup_email_verification)))
    ;
}

/// Show the input form. I choose the short name because the url looks nice in the address bar.
#[function_name::named]
pub async fn b1_authn_signup() -> ResultResponse {
    let body = T_2::html_templating_mod::read_template(SCOPE, function_name!());
    Ok(T_2::actix_mod::return_html_response_no_cache(body))
}

/// Ajax: receive json in the POST body.
/// return Random uuid for salt.
pub async fn b1_authn_signup_process_email(
    _app_state: DataAppState,
    _data_req: actix_web::web::Json<T_0::DataReqAuthnSignupProcessEmail>,
) -> ResultResponse {
    // the salt must not have hyphen. It must be b64 compatible. Therefore I use the Simple format.
    let salt = uuid::Uuid::new_v4().simple().to_string();
    // TODO: remember this salt in app_state for a few minutes for the next request.
    // It must be the same.
    // I don't want the client to make random salts
    let data_resp = T_0::DataRespAuthnSignupProcessEmail { salt };
    T_2::actix_mod::return_json_resp_from_object(data_resp)
}

/// insert in table b1_authn_signup and send verification email
#[function_name::named]
pub async fn b1_authn_signup_insert(
    app_state: DataAppState,
    data_req: actix_web::web::Json<T_0::DataReqAuthnSignupInsert>,
) -> ResultResponse {
    // I want to limit the emails that can signup. This is ok for some internal website or intranet.

    // TODO: b1_authn_signup_allowed_email

    if !data_req.user_email.contains("bestia") {
        return Err(LibError::SignupError {
            developer_friendly: String::from("Signup not allowed for this email."),
        }
        .into());
    }

    let verification_uuid = uuid::Uuid::new_v4().simple().to_string();
    let verified = false;
    // I already have all the params as values. I could give the references to the sql function in the correct params order.
    // But I want to give it as named parameters. Then the function will find out the correct order.
    let mut rust_named_params = T_2::rust_named_params_for_sql_mod::RustNamedParamsForSql::new();
    rust_named_params
        .insert("_user_email", &data_req.user_email)
        .insert("_password_hash", &data_req.hash)
        .insert("_verification_uuid", &verification_uuid)
        .insert("_verified", &verified);

    let _single_row = P_Func::run_sql_function_named_params_return_single_row(
        app_state,
        function_name!(),
        &mut rust_named_params,
    )
    .await?;

    let email_subject = "webpage_hits_admin - Email verification!";
    let email_body = format!(
        "Signup to webpage_hits_admin!
Please verify your email, so that we know it's you.
{}://{}/{}/{}/b1_authn_signup_email_verification?uuid={}
",
        *crate::SERVER_PROTOCOL,
        *crate::SERVER_DOMAIN_AND_PORT,
        T_0::APP_MAIN_ROUTE,
        SCOPE,
        verification_uuid
    );

    T_2::sendgrid_mod::send_email(&data_req.user_email, email_subject, email_body).await?;

    let data_resp = T_0::DataRespAuthnSignupInsert {
        signup_success: true,
    };
    T_2::actix_mod::return_json_resp_from_object(data_resp)
}

/// email verification link
#[function_name::named]
pub async fn b1_authn_signup_email_verification(
    mut req_payload: T_2::actix_mod::RequestAndPayload,
) -> ResultResponse {
    let mut sssr = ServerSideSingleRow::new(SCOPE, function_name!(), &mut req_payload).await;
    sssr.run_sql_and_process_html().await
}
