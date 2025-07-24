use chrono::{DateTime, Days, Duration, TimeZone, Timelike, Utc};
use chrono_tz::Tz;

pub mod prelude {
    pub use super::{Phone, scheduler};
}

#[derive(Debug)]
pub struct Phone {
    pub country_code: u16,
    pub area_code: u16,
    pub number: String,
}

impl Phone {
    pub fn new(number: String) -> Self {
        let mut area_code = number.clone();
        // remove all non-digit characters
        area_code.retain(|c| c.is_ascii_digit());
        // extract country code for future use
        let country_code: u16 = if number.len() == 8 {
            match area_code.chars().next() {
                Some(c) => match c.to_digit(10) {
                    Some(d) => d as u16,
                    None => 0,
                },
                None => 0,
            }
        } else {
            0
        };
        // trim the country code
        if area_code.len() == 11 {
            area_code = area_code[1..].to_string();
        }
        // extract the first three digits as the area code
        // and parse it as a u16
        let area_code = area_code[..3]
            .to_string()
            .parse::<u16>()
            .unwrap_or_default();
        Phone {
            country_code,
            area_code,
            number,
        }
    }
}

// TODO all these hardcoded values should be configurable
pub fn random_time_between_9_5(date: DateTime<Tz>) -> DateTime<Tz> {
    // our current hour
    let mut hour = date.hour();

    // do this tweak before we do the rest
    if hour > 17 {
        // scoot us to the next day
        date.checked_add_days(Days::new(1)).unwrap();
        // reset hour to 9am
        hour = 9;
    }

    // handle before 9am
    let start = if hour < 9 {
        date.with_hour(9).unwrap().with_minute(0).unwrap()
    }
    // handle after 5pm
    else if hour > 17 {
        date.with_hour(17).unwrap().with_minute(0).unwrap()
    }
    // handle between 9am and 5pm
    else {
        date.with_hour(hour).unwrap().with_minute(0).unwrap()
    };
    // if we are after 5pm, we need to add a day to the end time
    let end = date.with_hour(17).unwrap().with_minute(0).unwrap();
    // to ensure we don't go past the end time
    let duration = end.signed_duration_since(start);
    // apply a random offset within the duration
    let random_offset = rand::random::<u32>() % duration.num_minutes() as u32;
    // apply the random offset to the start time
    start + Duration::minutes(random_offset as i64)
}

pub fn scheduler(timezone: &Tz) -> String {
    // get now
    let utc: DateTime<Utc> = Utc::now();
    // get the current offset time
    let current_offset_time = timezone.from_local_datetime(&utc.naive_utc()).unwrap();
    // grab a time between 9am and 5pm in the given timezone
    random_time_between_9_5(current_offset_time).to_rfc3339()
}
