//! This example is derived from material found [here](https://tour.dlang.org/tour/en/gems/contract-programming).

#[macro_use]
extern crate adhesion;
#[macro_use]
extern crate galvanic_assert;
#[macro_use]
extern crate scan_rules;

use scan_rules::ScanError;
use std::str::FromStr;
use std::string::ToString;

#[derive(PartialEq)]
struct Date {
    year: i32,
    month: i32,
    day: i32
}

impl FromStr for Date {
    type Err = ScanError;

    contract! {
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            pre {
                assert!(s.len() == 10);
            }
            body {
                scan! { s;
                    (let year, "-", let month, "-", let day) => Date { year, month, day }
                }
            }
            post(ret) {
                if let &Ok(ref date) = &ret {
                    assert!(date.year >= 1900);
                    assert!(date.month >= 1 && date.month <= 12);
                    assert!(date.day >= 1 && date.day <= 31);
                }
            }
        }
    }
}

impl ToString for Date {
    contract! {
        fn to_string(&self) -> String {
            body {
                format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
            }
            post(s) {
                assert!(s.chars().filter(|c| c == &'-').count() == 2);

                let parts = s.split("-");
                let parts_lengths: Vec<usize> = parts.clone().map(|part| part.len()).collect();
                assert!(&parts_lengths[..] == &[4, 2, 2]);
                assert!(parts.clone().all(|part| i32::from_str(part).is_ok()))
            }
        }
    }
}

fn main() {
    let date = Date { year: 2016, month: 2, day: 7 };

    assert_that!(Date::from_str("2016-7-13").unwrap(), panics);
    assert!(Date::from_str("2016-07-13").unwrap() == Date {
        year: 2016,
        month: 7,
        day: 13
    });

    println!("{}", date.to_string());
}
