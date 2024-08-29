use std::fmt::{Debug, Display, Formatter};
use crate::structure::{PDate, Reader};

pub trait Format {
    fn format(&self, pattern : &str) -> String;
}

impl Format for PDate {
    fn format(&self, pattern: &str) -> String {
        pattern
            .replace("%Y",&self.year().to_string())
            .replace("%C",&self.year().to_string()[..2])
            .replace("%y",&self.year().to_string()[2..])
            .replace("%m",&format!("{:02}",self.month()))
            .replace("%B",&self.month_name())
            .replace("%d",&format!("{:02}",self.day()))
            .replace("%e",&format!("{:2}",self.day()))
            .replace("%A",&self.day_name())
            .replace("%w",&self.day_of_week().to_string())
            .replace("%U",&format!("{:02}",self.day_of_year() / 7))
            .replace("%j",&format!("{:03}",self.day_of_year()))
            .replace("%H",&format!("{:02}",self.hour()))
            .replace("%k",&format!("{:2}",self.hour()))
            .replace("%I",&format!("{:02}",self.hour_12()))
            .replace("%l",&format!("{:2}",self.hour_12()))
            .replace("%P",&self.time_of_day())
            .replace("%p",&self.short_time_of_day())
            .replace("%M",&format!("{:02}",self.minute()))
            .replace("%S",&format!("{:02}",self.second()))
            .replace("%f",&self.nano_second.to_string())
            .replace("%.f",&format!("{}.{}",self.second,self.nano_second))
            .replace("%:z",&self.format_timezone())
    }
}

impl Display for PDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.format("%Y-%m-%d %H:%M:%S %:z"))
    }
}

impl Debug for PDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.format("%Y-%m-%d %H:%M:%S %:z"))
    }
}
