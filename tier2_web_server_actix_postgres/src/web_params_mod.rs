// web_params_mod.rs

use std::collections::HashMap;

use crate::{
    actix_mod::ServiceRequestFromRequest,
    error_mod::{file_line_column, LibError},
};

// type aliases: for less verbose types and better readability of the code
type WebForm = actix_web::web::Form<Vec<(String, String)>>;
type WebQuery = actix_web::web::Query<Vec<(String, String)>>;

/// WebParams are just a key-value collection: HashMap(String,String)  
/// A similar collection is found inside POST(form) and GET(web query)  
/// But I want a collection independent of the POST/GET method  
/// [("id", "496953237"), ("webpage", "webpage short url"), ("hit_count", "0")]  
#[derive(Debug)]
pub struct WebParams(pub HashMap<String, String>);

impl WebParams {
    #[track_caller]
    pub async fn from_service_request(srv_req: &mut ServiceRequestFromRequest) -> WebParams {
        let query = srv_req.0.extract::<WebQuery>().await.unwrap();
        let form = srv_req.0.extract::<Option<WebForm>>().await.unwrap();

        Self::from_query_and_form(&query, &form)
    }

    /// get WebParams from POST(form) if exists or else GET(web query)  
    /// If Post(form) exists, then GET(web query) is ignored.  
    /// track_caller decoration makes Location::caller() return the caller location  
    /// for meaningful source code location of the actual error  
    #[track_caller]
    pub fn from_query_and_form(query: &WebQuery, form: &Option<WebForm>) -> WebParams {
        if let Some(form) = form {
            // into_iter() consumes the vector. The vector cannot be used after calling this.
            WebParams(form.0.clone().into_iter().collect())
        } else {
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
}
