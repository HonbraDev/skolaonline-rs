use std::hash::{Hash, Hasher};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{hash::hash_serde_json_value, models::predmet::Predmet};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RozvrhoveUdalostiResponse {
    pub udalosti: Vec<RozvrhovaUdalost>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RozvrhovaUdalost {
    pub udalost_id: String,
    pub datum: NaiveDateTime,
    pub poradi: i32,
    pub obdobi_dne_od_id: String,
    pub obdobi_dne_do_id: String,
    pub obdobi_dne_od_nazev: String,
    pub obdobi_dne_do_nazev: String,
    pub cas_od: NaiveDateTime,
    pub cas_do: NaiveDateTime,
    pub delka_pocet_hodin: i32,
    pub nazev: String,
    pub popis: Option<String>,
    pub typ_udalosti: TypUdalosti,
    pub druh_udalosti: Option<DruhUdalosti>,
    pub cyklus: Option<String>,
    pub predmet: Option<Predmet>,
    pub barva: String,
    pub barva_pisma: String,
    pub povolen_zapis_dochazky: bool,
    pub povolen_zapis_hodnoceni: bool,
    pub skupiny_udalosti: Vec<SkupinaUdalosti>,
    pub tridy_udalosti: Value, // idk
    pub mistnosti_udalosti: Vec<MistnostUdalosti>,
    pub ucitele_udalosti: Vec<UcitelUdalosti>,
    pub poznamka: Option<String>,
    pub probrane_ucivo: Option<String>,
    pub nahrazuje_hodiny: bool,
    pub je_suplovana_hodinami: bool,
    pub nahrazuje_hodiny_text: Option<String>,
    pub je_suplovana_hodinami_text: Option<String>,
    pub pocet_oducenych_hodin: Option<i32>,
}

impl Hash for RozvrhovaUdalost {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.udalost_id.hash(state);
        self.datum.hash(state);
        self.poradi.hash(state);
        self.obdobi_dne_od_id.hash(state);
        self.obdobi_dne_do_id.hash(state);
        self.obdobi_dne_od_nazev.hash(state);
        self.obdobi_dne_do_nazev.hash(state);
        self.cas_od.hash(state);
        self.cas_do.hash(state);
        self.delka_pocet_hodin.hash(state);
        self.nazev.hash(state);
        self.popis.hash(state);
        self.typ_udalosti.hash(state);
        self.druh_udalosti.hash(state);
        self.cyklus.hash(state);
        self.predmet.hash(state);
        self.barva.hash(state);
        self.barva_pisma.hash(state);
        self.povolen_zapis_dochazky.hash(state);
        self.povolen_zapis_hodnoceni.hash(state);
        self.skupiny_udalosti.hash(state);
        hash_serde_json_value(&self.tridy_udalosti, state);
        self.mistnosti_udalosti.hash(state);
        self.ucitele_udalosti.hash(state);
        self.poznamka.hash(state);
        self.probrane_ucivo.hash(state);
        self.nahrazuje_hodiny.hash(state);
        self.je_suplovana_hodinami.hash(state);
        self.nahrazuje_hodiny_text.hash(state);
        self.je_suplovana_hodinami_text.hash(state);
        self.pocet_oducenych_hodin.hash(state);
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TypUdalosti {
    pub typ_udalosti_id: String,
    pub popis: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct DruhUdalosti {
    pub typ_udalosti_id: String,
    pub druh_udalosti_id: String,
    pub nazev: String,
    pub popis: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct SkupinaUdalosti {
    pub skupina_id: String,
    pub skupina_nazev: String,
    pub priznak_druh_skupiny: String,
    pub trida_id: String,
    pub trida_nazev: String,
    pub priznak_absence: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct MistnostUdalosti {
    pub mistnost_id: String,
    pub nazev: String,
    pub popis: String,
    pub priznak_absence: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct UcitelUdalosti {
    pub ucitel_id: String,
    pub prijmeni: String,
    pub jmeno: String,
    pub zkratka: String,
    pub priznak_absence: bool,
}
