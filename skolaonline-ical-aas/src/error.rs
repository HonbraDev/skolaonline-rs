use std::io::Cursor;

use rocket::{
    http::Status,
    response::{self, Responder},
    Request, Response,
};
use skolaonline_ical::FetchCalendarError;
use skolaonline_util::BasicAuthDecodeError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CalendarEndpointError {
    #[error("Failed to parse auth string: {0}")]
    FailedToParseAuth(#[from] BasicAuthDecodeError),

    #[error("Failed to fetch the calendar: {0}")]
    FetchCalendar(#[from] FetchCalendarError),
}

impl<'r, 'o: 'r> Responder<'r, 'o> for CalendarEndpointError {
    fn respond_to(self, _req: &'r Request<'_>) -> response::Result<'o> {
        let status = match self {
            _ => Status::InternalServerError,
        };
        Response::build()
            .status(status)
            .streamed_body(Cursor::new(self.to_string()))
            .ok()
    }
}
