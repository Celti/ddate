// In jurisdictions that recognize copyright laws, the author or authors of
// this software dedicate any and all copyright interest in the software to
// the public domain. We make this dedication for the benefit of the public at
// large and to the detriment of our heirs and successors. We intend this
// dedication to be an overt act of relinquishment in perpetuity of all
// present and future rights to this software under copyright law.
//
// For more information, see the file UNLICENSE at this repository's root.

use ddate::DiscordianDate;
use chrono::{Local, NaiveDate};
use std::str::FromStr;

fn main() {
    if let Some(ymd) = std::env::args().nth(1) {
        let date = NaiveDate::from_str(&ymd).unwrap_or_else(|_| {
            println!("Could not parse provided date.");
            std::process::exit(1);
        });
        println!("{} is {}", &date, &date.to_poee());
    } else {
        println!("Today is {}", Local::today().to_poee());
    };
}
