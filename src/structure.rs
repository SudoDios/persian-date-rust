use chrono_tz::Tz;
/// Persian Date Structure
pub struct PDate {
    pub(crate) year : i32,
    pub(crate) month : u32,
    pub(crate) day : u32,
    pub(crate) grg_year : i32,
    pub(crate) grg_month : u32,
    pub(crate) grg_day : u32,
    pub(crate) hour : u32,
    pub(crate) minute : u32,
    pub(crate) second : u32,
    pub(crate) nano_second : u32,
    pub(crate) time_millis : i64,
    pub(crate) time_zone : Tz
}

pub trait Reader {
    fn year(&self) -> i32;
    fn grg_year(&self) -> i32;

    fn month(&self) -> u32;
    fn grg_month(&self) -> u32;

    fn day(&self) -> u32;
    fn grg_day(&self) -> u32;

    fn hour(&self) -> u32;
    fn hour_12(&self) -> u32;
    fn minute(&self) -> u32;
    fn second(&self) -> u32;
    fn nano_second(&self) -> u32;

    fn time_millis(&self) -> i64;
    fn time_zone(&self) -> Tz;

    fn day_name(&self) -> String;
    fn grg_day_name(&self) -> String;

    fn month_days(&self) -> u32;
    fn grg_month_days(&self) -> u32;

    fn month_name(&self) -> String;
    fn grg_month_name(&self) -> String;

    fn is_leap(&self) -> bool;
    fn is_grg_leap(&self) -> bool;
    fn day_of_week(&self) -> u32;
    fn day_of_year(&self) -> u32;
    fn time_of_day(&self) -> String;
    fn short_time_of_day(&self) -> String;
    fn is_mid_night(&self) -> bool;
}

pub trait Setter {
    fn set_ymd(&mut self, year : i32,month : u32,day : u32);
    fn set_grg_ymd(&mut self, year : i32,month : u32,day : u32);
    fn set_year(&mut self, year : i32);
    fn set_grg_year(&mut self, year : i32);

    fn set_month(&mut self, month : u32);
    fn set_grg_month(&mut self, month : u32);

    fn set_day(&mut self, day : u32);
    fn set_grg_day(&mut self, day : u32);

    fn set_hour(&mut self, hour : u32);
    fn set_minute(&mut self, minute : u32);
    fn set_second(&mut self, second : u32);

    fn set_time_millis(&mut self,millis : i64);
    fn set_time_zone(&mut self,timezone : Tz);

    fn add_years(&mut self,years : u32);
    fn add_months(&mut self,months : u32);
    fn add_weeks(&mut self,weeks : i64);
    fn add_days(&mut self,days : i64);
    fn add_hours(&mut self,hours : i64);
    fn add_minutes(&mut self,minutes : i64);
    fn add_seconds(&mut self,seconds : i64);
}