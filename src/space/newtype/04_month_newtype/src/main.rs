mod month {
    pub struct Month(u64);

    #[derive(Debug)]
    pub struct InvalidMonthNumber;

    impl Month {
        pub fn from_number(month: u64) -> Result<Month, InvalidMonthNumber> {
            if month < 1 || month > 12 {
                Err(InvalidMonthNumber)
            } else {
                Ok(Month(month))
            }
        }
    }

    pub fn get_english_month_name(month: Month) -> String {
        match month.0 {
            1 => "January".to_string(),
            2 => "February".to_string(),
            3 => "March".to_string(),
            4 => "April".to_string(),
            5 => "May".to_string(),
            6 => "June".to_string(),
            7 => "July".to_string(),
            8 => "August".to_string(),
            9 => "September".to_string(),
            10 => "October".to_string(),
            11 => "November".to_string(),
            12 => "December".to_string(),
            _ => panic!("Month with an invalid number should not be possible"),
        }
    }
}

use month::*;

fn main() {
    let maybe_month = Month::from_number(0);
    assert!(maybe_month.is_err());

    let maybe_month = Month::from_number(9);
    assert!(maybe_month.is_ok());

    let month = maybe_month.unwrap();
    println!("{}", get_english_month_name(month))
}
