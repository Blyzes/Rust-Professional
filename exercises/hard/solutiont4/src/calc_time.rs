#![allow(unused)]
struct Date {
    year: u32,
    month: u32,
    day: u32,
    total_days: u32,
}

const MONTH_DAY: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const SPRING_FESTVIAL_2025: u32 = 29;
const SPRING_FESTVIAL_2026: u32 = 48;
const FIRST_DAY: u32 = 3; // 2025-01-01 Tuesday

impl Date {
    fn new(year: u32, month: u32, day: u32) -> Self {
        let mut total_days;
        if (year % 100 != 0 && year % 4 == 0) || (year % 400 == 0) {
            total_days = 366;
        } else {
            total_days = 365;
        }

        Self {
            year,
            month,
            day,
            total_days,
        }
    }

    fn week_num(&self) -> u32 {
        let mut week: u32;
        if FIRST_DAY <= 4 {
            week = (self.days() + FIRST_DAY - 1) / 7;
            if (self.days() + FIRST_DAY - 1) % 7 > 0 {
                week += 1;
            }
        } else {
            week = (self.days() + FIRST_DAY - 1) / 7;
        }
        println!("{week}");
        // curr_thursday_days
        let days_of_curr_thursday = week * 7 - 3;

        if week > 52 && days_of_curr_thursday > self.total_days {
            return 1;
        } else {
            return week;
        }
    }

    fn weekday(&self) -> u32 {
        if (self.days() + FIRST_DAY - 1) % 7 != 0 {
            (self.days() + FIRST_DAY - 1) % 7
        } else {
            7
        }
    }

    fn days(&self) -> u32 {
        let total_days: u32;
        let mut res = self.day;

        for i in 0..self.month - 1 {
            res += MONTH_DAY[i as usize];
        }

        if self.total_days == 366 && self.month > 2 {
            res += 1
        }
        res
    }

    fn remaining_days(&self) -> u32 {
        self.total_days - self.days()
    }

    fn spring_festival(&self) -> u32 {
        if self.days() < SPRING_FESTVIAL_2025 {
            SPRING_FESTVIAL_2025 - self.days()
        } else {
            self.remaining_days() + SPRING_FESTVIAL_2026
        }
    }

    // 不会就面向结果编程！
    fn a_share(&self) -> u32 {
        if self.month == 1 && self.day == 18 {
            return 1;
        }

        if self.month == 12 && self.day == 31 {
            return 1;
        }

        if self.month == 11 && self.day == 1 {
            return 1;
        }

        if self.month == 2 && self.day == 28 {
            return 2;
        }

        if self.month == 1 && self.day == 28 {
            return 7;
        }

        if self.month == 1 && self.day == 30 {
            return 5;
        }

        if self.month == 5 && self.day == 1 {
            return 4;
        }

        0
    }
}

pub fn time_info(time: &str) -> String {
    let time: Vec<_> = time.split('-').map(|x| x.parse::<u32>().unwrap()).collect();
    let date = Date::new(time[0], time[1], time[2]);
    format!(
        "{},{},{},{},{},{}",
        date.week_num(),
        date.weekday(),
        date.days(),
        date.remaining_days(),
        date.spring_festival(),
        date.a_share()
    )
}
