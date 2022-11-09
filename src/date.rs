use std::cmp::Ordering;
use std::convert::Into;
use std::ops::{Add, AddAssign};

use crate::state::Alert;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Date {
    month: u32,
    day: u32,
    year: u32,
}

impl Date {
    pub fn new(month: u32, day: u32, year: u32) -> Date {
        Date { month, day, year }.check()
    }

    pub fn display(&self) -> String {
        [
            "",
            &self.month.to_string(),
            "/",
            &self.day.to_string(),
            "/",
            &self.year.to_string(),
        ]
        .concat()
    }

    pub fn ugly_display(&self) -> String {
        [
            "",
            &self.day.to_string(),
            "/",
            &self.month.to_string(),
            "/",
            &self.year.to_string(),
        ]
        .concat()
    }

    pub fn check(self) -> Date {
        assert!((1..=12).contains(&self.month));
        if [1, 3, 5, 7, 8, 10, 12].contains(&self.month) {
            assert!((1..=31).contains(&self.day));
        } else if [4, 6, 9, 11].contains(&self.month) {
            assert!((1..=30).contains(&self.day));
        } else {
            if self.is_leap() {
                assert!((1..=29).contains(&self.day));
            } else {
                assert!((1..=28).contains(&self.day));
            }
        }
        self
    }

    pub fn is_leap(&self) -> bool {
        (self.year % 4 == 0 && self.year % 100 != 0) || self.year % 400 == 0
    }

    pub fn to_days(&self) -> u32 {
        let mut count = 0;
        count += self.day;
        for _ in 1..=self.month {
            match self.month {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => count += 31,
                4 | 6 | 9 | 11 => count += 30,
                2 => match self.is_leap() {
                    true => count += 29,
                    false => count += 28,
                },
                _ => panic!("month is not within 1..12!"),
            }
        }
        count += if self.is_leap() {
            self.year * 366
        } else {
            self.year * 365
        };
        count
    }

    pub fn add_days(&mut self, days: i32) -> Option<Alert> {
        let mut alert = None;
        for _ in 0..days {
            self.day += 1;
            match self.month {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => {
                    if self.day > 31 {
                        self.day = 1;
                        self.month += 1;
                        alert = Some(Alert::NewMonth);
                    }
                }
                4 | 6 | 9 | 11 => {
                    if self.day > 30 {
                        self.day = 1;
                        self.month += 1;
                        alert = Some(Alert::NewMonth);
                    }
                }
                2 => {
                    if self.is_leap() {
                        if self.day > 29 {
                            self.day = 1;
                            self.month += 1;
                            alert = Some(Alert::NewMonth);
                        }
                    } else {
                        if self.day > 28 {
                            self.day = 1;
                            self.month += 1;
                            alert = Some(Alert::NewMonth);
                        }
                    }
                }
                _ => {
                    panic!("month is not within 1..12!");
                }
            }

            if self.month > 12 {
                self.month = 1;
                self.year += 1;
                alert = Some(Alert::NewYear);
            }
        }

        alert
    }

    pub fn add_months(&mut self, months: u32) -> Date {
        self.month += months;
        if self.month > 12 {
            self.year += self.month / 12;
            self.month %= 12;
        }
        *self
    }

    pub fn add_years(&mut self, years: u32) -> Date {
        self.year += years;
        *self
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Date) -> Option<Ordering> {
        match self.year.cmp(&other.year) {
            Ordering::Less => Some(Ordering::Less),
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Equal => match self.month.cmp(&other.month) {
                Ordering::Less => Some(Ordering::Less),
                Ordering::Greater => Some(Ordering::Greater),
                Ordering::Equal => match self.day.cmp(&other.day) {
                    Ordering::Less => Some(Ordering::Less),
                    Ordering::Greater => Some(Ordering::Greater),
                    Ordering::Equal => Some(Ordering::Equal),
                },
            },
        }
    }

    fn lt(&self, other: &Date) -> bool {
        (self.year < other.year)
            || (self.year == other.year && self.month < other.month)
            || (self.year == other.year && self.month == other.month && self.day < self.day)
    }

    fn le(&self, other: &Date) -> bool {
        (self.year < other.year)
            || (self.year == other.year && self.month < other.month)
            || (self.year == other.year && self.month == other.month && self.day <= self.day)
    }

    fn gt(&self, other: &Date) -> bool {
        (self.year > other.year)
            || (self.year == other.year && self.month > other.month)
            || (self.year == other.year && self.month == other.month && self.day > self.day)
    }
    fn ge(&self, other: &Date) -> bool {
        (self.year > other.year)
            || (self.year == other.year && self.month > other.month)
            || (self.year == other.year && self.month == other.month && self.day >= self.day)
    }
}

impl<T> Add<T> for Date
where
    T: Into<i32>,
{
    type Output = Date;

    fn add(self, value: T) -> Date {
        let mut date: Date = self;
        date.add_days(value.into());
        date
    }
}

impl<T> AddAssign<T> for Date
where
    T: Into<i32>,
{
    fn add_assign(&mut self, value: T) {
        *self = *self + value.into();
    }
}
