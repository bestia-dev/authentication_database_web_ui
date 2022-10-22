// error_mod.rs

/// enum for all library errors with thiserror \
/// thiserror generates the Display trait for enum variants \
/// user_friendly is for user message, developer_friendly is for developer log
#[derive(thiserror::Error, Debug)]
pub enum LibError {
    /// Database connection error
    #[error("Database connection error")]
    DatabaseConnection,

    /// Mutex error
    #[error("Mutex error")]
    MutexError,

    /// Authentication failed
    #[error("Authentication failed")]
    AuthenticationFailed,

    /// PasswordHash error
    #[error("PasswordHash error")]
    PasswordHash,
    /// Query error: {user_friendly}
    #[error("Query error: {user_friendly}")]
    QueryError {
        source_error: tokio_postgres::Error,
        user_friendly: String,
        developer_friendly: String,
        source_line_column: String,
    },
    /// The value does not exist in web query: {user_friendly}
    #[error("The value does not exist in web query: {user_friendly}")]
    GetStrFromWebParams {
        user_friendly: String,
        developer_friendly: String,
        source_line_column: String,
    },
    /// The value is not i32: {user_friendly}
    #[error("The value is not i32: {user_friendly}")]
    GetI32FromWebParams {
        user_friendly: String,
        developer_friendly: String,
        source_line_column: String,
    },
    /// The column does not exist in row: {user_friendly}
    #[error("The column does not exist in row: {user_friendly}")]
    RowTryGet {
        user_friendly: String,
        developer_friendly: String,
        source_line_column: String,
    },
    /// The query returned zero rows.
    #[error("The query returned zero rows.")]
    QueryReturnZeroRow {
        developer_friendly: String,
        source_line_column: String,
    },
    /// The query returned more than one row.
    #[error("The query returned more than one row.")]
    QueryReturnMoreThanOneRow {
        developer_friendly: String,
        source_line_column: String,
    },
    /// Serde_json parse error.
    #[error("Serde_json parse error.")]
    SerdeJsonParseError {
        developer_friendly: String,
        source_line_column: String,
    },
    /*
        #[error(transparent)]
        Unknown(#[from] anyhow::Error),
    */
}

/// Actix error has the trait 'ResponseError' for custom errors to return a coherent error response to the client.
/// This web application is heterogeneous and can return html or json responses or anything else.
/// This trait cannot distinguish between this two. I have to choose one option and stick with it.
/// When there is code running on the client I can receive anything and transform it as I wish.
/// But for clean server side rendering there is no option of manipulating anything on the client.
/// So I choose that the only reasonable response is fully server side rendered HTML.
/// I will use the 'id' attribute to later extract data out of HTML when needed.
impl actix_web::ResponseError for LibError {
    /// html status code for error
    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    /// Log the developer_friendly on the server
    /// respond the user_friendly to the user
    fn error_response(&self) -> actix_web::HttpResponse {
        let status_code = self.status_code();
        // more information for the developer
        // I need the exact time to match the user message with the log
        let time = time_epoch_as_millis();

        // log is developer friendly with many more info
        log::error!("{time} {}\n{:#?}", self, self);

        // only the user-friendly error for the user
        let error_text = format!("{time} {}", self);
        let body = crate::html_templating_mod::read_template("error", "error");
        let body = body.replace("{error_text}", &error_text);

        actix_web::HttpResponse::build(status_code).body(body)
    }
}

/// time as a big Unix epoch int
pub fn time_epoch_as_millis() -> u128 {
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    time
}

pub fn file_line_column(source_caller_location: &std::panic::Location) -> String {
    format!(
        "{}:{}:{}",
        source_caller_location.file(),
        source_caller_location.line(),
        source_caller_location.column()
    )
}

// implementing from, when I need to convert one type of error to the other

impl From<serde_json::error::Error> for LibError {
    fn from(_err: serde_json::error::Error) -> LibError {
        LibError::SerdeJsonParseError {
            developer_friendly: String::new(),
            source_line_column: String::new(),
        }
    }
}

impl<T> From<std::sync::PoisonError<T>> for LibError {
    fn from(_err: std::sync::PoisonError<T>) -> LibError {
        LibError::MutexError
    }
}
