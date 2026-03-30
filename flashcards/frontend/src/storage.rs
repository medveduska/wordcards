use gloo_storage::{LocalStorage, Storage};

use crate::model::{Dataset, PersistedState};

const STORAGE_KEY: &str = "flashcards_app_state";
const DATASETS_KEY: &str = "flashcards_datasets_list";

pub fn load_persisted_state() -> Option<PersistedState> {
    LocalStorage::get(STORAGE_KEY).ok()
}

pub fn save_persisted_state(state: &PersistedState) {
    let _ = LocalStorage::set(STORAGE_KEY, state);
}

pub fn load_datasets() -> Vec<Dataset> {
    LocalStorage::get(DATASETS_KEY).ok().unwrap_or_default()
}

pub fn save_datasets(datasets: &[Dataset]) {
    let _ = LocalStorage::set(DATASETS_KEY, datasets);
}
