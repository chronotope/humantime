use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{space0, space1},
    combinator::opt,
    multi::many1,
    number::complete::double,
    sequence::{separated_pair, terminated},
    Finish, IResult, Parser,
};
use std::time::Duration;
use thiserror::Error;

#[derive(Debug)]
enum Unit {
    Nanos,
    Micros,
    Millis,
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
    Months,
    Years,
}

fn parse_decimal(value: f64) -> Option<f64> {
    if value >= 0.0 && value <= u64::MAX as f64 {
        Some(value)
    } else {
        None
    }
}

// Convert parsed units to seconds and nanoseconds
fn convert_to_duration(value: f64, unit: Unit) -> Duration {
    let total_seconds = match unit {
        Unit::Nanos => value * 1e-9,
        Unit::Micros => value * 1e-6,
        Unit::Millis => value * 1e-3,
        Unit::Seconds => value,
        Unit::Minutes => value * 60.,
        Unit::Hours => value * 3600.,
        Unit::Days => value * 86400.,
        Unit::Weeks => value * 604800.,
        Unit::Months => value * 30.44 * 86400.,
        Unit::Years => value * 365.25 * 86400.,
    };

    let seconds = total_seconds.floor() as u64;
    let nanos = ((total_seconds - total_seconds.floor()) * 1e9).round() as u32;

    Duration::new(seconds, nanos)
}

// Parse a unit name
fn unit(input: &str) -> IResult<&str, Unit> {
    let nanosecond = alt((tag("nanos"), tag("nsec"), tag("ns"))).map(|_| Unit::Nanos);
    let microsecond = alt((tag("micros"), tag("usec"), tag("us"))).map(|_| Unit::Micros);
    let millisecond = alt((tag("millis"), tag("msec"), tag("ms"))).map(|_| Unit::Millis);
    let seconds = alt((
        tag("seconds"),
        tag("second"),
        tag("secs"),
        tag("sec"),
        tag("s"),
    ))
    .map(|_| Unit::Seconds);
    let minutes = alt((
        tag("minutes"),
        tag("minute"),
        tag("mins"),
        tag("min"),
        tag("m"),
    ))
    .map(|_| Unit::Minutes);
    let hours = alt((
        tag("hours"),
        tag("hour"),
        tag("hrs"),
        tag("hr"),
        tag("h"),
        tag("H"),
    ))
    .map(|_| Unit::Hours);
    let days = alt((
        tag("days"),
        tag("day"),
        tag("dys"),
        tag("dy"),
        tag("d"),
        tag("D"),
    ))
    .map(|_| Unit::Days);
    let weeks = alt((
        tag("weeks"),
        tag("week"),
        tag("wks"),
        tag("wk"),
        tag("w"),
        tag("W"),
    ))
    .map(|_| Unit::Weeks);
    let months = alt((
        tag("months"),
        tag("month"),
        tag("mths"),
        tag("mth"),
        tag("M"),
    ))
    .map(|_| Unit::Months);
    let years = alt((
        tag("years"),
        tag("year"),
        tag("yrs"),
        tag("yr"),
        tag("y"),
        tag("Y"),
    ))
    .map(|_| Unit::Years);

    alt((
        months,
        days,
        weeks,
        years,
        nanosecond,
        microsecond,
        millisecond,
        seconds,
        minutes,
        hours,
    ))
    .parse(input)
}

fn number(input: &str) -> IResult<&str, f64> {
    double.map_opt(parse_decimal).parse(input)
}

// Parse a float followed by a unit
fn time_span(input: &str) -> IResult<&str, Duration> {
    let (input, (value, unit)) =
        terminated(separated_pair(number, opt(space0), unit), opt(space1)).parse(input)?;
    Ok((input, convert_to_duration(value, unit)))
}

