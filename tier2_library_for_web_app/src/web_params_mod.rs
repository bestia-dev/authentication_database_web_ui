// tier2_library_for_web_app/src/web_params_mod.rs

// to enable the trait FromRequest for actix_web::web::Form and actix_web::web::Query
use actix_web::FromRequest;

use super::{
    actix_mod::RequestAndPayload,
    error_mod::{file_line_column, LibError},
};

// type aliases: for less verbose types and better readability of the code
type WebForm = actix_web::web::Form<Vec<(String, String)>>;
type WebQuery = actix_web::web::Query<Vec<(String, String)>>;

/// WebParams are just a key-value collection: std::collections::HashMap(String,String)  
/// A similar collection is found inside POST(form) and GET(web query)  
/// But I want a collection independent of the POST/GET method  
/// [("id", "496953237"), ("webpage", "webpage short url"), ("hit_count", "0")]  
#[derive(Debug)]
pub struct WebParams(pub std::collections::HashMap<String, String>);

impl WebParams {
    /// get WebParams from POST(web form) if exists or else GET(web query)  
    /// If Post(form) exists, then GET(web query) is ignored.  
    /// track_caller decoration makes Location::caller() return the caller location  
    /// for meaningful source code location of the actual error  
    #[track_caller]
    pub async fn from_request_and_payload(req_payload: &mut RequestAndPayload) -> WebParams {
        // form will consume the Payload! All other extractor will get it empty.
        let form: Option<WebForm> =
            actix_web::web::Form::from_request(&req_payload.req, &mut req_payload.payload)
                .await
                .ok();

        if let Some(form) = form {
            // into_iter() consumes the vector. The vector cannot be used after calling this.
            WebParams(form.0.clone().into_iter().collect())
        } else {
            let query: WebQuery =
                actix_web::web::Query::from_request(&req_payload.req, &mut req_payload.payload)
                    .await
                    .unwrap();
            WebParams(query.0.clone().into_iter().collect())
        }
    }

    /// data from WebParams as &str  
    #[track_caller]
    pub fn get_str(&self, param_name: &str) -> Result<&str, LibError> {
        // convert from Option-None to Error with .ok_or()
        let value = self
            .0
            .get(param_name)
            .ok_or(LibError::GetStrFromWebParams {
                user_friendly: param_name.to_string(),
                developer_friendly: format!("{:?}", self.0),
                source_line_column: file_line_column(&std::panic::Location::caller()),
            })?;

        Ok(value)
    }

    /// data from WebParams as i32  
    #[track_caller]
    pub fn get_i32(&self, param_name: &str) -> Result<i32, LibError> {
        let value = self.get_str(param_name)?.parse::<i32>().map_err(|_err| {
            LibError::GetI32FromWebParams {
                user_friendly: param_name.to_string(),
                developer_friendly: format!("{:?}", self.0),
                source_line_column: file_line_column(&std::panic::Location::caller()),
            }
        })?;
        Ok(value)
    }

    /// data from WebParams as bool  
    #[track_caller]
    pub fn get_bool(&self, param_name: &str) -> Result<bool, LibError> {
        let value = self.get_str(param_name)?.parse::<bool>().map_err(|_err| {
            LibError::GetI32FromWebParams {
                user_friendly: param_name.to_string(),
                developer_friendly: format!("{:?}", self.0),
                source_line_column: file_line_column(&std::panic::Location::caller()),
            }
        })?;
        Ok(value)
    }
}
