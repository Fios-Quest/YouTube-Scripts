#[derive(PartialEq, Debug)]
struct InvalidMonth;

#[derive(Copy, Clone, Debug)]
struct Year(u64);

#[derive(Copy, Clone, Debug)]
struct Month(u64);

#[derive(Copy, Clone, Debug)]
struct Day(u64);

fn get_english_month_name(month: Month) -> Result<String, InvalidMonth> {
    match month.0 {
        1 => Ok("January".to_string()),
        2 => Ok("February".to_string()),
        3 => Ok("March".to_string()),
        4 => Ok("April".to_string()),
        5 => Ok("May".to_string()),
        6 => Ok("June".to_string()),
        7 => Ok("July".to_string()),
        8 => Ok("August".to_string()),
        9 => Ok("September".to_string()),
        10 => Ok("October".to_string()),
        11 => Ok("November".to_string()),
        12 => Ok("December".to_string()),
        _ => Err(InvalidMonth),
    }
}

fn main() {
    let month = Month(11);
    assert_eq!(get_english_month_name(month), Ok("November".to_string()));

    // let year = Year(2025);
    // let day = Day(4);
    // assert_eq!(get_english_month_name(year), Err(InvalidMonth));
    // assert_eq!(get_english_month_name(day), Err(InvalidMonth));
}
