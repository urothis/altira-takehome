use serde::{Deserialize, Serialize};

pub mod prelude {
    pub use super::{ScheduleRequest, ToScheduleEntry};
}

// request data
#[derive(Serialize, Deserialize, Clone)]
pub struct ScheduleRequest(Vec<ToScheduleEntry>);

// convert our request into an iterator
impl IntoIterator for ScheduleRequest {
    type Item = ToScheduleEntry;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// To Schedule Entry
#[derive(Serialize, Deserialize, Clone)]
pub struct ToScheduleEntry {
    pub name: String,
    pub phone: String,
    // ISO 8601 string
    pub scheduled: Option<String>,
}
