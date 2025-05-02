use crate::model_util::UniversalState;

#[derive(Default, Clone)]
pub struct AppState {
    pub client: reqwest::Client,
    pub universal_state: UniversalState,
}
