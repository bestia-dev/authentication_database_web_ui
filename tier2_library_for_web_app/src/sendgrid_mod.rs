// tier2_library_for_web_app/src/sendgrid_mod.rs

lazy_static::lazy_static! {
    static ref SENDGRID_API_KEY: String = {
        // The secret API key must be in env variables.
        let api_key_check = std::env::vars().find(|var| var.0 == "SENDGRID_API_KEY");
        let Some(key) = api_key_check
        else{
            panic!("Must supply SENDGRID_API_KEY in environment variables to use sendgrid!")
        };
        let api_key = key.1;
        api_key
    };
}

/// Send mail with async reqwest. Just a json POST to the sendgrid API.
pub async fn send_email(
    user_email: &str,
    subject: &str,
    email_body: String,
) -> Result<(), crate::error_mod::LibError> {
    let req = reqwest::Client::new()
        .post("https://api.sendgrid.com/v3/mail/send")
        .header("Authorization", &format!("Bearer {}", *SENDGRID_API_KEY))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "personalizations": [
                {
                    "to": [
                        {
                            "email": user_email
                        }
                    ]
                }
            ],
            "from": {
                "email": "info@bestia.dev"
            },
            "subject": subject,
            "content": [
                {
                    "type": "text/plain",
                    "value": email_body
                }
            ]
        }));

    match req.send().await {
        Err(err) => Err(crate::error_mod::LibError::SendEmailError {
            developer_friendly: format!("ERROR: {:#?}", err),
        }),
        Ok(body) => {
            if body.status() == reqwest::StatusCode::ACCEPTED {
                Ok(())
            } else {
                Err(crate::error_mod::LibError::SendEmailError {
                    developer_friendly: format!("ERROR Response: {:#?}", body),
                })
            }
        }
    }
}
