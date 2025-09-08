#![deny(missing_docs)]

//! The [MTGJSON](https://mtgjson.com) API provides a repository of card information for Magic.
//! This library provides types for models from the API, for use in apps that need card information.

mod booster;
mod card;
mod deck_set;
mod files;
mod identifiers;
mod language;
mod meta;
mod purchase_urls;
mod sealed_product;
mod set;

pub use booster::*;
pub use card::*;
pub use deck_set::*;
pub use files::*;
pub use identifiers::*;
pub use language::*;
pub use meta::*;
pub use purchase_urls::*;
pub use sealed_product::*;
pub use set::*;
