use chrono::{NaiveDate, TimeZone, Utc};
use chrono_tz::Tz;
pub use icalendar;
use icalendar::{Calendar, CalendarDateTime, Component, Event, EventLike};
use skolaonline::{models::rozvrh::RozvrhovaUdalost, SOClient};
pub use skolaonline::{SOError, SOResult};
use thiserror::Error;

const TZ: Tz = chrono_tz::Europe::Prague;

pub async fn fetch_calendar(
    username: &str,
    password: &str,
    start: NaiveDate,
    end: Option<NaiveDate>,
    config: &CalendarConverterConfig,
) -> Result<Calendar, FetchCalendarError> {
    let client = SOClient::new(username, password);

    // See #1
    // if !client.get_auth_status().await? {
    //     return Err(FetchCalendarError::Unauthorized);
    // }

    let events = client.get_events(start, end).await?;

    let calendar = convert_to_ical(&events, config);

    Ok(calendar)
}

#[derive(Debug, Clone)]
pub struct CalendarConverterConfig {
    pub teachers_as_attendees: bool,
    pub groups_as_attendees: bool,
    pub universal_description: bool,
}

impl Default for CalendarConverterConfig {
    fn default() -> Self {
        Self {
            teachers_as_attendees: false,
            groups_as_attendees: true,
            universal_description: true,
        }
    }
}

#[derive(Debug, Error)]
pub enum FetchCalendarError {
    #[error("Unauthorized")]
    Unauthorized,

    #[error("Failed to fetch events")]
    SOError(#[from] SOError),
}

pub fn convert_to_ical(
    udalosti: &Vec<RozvrhovaUdalost>,
    config: &CalendarConverterConfig,
) -> Calendar {
    let mut cal = Calendar::new();

    for udalost in udalosti {
        cal.push(convert_event_to_ical(udalost, config));
    }

    cal
}

pub fn convert_event_to_ical(
    udalost: &RozvrhovaUdalost,
    config: &CalendarConverterConfig,
) -> Event {
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
            .as_ref()
            .map(|predmet| &predmet.nazev)
            .unwrap_or(&udalost.nazev),
    );

    // Location
    let location_str = {
        udalost
            .mistnosti_udalosti
            .iter()
            .map(|m| m.nazev.clone())
            .collect::<Vec<_>>()
            .join(", ")
    };
    if !location_str.is_empty() {
        eve.location(&location_str);
    }

    // Teachers and groups as native attendees (this requires the user to download
    // contacts separetely)
    if config.teachers_as_attendees {
        for ucitel in &udalost.ucitele_udalosti {
            eve.add_multi_property(
                "ATTENDEE",
                &format!(
                    "MAILTO:{}-{}@teachers.ical.skolaonline.cz",
                    ucitel.zkratka, ucitel.ucitel_id
                ),
            );
        }
    }
    if config.groups_as_attendees {
        for skupina in &udalost.skupiny_udalosti {
            eve.add_multi_property(
                "ATTENDEE",
                &format!("MAILTO:{}@groups.ical.skolaonline.cz", skupina.skupina_id),
            );
        }
    }

    // Teachers
    let teachers_str = config.universal_description.then(|| {
        udalost
            .ucitele_udalosti
            .iter()
            .map(|u| format!("{} {}", u.jmeno, u.prijmeni))
            .collect::<Vec<_>>()
            .join(", ")
    });

    // Groups
    let groups_str = config.universal_description.then(|| {
        udalost
            .skupiny_udalosti
            .iter()
            .map(|s| {
                if s.skupina_nazev == s.trida_nazev {
                    s.skupina_nazev.clone()
                } else {
                    format!("{} ({})", s.skupina_nazev, s.trida_nazev)
                }
            })
            .collect::<Vec<_>>()
            .join(", ")
    });

    // Description; The `poznamka` prop seems to be unused, but I'll still include
    // it
    let description = [
        config.universal_description.then_some(location_str),
        teachers_str,
        groups_str,
        udalost.popis.clone(),
        udalost.poznamka.clone(),
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
