use serde::{Deserialize, Serialize};

/// Describes a list of legalities in play formats for a Card.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[allow(missing_docs)]
pub struct Legalities {
    pub alchemy: Option<String>,
    pub brawl: Option<String>,
    pub commander: Option<String>,
    pub duel: Option<String>,
    pub explorer: Option<String>,
    pub future: Option<String>,
    // TODO: remove if not needed
    // pub frontier: Option<String>,
    pub gladiator: Option<String>,
    pub historic: Option<String>,
    pub historic_brawl: Option<String>,
    pub legacy: Option<String>,
    pub modern: Option<String>,
    pub oathbreaker: Option<String>,
    pub old_school: Option<String>,
    pub pauper: Option<String>,
    pub pauper_commander: Option<String>,
    pub penny: Option<String>,
    pub pioneer: Option<String>,
    pub predh: Option<String>,
    pub premodern: Option<String>,
    pub standard: Option<String>,
    pub standardbrawl: Option<String>,
    pub timeless: Option<String>,
    pub vintage: Option<String>,
}
