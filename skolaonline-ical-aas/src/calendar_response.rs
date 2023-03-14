use std::io::Cursor;

use rocket::{
    http::{ContentType, Status},
    response::{self, Responder},
    Request, Response,
};
use skolaonline_ical::icalendar::Calendar;

pub struct CalendarResponse(pub Calendar);

impl<'r, 'o: 'r> Responder<'r, 'o> for CalendarResponse {
    fn respond_to(self, _req: &'r Request<'_>) -> response::Result<'o> {
        Response::build()
            .status(Status::Ok)
            .header(ContentType::Calendar)
            .streamed_body(Cursor::new(self.0.to_string()))
            .ok()
    }
}
