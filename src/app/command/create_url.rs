use crate::idProvider::IdProvider;
pub trait CreateShortUrlReporitory {
    fn save(&self, full_url: String, id: String) -> Result<(), String>;
}

pub struct CreateShortUrl<I, R>
where
    I: IdProvider,
    R: CreateShortUrlReporitory,
{
    id_provider: I,
    repo: R,
}

impl<I, R> CreateShortUrl<I, R>
where
    I: IdProvider,
    R: CreateShortUrlReporitory,
{
    pub fn new(id_provider: I, repo: R) -> Self {
        Self { id_provider, repo }
    }
    pub async fn exucute(&self, full_url: String) -> Result<String, String> {
        let id = self.id_provider.provide();
        self.repo.save(full_url, id.clone())?;
        Ok(id)
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use dashmap::DashMap;

    use crate::app::adapters::inmemory::repository::InMemoryRepository;

    use super::*;

    #[tokio::test]
    async fn test_create_short_url() {
        let id_provider = crate::idProvider::FakeIdProvider::new("123".to_owned());
        let repo = Arc::new(DashMap::new());

        let store = InMemoryRepository::new(repo);
        let command = CreateShortUrl::new(id_provider, store);

        let result = command.exucute("https://www.google.com".to_owned()).await;
        assert_ne!(result, Ok("".to_owned()));
    }

    #[tokio::test]
    async fn test_create_url() {
        let nano_id_provider = crate::idProvider::NanoIdProvider;
        let repo = Arc::new(DashMap::new());

        let store = InMemoryRepository::new(repo);
        let command = CreateShortUrl::new(nano_id_provider, store);
        let result1 = command.exucute("https://www.google.com".to_owned()).await;
        let result2 = command.exucute("https://www.google.com".to_owned()).await;
        assert_ne!(result1, result2);
    }
    #[tokio::test]
    async fn after_save_store_short() {
        let nano_id_provider = crate::idProvider::NanoIdProvider;
        let store = Arc::new(DashMap::new());
        let repo = InMemoryRepository::new(store.clone());
        let command = CreateShortUrl::new(nano_id_provider, repo);
        let id = command
            .exucute("https://www.google.com".to_owned())
            .await
            .unwrap();
        assert_eq!(store.len(), 1);

        let full_url = store.get(&id).unwrap();
        assert_eq!(*full_url.value(), "https://www.google.com".to_owned());
    }
}
