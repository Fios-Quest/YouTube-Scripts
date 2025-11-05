#[derive(PartialEq, Debug)]
struct InvalidMonth;

fn get_english_month_name(month: u64) -> Result<String, InvalidMonth> {
    match month {
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
    let year: u64 = 2025;
    let month: u64 = 11;
    let day: u64 = 4;

    println!("year  {year:>4} -> {:?}", get_english_month_name(year));
    println!("month {month:>4} -> {:?}", get_english_month_name(month));
    println!("day   {day:>4} -> {:?}", get_english_month_name(day));
}
