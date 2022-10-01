use std::fmt;

use time::{OffsetDateTime, UtcOffset};

pub fn to_rfc3339_nanos(t: OffsetDateTime) -> String {
    // yyyy-mm-ddThh:mm:ssZ
    let mut buf = String::with_capacity(20);

    fmt_as_rfc3339_nanos(t, &mut buf).unwrap();

    buf
}

pub fn fmt_as_rfc3339_nanos(t: OffsetDateTime, f: &mut impl fmt::Write) -> fmt::Result {
    let t = t.to_offset(UtcOffset::UTC);
    let nanos = t.nanosecond();
    if nanos == 0 {
        write!(
            f,
            "{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z",
            year = t.year(),
            month = t.month() as u8,
            day = t.day(),
            hour = t.hour(),
            minute = t.minute(),
            second = t.second(),
        )
    } else {
        let mut secfrac = nanos;
        let mut secfrac_width = 9;
        while secfrac % 10 == 0 {
            secfrac /= 10;
            secfrac_width -= 1;
        }
        write!(
            f,
            "{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}.{secfrac:0sfw$}Z",
            year = t.year(),
            month = t.month() as u8,
            day = t.day(),
            hour = t.hour(),
            minute = t.minute(),
            second = t.second(),
            secfrac = secfrac,
            sfw = secfrac_width,
        )
    }
}
