use anyhow::Result;
use chrono::NaiveDate;

use crate::client::SOClient;
use crate::types::*;

const DATE_FORMAT: &str = "%Y-%m-%d";

impl SOClient {
    /// Checks if the user's credentials are valid
    pub async fn get_auth_status(&self) -> Result<bool> {
        Ok(self.get("/AuthorizationStatus").await?.data)
    }

    /// Gets the user's profile
    pub async fn get_user_info(&self, username: Option<&str>) -> Result<UzivatelInfo> {
        let username = username.unwrap_or(&self.username);
        Ok(self.get(&format!("/UzivatelInfo/{username}")).await?.data)
    }

    /// Get the user's events in the given date range
    pub async fn get_events(
        &self,
        start: NaiveDate,
        end: Option<NaiveDate>,
    ) -> Result<Vec<RozvrhovaUdalost>> {
        let end = end.unwrap_or(start);
        let start = start.format(DATE_FORMAT).to_string();
        let end = end.format(DATE_FORMAT).to_string();

        let resp: RozvrhoveUdalostiResponse = self
            .get(&format!("/RozvrhoveUdalosti/{start}/{end}"))
            .await?
            .data;

        Ok(resp.udalosti)
    }
}
