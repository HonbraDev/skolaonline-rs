use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use axum_auth::AuthBasic;
use skolaonline_ical::fetch_calendar;
use thiserror::Error;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/calendar/v2", get(calendar));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn calendar(
    AuthBasic((username, password)): AuthBasic,
) -> Result<Response<String>, AppError> {
    match password {
        Some(password) => {
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
        None => Err(AppError(anyhow::Error::msg("Unauthorized"))),
    }
}

#[derive(Debug, Error)]
enum CalendarError {
    #[error("Unauthorized")]
    Unauthorized,

    #[error("Invalid credentials")]
    InvalidCredentials,
}

impl IntoResponse for CalendarError {
    fn into_response(self) -> Response {}
}

struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
