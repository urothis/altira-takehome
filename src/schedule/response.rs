use crate::prelude::*;
use axum::{Json, response::IntoResponse};
use serde::{Deserialize, Serialize};

pub mod prelude {
    pub use super::{ScheduleReturn, ScheduledEntry};
}

// Return data
#[derive(Serialize, Deserialize, Clone)]
pub struct ScheduleReturn(Vec<ScheduledEntry>);

// into response
impl IntoResponse for ScheduleReturn {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

// Returned Scheduled Entry
#[derive(Serialize, Deserialize, Clone)]
pub struct ScheduledEntry {
    pub name: String,
    pub phone: String,
    pub timezone: String,
    // ISO 8601 string
    pub scheduled: String,
}

// we only need to go from the ToScheduleEntry to a ScheduledEntry
impl TryFrom<ToScheduleEntry> for ScheduledEntry {
    type Error = ();
    fn try_from(entry: ToScheduleEntry) -> Result<Self, Self::Error> {
        // crudely parse our phone number down
        let phone = Phone::new(entry.phone);
        // Tz
        let timezone = match timezone_from_areacode(phone.area_code) {
            Some(timezone) => timezone,
            None => chrono_tz::UTC,
        };
        // response
        Ok(ScheduledEntry {
            name: entry.name,
            phone: phone.number,
            timezone: timezone.name().to_string(),
            scheduled: match entry.scheduled {
                Some(time) => time,
                None => scheduler(&timezone),
            },
        })
    }
}

// collection conversion
impl TryFrom<ScheduleRequest> for ScheduleReturn {
    type Error = ();
    fn try_from(entries: ScheduleRequest) -> Result<Self, Self::Error> {
        Ok(ScheduleReturn(
            entries
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}
