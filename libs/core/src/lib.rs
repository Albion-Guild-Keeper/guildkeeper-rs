// libs/core_lib/src/lib.rs
pub mod errors;
pub mod config_models;
pub mod models;
pub mod persistence;
pub mod utils;

// Riesporta il tipo Result per comodità nell'intero crate core_lib
pub use errors::Result;
// Puoi riesportare anche CoreError se lo usi spesso direttamente
pub use errors::CoreError;