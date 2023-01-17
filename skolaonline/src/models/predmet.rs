use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Predmet {
    /// null if returned in the `rozvrh` endpoint\
    /// string if returned in the `predmety` endpoint
    pub skolni_rok_id: Option<String>,
    pub predmet_id: String,
    pub zkratka: String,
    pub nazev: String,
    pub priznak_druh_predmetu: Option<String>,
    pub poradi_zobrazeni: Value,
}
