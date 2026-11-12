//! Look up records in the bundled UnitsDB projection.

use serde_json::Value;
use std::sync::OnceLock;

pub const DATASET_BYTES: &[u8] = include_bytes!("../data/unitsdb.json");
pub const MANIFEST_BYTES: &[u8] = include_bytes!("../data/manifest.json");
const INDEX_BYTES: &[u8] = include_bytes!("../data/lookup-index.json");

static DATASET: OnceLock<Value> = OnceLock::new();
static INDEX: OnceLock<Value> = OnceLock::new();

fn dataset() -> &'static Value {
    DATASET.get_or_init(|| serde_json::from_slice(DATASET_BYTES).expect("bundled UnitsDB dataset"))
}

fn index() -> &'static Value {
    INDEX.get_or_init(|| serde_json::from_slice(INDEX_BYTES).expect("bundled UnitsDB lookup index"))
}

/// A register collection in the original producer order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Collection {
    Units,
    Prefixes,
    Dimensions,
    Quantities,
    Currencies,
    Scales,
}

/// A published lookup key family.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Lookup {
    UnitSymbol,
    UnitId,
    UnitUri,
    PrefixSymbol,
    PrefixId,
    DimensionId,
    QuantityId,
    CurrencyCode,
    CurrencyNumeric,
    ScaleId,
    RecordId,
}

/// All records in a collection, including their original identifiers and metadata.
pub fn records(collection: Collection) -> &'static [Value] {
    let key = match collection {
        Collection::Units => "units",
        Collection::Prefixes => "prefixes",
        Collection::Dimensions => "dimensions",
        Collection::Quantities => "quantities",
        Collection::Currencies => "currencies",
        Collection::Scales => "scales",
    };
    dataset()[key].as_array().expect("bundled collection")
}

/// Find a record by its exact key. Currency alpha codes are case insensitive;
/// numeric currency codes use their canonical three-digit string.
pub fn find(kind: Lookup, key: &str) -> Option<&'static Value> {
    let kind_name = match kind {
        Lookup::UnitSymbol => "unit_symbol",
        Lookup::UnitId => "unit_id",
        Lookup::UnitUri => "unit_uri",
        Lookup::PrefixSymbol => "prefix_symbol",
        Lookup::PrefixId => "prefix_id",
        Lookup::DimensionId => "dimension_id",
        Lookup::QuantityId => "quantity_id",
        Lookup::CurrencyCode => "currency_code",
        Lookup::CurrencyNumeric => "currency_numeric",
        Lookup::ScaleId => "scale_id",
        Lookup::RecordId => "record_id",
    };
    let upper;
    let key = if kind == Lookup::CurrencyCode {
        upper = key.to_ascii_uppercase();
        upper.as_str()
    } else {
        key
    };
    let location = index().get(kind_name)?.get(key)?;
    let table = location.get(0)?.as_str()?;
    let offset = usize::try_from(location.get(1)?.as_u64()?).ok()?;
    dataset().get(table)?.get(offset)
}

/// Source release and generation metadata from the bundled dataset.
pub fn provenance() -> &'static Value {
    &dataset()["provenance"]
}
