use chrono::{Duration, Local};
use rocket::{form::FromForm, http::Status, response::Redirect, Route};
use rocket_basicauth::BasicAuth;
use skolaonline_ical::CalendarConverterConfig;
use skolaonline_util::basic_auth_decode;

use crate::{calendar_response::CalendarResponse, error::CalendarEndpointError};

pub fn routes() -> Vec<Route> {
    routes![
        calendar_query_auth,
        calendar_header_auth,
        calendar_browser,
        calendar_other
    ]
}

#[derive(Debug, FromForm)]
struct Args {
    #[field(default = 7)]
    days_back: u8,

    #[field(default = 28)]
    days_forward: u8,

    #[field(default = false)]
    teachers_as_attendees: bool,

    #[field(default = false)]
    groups_as_attendees: bool,

    #[field(default = true)]
    universal_description: bool,
}

impl From<Args> for CalendarConverterConfig {
    fn from(value: Args) -> Self {
        Self {
            teachers_as_attendees: value.teachers_as_attendees,
            groups_as_attendees: value.groups_as_attendees,
            universal_description: value.universal_description,
        }
    }
}

#[get("/?<auth>&<args..>", format = "text/calendar", rank = 1)]
async fn calendar_query_auth(
    auth: &str,
    args: Args,
) -> Result<CalendarResponse, CalendarEndpointError> {
    let (username, password) = basic_auth_decode(auth)?;

    let today = Local::now().date_naive();
    let date_from = today - Duration::days(args.days_back.into());
    let date_to = today + Duration::days(args.days_forward.into());

    let calendar = skolaonline_ical::fetch_calendar(
        &username,
        &password,
        date_from,
        Some(date_to),
        &args.into(),
    )
    .await?;

    Ok(CalendarResponse(calendar))
}

#[get("/?<args..>", format = "text/calendar", rank = 2)]
async fn calendar_header_auth(
    auth: BasicAuth,
    args: Args,
) -> Result<CalendarResponse, CalendarEndpointError> {
    let today = Local::now().date_naive();
    let date_from = today - Duration::days(args.days_back.into());
    let date_to = today + Duration::days(args.days_forward.into());

    let calendar = skolaonline_ical::fetch_calendar(
        &auth.username,
        &auth.password,
        date_from,
        Some(date_to),
        &args.into(),
    )
    .await?;

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
