use crate::error::MyError;
use std::ops::AddAssign;
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
    // let b = chrono::Duration::days(30);
    // let c = a + b;
    // dbg!(c.num_days());
    let mut num = 0_i64;
    for (idx, c) in s.chars().enumerate() {
        match c {
            '0'..='9' => {
                num = num.checked_mul(10).ok_or(MyError::ParseDurationError {
                    pos: idx,
                    reason: "mul overflow!".to_string(),
                })?;
                dbg!(&num);
                num = num.checked_add((c as u8 - b'0') as i64).ok_or(
                    MyError::ParseDurationError {
                        pos: idx,
                        reason: "plus overflow!".to_string(),
                    },
                )?;
                //
                // if num < 0 {
                //     // maybe overflow!!
                //     return Err(MyError::ParseDurationError {
                //         pos: idx,
                //         reason: "overflow".into(),
                //     });
                // }
            }
            'y' => {
                res.add_assign(chrono::Duration::days(DAYS_PER_YEAR * num));
                num = 0;
            }
            'M' => {
                res.add_assign(chrono::Duration::days(DAYS_PER_MONTH * num));
                num = 0;
            }
            'd' => {
                res.add_assign(chrono::Duration::days(num));
                num = 0;
            }
            'h' => {
                res.add_assign(chrono::Duration::hours(num));
                num = 0;
            }
            'm' => {
                res.add_assign(chrono::Duration::minutes(num));
                num = 0;
            }
            's' => {
                res.add_assign(chrono::Duration::seconds(num));
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