#[derive(Debug, Error, PartialEq)]
pub enum ParseError {
    #[error("input is empty")]
    EmptyInput,
    #[error("input was not fully parsed")]
    InputLeftOver,
    #[error("parse error: {0}")]
    Nom(#[from] nom::error::Error<String>),
}

// Parse the full duration object
pub fn parse_duration_new(input: &str) -> Result<Duration, ParseError> {
    let input = input.trim();
    if input.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    let (input, durations) = many1(time_span)
        .parse(input)
        .map_err(|e| e.to_owned())
        .finish()?;

    if !input.trim().is_empty() {
        return Err(ParseError::InputLeftOver);
    }

    let total_duration = durations
        .into_iter()
        .fold(Duration::new(0, 0), |acc, duration| acc + duration);
    Ok(total_duration)
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use crate::new::parse_duration_new;

    macro_rules! assert_parse_duration_ok {
        ($input:expr, $secs:expr, $nanos:expr) => {
            assert_eq!(parse_duration_new($input), Ok(Duration::new($secs, $nanos)));
        };
    }

    macro_rules! assert_parse_duration_err {
        ($input:expr) => {
            assert_eq!(
                parse_duration_new($input),
                Err(crate::new::ParseError::Nom(nom::error::Error::new(
                    $input.to_owned(),
                    nom::error::ErrorKind::MapOpt
                )))
            );
        };
    }

    #[test]
    fn test_nanosecond() {
        assert_parse_duration_ok!("1nanos", 0, 1);
        assert_parse_duration_ok!("1 nanos", 0, 1);

        assert_parse_duration_ok!("2nsec", 0, 2);
        assert_parse_duration_ok!("2 nsec", 0, 2);

        assert_parse_duration_ok!("3ns", 0, 3);
        assert_parse_duration_ok!("3 ns", 0, 3);
    }

    #[test]
    fn test_microsecond() {
        assert_parse_duration_ok!("1micros", 0, 1000);
        assert_parse_duration_ok!("1 micros", 0, 1000);

        assert_parse_duration_ok!("2usec", 0, 2000);
        assert_parse_duration_ok!("2 usec", 0, 2000);

        assert_parse_duration_ok!("3us", 0, 3000);
        assert_parse_duration_ok!("3 us", 0, 3000);
    }

    #[test]
    fn test_millisecond() {
        assert_parse_duration_ok!("1millis", 0, 1_000_000);
        assert_parse_duration_ok!("1 millis", 0, 1_000_000);

        assert_parse_duration_ok!("2msec", 0, 2_000_000);
        assert_parse_duration_ok!("2 msec", 0, 2_000_000);

        assert_parse_duration_ok!("3ms", 0, 3_000_000);
        assert_parse_duration_ok!("3 ms", 0, 3_000_000);
    }

    #[test]
    fn test_seconds() {
        assert_parse_duration_ok!("1seconds", 1, 0);
        assert_parse_duration_ok!("1 seconds", 1, 0);

        assert_parse_duration_ok!("2second", 2, 0);
        assert_parse_duration_ok!("2 second", 2, 0);

        assert_parse_duration_ok!("3secs", 3, 0);
        assert_parse_duration_ok!("3 secs", 3, 0);

        assert_parse_duration_ok!("4sec", 4, 0);
        assert_parse_duration_ok!("4 sec", 4, 0);

        assert_parse_duration_ok!("5s", 5, 0);
        assert_parse_duration_ok!("5 s", 5, 0);
    }

    #[test]
    fn test_minutes() {
        assert_parse_duration_ok!("1minutes", 1 * 60, 0);
        assert_parse_duration_ok!("1 minutes", 1 * 60, 0);

        assert_parse_duration_ok!("2minute", 2 * 60, 0);
        assert_parse_duration_ok!("2 minute", 2 * 60, 0);

        assert_parse_duration_ok!("3mins", 3 * 60, 0);
        assert_parse_duration_ok!("3 mins", 3 * 60, 0);

        assert_parse_duration_ok!("4min", 4 * 60, 0);
        assert_parse_duration_ok!("4 min", 4 * 60, 0);

        assert_parse_duration_ok!("5m", 5 * 60, 0);
        assert_parse_duration_ok!("5 m", 5 * 60, 0);
    }

    #[test]
    fn test_hours() {
        assert_parse_duration_ok!("1hours", 1 * 3600, 0);
        assert_parse_duration_ok!("1 hours", 1 * 3600, 0);

        assert_parse_duration_ok!("2hour", 2 * 3600, 0);
        assert_parse_duration_ok!("2 hour", 2 * 3600, 0);

        assert_parse_duration_ok!("3hrs", 3 * 3600, 0);
        assert_parse_duration_ok!("3 hrs", 3 * 3600, 0);

        assert_parse_duration_ok!("4hr", 4 * 3600, 0);
        assert_parse_duration_ok!("4 hr", 4 * 3600, 0);

        assert_parse_duration_ok!("5h", 5 * 3600, 0);
        assert_parse_duration_ok!("5 h", 5 * 3600, 0);

        assert_parse_duration_ok!("5H", 5 * 3600, 0);
        assert_parse_duration_ok!("5 H", 5 * 3600, 0);
    }

    #[test]
    fn test_days() {
        assert_parse_duration_ok!("1days", 1 * 86400, 0);
        assert_parse_duration_ok!("1 days", 1 * 86400, 0);

        assert_parse_duration_ok!("2day", 2 * 86400, 0);
        assert_parse_duration_ok!("2 day", 2 * 86400, 0);

        assert_parse_duration_ok!("3dys", 3 * 86400, 0);
        assert_parse_duration_ok!("3 dys", 3 * 86400, 0);

        assert_parse_duration_ok!("4dy", 4 * 86400, 0);
        assert_parse_duration_ok!("4 dy", 4 * 86400, 0);

        assert_parse_duration_ok!("5d", 5 * 86400, 0);
        assert_parse_duration_ok!("5 d", 5 * 86400, 0);

        assert_parse_duration_ok!("5D", 5 * 86400, 0);
        assert_parse_duration_ok!("5 D", 5 * 86400, 0);
    }

    #[test]
    fn test_combo() {
        assert_parse_duration_ok!("20 min 17 nsec", 1200, 17);
        assert_parse_duration_ok!("2h 15m", 8100, 0);
    }

    #[test]
    fn test_overlow() {
        assert_parse_duration_err!("100000000000000000000ns");
        assert_parse_duration_err!("100000000000000000000us");
        assert_parse_duration_err!("100000000000000000000ms");
        assert_parse_duration_err!("100000000000000000000s");
        assert_parse_duration_err!("100000000000000000000m");
        assert_parse_duration_err!("100000000000000000000h");
        assert_parse_duration_err!("100000000000000000000d");
        assert_parse_duration_err!("100000000000000000000w");
        assert_parse_duration_err!("100000000000000000000M");
        assert_parse_duration_err!("100000000000000000000Y");
    }
}
