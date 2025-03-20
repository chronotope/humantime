use std::{fmt, time::Duration};

use crate::units::Unit;

/// A wrapper type that allows you to Display a Duration
#[derive(Debug, Clone)]
pub struct FormattedDuration(Duration);

/// Formats duration into a human-readable string
///
/// Note: this format is guaranteed to have same value when using
/// parse_duration, but we can change some details of the exact composition
/// of the value.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
/// use humantime::format_duration;
///
/// let val1 = Duration::new(9420, 0);
/// assert_eq!(format_duration(val1).to_string(), "2h 37m");
/// let val2 = Duration::new(0, 32_000_000);
/// assert_eq!(format_duration(val2).to_string(), "32ms");
/// ```
pub fn format_duration(val: Duration) -> FormattedDuration {
    FormattedDuration(val)
}

fn item_plural(f: &mut fmt::Formatter, started: &mut bool, name: &str, value: u64) -> fmt::Result {
    if value > 0 {
        if *started {
            f.write_str(" ")?;
        }
        write!(f, "{}{}", value, name)?;
        if value > 1 {
            f.write_str("s")?;
        }
        *started = true;
    }
    Ok(())
}

fn item(f: &mut fmt::Formatter, started: &mut bool, name: &str, value: u64) -> fmt::Result {
    if value > 0 {
        if *started {
            f.write_str(" ")?;
        }
        write!(f, "{}{}", value, name)?;
        *started = true;
    }
    Ok(())
}

impl FormattedDuration {
    /// Returns a reference to the [`Duration`][] that is being formatted.
    pub fn get_ref(&self) -> &Duration {
        &self.0
    }
}

impl fmt::Display for FormattedDuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let secs = self.0.as_secs_f64();

        if secs == 0.0 {
            f.write_str("0s")?;
            return Ok(());
        }

        let (years, secs) = Unit::Years.from_second(secs);
        let (months, secs) = Unit::Months.from_second(secs);
        let (days, secs) = Unit::Days.from_second(secs);
        let (hours, secs) = Unit::Hours.from_second(secs);
        let (minutes, secs) = Unit::Minutes.from_second(secs);
        let (seconds, secs) = Unit::Seconds.from_second(secs);
        let (millis, secs) = Unit::Millis.from_second(secs);
        let (micros, secs) = Unit::Micros.from_second(secs);
        let (nanos, _) = Unit::Micros.from_second(secs);

        let started = &mut false;
        item_plural(f, started, "year", years)?;
        item_plural(f, started, "month", months)?;
        item_plural(f, started, "day", days)?;
        item(f, started, "h", hours)?;
        item(f, started, "m", minutes)?;
        item(f, started, "s", seconds)?;
        item(f, started, "ms", millis)?;
        item(f, started, "us", micros)?;
        item(f, started, "ns", nanos)?;
        Ok(())
    }
}
