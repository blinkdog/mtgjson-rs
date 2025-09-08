use std::collections::HashMap;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{
    booster::Booster,
    card::{SetCard, TokenCard},
    deck_set::DeckSet,
    language::Language,
    sealed_product::SealedProduct,
};

/// Describes the properties and values of an individual Set.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Set {
    /// The number of cards in the set. This will default
    /// to `total_set_size` if not available. Wizards of the Coast
    /// sometimes prints extra cards beyond the set size into
    /// promos or supplemental products.
    pub base_set_size: usize,

    /// The block name the set was in.
    pub block: Option<String>,

    /// A breakdown of possibilities and weights of cards in a booster pack.
    pub booster: Option<Booster>,

    /// The list of cards in the set.
    pub cards: Vec<SetCard>,

    /// The Cardsphere set identifier.
    pub cardsphere_set_id: Option<u32>,

    /// The set code for the set.
    pub code: String,

    /// The alternate set code Wizards of the Coast uses for a select few duel deck sets.
    pub code_v3: Option<String>,

    /// All decks associated to the set.
    pub decks: Option<Vec<DeckSet>>,

    /// If the set is available only outside the United States of America.
    pub is_foreign_only: Option<bool>,

    /// If the set is only available in foil.
    pub is_foil_only: bool,

    /// If the set is only available in non-foil.
    pub is_non_foil_only: Option<bool>,

    /// If the set is only available in online game variations.
    pub is_online_only: bool,

    /// If the set is available only in paper.
    pub is_paper_only: Option<bool>,

    /// If the set is still in preview (spoiled). Preview sets do not have complete data.
    pub is_partial_preview: Option<bool>,

    /// The matching Keyrune code for set image icons.
    pub keyrune_code: String,

    /// The languages the set was printed in.
    pub languages: Option<Vec<Language>>,

    /// The Magic Card Market set identifier.
    pub mcm_id: Option<u32>,

    /// The split Magic Card Market set identifier if a set is printed in two sets. This identifier represents the second set's identifier.
    pub mcm_id_extras: Option<u32>,

    /// The Magic Card Market set name.
    pub mcm_name: Option<String>,

    /// The set code for the set as it appears on Magic: The Gathering Online.
    pub mtgo_code: Option<String>,

    /// The name of the set.
    pub name: String,

    /// The parent set code for set variations like promotions, guild kits, etc.
    pub parent_code: Option<String>,

    /// The release date in ISO 8601 format for the set.
    pub release_date: NaiveDate,

    /// The sealed product information for the set.
    pub sealed_product: Option<Vec<SealedProduct>>,

    /// The group identifier of the set on TCGplayer.
    pub tcgplayer_group_id: Option<u32>,

    /// The tokens available to the set.
    pub tokens: Vec<TokenCard>,

    /// The total number of cards in the set, including promotional
    /// and related supplemental products but excluding Alchemy
    /// modifications - however those cards are included in the set itself.
    pub total_set_size: usize,

    /// The translated set name by language.
    pub translations: HashMap<Language, Option<String>>,

    /// The expansion type of the set.
    #[serde(rename = "type")]
    pub set_type: String,
}
