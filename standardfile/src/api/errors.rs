use actix_web::{HttpResponse, http::StatusCode, error};
use actix::MailboxError;

#[derive(Serialize, Deserialize)]
struct ErrorMsg {
    error: Msg
}
#[derive(Serialize, Deserialize)]
struct Msg {
    message: String,
    status: u16
}

#[derive(Debug,Fail)]
pub enum SFError {
    #[fail(display = "Please provide email via GET parameter.")]
    MissingEmail,

    #[fail(display = "Unable to register.")]
    UnableToRegister,

    #[fail(display = "This email is already registered.")]
    AlreadyRegistered,

    #[fail(display = "Invalid login credentials.")]
    InvalidCredentials,

    #[fail(display = "Invalid email or password.")]
    InvalidEmailOrPassword,

    #[fail(display = "Internal Failure")]
    InternalFailure,
}

impl error::ResponseError for SFError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            SFError::MissingEmail => to_err_response(StatusCode::BAD_REQUEST, self),
            SFError::UnableToRegister => to_err_response(StatusCode::UNAUTHORIZED, self),
            SFError::AlreadyRegistered => to_err_response(StatusCode::UNAUTHORIZED, self),
            SFError::InvalidCredentials => to_err_response(StatusCode::UNAUTHORIZED, self),
            SFError::InvalidEmailOrPassword => to_err_response(StatusCode::UNAUTHORIZED, self),

            SFError::InternalFailure => to_err_response(StatusCode::INTERNAL_SERVER_ERROR, self),
       }
    }
}

impl From<MailboxError> for SFError {
    fn from(error: MailboxError) -> Self {
        SFError::InternalFailure // Doesnt matter, just throw a 500 cleanly.
    }
}

fn to_err_response(status: StatusCode, err: &SFError) -> HttpResponse {
    HttpResponse::build(status)
        .json(ErrorMsg {
                    error: Msg {
                        message: format!("{}",err).to_string(),
                        status: status.as_u16()
                    }
        })
}