use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_auth::AuthBasic;
use skolaonline_ical::{fetch_calendar, FetchCalendarError, SOError};
use thiserror::Error;

pub async fn calendar(
    AuthBasic((username, password)): AuthBasic,
) -> Result<Response<String>, CalendarError> {
    match password {
        Some(password) => {
            if password.is_empty() {
                return Err(CalendarError::NoCredentials);
            }

            let today = chrono::Local::now().date_naive();
            let from = today - chrono::Duration::days(7);
            let to = today + chrono::Duration::days(30);

            let calendar = fetch_calendar(&username, &password, from, Some(to)).await?;

            let response = Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/calendar")
                .body(calendar.to_string())
                .unwrap();

            Ok(response)
        }
        None => Err(CalendarError::NoCredentials),
    }
}

#[derive(Debug, Error)]
pub enum CalendarError {
    #[error("No credentials supplied")]
    NoCredentials,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Calendar error: {0}")]
    OtherFetchError(anyhow::Error),
}

impl IntoResponse for CalendarError {
    fn into_response(self) -> Response {
        match self {
            CalendarError::NoCredentials => (
                StatusCode::UNAUTHORIZED,
                "Please provide your credentials using the HTTP Authorization header.\nMore information: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Authorization#basic",
            )
                .into_response(),

            CalendarError::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "The remote server has rejected your credentials. Please check that they're valid.").into_response()
            }

            CalendarError::OtherFetchError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("An error has occured while fetching the calendar. More information can be found below:\n{err}"),
            )
                .into_response(),
        }
    }
}

impl From<FetchCalendarError> for CalendarError {
    fn from(value: FetchCalendarError) -> Self {
        match value {
            FetchCalendarError::Unauthorized => CalendarError::InvalidCredentials,
            FetchCalendarError::SOError(err) => match err {
                SOError::BadStatus(status) => {
                    if status == 401 {
                        CalendarError::InvalidCredentials
                    } else {
                        CalendarError::OtherFetchError(err.into())
                    }
                }
                _ => CalendarError::OtherFetchError(err.into()),
            },
            FetchCalendarError::Other(err) => CalendarError::OtherFetchError(err),
        }
    }
}
