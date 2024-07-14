use axum::extract::FromRef;

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub(crate) config:Config,
    
}

impl FromRef<AppState> for Config {
    fn from_ref(input: &AppState) -> Self {
        input.config.clone()
    }
}