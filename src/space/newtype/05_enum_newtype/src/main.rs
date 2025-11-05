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

    #[derive(Debug)]
    pub struct InvalidMonthNumber;

    impl Month {
        pub fn from_number(month: u64) -> Result<Month, InvalidMonthNumber> {
            match month {
                1 => Ok(Month::January),
                2 => Ok(Month::February),
                3 => Ok(Month::March),
                4 => Ok(Month::April),
                5 => Ok(Month::May),
                6 => Ok(Month::June),
                7 => Ok(Month::July),
                8 => Ok(Month::August),
                9 => Ok(Month::September),
                10 => Ok(Month::October),
                11 => Ok(Month::November),
                12 => Ok(Month::December),
                _ => Err(InvalidMonthNumber),
            }
        }
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
    let maybe_month = Month::from_number(0);
    assert!(maybe_month.is_err());

    let maybe_month = Month::from_number(9);
    assert!(maybe_month.is_ok());

    let month = Month::November;
    println!("{}", get_english_month_name(month))
}
