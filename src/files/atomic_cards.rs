use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{card::AtomicCard, meta::Meta};

/// Every unique card, grouped by name.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtomicCards {
    /// Metadata about the data set.
    pub meta: Meta,

    /// Cards grouped by name.
    pub data: HashMap<String, Vec<AtomicCard>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;

    #[cfg_attr(not(feature = "local_tests"), ignore)]
    #[test]
    fn test_atomic_cards_local() {
        let file = File::open("testdata/AtomicCards.json").unwrap();
        let reader = BufReader::new(file);
        let _: AtomicCards = serde_json::from_reader(reader).unwrap();
    }

    #[cfg_attr(not(feature = "network_tests"), ignore)]
    #[test]
    fn test_atomic_cards_network() {
        let _: AtomicCards = reqwest::blocking::get("https://mtgjson.com/api/v5/AtomicCards.json")
            .unwrap()
            .json()
            .unwrap();
    }
}
