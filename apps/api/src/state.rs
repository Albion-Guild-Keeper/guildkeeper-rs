// IN: apps/rest_api/src/state.rs
use std::sync::Arc;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;

use crate::config::Settings;

#[derive(Clone)] 
pub struct AppState {
    pub db: Surreal<Any>,
    pub settings: Arc<Settings>,

}
