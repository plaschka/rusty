window.SIDEBAR_ITEMS = {"constant":[["MAX_DATE","The maximum possible `Date`."],["MAX_DATETIME","The maximum possible `DateTime<Utc>`."],["MIN_DATE","The minimum possible `Date`."],["MIN_DATETIME","The minimum possible `DateTime<Utc>`."]],"enum":[["Month","The month of the year."],["RoundingError","An error from rounding by `Duration`"],["SecondsFormat","Specific formatting options for seconds. This may be extended in the future, so exhaustive matching in external code is not recommended."],["Weekday","The day of week."]],"mod":[["format","Formatting (and parsing) utilities for date and time."],["naive","Date and time types unconcerned with timezones."],["offset","The time zone, which calculates offsets from the local time to UTC."],["prelude","A convenience module appropriate for glob imports (`use chrono::prelude::*;`)."]],"struct":[["Date","ISO 8601 calendar date with time zone."],["DateTime","ISO 8601 combined date and time with time zone."],["Duration","ISO 8601 time duration with nanosecond precision."],["Months","A duration in calendar months"],["OutOfRangeError","Represents error when converting `Duration` to/from a standard library implementation"],["ParseMonthError","An error resulting from reading `<Month>` value with `FromStr`."],["ParseWeekdayError","An error resulting from reading `Weekday` value with `FromStr`."]],"trait":[["Datelike","The common set of methods for date component."],["DurationRound","Extension trait for rounding or truncating a DateTime by a Duration."],["SubsecRound","Extension trait for subsecond rounding or truncation to a maximum number of digits. Rounding can be used to decrease the error variance when serializing/persisting to lower precision. Truncation is the default behavior in Chrono display formatting.  Either can be used to guarantee equality (e.g. for testing) when round-tripping through a lower precision format."],["Timelike","The common set of methods for time component."]]};