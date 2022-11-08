// tier2_webpage_hits_admin/src/b1_authn_signup_mod.rs

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
    let mut sql_params = t2::sql_params_mod::SqlParams::new();
    sql_params.insert(
        "_user_email",
        PosType::String(data_req.user_email.to_string()),
    );
    sql_params.insert("_password_hash", PosType::String(data_req.hash.to_string()));
    sql_params.insert("_verified", PosType::Bool(false));

    let mut pg_func = t2::postgres_function_mod::PostgresFunction::new_with_sql_params(
        app_state,
        "b1_authn_signup_insert",
        sql_params,
    );
    let _single_row = pg_func.run_sql_function_return_single_row().await?;

    // TODO: send verification mail. It must be async and not blocking, because of reqwest!

    /*
    curl --request POST \
  --url https://api.sendgrid.com/v3/mail/send \
  --header 'Authorization: Bearer YOUR_API_KEY' \
  --header 'Content-Type: application/json' \
  --data '{"personalizations": [{"to": [{"email": "recipient@example.com"}]}],"from": {"email": "sendeexampexample@example.com"},"subject": "Hello, World!","content": [{"type": "text/plain", "value": "Heya!"}]}'

    */

    let api_key_check = std::env::vars().find(|var| var.0 == "SENDGRID_API_KEY");
    let Some(key) = api_key_check
    else{
        panic!("Must supply API key in environment variables to use!")
    };
    let api_key = key.1;

    //let mut cool_header = std::collections::HashMap::with_capacity(2);
    //cool_header.insert(String::from("x-cool"), String::from("indeed"));
    //cool_header.insert(String::from("x-cooler"), String::from("cold"));

    let p = sendgrid::v3::Personalization::new(sendgrid::v3::Email::new("info@bestia.dev"));
        //.add_headers(cool_header);

    let m = sendgrid::v3::Message::new(sendgrid::v3::Email::new("luciano.bestia@gmail.com"))
        .set_subject("Subject")
        .add_content(
            sendgrid::v3::Content::new()
                .set_content_type("text/html")
                .set_value("Test"),
        )
        .add_personalization(p);

    let sender = sendgrid::v3::Sender::new(api_key);

    match sender.send(&m).await {
        Err(err) => println!("Error: {}", err),
        Ok(body) => println!("Response: {:?}", body),
    };

    let data_resp = t0::DataRespAuthnSignupInsert {
        signup_success: true,
    };
    t2::actix_mod::return_json_resp_from_object(data_resp)
}
