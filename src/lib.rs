// In jurisdictions that recognize copyright laws, the author or authors of
// this software dedicate any and all copyright interest in the software to
// the public domain. We make this dedication for the benefit of the public at
// large and to the detriment of our heirs and successors. We intend this
// dedication to be an overt act of relinquishment in perpetuity of all
// present and future rights to this software under copyright law.
// 
// For more information, see the file UNLICENSE at this repository's root.

//! # ddate 0.4.0
//!
//! Discordian dates for Rust. It implements a simple `DiscordianDate` trait
//! for any type implementing Chrono's `Datelike`, allowing simple string
//! output (similar to the `to_rfc2822` and `to_rfc3339` methods already on
//! several of Chrono's types.
//!
//! Also included is a `ddate` binary with output similar to the tool of the
//! same name originally included in `util-linux` (though no features beyond
//! simple output of the current date).

use chrono::Datelike;

/// The apostolic holydays of the Discordian calendar.
const APOSTLES: [&str; 5] = ["Mungday", "Mojoday", "Syaday", "Zaraday", "Maladay"];
/// The seasonal holydays of the Discordian calendar.
const HOLYDAYS: [&str; 5] = ["Chaoflux", "Discoflux", "Confuflux", "Bureflux", "Afflux"];
/// The seasons of the Discordian calendar.
const SEASONS: [&str; 5] = [
    "Chaos",
    "Discord",
    "Confusion",
    "Bureaucracy",
    "The Aftermath",
];
/// The days of the Discordian week.
const WEEKDAYS: [&str; 5] = [
    "Sweetmorn",
    "Boomtime",
    "Pungenday",
    "Prickle-Prickle",
    "Setting Orange",
];

/// The day of the season that an apostolic holyday occurs on.
const APOSTLE_HOLYDAY: usize = 5;
/// The day of the year that St. Tib's Day occurs on (in leap years).
const ST_TIBS_DAY: usize = 59;
/// The number of days in each season.
const SEASON_DAYS: usize = 73;
/// The day of the season that a seasonal holyday occurs on.
const SEASON_HOLYDAY: usize = 50;
/// The number of days in each week.
const WEEK_DAYS: usize = 5;
/// The Curse of Greyface occurred in 1166 B.C.E.
const CURSE_OF_GREYFACE: i32 = 1166;

/// Extends chrono's
/// [`Datelike`](https://docs.rs/chrono/0.4.0/chrono/trait.Datelike.html) to
/// display Discordian calendar dates.
pub trait DiscordianDate: Datelike {
    /// Returns a Discordian calendar date string.
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use ddate::DiscordianDate;
    ///
    /// # fn main() {
    /// let ddate = NaiveDate::from_ymd(2017, 11, 4).to_poee();
    ///
    /// assert_eq!("Pungenday, the 16th day of The Aftermath in the YOLD 3183", ddate);
    /// # }
    /// ```
    fn to_poee(&self) -> String {
        let day = self.ordinal0() as usize;
        let leap = self.year() % 4 == 0 && self.year() % 100 != 0 || self.year() % 400 == 0;
        let year = self.year() + CURSE_OF_GREYFACE;

        if leap && day == ST_TIBS_DAY {
            return format!("St. Tib's Day, in the YOLD {}", year);
        }

        let day_offset = if leap && day > ST_TIBS_DAY {
            day - 1
        } else {
            day
        };

        let day_of_season = day_offset % SEASON_DAYS + 1;

        let season = SEASONS[day_offset / SEASON_DAYS];
        let weekday = WEEKDAYS[day_offset % WEEK_DAYS];

        let holiday = if day_of_season == APOSTLE_HOLYDAY {
            format!("\nCelebrate {}", APOSTLES[day_offset / SEASON_DAYS])
        } else if day_of_season == SEASON_HOLYDAY {
            format!("\nCelebrate {}", HOLYDAYS[day_offset / SEASON_DAYS])
        } else {
            String::with_capacity(0)
        };

        format!(
            "{}, the {} day of {} in the YOLD {}{}",
            weekday,
            ordinalize(day_of_season),
            season,
            year,
            holiday
        )
    }
}

impl<T: Datelike> DiscordianDate for T {}

/// A helper function to ordinalize a numeral.
fn ordinalize(num: usize) -> String {
    let s = num.to_string();

    let suffix = if s.ends_with('1') && !s.ends_with("11") {
        "st"
    } else if s.ends_with('2') && !s.ends_with("12") {
        "nd"
    } else if s.ends_with('3') && !s.ends_with("13") {
        "rd"
    } else {
        "th"
    };

    s + suffix
}


#[cfg(test)]
mod tests {
    use super::DiscordianDate;
    use chrono::{TimeZone, Utc};

    #[test]
    fn day_one_test() {
        assert_eq!(
            "Sweetmorn, the 1st day of Chaos in the YOLD 0",
            Utc.ymd(-1166, 1, 1).to_poee()
        );
    }

    #[test]
    fn ante_tibs_test() {
        assert_eq!(
            "Prickle-Prickle, the 59th day of Chaos in the YOLD 3166",
            Utc.ymd(2000, 2, 28).to_poee()
        );
    }

    #[test]
    fn tibs_test() {
        assert_eq!(
            "St. Tib's Day, in the YOLD 3166",
            Utc.ymd(2000, 2, 29).to_poee()
        );
    }

    #[test]
    fn post_tibs_test() {
        assert_eq!(
            "Setting Orange, the 60th day of Chaos in the YOLD 3166",
            Utc.ymd(2000, 3, 1).to_poee()
        );
    }

    #[test]
    fn ante_anti_tibs_test() {
        assert_eq!(
            "Prickle-Prickle, the 59th day of Chaos in the YOLD 2232",
            Utc.ymd(1066, 2, 28).to_poee()
        );
    }

    #[test]
    fn post_anti_tibs_test() {
        assert_eq!(
            "Setting Orange, the 60th day of Chaos in the YOLD 2232",
            Utc.ymd(1066, 3, 1).to_poee()
        );
    }

    #[test]
    fn holy_test() {
        assert_eq!(
            "Prickle-Prickle, the 50th day of Bureaucracy in the YOLD 3183\nCelebrate Bureflux",
            Utc.ymd(2017, 9, 26).to_poee()
        );
    }

    #[test]
    fn apos_test() {
        assert_eq!(
            "Boomtime, the 5th day of The Aftermath in the YOLD 3183\nCelebrate Maladay",
            Utc.ymd(2017, 10, 24).to_poee()
        );
    }
}
