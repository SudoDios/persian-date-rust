use std::cmp::Ordering;
use std::ops::Add;
use chrono::{DateTime, Datelike, Duration, Local, Months, NaiveDate, TimeZone, Timelike, Weekday};
use chrono_tz::Tz;
use crate::structure::{PDate, Reader, Setter};

pub mod format;
pub mod structure;

const DAY_NAMES : [&str; 7] = ["شنبه", "یک‌شنبه", "دوشنبه", "سه‌شنبه", "چهارشنبه", "پنج‌شنبه", "جمعه"];
const EN_DAY_NAMES : [&str; 7] = ["Saturday", "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
const MONTH_NAMES : [&str; 12] = ["فروردین", "اردیبهشت", "خرداد", "تیر", "مرداد", "شهریور", "مهر", "آبان", "آذر", "دی", "بهمن", "اسفند"];
const EN_MONTH_NAMES : [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

impl Default for PDate {
    fn default() -> Self {
        PDate {
            year: 0,
            month: 0,
            day: 0,
            grg_year: 0,
            grg_month: 0,
            grg_day: 0,
            hour: 0,
            minute: 0,
            second: 0,
            nano_second: 0,
            time_millis: 0,
            time_zone: Tz::Iran,
        }
    }
}

impl PartialEq<Self> for PDate {
    fn eq(&self, other: &Self) -> bool {
        self.time_millis() == other.time_millis()
    }
}

impl PartialOrd for PDate {
    fn partial_cmp(&self, other: &PDate) -> Option<Ordering> {
        self.time_millis().partial_cmp(&other.time_millis())
    }
}

impl PDate {

    /** initialize */
    pub fn now() -> Self {
        let current_millis = chrono::Utc::now().timestamp_millis();
        Self::from_time_millis(current_millis)
    }
    pub fn from_time_millis(millis: i64) -> Self {
        let mut pdate = PDate::default();
        let date = DateTime::from_timestamp_millis(millis).unwrap().with_timezone(&pdate.time_zone);
        pdate.update_from_date(&date);
        pdate
    }
    pub fn from_gregorian_date(year: i32, month: u32, day: u32) -> Self {
        let date = chrono::Utc::now().with_year(year).unwrap().with_month(month).unwrap().with_day(day).unwrap();
        Self::from_time_millis(date.timestamp_millis())
    }
    pub fn from_jalali_date(year: i32, month: u32, day: u32) -> Self {
        let conv_date = Self::jalali_to_gregorian(year, month as i32, day as i32);
        let date = Local::now().with_year(conv_date[0]).unwrap()
            .with_month(conv_date[1] as u32).unwrap().with_day(conv_date[2] as u32).unwrap();
        Self::from_time_millis(date.timestamp_millis())
    }

    /** main functions */
    fn get_zoned_date(&self) -> DateTime<Tz> {
        self.time_zone.timestamp_millis_opt(self.time_millis).unwrap()
    }

    pub fn format_timezone(&self) -> String {
        let date = self.get_zoned_date();
        date.format("%:z").to_string()
    }

    fn gregorian_to_jalali(gy : i32,gm : i32,gd : i32) -> Vec<i32> {
        let mut out = vec![
            if gm > 2 { gy + 1 } else { gy },
            0,
            0
        ];
        {
            let g_d_m = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
            out[2] = 355666 + (365 * gy) + ((out[0] + 3) / 4) - ((out[0] + 99) / 100)
                + ((out[0] + 399) / 400) + gd + g_d_m[gm as usize - 1]
        }
        out[0] = -1595 + (33 * (out[2] / 12053));
        out[2] %= 12053;
        out[0] += 4 * (out[2] / 1461);
        out[2] %= 1461;
        if out[2] > 365 {
            out[0] += (out[2] - 1) / 365;
            out[2] = (out[2] - 1) % 365;
        }
        if out[2] < 186 {
            out[1] = 1 + (out[2] / 31);
            out[2] = 1 + (out[2] % 31);
        } else {
            out[1] = 7 + ((out[2] - 186) / 30);
            out[2] = 1 + ((out[2] - 186) % 30);
        }
        out
    }

    fn jalali_to_gregorian(mut jy : i32,jm : i32,jd : i32) -> Vec<i32> {
        jy += 1595;
        let mut out = vec![
            0,
            0,
            -355668 + (365 * jy) + ((jy / 33) * 8) + (((jy % 33) + 3) / 4) + jd + if jm < 7 {(jm - 1) * 31} else {((jm - 7) * 30) + 186}
        ];
        out[0] = 400 * (out[2] / 146097);
        out[2] %= 146097;
        if out[2] > 36524 {
            out[0] += 100 * (-(-out[2]) / 36524);
            out[2] %= 36524;
            if out[2] >= 365 {
                out[2] += 1;
            }
        }
        out[0] += 4 * (out[2] / 1461);
        out[2] %= 1461;
        if out[2] > 365 {
            out[0] += (out[2] - 1) / 365;
            out[2] = (out[2] - 1) % 365;
        }
        let sal_a = [0, 31, if (out[0] % 4 == 0 && out[0] % 100 != 0) || (out[0] % 400 == 0) {29} else {28},
            31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        while out[1] < 13 && out[2] > sal_a[out[1] as usize] {
            out[2] -= sal_a[out[1] as usize];
            out[1] += 1;
        }
        out[2] += 1;
        out
    }

    fn get_date_from_jalali(&self,jalali : &Vec<i32>) -> DateTime<Tz> {
        self.time_zone.timestamp_millis_opt(self.time_millis).unwrap()
            .with_year(jalali[0]).unwrap().with_month(jalali[1] as u32).unwrap().with_day(jalali[2] as u32).unwrap()
    }

    /** updaters */
    fn update_from_date(&mut self,date_time: &DateTime<Tz>) {
        let to_jalali = Self::gregorian_to_jalali(date_time.year(), date_time.month() as i32, date_time.day() as i32);
        self.year = to_jalali[0];
        self.month = to_jalali[1] as u32;
        self.day = to_jalali[2] as u32;

        self.grg_year = date_time.year();
        self.grg_month = date_time.month();
        self.grg_day = date_time.day();

        self.hour = date_time.hour();
        self.minute = date_time.minute();
        self.second = date_time.second();
        self.nano_second = date_time.nanosecond();
        self.time_millis = date_time.timestamp_millis()
    }
}

impl Reader for PDate {
    fn year(&self) -> i32 {
        self.year
    }

    fn grg_year(&self) -> i32 {
        self.grg_year
    }

    fn month(&self) -> u32 {
        self.month
    }

    fn grg_month(&self) -> u32 {
        self.grg_month
    }

    fn day(&self) -> u32 {
        self.day
    }

    fn grg_day(&self) -> u32 {
        self.grg_day
    }

    fn hour(&self) -> u32 {
        self.hour
    }

    fn hour_12(&self) -> u32 {
        if self.hour <= 12 { self.hour } else { self.hour - 12 }
    }

    fn minute(&self) -> u32 {
        self.minute
    }

    fn second(&self) -> u32 {
        self.second
    }

    fn nano_second(&self) -> u32 {
        self.nano_second
    }

    fn time_millis(&self) -> i64 {
        self.time_millis
    }

    fn time_zone(&self) -> Tz {
        self.time_zone
    }

    fn day_name(&self) -> String {
        DAY_NAMES[self.day_of_week() as usize].to_string()
    }

    fn grg_day_name(&self) -> String {
        EN_DAY_NAMES[self.day_of_week() as usize].to_string()
    }

    fn month_days(&self) -> u32 {
        if self.month <= 6 {
            31
        } else if self.month <= 11 || self.is_leap() {
            30
        } else {
            29
        }
    }

    fn grg_month_days(&self) -> u32 {
        let date = self.get_zoned_date();
        NaiveDate::from_ymd_opt(
            match date.month() {
                12 => date.year() + 1,
                _ => date.year(),
            },
            match date.month() {
                12 => 1,
                _ => date.month() + 1,
            },
            1
        ).unwrap().signed_duration_since(NaiveDate::from_ymd_opt(date.year(),date.month(),1).unwrap()).num_days() as u32
    }

    fn month_name(&self) -> String {
        MONTH_NAMES.get((self.month -1) as usize).expect("").to_string()
    }

    fn grg_month_name(&self) -> String {
        EN_MONTH_NAMES.get((self.grg_month -1) as usize).expect("").to_string()
    }

    fn is_leap(&self) -> bool {
        let ref_year = 1375f32;
        let mut start_year = 1375f32;
        let year_res : f32 = (self.year as f32) - ref_year;
        if year_res == 0f32 || year_res % 33f32 == 0f32 {
            return true
        };
        if year_res > 0f32 {
            if year_res > 33f32 {
                let numb = year_res / 33f32;
                start_year = ref_year + (numb.floor() * 33f32);
            }
        } else if year_res > -33f32 {
            start_year = ref_year - 33f32;
        } else {
            let numb = (year_res / 33f32).abs();
            start_year = ref_year - (numb.ceil() * 33f32);
        }
        let leap_years = [start_year,start_year + 4f32,start_year + 8f32,
            start_year + 12f32,start_year + 16f32,start_year + 20f32,start_year + 24f32,start_year + 28f32,start_year + 33f32];
        leap_years.contains(&(self.year as f32))
    }

    fn is_grg_leap(&self) -> bool {
        if self.grg_year % 4 == 0 {
            if self.grg_year % 100 == 0 {
                return self.grg_year % 400 == 0;
            }
            return true
        }
        false
    }

    fn day_of_week(&self) -> u32 {
        let date = self.get_zoned_date();
        if date.weekday() == Weekday::Sat {
            return 0
        }
        date.weekday().num_days_from_sunday() + 1
    }

    fn day_of_year(&self) -> u32 {
        let mut day = self.day;
        for i in 1..self.month {
            if i <= 6 {
                day += 31
            } else {
                day += 30
            }
        }
        day
    }

    fn time_of_day(&self) -> String {
        if self.is_mid_night() { "قبل از ظهر".to_string() } else { "بعد از ظهر".to_string() }
    }

    fn short_time_of_day(&self) -> String {
        if self.is_mid_night() { "ق.ظ".to_string() } else { "ب.ظ".to_string() }
    }

    fn is_mid_night(&self) -> bool {
        self.hour < 12
    }
}

impl Setter for PDate {
    fn set_ymd(&mut self, year: i32, month: u32, day: u32) {
        self.set_year(year);
        self.set_month(month);
        self.set_day(day)
    }

    fn set_grg_ymd(&mut self, year: i32, month: u32, day: u32) {
        self.set_grg_year(year);
        self.set_grg_month(month);
        self.set_grg_day(day)
    }

    fn set_year(&mut self, year: i32) {
        if year < 1 {
            println!("Year must be greater than 0");
            return;
        }
        let conv_date = Self::jalali_to_gregorian(year, self.month as i32, self.day as i32);
        let date = self.get_date_from_jalali(&conv_date);
        self.update_from_date(&date)
    }

    fn set_grg_year(&mut self, year: i32) {
        if year < 1 {
            println!("Year must be greater than 0");
            return;
        }
        let date = self.get_zoned_date();
        let date = date.with_year(year).unwrap();
        self.update_from_date(&date)
    }

    fn set_month(&mut self, month: u32) {
        if !(1..=12).contains(&month) {
            println!("Month must be between 1 and 12");
            return;
        }
        let conv_date = Self::jalali_to_gregorian(self.year, month as i32, self.day as i32);
        let date = self.get_date_from_jalali(&conv_date);
        self.update_from_date(&date)
    }

    fn set_grg_month(&mut self, month: u32) {
        if !(1..=12).contains(&month) {
            println!("Month must be between 1 and 12");
            return;
        }
        let date = self.get_zoned_date();
        let date = date.with_month(month).unwrap();
        self.update_from_date(&date)
    }

    fn set_day(&mut self, day: u32) {
        if !(1..=31).contains(&day) {
            println!("Day must be between 1 and 31");
            return;
        }
        if day > self.month_days() {
            println!("Day must be between 1-{}",self.month_days());
            return;
        }
        let conv_date = Self::jalali_to_gregorian(self.year, self.month as i32, day as i32);
        let date = self.get_date_from_jalali(&conv_date);
        self.update_from_date(&date)
    }

    fn set_grg_day(&mut self, day: u32) {
        if !(1..=31).contains(&day) {
            println!("Day must be between 1 and 31");
            return;
        }
        if day > self.grg_month_days() {
            println!("Day must be between 1-{}",self.grg_month_days());
            return;
        }
        let date = self.get_zoned_date();
        let date = date.with_day(day).unwrap();
        self.update_from_date(&date)
    }

    fn set_hour(&mut self, hour: u32) {
        if hour > 23 {
            println!("Hour must be between 0 and 23");
            return;
        }
        let date = self.get_zoned_date();
        let date = date.with_hour(hour).unwrap();
        self.update_from_date(&date)
    }

    fn set_minute(&mut self, minute: u32) {
        if minute > 59 {
            println!("Minute must be between 0 and 59");
            return;
        }
        let date = self.get_zoned_date();
        let date = date.with_minute(minute).unwrap();
        self.update_from_date(&date)
    }

    fn set_second(&mut self, second: u32) {
        if second > 59 {
            println!("Second must be between 0 and 59");
            return;
        }
        let date = self.get_zoned_date();
        let date = date.with_second(second).unwrap();
        self.update_from_date(&date)
    }

    fn set_time_millis(&mut self, millis: i64) {
        let date = self.time_zone.timestamp_millis_opt(millis).unwrap();
        self.update_from_date(&date)
    }

    fn set_time_zone(&mut self, timezone: Tz) {
        self.time_zone = timezone;
        let date = timezone.timestamp_millis_opt(self.time_millis).unwrap();
        self.update_from_date(&date)
    }

    fn add_years(&mut self, years: u32) {
        if years >= 1 {
            let date = self.get_zoned_date();
            let date = date.add(Months::new(years * 12));
            self.update_from_date(&date)
        }
    }

    fn add_months(&mut self, months: u32) {
        if months >= 1 {
            let date = self.get_zoned_date();
            let date = date.add(Months::new(months));
            self.update_from_date(&date)
        }
    }

    fn add_weeks(&mut self, weeks: i64) {
        if weeks >= 1 {
            let date = self.get_zoned_date();
            let date = date.add(Duration::days(weeks * 7));
            self.update_from_date(&date)
        }
    }

    fn add_days(&mut self, days: i64) {
        if days >= 1 {
            let date = self.get_zoned_date();
            let date = date.add(Duration::days(days));
            self.update_from_date(&date)
        }
    }

    fn add_hours(&mut self, hours: i64) {
        if hours >= 1 {
            let date = self.get_zoned_date();
            let date = date.add(Duration::hours(hours));
            self.update_from_date(&date)
        }
    }

    fn add_minutes(&mut self, minutes: i64) {
        if minutes >= 1 {
            let date = self.get_zoned_date();
            let date = date.add(Duration::minutes(minutes));
            self.update_from_date(&date)
        }
    }

    fn add_seconds(&mut self, seconds: i64) {
        if seconds >= 1 {
            let date = self.get_zoned_date();
            let date = date.add(Duration::seconds(seconds));
            self.update_from_date(&date)
        }
    }
}