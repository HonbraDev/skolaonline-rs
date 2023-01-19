use chrono::NaiveDate;
use std::collections::HashMap;

use crate::{
    models::{
        hodnoceni::{DruhHodnoceni, VypisHodnoceniStudentResponse},
        predmet::Predmet,
        rozvrh::{RozvrhovaUdalost, RozvrhoveUdalostiResponse},
        uzivatel_info::UzivatelInfo,
    },
    SOClient, SOResult,
};

const DATE_FORMAT: &str = "%Y-%m-%d";

impl SOClient {
    /// Checks if the user's credentials are valid
    pub async fn get_auth_status(&self) -> SOResult<bool> {
        self.get("/AuthorizationStatus").await
    }

    /// Gets the user's profile
    pub async fn get_user_info(&self, username: Option<&str>) -> SOResult<UzivatelInfo> {
        let username = username.unwrap_or(&self.username);
        self.get(&format!("/UzivatelInfo/{username}")).await
    }

    /// Get the user's events in the given date range
    pub async fn get_events(
        &self,
        start: NaiveDate,
        end: Option<NaiveDate>,
    ) -> SOResult<Vec<RozvrhovaUdalost>> {
        let end = end.unwrap_or(start);
        let start = start.format(DATE_FORMAT).to_string();
        let end = end.format(DATE_FORMAT).to_string();

        let udalosti = self
            .get::<RozvrhoveUdalostiResponse>(&format!("/RozvrhoveUdalosti/{start}/{end}"))
            .await?
            .udalosti;

        Ok(udalosti)
    }

    /// Get the user's grades
    pub async fn get_grades(&self) -> SOResult<VypisHodnoceniStudentResponse> {
        self.get("/VypisHodnoceniStudent").await
    }

    /// Get all the subjects in the user's school
    pub async fn get_subjects(&self) -> SOResult<HashMap<String, Predmet>> {
        let subjects = self
            .get::<Vec<Predmet>>("/Predmety")
            .await?
            .into_iter()
            .map(|subject| (subject.predmet_id.clone(), subject))
            .collect();

        Ok(subjects)
    }

    /// Get all the grade types in the user's school
    pub async fn get_grade_types(&self) -> SOResult<HashMap<String, DruhHodnoceni>> {
        let grade_types = self
            .get::<Vec<DruhHodnoceni>>("/DruhyHodnoceni")
            .await?
            .into_iter()
            .map(|grade_type| (grade_type.druh_hodnoceni_id.clone(), grade_type))
            .collect();

        Ok(grade_types)
    }
}
