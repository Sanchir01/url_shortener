use crate::{
    app::{command::create_url::CreateShortUrl, query::get_full_url::GetFullUrl},
    idProvider::IdProvider,
};

use super::{
    command::create_url::CreateShortUrlReporitory, query::get_full_url::GetFullUrlRepository,
};

pub struct Container<I, R, Q>
where
    I: IdProvider,
    R: CreateShortUrlReporitory,
    Q: GetFullUrlRepository,
{
    pub short_command: CreateShortUrl<I, R>,
    pub get_query: GetFullUrl<Q>,
}

impl<I, R, Q> Container<I, R, Q>
where
    I: IdProvider,
    R: CreateShortUrlReporitory,
    Q: GetFullUrlRepository,
{
    pub fn new(id_provider: I, repo: R, query_repo: Q) -> Self {
        let short_command = CreateShortUrl::new(id_provider, repo);
        let get_query = GetFullUrl::new(query_repo);

        Container {
            short_command,
            get_query,
        }
    }
}
