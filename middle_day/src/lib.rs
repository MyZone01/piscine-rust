
pub use chrono::{Datelike, NaiveDate, Weekday as wd};

pub fn middle_day(year: i32) -> Option<wd> {
    let last_day = NaiveDate::from_ymd_opt(year, 12, 31)?;

    let days_in_year = last_day.ordinal();

    if days_in_year % 2 == 0 {
        None
    } else {
        let middle_day = NaiveDate::from_ymd_opt(year, 1, 1)?.checked_add_signed(chrono::Duration::days(days_in_year as i64 / 2))?;
        Some(middle_day.weekday())
    }
}

// pub fn middle_day(year:u32)->Option<wd>{
//    if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
//        None
//    } else {
//     let mid_day= (365/2 as i32) + 1;
//     let date = chrono::NaiveDate::from_ymd_opt(year as i32, 1, 1).unwrap() + chrono::Duration::days(mid_day as i64 - 1);
//     Some(date.weekday())
//    }
// }