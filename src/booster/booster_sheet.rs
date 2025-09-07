use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Sheets of cards to use within booster packs.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoosterSheet {
    /// Colors of the sheet need to be balanced.
    pub balance_colors: Option<bool>,

    /// Cards used on a sheet.
    pub cards: HashMap<Uuid, u64>,

    /// Whether the sheet is foiled.
    pub foil: bool,

    /// Sum of all card weights.
    pub total_weight: u64,
}
