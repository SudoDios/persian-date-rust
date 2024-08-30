use chrono_tz::Tz;

/// Persian Date Structure
pub struct PDate {
    /// Jalali year
    pub(crate) year : i32,
    /// Jalali month
    pub(crate) month : u32,
    /// Jalali day
    pub(crate) day : u32,
    /// Gregorian year
    pub(crate) grg_year : i32,
    /// Gregorian month
    pub(crate) grg_month : u32,
    /// Gregorian day
    pub(crate) grg_day : u32,
    /// Hour in day
    pub(crate) hour : u32,
    /// Minute in hour
    pub(crate) minute : u32,
    /// Second in minute
    pub(crate) second : u32,
    /// Nanosecond
    pub(crate) nano_second : u32,
    /// Timestamp milliseconds
    pub(crate) time_millis : i64,
    /// Current timezone
    pub(crate) time_zone : Tz
}

/// Reader Trait for fetching data from date
pub trait Reader {
    /// Return the jalali year
    fn year(&self) -> i32;
    /// Return the gregorian year
    fn grg_year(&self) -> i32;

    /// Return the jalali month
    fn month(&self) -> u32;
    /// Return the gregorian month
    fn grg_month(&self) -> u32;

    /// Return the jalali day
    fn day(&self) -> u32;
    /// Return the gregorian day
    fn grg_day(&self) -> u32;

    /// Return hour in day
    fn hour(&self) -> u32;
    /// Return hour in day with 12 time format
    fn hour_12(&self) -> u32;
    /// Return minute in hour
    fn minute(&self) -> u32;
    /// Return second in minute
    fn second(&self) -> u32;
    /// Return nanoseconds
    fn nano_second(&self) -> u32;

    /// Return timestamp milliseconds
    fn time_millis(&self) -> i64;
    /// Return using timezone - Default is Iran/Tehran
    fn time_zone(&self) -> Tz;

    /// Return name of week day
    ///
    /// # Example
    ///
    /// ```
    /// use persian_date::structure::{PDate,Reader};
    ///
    /// let pdate = PDate::now();
    /// println!("{}",pdate.day_name())
    /// // print like : شنبه - یک شنبه - دوشنبه - etc
    /// ```
    fn day_name(&self) -> String;

    /// Return name of week day in gregorian
    ///
    /// # Example
    ///
    /// ```
    /// use persian_date::structure::{PDate,Reader};
    ///
    /// let pdate = PDate::now();
    /// println!("{}",pdate.grg_day_name())
    /// // print like : Saturday - Sunday - Monday - etc
    /// ```
    fn grg_day_name(&self) -> String;

    /// Return count of days in month
    fn month_days(&self) -> u32;
    /// Return count of days in gregorian month
    fn grg_month_days(&self) -> u32;

    /// Return name of month
    ///
    /// # Example
    ///
    /// ```
    /// use persian_date::structure::{PDate,Reader};
    ///
    /// let pdate = PDate::now();
    /// println!("{}",pdate.month_name())
    /// // print like : مهر - آبان - آذر - etc
    /// ```
    fn month_name(&self) -> String;

    /// Return name of gregorian month
    ///
    /// # Example
    ///
    /// ```
    /// use persian_date::structure::{PDate,Reader};
    ///
    /// let pdate = PDate::now();
    /// println!("{}",pdate.grg_month_name())
    /// // print like : January - February - March - etc
    /// ```
    fn grg_month_name(&self) -> String;

    /// Return jalali year is leap or not
    fn is_leap(&self) -> bool;
    /// Return gregorian year is leap or not
    fn is_grg_leap(&self) -> bool;
    /// Return day in week
    fn day_of_week(&self) -> u32;
    /// Return day in year
    fn day_of_year(&self) -> u32;

    /// Return time of the day
    ///
    /// # Example
    ///
    /// ```
    /// use persian_date::structure::{PDate,Reader};
    ///
    /// let pdate = PDate::now();
    /// println!("{}",pdate.time_of_day())
    /// // print like : قبل از ظهر - بعد از ظهر
    /// ```
    fn time_of_day(&self) -> String;

    /// Return time of the day in short format
    ///
    /// # Example
    ///
    /// ```
    /// use persian_date::structure::{PDate,Reader};
    ///
    /// let pdate = PDate::now();
    /// println!("{}",pdate.short_time_of_day())
    /// // print like : ق.ظ - ب.ظ
    /// ```
    fn short_time_of_day(&self) -> String;

    /// Return current time is before of 12 or not
    fn is_mid_night(&self) -> bool;
}

/// Setter Trait for Set/Update/Change data from date
/// All Add* will be handled to the next date when they exceed the limitation
pub trait Setter {
    /// Set jalali year - month - day (update date to this)
    fn set_ymd(&mut self, year : i32,month : u32,day : u32);
    /// Set gregorian year - month - day (update date to this)
    fn set_grg_ymd(&mut self, year : i32,month : u32,day : u32);
    /// Set jalali year
    fn set_year(&mut self, year : i32);
    /// Set gregorian year
    fn set_grg_year(&mut self, year : i32);

    /// Set jalali month
    fn set_month(&mut self, month : u32);
    /// Set gregorian month
    fn set_grg_month(&mut self, month : u32);

    /// Set jalali day
    fn set_day(&mut self, day : u32);
    /// Set gregorian day
    fn set_grg_day(&mut self, day : u32);

    /// Set hour in day
    fn set_hour(&mut self, hour : u32);
    /// Set minutes in hour
    fn set_minute(&mut self, minute : u32);
    /// Set seconds in minute
    fn set_second(&mut self, second : u32);

    /// Set timestamp milliseconds
    fn set_time_millis(&mut self,millis : i64);

    /// Set/Change timezone
    fn set_time_zone(&mut self,timezone : Tz);

    /// Add years to date
    fn add_years(&mut self,years : u32);
    /// Add months to date
    fn add_months(&mut self,months : u32);
    /// Add weeks to date
    fn add_weeks(&mut self,weeks : i64);
    /// Add days to date
    fn add_days(&mut self,days : i64);
    /// Add hours to date
    fn add_hours(&mut self,hours : i64);
    /// Add minutes to date
    fn add_minutes(&mut self,minutes : i64);
    /// Add seconds to date
    fn add_seconds(&mut self,seconds : i64);
}