// tier2_web_server_actix_postgres/src/a0_library_mod/postgres_type_mod.rs

// This is a subset of postgres data-types that can be used in this Rust project.
// If you need some more, just add it. If you need even more flexibility, then instead of enums,
// you implements traits. But for this simple example I don't need to complicate.
// Postgres have many different names for the same data types and it is confusing.
// I will use everywhere only what they call udt names: int4, varchar, text,... to disambiguate.

/// PostgresUdtType names as string
/// The names are strictly in lowercase, but Rust insist the enum variant are capitalized.
/// I will serialize as snake_case. That will be ok here, because we have always only 1 word
#[derive(strum::AsRefStr, strum::EnumString, Debug)]
#[strum(serialize_all = "snake_case")]
pub enum PostgresUdtType {
    /// 4 bytes
    Int4,
    /// varchar
    Varchar,
    /// names of postgres objects
    Name,
    /// text (max 2GB)
    Text,
    /// bool
    Bool,
}

/// PostgresValue can contain values of different data types.
/// For this simple example this is easier then implementing traits for every type.
/// I want deliberately limit the use to just a few data types for simplicity.
#[derive(Debug, Clone)]
pub enum PostgresValueMultiType {
    String(String),
    I32(i32),
}
