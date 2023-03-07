use chrono::{Duration, Local};
use rocket::{
    form::FromForm,
    http::Status,
    response::{status, Redirect},
};
use skolaonline_util::basic_auth_decode;

use crate::error::CalendarEndpointError;

#[get("/")]
pub fn index() -> status::Custom<&'static str> {
    status::Custom(Status::ImATeapot, "hello your computer has virus")
}

#[derive(Debug, FromForm)]
pub struct Args {
    auth: String,

    #[field(default = 7)]
    days_back: u8,

    #[field(default = 28)]
    days_forward: u8,
}

#[get("/calendar/v2?<args..>", format = "text/calendar", rank = 1)]
pub async fn calendar(args: Args) -> Result<String, CalendarEndpointError> {
    let (username, password) = basic_auth_decode(&args.auth)?;

    let today = Local::now().date_naive();
    let date_from = today - Duration::days(args.days_back.into());
    let date_to = today + Duration::days(args.days_forward.into());

    let calendar =
        skolaonline_ical::fetch_calendar(&username, &password, date_from, Some(date_to)).await?;

    Ok(calendar.to_string())
}

#[get("/calendar/v2", format = "text/html", rank = 3)]
pub fn calendar_browser() -> &'static str {
    "Sorry, you need a calendar client to view the calendar."
}

#[get("/calendar/v2", rank = 4)]
pub fn calendar_not_acceptable() -> Redirect {
    Redirect::to(uri!("https://http.cat/406"))
}
