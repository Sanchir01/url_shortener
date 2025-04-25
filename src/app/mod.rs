pub mod adapters;
pub mod command;
pub mod di;
pub mod ports;
pub mod query;
#[cfg(test)]
mod test {
    use std::sync::Arc;

    use crate::app::adapters::inmemory::repository::InMemoryRepository;
    use dashmap::DashMap;

    #[tokio::test]
    async fn create_and_get_short_url() {
        let store = Arc::new(DashMap::new());
        let repo = InMemoryRepository::new(store.clone());
        let create_command = crate::app::command::create_url::CreateShortUrl::new(
            crate::idProvider::NanoIdProvider,
            repo.clone(),
        );

        let get_query = crate::app::query::get_full_url::GetFullUrl::new(repo);

        let res = create_command
            .exucute("https://www.google.com".to_owned())
            .await;
        let res2 = get_query.exucute(&res.unwrap()).await.unwrap();

        assert_eq!(res2, "https://www.google.com".to_owned());
    }
}
