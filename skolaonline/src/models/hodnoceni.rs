use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use crate::hash::{hash_f64, hash_opt_f64};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "PascalCase")]
pub struct VypisHodnoceniStudentResponse {
    pub obdobi: VypisHodnoceniStudentResponseObdobi,
    pub hodnoceni: Vec<Hodnoceni>,
}

/// Naming things is hard
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
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

impl Hash for Hodnoceni {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.udalost_id.hash(state);
        self.student_id.hash(state);
        self.skolni_rok_id.hash(state);
        self.skolni_rok_nazev.hash(state);
        self.pololeti_id.hash(state);
        self.pololeti_nazev.hash(state);
        self.datum.hash(state);
        self.obdobi_dne_id.hash(state);
        self.obdobi_dne_nazev.hash(state);
        self.druh_hodnoceni_id.hash(state);
        self.nazev.hash(state);
        self.popis.hash(state);
        self.predmet_id.hash(state);
        self.ucitel_id.hash(state);
        self.druh_vysledku.hash(state);
        self.vysledek.hash(state);
        self.vysledek_text.hash(state);
        hash_opt_f64(&self.hodnoceni_max_bodu, state);
        hash_opt_f64(&self.procenta, state);
        hash_opt_f64(&self.znamka, state);
    }
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

impl Hash for DruhHodnoceni {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.druh_hodnoceni_id.hash(state);
        self.nazev.hash(state);
        self.popis.hash(state);
        hash_f64(&self.vaha, state);
        self.poradi_zobrazeni.hash(state);
    }
}
