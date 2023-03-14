use chrono::{Duration, Local};
use rocket::{form::FromForm, http::Status, response::Redirect, Route};
use skolaonline_util::basic_auth_decode;

use crate::{calendar_response::CalendarResponse, error::CalendarEndpointError};

pub fn routes() -> Vec<Route> {
    routes![calendar, calendar_browser, calendar_other]
}

#[derive(Debug, FromForm)]
struct Args {
    auth: String,

    #[allow(clippy::needless_late_init)]
    #[field(default = 7)]
    days_back: u8,

    #[field(default = 28)]
    days_forward: u8,
}

#[get("/?<args..>", format = "text/calendar", rank = 1)]
async fn calendar(args: Args) -> Result<CalendarResponse, CalendarEndpointError> {
    let (username, password) = basic_auth_decode(&args.auth)?;

    let today = Local::now().date_naive();
    let date_from = today - Duration::days(args.days_back.into());
    let date_to = today + Duration::days(args.days_forward.into());

    let calendar =
        skolaonline_ical::fetch_calendar(&username, &password, date_from, Some(date_to)).await?;

    Ok(CalendarResponse(calendar))
}

#[get("/", format = "text/html", rank = 3)]
fn calendar_browser() -> (Status, &'static str) {
    (
        Status::NotAcceptable,
        "You need a calendar client to view a calendar",
    )
}

#[get("/", rank = 4)]
fn calendar_other() -> Redirect {
    Redirect::to(uri!("https://http.cat/406"))
}
