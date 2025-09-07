use serde::{Deserialize, Serialize};

/// Languages that cards are printed in and translated to.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum Language {
    #[serde(rename = "English")]
    English,

    #[serde(rename = "Ancient Greek")]
    AncientGreek,

    #[serde(rename = "Arabic")]
    Arabic,

    #[serde(rename = "Chinese Simplified")]
    ChineseSimplified,

    #[serde(rename = "Chinese Traditional")]
    ChineseTraditional,

    #[serde(rename = "French")]
    French,

    #[serde(rename = "German")]
    German,

    #[serde(rename = "Hebrew")]
    Hebrew,

    #[serde(rename = "Italian")]
    Italian,

    #[serde(rename = "Japanese")]
    Japanese,

    #[serde(rename = "Korean")]
    Korean,

    #[serde(rename = "Latin")]
    Latin,

    #[serde(rename = "Phyrexian")]
    Phyrexian,

    #[serde(rename = "Portuguese (Brazil)")]
    Portuguese,

    #[serde(rename = "Quenya")]
    Quenya,

    #[serde(rename = "Russian")]
    Russian,

    #[serde(rename = "Sanskrit")]
    Sanskrit,

    #[serde(rename = "Spanish")]
    Spanish,
}
