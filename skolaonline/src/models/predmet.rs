use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::hash::hash_serde_json_value;

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

impl Hash for Predmet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.skolni_rok_id.hash(state);
        self.predmet_id.hash(state);
        self.zkratka.hash(state);
        self.nazev.hash(state);
        self.priznak_druh_predmetu.hash(state);
        hash_serde_json_value(&self.poradi_zobrazeni, state);
    }
}
