use anyhow::Result;
use chrono::{NaiveDate, TimeZone, Utc};
use chrono_tz::Tz;
use icalendar::{Calendar, CalendarDateTime, Component, Event};
use skolaonline::{models::rozvrh::RozvrhovaUdalost, SOClient};
use thiserror::Error;

pub use skolaonline::{SOError, SOResult};

const TZ: Tz = chrono_tz::Europe::Prague;

pub async fn fetch_calendar(
    username: &str,
    password: &str,
    start: NaiveDate,
    end: Option<NaiveDate>,
) -> Result<Calendar, FetchCalendarError> {
    let client = SOClient::new(username, password);

    if !client.get_auth_status().await? {
        return Err(FetchCalendarError::Unauthorized);
    }

    let events = client.get_events(start, end).await?;

    let calendar = convert_to_ical(events);

    Ok(calendar)
}

#[derive(Debug, Error)]
pub enum FetchCalendarError {
    #[error("Unauthorized")]
    Unauthorized,

    #[error("Failed to fetch events")]
    SOError(#[from] SOError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub fn convert_to_ical(udalosti: Vec<RozvrhovaUdalost>) -> Calendar {
    let mut cal = Calendar::new();

    for udalost in udalosti {
        cal.push(convert_event_to_ical(udalost));
    }

    cal
}

pub fn convert_event_to_ical(udalost: RozvrhovaUdalost) -> Event {
    let mut eve = Event::new();

    // Date & time
    let start = TZ
        .from_local_datetime(&udalost.cas_od)
        .unwrap()
        .with_timezone(&Utc);
    let end = TZ
        .from_local_datetime(&udalost.cas_do)
        .unwrap()
        .with_timezone(&Utc);
    eve.starts(CalendarDateTime::from(start));
    eve.ends(CalendarDateTime::from(end));

    // Title
    eve.summary(
        udalost
            .predmet
            .is_some()
            .then(|| udalost.predmet.unwrap().nazev)
            .unwrap_or(udalost.nazev)
            .as_str(),
    );

    // Location
    let location_str = udalost
        .mistnosti_udalosti
        .into_iter()
        .map(|m| m.nazev)
        .collect::<Vec<String>>()
        .join(", ");
    if !location_str.is_empty() {
        eve.location(location_str.as_str());
    }

    // Teachers
    let teachers_str = udalost
        .ucitele_udalosti
        .into_iter()
        .map(|u| format!("{} {}", u.jmeno, u.prijmeni))
        .collect::<Vec<String>>()
        .join(", ");

    // Groups
    let groups_str = udalost
        .skupiny_udalosti
        .into_iter()
        .map(|s| {
            if s.skupina_nazev == s.trida_nazev {
                s.skupina_nazev
            } else {
                format!("{} ({})", s.skupina_nazev, s.trida_nazev)
            }
        })
        .collect::<Vec<String>>()
        .join(", ");

    // Description; The `poznamka` prop seems to be unused, but I'll still include it
    let description = [
        Some(location_str),
        Some(teachers_str),
        Some(groups_str),
        udalost.popis,
        udalost.poznamka,
    ]
    .into_iter()
    .flatten()
    .filter(|s| !s.is_empty())
    .collect::<Vec<String>>()
    .join("\n");
    if !description.is_empty() {
        eve.description(description.as_str());
    }

    eve
}
