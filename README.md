# ddate 0.4.0
This is a Rust crate providing output similar to the classic `ddate` tool once
a part of `util-linux`. It extends [`chrono`'s] `Datelike` trait to provide
Discordian calendar date output for such objects.

The API not being expected to change, this crate's “minor” version is being
kept in sync with [`chrono`'s] until that crate achieves 1.0.

## Usage
```rust
use chrono::NaiveDate;
use ddate::DiscordianDate;

fn main() {
    assert_eq!(
        "Pungenday, the 16th day of The Aftermath in the YOLD 3183",
        NaiveDate::from_ymd(2017, 11, 4).to_poee()
    );
}
```

## Unlicense and Copyright
This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or distribute
this software, either in source code form or as a compiled binary, for any
purpose, commercial or non-commercial, and by any means.

In jurisdictions that recognize copyright laws, the author or authors of this
software dedicate any and all copyright interest in the software to the public
domain. We make this dedication for the benefit of the public at large and to
the detriment of our heirs and successors. We intend this dedication to be an
overt act of relinquishment in perpetuity of all present and future rights to
this software under copyright law.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL THE
AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

For more information, please refer to <http://unlicense.org/>.

[`chrono`'s]: https://github.com/chronotope/chrono
