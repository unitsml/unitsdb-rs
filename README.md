# UnitsDB for Rust

The package embeds the source-pinned UnitsDB dataset and the lookup results
from its TypeScript producer. `data/manifest.json` records the producer
revision, SHA-256 seals, collection sizes, and lookup inventory.

Use `unitsdb_rs::records(Collection::Units)` to enumerate records, and
`unitsdb_rs::find(Lookup::UnitSymbol, "m")` to look up a unit. Records keep
conversion factors and other decimal strings as supplied by the source.
Unknown keys return `None`. Currency alpha codes ignore ASCII case, and
numeric codes use their three-digit strings.
