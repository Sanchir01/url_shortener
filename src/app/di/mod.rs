use crate::{app::command::create_url::CreateShortUrlCommand, idProvider::IdProvider};

use super::{
    command::create_url::CreateShortUrlReporitory, query::get_full_url::GetFullUrlRepository,
};

pub struct Container<I, R, Q>
where
    I: IdProvider,
    R: CreateShortUrlReporitory,
    Q: GetFullUrlRepository,
{
    pub short_command: CreateShortUrlCommand<I, R>,
    pub get_query: GetFullUrlQuery<Q>,
}
