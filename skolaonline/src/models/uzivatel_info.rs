use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct UzivatelInfo {
    pub database_id: String,
    pub organizace_id: String,
    pub osoba_id: String,
    pub uzivatel_id: String,
    pub uziv_jmeno: String,
    pub kategorie_id_csv: String,
    pub jmeno: String,
    pub trida_id: String,
    pub trida_nazev: String,
}
