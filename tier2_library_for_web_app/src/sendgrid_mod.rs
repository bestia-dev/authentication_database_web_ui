// tier2_library_for_web_app/src/sendgrid_mod.rs

lazy_static::lazy_static! {
    static ref SENDGRID_API_KEY: String = {
        // The secret API key must be in env variables encrypted with the master_key.
        let api_key_check = std::env::vars().find(|var| var.0 == "SENDGRID_API_KEY");
        let Some(key) = api_key_check
        else{
            panic!("Must supply SENDGRID_API_KEY in environment variables to use sendgrid!")
        };
        let encrypted_string = key.1;
        // region: decrypt
        let master_key = std::env::vars().find(|var| var.0 == "MASTER_KEY").unwrap().1;
        log::info!("mk =  {master_key}");
        let home_dir = dirs::home_dir().unwrap();
        let home_dir = home_dir.to_string_lossy();
        let master_key =  std::fs::read_to_string(&format!("{home_dir}/.ssh/{master_key}")).unwrap();
        let master_key =  master_key.trim();
        use aes_gcm::{
            aead::{Aead, KeyInit},
            Aes256Gcm, Nonce,
        };
        let master_key_byte = hex::decode(&master_key).unwrap();
        let encrypted_string_byte = hex::decode(&encrypted_string).unwrap();
        let key = aes_gcm::aead::generic_array::GenericArray::from_slice(&master_key_byte);
        let cipher = Aes256Gcm::new(&key);
        let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message
        let plaintext = cipher.decrypt(nonce, encrypted_string_byte.as_ref()).unwrap();

        let api_key = String::from_utf8(plaintext).unwrap();
        // endregion: decrypt
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
