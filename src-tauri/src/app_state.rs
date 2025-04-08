#[derive(Default, Clone)]
pub struct AppState {
    pub client: reqwest::Client,
    pub universal_state: helium_types::UniversalState,
}
