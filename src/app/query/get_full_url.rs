pub trait GetFullUrlRepository {
    fn save(&self, id: &str) -> Result<String, String>;
}
pub struct GetFullUrl<R>
where
    R: GetFullUrlRepository,
{
    repo: R,
}

impl<R> GetFullUrl<R>
where
    R: GetFullUrlRepository,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
    pub fn exucute(&self, id: &str) -> Result<String, String> {
        self.repo.save(id)
    }
}

#[cfg(test)]

mod test {
    use std::sync::Arc;

    use dashmap::DashMap;

    use crate::app::adapters::inmemory::repository::InMemoryRepository;

    use super::*;

    #[tokio::test]
    async fn test_get_full_url() {
        struct FakeRepository;

        impl GetFullUrlRepository for FakeRepository {
            fn save(&self, id: &str) -> Result<String, String> {
                Ok("https://www.google.com".to_owned())
            }
        }

        let repo = FakeRepository;

        let query = GetFullUrl::new(repo);

        let res = query.exucute("123");

        assert_eq!(res, Ok("https://www.google.com".to_owned()));
    }

    #[tokio::test]
    async fn get_from_inmemory_repo() {
        let store = Arc::new(DashMap::new());
        store.insert("123".to_owned(), "https://www.google.com".to_owned());

        let repo = InMemoryRepository::new(store);
        let query = GetFullUrl::new(repo);

        let res = query.exucute("123");
        assert_eq!(res, Ok("https://www.google.com".to_owned()));
    }

    #[tokio::test]
    async fn get_two_different_full_url() {
        let store = Arc::new(DashMap::new());
        store.insert("123".to_owned(), "https://www.google.com".to_owned());
        store.insert("456".to_owned(), "https://www.google.com".to_owned());
        let repo = InMemoryRepository::new(store);
        let query = GetFullUrl::new(repo);

        let res = query.exucute("123");
        let res2 = query.exucute("456");

        assert_eq!(res, Ok("https://www.google.com".to_owned()));
        assert_eq!(res2, Ok("https://www.google.com".to_owned()));
    }
}
