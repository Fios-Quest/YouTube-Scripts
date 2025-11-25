mod month {
    #[repr(u64)]
    pub enum Month {
        January = 1,
        February = 2,
        March = 3,
        April = 4,
        May = 5,
        June = 6,
        July = 7,
        August = 8,
        September = 9,
        October = 10,
        November = 11,
        December = 12,
    }

    pub fn get_english_month_name(month: Month) -> String {
        match month {
            Month::January => "January".to_string(),
            Month::February => "February".to_string(),
            Month::March => "March".to_string(),
            Month::April => "April".to_string(),
            Month::May => "May".to_string(),
            Month::June => "June".to_string(),
            Month::July => "July".to_string(),
            Month::August => "August".to_string(),
            Month::September => "September".to_string(),
            Month::October => "October".to_string(),
            Month::November => "November".to_string(),
            Month::December => "December".to_string(),
        }
    }
}

use month::*;

fn main() {
    let month = Month::November;
    println!("{}", get_english_month_name(month));

    let _ = Month::January;
    let _ = Month::February;
    let _ = Month::March;
    let _ = Month::April;
    let _ = Month::May;
    let _ = Month::June;
    let _ = Month::July;
    let _ = Month::August;
    let _ = Month::September;
    let _ = Month::October;
    let _ = Month::November;
    let _ = Month::December;
}
