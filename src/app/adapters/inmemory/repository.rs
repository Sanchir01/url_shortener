use std::sync::Arc;

use dashmap::DashMap;

#[derive(Clone)]
pub struct InMemoryRepository {
    store: Arc<DashMap<String, String>>,
}

impl InMemoryRepository {
    pub fn new(store: Arc<DashMap<String, String>>) -> Self {
        Self { store }
    }
}

impl crate::app::command::create_url::CreateShortUrlReporitory for InMemoryRepository {
    fn save(&self, full_url: String, id: String) -> Result<(), String> {
        self.store.insert(id, full_url);
        Ok(())
    }
}

impl crate::app::query::get_full_url::GetFullUrlRepository for InMemoryRepository {
    fn save(&self, id: &str) -> Result<String, String> {
        match self.store.get(id) {
            Some(value) => Ok(value.clone()),
            None => Err("not found".to_owned()),
        }
    }
}
