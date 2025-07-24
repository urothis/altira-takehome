use log::info;
use tokio::signal;

pub mod prelude {
    pub use super::{shutdown_signal, timezone_from_areacode};
}

// just copied data from https://greatdata.com/info/currenttime, area codes are updated yearly.
// So this function would need to be updated often if we couldn't find a good source to pull from.
pub fn timezone_from_areacode(area_code: u16) -> Option<chrono_tz::Tz> {
    info!("Area code: {area_code}");
    match area_code {
        201 => Some(chrono_tz::US::Eastern),
        202 => Some(chrono_tz::US::Eastern),
        203 => Some(chrono_tz::US::Eastern),
        205 => Some(chrono_tz::US::Central),
        206 => Some(chrono_tz::US::Pacific),
        207 => Some(chrono_tz::US::Eastern),
        208 => Some(chrono_tz::US::Mountain),
        209 => Some(chrono_tz::US::Pacific),
        210 => Some(chrono_tz::US::Central),
        // TODO the rest...
        989 => Some(chrono_tz::US::Eastern),
        _ => None,
    }
}

pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
