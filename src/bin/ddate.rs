// In jurisdictions that recognize copyright laws, the author or authors of
// this software dedicate any and all copyright interest in the software to
// the public domain. We make this dedication for the benefit of the public at
// large and to the detriment of our heirs and successors. We intend this
// dedication to be an overt act of relinquishment in perpetuity of all
// present and future rights to this software under copyright law.
// 
// For more information, see the file UNLICENSE at this repository's root.

extern crate chrono;
extern crate ddate;

use ddate::DiscordianDate;
use chrono::Local;

fn main() {
    println!("Today is {}", Local::today().to_poee());
}
