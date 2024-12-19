use crate::error::MyError;
use std::time::Duration;

const DAYS_PER_YEAR: i64 = 366;

const DAYS_PER_MONTH: i64 = 31;

/// parse a duration from str
///
/// The str may like
/// ```
/// 1y    --> 1 years
/// 1M    --> 1 month
/// 10d   --> 10 days
/// 1h    --> 1 hour
/// 1m    --> 1 minus
/// 1s    --> 1 second
/// ```
/// and this can be mixed together
/// ```
/// 1y10d
/// ```
pub(crate) fn parse_duration(s: &str) -> Result<Duration, MyError> {
    let mut res = chrono::Duration::zero();
    let mut num = 0_i64;
    for (idx, c) in s.chars().enumerate() {
        match c {
            '0'..='9' => {
                num = num.checked_mul(10).ok_or(MyError::ParseDurationError {
                    pos: idx,
                    reason: "mul overflow!".to_string(),
                })?;
                num = num.checked_add((c as u8 - b'0') as i64).ok_or(
                    MyError::ParseDurationError {
                        pos: idx,
                        reason: "plus overflow!".to_string(),
                    },
                )?;
            }
            'y' => {
                let days = num
                    .checked_mul(DAYS_PER_YEAR)
                    .ok_or(MyError::ParseDurationError {
                        pos: idx,
                        reason: "add year overflow!".to_string(),
                    })?;
                res = res.checked_add(&chrono::Duration::days(days)).ok_or(
                    MyError::ParseDurationError {
                        pos: idx,
                        reason: "add year overflow!".to_string(),
                    },
                )?;
                num = 0;
            }
            'M' => {
                let days = num
                    .checked_mul(DAYS_PER_MONTH)
                    .ok_or(MyError::ParseDurationError {
                        pos: idx,
                        reason: "add month overflow".to_string(),
                    })?;
                res = res.checked_add(&chrono::Duration::days(days)).ok_or(
                    MyError::ParseDurationError {
                        pos: idx,
                        reason: "add month overflow".to_string(),
                    },
                )?;
                num = 0;
            }
            'd' => {
                res = res.checked_add(&chrono::Duration::days(num)).ok_or(
                    MyError::ParseDurationError {
                        pos: idx,
                        reason: "add day overflow".to_string(),
                    },
                )?;
                num = 0;
            }
            'h' => {
                res = res.checked_add(&chrono::Duration::hours(num)).ok_or(
                    MyError::ParseDurationError {
                        pos: idx,
                        reason: "add hour overflow".to_string(),
                    },
                )?;
                num = 0;
            }
            'm' => {
                res = res.checked_add(&chrono::Duration::minutes(num)).ok_or(
                    MyError::ParseDurationError {
                        pos: idx,
                        reason: "add minute overflow".to_string(),
                    },
                )?;
                num = 0;
            }
            's' => {
                res = res.checked_add(&chrono::Duration::seconds(num)).ok_or(
                    MyError::ParseDurationError {
                        pos: idx,
                        reason: "add second overflow".to_string(),
                    },
                )?;
                num = 0;
            }
            _ => {
                return Err(MyError::ParseDurationError {
                    pos: idx,
                    reason: format!("invalid char : {}", c),
                });
            }
        }
    }

    res.to_std().map_err(|e| MyError::ParseDurationError {
        pos: 0,
        reason: e.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_duration() -> eyre::Result<()> {
        let dur = parse_duration("17y")?;
        dbg!(dur);
        // i64::MAX
        Ok(())
    }
}
