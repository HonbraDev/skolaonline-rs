use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VypisHodnoceniStudentResponse {
    pub obdobi: VypisHodnoceniStudentResponseObdobi,
    pub hodnoceni: Vec<Hodnoceni>,
}

/// Naming things is hard
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct VypisHodnoceniStudentResponseObdobi {
    pub datum_od: String, // NaiveDateTime,
    pub datum_do: String, // NaiveDateTime,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Hodnoceni {
    pub udalost_id: String,
    pub student_id: String,
    pub skolni_rok_id: String,
    pub skolni_rok_nazev: String,
    pub pololeti_id: String,
    pub pololeti_nazev: String,
    pub datum: String,
    pub obdobi_dne_id: String,
    pub obdobi_dne_nazev: String,
    pub druh_hodnoceni_id: String,
    pub nazev: String,
    pub popis: Option<String>,
    pub predmet_id: String,
    pub ucitel_id: String,
    pub druh_vysledku: String,
    pub vysledek: Option<String>,
    pub vysledek_text: Option<String>,
    pub hodnoceni_max_bodu: Option<f64>,
    pub procenta: Option<f64>,
    pub znamka: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct DruhHodnoceni {
    pub druh_hodnoceni_id: String,
    pub nazev: String,
    pub popis: Option<String>,
    pub vaha: f64,
    pub poradi_zobrazeni: i64,
}
