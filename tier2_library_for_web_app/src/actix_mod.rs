// tier2_library_for_web_app/src/actix_mod.rs

// type aliases: for less verbose types and better readability of the code
pub type ResultResponse = actix_web::Result<actix_web::HttpResponse>;
pub type DataAppState = actix_web::web::Data<super::AppState>;

use super::error_mod::time_epoch_as_millis;

/// fn to return a response when we have the body
/// web apps modify data all the time, so caching is not good
pub fn return_html_response_no_cache(body: String) -> actix_web::HttpResponse {
    use actix_web::http::header;
    actix_web::HttpResponse::Ok()
        .append_header(header::ContentType(mime::TEXT_HTML_UTF_8))
        .append_header(header::CacheControl(vec![header::CacheDirective::NoStore]))
        .body(body)
}

/// fn to return a json response when we have the a serializable object.
/// It does not return a Result object, only just a response object.
/// The error must be already processed and somehow put inside the data_resp enum or struct.
pub fn return_json_resp_from_object(data_resp: impl serde::Serialize) -> ResultResponse {
    let json_body = serde_json::to_string(&data_resp)?;
    use actix_web::http::header;
    Ok(actix_web::HttpResponse::Ok()
        .append_header(header::ContentType(mime::APPLICATION_JSON))
        .append_header(header::CacheControl(vec![header::CacheDirective::NoStore]))
        .body(json_body))
}

/// fn to return a json response when we have the a serializable object
/// only one function returns also a cookie for session_id
/// web apps modify data all the time, so caching is not good
pub fn return_json_resp_from_object_with_cookie(
    data_resp: impl serde::Serialize,
    cookie: actix_web::cookie::Cookie,
) -> ResultResponse {
    let json_body = serde_json::to_string(&data_resp)?;
    use actix_web::http::header;
    let response = actix_web::HttpResponse::Ok()
        .append_header(header::ContentType(mime::APPLICATION_JSON))
        .append_header(header::CacheControl(vec![header::CacheDirective::NoStore]))
        .cookie(cookie)
        .body(json_body);
    Ok(response)
}

/// when a request is received we check for the session cookie
/// if the cookie does not exist or is incorrect return None
pub fn on_request_received_is_session_cookie_ok(req: &actix_web::dev::ServiceRequest) -> bool {
    log::info!("{}", req.path());
    // Some resources must not be redirected
    if req.path().starts_with(&format!(
        "/{}/b2_authn_login_mod/",
        common_code::APP_MAIN_ROUTE
    )) || req
        .path()
        .starts_with(&format!("/{}/css/", common_code::APP_MAIN_ROUTE))
        || req
            .path()
            .starts_with(&format!("/{}/pkg/", common_code::APP_MAIN_ROUTE))
        || req
            .path()
            .starts_with(&format!("/{}/images/", common_code::APP_MAIN_ROUTE))
    {
        true
    } else {
        match req.cookie("session_id") {
            None => false,
            Some(cookie) => {
                // lock the mutex until it goes out of scope at end of function
                let mut sessions = req
                    .app_data::<actix_web::web::Data<super::AppState>>()
                    .unwrap()
                    .active_sessions
                    .lock()
                    .unwrap();
                // cloned() transforms Option<&T> to Option<T>
                let cookie_opt = sessions.get(cookie.value()).cloned();
                match cookie_opt {
                    None => false,
                    Some((user_email, last_access_time_in_millis)) => {
                        // log::info!( "session: {} {} {}", cookie.value(), &user_email, last_access_time_in_millis );
                        // expires in 10 minutes of inactivity
                        if time_epoch_as_millis() - last_access_time_in_millis > 600_000 {
                            log::info!("The cookie has expired after 10 minutes.");
                            // remove it from sessions
                            sessions.remove(cookie.value());
                            return false;
                        } else {
                            // add local data to this request with req.extension. It can then be retrieved later from request.
                            // this map does not differentiate by name, but by type???
                            // so instead of String, I should use a special type for user_email??
                            // this use is to the bring extensions_mut into scope
                            use actix_web::HttpMessage;
                            req.extensions_mut().insert(user_email.clone());

                            // update last_access_time
                            // log::info!("update last_access_time");
                            sessions.insert(
                                cookie.value().to_string(),
                                (user_email, super::error_mod::time_epoch_as_millis()),
                            );

                            return true;
                        }
                    }
                }
            }
        }
    }
}

/// redirect to login page
pub fn redirect_to_login_page(
    req: actix_web::dev::ServiceRequest,
) -> actix_web::dev::ServiceResponse {
    log::warn!("Request with no correct session cookie. Redirect it to the login page.");
    let host = req.connection_info().host().to_owned();
    let scheme = req.connection_info().scheme().to_owned();
    let url = format!(
        "{scheme}://{host}/{}/b2_authn_login_mod/b2_authn_login",
        common_code::APP_MAIN_ROUTE
    );
    req.into_response(
        // code "Found" 302 is the de-facto standard for redirects for login
        actix_web::HttpResponse::Found()
            .append_header((actix_web::http::header::LOCATION, url))
            .finish(),
    )
}

use actix_utils::future::Ready;
use actix_web::{error::Error, web::Data};

/// TODO: experiment to make an extractor to return HttpRequest and Payload 2022-10-21\
/// Just like ServiceRequest.
pub struct RequestAndPayload {
    pub req: actix_web::HttpRequest,
    pub payload: actix_web::dev::Payload,
}

impl actix_web::FromRequest for RequestAndPayload {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    #[inline]
    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let req_payload = RequestAndPayload {
            req: req.to_owned(),
            payload: payload.take(),
        };

        actix_utils::future::ok(req_payload)
    }
}

use actix_web::FromRequest;

impl RequestAndPayload {
    pub async fn web_params(&mut self) -> super::web_params_mod::WebParams {
        super::web_params_mod::WebParams::from_request_and_payload(self).await
    }

    pub async fn app_state(&mut self) -> Data<super::AppState> {
        actix_web::web::Data::from_request(&self.req, &mut self.payload)
            .await
            .unwrap()
    }
}
