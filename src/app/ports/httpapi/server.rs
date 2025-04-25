use axum::{
    Json, Router,
    extract::{MatchedPath, Path, Request, State},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    app::{
        command::create_url::CreateShortUrlReporitory, di::Container,
        query::get_full_url::GetFullUrlRepository,
    },
    idProvider::IdProvider,
};

pub struct Server<I, R, Q>
where
    I: IdProvider + Send + Sync + 'static,
    R: CreateShortUrlReporitory + Send + Sync + 'static,
    Q: GetFullUrlRepository + Send + Sync + 'static,
{
    port: u16,
    container: Arc<Container<I, R, Q>>,
}

impl<I, R, Q> Server<I, R, Q>
where
    I: IdProvider + Send + Sync + 'static,
    R: CreateShortUrlReporitory + Send + Sync + 'static,
    Q: GetFullUrlRepository + Send + Sync + 'static,
{
    pub fn new(port: u16, container: Arc<Container<I, R, Q>>) -> Self {
        Server { port, container }
    }
    pub async fn run(self) {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "urlshortner=debug,tower_http=debug".into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        let router = get_router(self.container);
        axum::serve(listener, router).await.unwrap();
    }
}

#[derive(Deserialize, Serialize)]
struct CreateShortURLRequest {
    url: String,
}

#[derive(Deserialize, Serialize)]
struct ShortUrlResponse {
    id: String,
}
fn get_router<I, R, Q>(container: Arc<Container<I, R, Q>>) -> Router
where
    I: IdProvider + Send + Sync + 'static,
    R: CreateShortUrlReporitory + Send + Sync + 'static,
    Q: GetFullUrlRepository + Send + Sync + 'static,
{
    Router::new()
        .route("/:id", get(get_full_url))
        .route("/", post(shorten_url))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|req: &Request| {
                    let method = req.method();
                    let uri = req.uri();
                    let matched_path = req
                        .extensions()
                        .get::<MatchedPath>()
                        .map(|matched_path| matched_path.as_str());

                    tracing::debug_span!("request", %method, %uri, matched_path)
                })
                .on_failure(()),
        )
        .with_state(container)
}
async fn get_full_url<I, R, Q>(
    Path(id): Path<String>,
    State(container): State<Arc<Container<I, R, Q>>>,
) -> Result<Json<FullUrlResponse>, String>
where
    I: IdProvider + Send + Sync + 'static,
    R: CreateShortUrlReporitory + Send + Sync + 'static,
    Q: GetFullUrlRepository + Send + Sync + 'static,
{
    container
        .get_query
        .exucute(&id)
        .await
        .map(|url| Json(FullUrlResponse { url }))
}

#[derive(Deserialize, Serialize, Debug)]
struct FullUrlResponse {
    url: String,
}

async fn shorten_url() {
    println!("hello world")
}
