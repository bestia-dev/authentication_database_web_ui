// tier2_webpage_hits_admin/src/b1_authn_signup_mod.rs

use reqwest::StatusCode;
use t2::server_side_single_row_mod::ServerSideSingleRow;
use tier0_common_code as t0;
use tier2_library_for_web_app as t2;

//use t0::APP_MAIN_ROUTE;
use t2::actix_mod::DataAppState;
use t2::actix_mod::ResultResponse;
//use t2::error_mod::LibError;
//use t2::postgres_function_mod::PostgresFunction;
//use t2::postgres_mod::get_string_from_row;
use t2::postgres_type_mod::PostgresValueMultiType as PosType;

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
    let body = t2::html_templating_mod::read_template(SCOPE, function_name!());
    Ok(t2::actix_mod::return_html_response_no_cache(body))
}

/// Ajax: receive json in the POST body.
/// return Random uuid for salt.
pub async fn b1_authn_signup_process_email(
    _app_state: DataAppState,
    _data_req: actix_web::web::Json<t0::DataReqAuthnSignupProcessEmail>,
) -> ResultResponse {
    // the salt must not have hyphen. It must be b64 compatible. Therefore I use the Simple format.
    let salt = uuid::Uuid::new_v4().simple().to_string();
    // TODO: remember this salt in app_state for a few minutes for the next request.
    // It must be the same.
    // I don't want the client to make random salts
    let data_resp = t0::DataRespAuthnSignupProcessEmail { salt };
    t2::actix_mod::return_json_resp_from_object(data_resp)
}

/// insert in table b1_authn_signup and send verification email
pub async fn b1_authn_signup_insert(
    app_state: DataAppState,
    data_req: actix_web::web::Json<t0::DataReqAuthnSignupInsert>,
) -> ResultResponse {
    let verification_uuid = uuid::Uuid::new_v4().simple().to_string();
    let mut sql_params = t2::sql_params_mod::SqlParams::new();
    sql_params.insert(
        "_user_email",
        PosType::String(data_req.user_email.to_string()),
    );
    sql_params.insert("_password_hash", PosType::String(data_req.hash.to_string()));
    sql_params.insert(
        "_verification_uuid",
        PosType::String(verification_uuid.to_string()),
    );
    sql_params.insert("_verified", PosType::Bool(false));

    let mut pg_func = t2::postgres_function_mod::PostgresFunction::new_with_sql_params(
        app_state,
        "b1_authn_signup_insert",
        sql_params,
    );
    let _single_row = pg_func.run_sql_function_return_single_row().await?;

    // Send verification mail with async reqwest. Just a json POST to the API.
    // The secret API key must be in env variables.
    let api_key_check = std::env::vars().find(|var| var.0 == "SENDGRID_API_KEY");
    let Some(key) = api_key_check
    else{
        panic!("Must supply SENDGRID_API_KEY in environment variables to use sendgrid!")
    };
    let api_key = key.1;

    let req = reqwest::Client::new()
        .post("https://api.sendgrid.com/v3/mail/send")
        .header("Authorization", &format!("Bearer {api_key}"))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
        "personalizations": [
            {
                "to": [
                    {
                        "email": &data_req.user_email
                    }
                ]
            }
        ],
        "from": {
            "email": "info@bestia.dev"
        },
        "subject": "webpage_hits_admin - Email verification!",
        "content": [
            {
                "type": "text/plain",
                "value": format!("Signup to webpage_hits_admin!
        Please verify your email, so that we know it's you.
        {}://{}/{}/{}/b1_authn_signup_email_verification?uuid={}
        ",*crate::SERVER_PROTOCOL, *crate::SERVER_DOMAIN_AND_PORT,t0::APP_MAIN_ROUTE,SCOPE, verification_uuid)
            }
            ]
        }));
    match req.send().await {
        Err(err) => log::error!("Error: {}", err),
        Ok(body) => {
            if body.status() == StatusCode::ACCEPTED {
                log::info!("Email sent ok!");
            } else {
                log::error!("ERROR Response: {:#?}", body);
            }
        }
    };

    let data_resp = t0::DataRespAuthnSignupInsert {
        signup_success: true,
    };
    t2::actix_mod::return_json_resp_from_object(data_resp)
}

/// email verification link
#[function_name::named]
pub async fn b1_authn_signup_email_verification(
    mut req_payload: t2::actix_mod::RequestAndPayload,
) -> ResultResponse {
    let mut sssr = ServerSideSingleRow::new(SCOPE, function_name!(), &mut req_payload).await;
    sssr.run_sql_and_process_html().await
}
