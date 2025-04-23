use crate::idProvider::{self, IdProvider};

pub struct CreateShortUrl<I>
where
    I: IdProvider,
{
    id_provider: I,
}

impl<I> CreateShortUrl<I>
where
    I: IdProvider,
{
    pub fn new(id_provider: I) -> Self {
        Self { id_provider }
    }
    pub async fn exucute(&self, full_url: String) -> Result<String, String> {
        let id = self.id_provider.provide();
        Ok(id)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_create_short_url() {
        let id_provider = crate::idProvider::FakeIdProvider::new("123".to_owned());
        let command = CreateShortUrl::new(id_provider);
        let result = command.exucute("https://www.google.com".to_owned()).await;
        assert_eq!(result, Ok("1".to_owned()));
    }
    #[tokio::test]
    async fn test_create_url() {
        let id_provider = crate::idProvider::FakeIdProvider::new("123".to_owned());
        let command = CreateShortUrl::new(id_provider);
        let result1 = command.exucute("https://www.google.com".to_owned()).await;
        let result2 = command.exucute("https://www.google.com".to_owned()).await;
        assert_ne!(result1, result2);
    }
}
