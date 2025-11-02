pub fn is_leap_year(year: u64) -> bool {
    match year % 100 {
        0 =>  matches!(year % 400, 0),
        _ => matches!(year % 4, 0),
    }
}
