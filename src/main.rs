pub mod auth;
pub mod schedule;
pub mod utility;

pub mod prelude {
    pub use super::{auth::prelude::*, schedule::prelude::*, utility::prelude::*};
}

use crate::prelude::*;
use axum::routing::post;
use axum::{Json, http::StatusCode};
use axum::{Router, middleware::from_fn};

use std::time::Duration;
use tokio::net::TcpListener;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // nice logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().without_time())
        .init();
    // axum router
    let app = Router::new()
        .route("/schedule", post(create_schedule))
        .route_layer(from_fn(crate::auth::auth_middleware))
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::new(Duration::from_secs(10)),
        ));
    // read the env variable PORT
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    // build the listener
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    // loop
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

// main endpoint
pub async fn create_schedule(
    Json(payload): Json<ScheduleRequest>,
) -> Result<ScheduleReturn, StatusCode> {
    match payload.try_into() {
        Ok(schedule) => Ok(schedule),
        // TODO: proper error handling passed back up to the request
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}
