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
        let (weeks, secs) = Unit::Weeks.from_second(secs);
        let (days, secs) = Unit::Days.from_second(secs);
        let (hours, secs) = Unit::Hours.from_second(secs);
        let (minutes, secs) = Unit::Minutes.from_second(secs);
        let (seconds, secs) = Unit::Seconds.from_second(secs);
        let (millis, secs) = Unit::Millis.from_second(secs);
        let (micros, secs) = Unit::Micros.from_second(secs);
        let (nanos, _) = Unit::Nanos.from_second(secs);

        let started = &mut false;
        item_plural(f, started, "year", years)?;
        item_plural(f, started, "month", months)?;
        item_plural(f, started, "week", weeks)?;
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

#[cfg(test)]
mod test {
    use crate::{
        constants::{
            DAY_TO_SECOND, HOUR_TO_SECOND, MINUTE_TO_SECOND, MONTH_TO_SECOND, SECOND_TO_SECOND,
            WEEK_TO_SECOND, YEAR_TO_SECOND,
        },
        format_duration,
    };
    use std::time::Duration;

    macro_rules! assert_format_duration {
        ($input:expr, $secs:expr, $nanos:expr) => {
            assert_eq!(
                $input,
                format_duration(Duration::new($secs, $nanos)).to_string()
            );
        };
    }

    #[test]
    fn test_units() {
        assert_format_duration!("1ns", 0, 1);
        assert_format_duration!("1us", 0, 1000);
        assert_format_duration!("1ms", 0, 1_000_000);
        assert_format_duration!("1s", 1, 0);
        assert_format_duration!("1m", 60, 0);
        assert_format_duration!("1h", 3600, 0);

        assert_format_duration!("1day", DAY_TO_SECOND as u64, 0);
        assert_format_duration!("2days", 2 * DAY_TO_SECOND as u64, 0);

        assert_format_duration!("1month", MONTH_TO_SECOND as u64, 0);
        assert_format_duration!("2months", 2 * MONTH_TO_SECOND as u64, 0);

        assert_format_duration!("1week", WEEK_TO_SECOND as u64, 0);
        assert_format_duration!("2weeks", 2 * WEEK_TO_SECOND as u64, 0);

        assert_format_duration!("1year", YEAR_TO_SECOND as u64, 0);
        assert_format_duration!("2years", 2 * YEAR_TO_SECOND as u64, 0);
    }

    #[test]
    fn test_combo() {
        let year = 2 * YEAR_TO_SECOND as u64;
        let month = 2 * MONTH_TO_SECOND as u64;
        let week = 2 * WEEK_TO_SECOND as u64;
        let day = 2 * DAY_TO_SECOND as u64;
        let hour = 2 * HOUR_TO_SECOND as u64;
        let minute = 2 * MINUTE_TO_SECOND as u64;
        let second = 2 * SECOND_TO_SECOND as u64;

        assert_format_duration!(
            "2years 2months 2weeks 2days 2h 2m 2s 200ms 2ns",
            year + month + week + day + hour + minute + second,
            200000000
        );
    }
}
