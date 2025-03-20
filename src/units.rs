use crate::constants::{
    DAY_TO_SECOND, HOUR_TO_SECOND, MICRO_TO_SECOND, MILLIS_TO_SECOND, MINUTE_TO_SECOND,
    MONTH_TO_SECOND, NANO_TO_SECOND, SECOND_TO_SECOND, WEEK_TO_SECOND, YEAR_TO_SECOND,
};

#[derive(Debug)]
pub(crate) enum Unit {
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

impl Unit {
    pub(crate) fn to_second(&self, value: f64) -> f64 {
        match self {
            Unit::Nanos => value * NANO_TO_SECOND,
            Unit::Micros => value * MICRO_TO_SECOND,
            Unit::Millis => value * MILLIS_TO_SECOND,
            Unit::Seconds => value * SECOND_TO_SECOND,
            Unit::Minutes => value * MINUTE_TO_SECOND,
            Unit::Hours => value * HOUR_TO_SECOND,
            Unit::Days => value * DAY_TO_SECOND,
            Unit::Weeks => value * WEEK_TO_SECOND,
            Unit::Months => value * MONTH_TO_SECOND,
            Unit::Years => value * YEAR_TO_SECOND,
        }
    }

    pub(crate) fn from_second(&self, value: f64) -> (u64, f64) {
        match self {
            Unit::Nanos => (
                (value / NANO_TO_SECOND).trunc() as u64,
                value % NANO_TO_SECOND,
            ),
            Unit::Micros => (
                (value / MICRO_TO_SECOND).trunc() as u64,
                value % MICRO_TO_SECOND,
            ),
            Unit::Millis => (
                (value / MILLIS_TO_SECOND).trunc() as u64,
                value % MILLIS_TO_SECOND,
            ),
            Unit::Seconds => (
                (value / SECOND_TO_SECOND).trunc() as u64,
                value % SECOND_TO_SECOND,
            ),
            Unit::Minutes => (
                (value / MINUTE_TO_SECOND).trunc() as u64,
                value % MINUTE_TO_SECOND,
            ),
            Unit::Hours => (
                (value / HOUR_TO_SECOND).trunc() as u64,
                value % HOUR_TO_SECOND,
            ),
            Unit::Days => (
                (value / DAY_TO_SECOND).trunc() as u64,
                value % DAY_TO_SECOND,
            ),
            Unit::Weeks => (
                (value / WEEK_TO_SECOND).trunc() as u64,
                value % WEEK_TO_SECOND,
            ),
            Unit::Months => (
                (value / MONTH_TO_SECOND).trunc() as u64,
                value % MONTH_TO_SECOND,
            ),
            Unit::Years => (
                (value / YEAR_TO_SECOND).trunc() as u64,
                value % YEAR_TO_SECOND,
            ),
        }
    }
}
