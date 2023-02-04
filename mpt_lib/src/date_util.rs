use chrono::NaiveDate;
use chrono::{Datelike, Days, Months};

use crate::enums::{ClDateMoveAction, ClFrequency};

pub fn is_leap_year(year: i32) -> bool {
    return year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
}

pub fn last_day_of_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => panic!("invalid month: {}", month),
    }
}

pub fn from_int(n_date: u64, date: &mut NaiveDate) -> bool {
    let default_date = NaiveDate::from_ymd_opt(1900, 01, 01).unwrap();
    match default_date.checked_add_days(Days::new((n_date - 2).into())) {
        Some(new_date) => *date = new_date,
        _ => return false,
    }
    true
}

pub fn to_int(date: &NaiveDate) -> u64 {
    let default_date = NaiveDate::from_ymd_opt(1900, 01, 01).unwrap();
    (*date - default_date).num_days() as u64 + 2
}

pub fn to_week_begin(date: &mut NaiveDate) {
    *date = *date - Days::new((date.weekday().number_from_sunday() - 1).into());
}

pub fn to_month_begin(date: &mut NaiveDate) {
    *date = NaiveDate::from_ymd_opt(date.year(), date.month(), 1).unwrap();
}

pub fn to_quarter_begin(date: &mut NaiveDate) {
    let month = (date.month() + 2) / 3 * 3 - 2;
    *date = NaiveDate::from_ymd_opt(date.year(), month, 1).unwrap();
}

pub fn to_semi_annu_begin(date: &mut NaiveDate) {
    let month = (date.month() + 5) / 6 * 6 - 5;
    *date = NaiveDate::from_ymd_opt(date.year(), month, 1).unwrap();
}

pub fn to_year_begin(date: &mut NaiveDate) {
    *date = NaiveDate::from_ymd_opt(date.year(), 1, 1).unwrap();
}

pub fn to_period_begin(freq: ClFrequency, date: &mut NaiveDate) {
    match freq {
        ClFrequency::ClFrequencyWeekly => to_week_begin(date),
        ClFrequency::ClFrequencyMonthly => to_month_begin(date),
        ClFrequency::ClFrequencyQuarterly => to_quarter_begin(date),
        ClFrequency::ClFrequencySemiannually => to_semi_annu_begin(date),
        ClFrequency::ClFrequencyAnnually => to_year_begin(date),
        _ => (),
    }
}

pub fn to_week_end(date: &mut NaiveDate) {
    *date = *date + Days::new((7 - date.weekday().number_from_sunday()).into());
}

pub fn to_month_end(date: &mut NaiveDate) {
    *date = *date + Days::new((last_day_of_month(date.year(), date.month()) - date.day()).into());
}

pub fn to_quarter_end(date: &mut NaiveDate) {
    let month = (date.month() + 2) / 3 * 3;
    *date =
        NaiveDate::from_ymd_opt(date.year(), month, last_day_of_month(date.year(), month)).unwrap();
}

pub fn to_semi_annu_end(date: &mut NaiveDate) {
    let month = (date.month() + 5) / 6 * 6;
    *date =
        NaiveDate::from_ymd_opt(date.year(), month, last_day_of_month(date.year(), month)).unwrap();
}

pub fn to_year_end(date: &mut NaiveDate) {
    *date = NaiveDate::from_ymd_opt(date.year(), 12, last_day_of_month(date.year(), 12)).unwrap();
}

pub fn to_period_end(freq: ClFrequency, date: &mut NaiveDate) {
    match freq {
        ClFrequency::ClFrequencyWeekly => to_week_end(date),
        ClFrequency::ClFrequencyMonthly => to_month_end(date),
        ClFrequency::ClFrequencyQuarterly => to_quarter_end(date),
        ClFrequency::ClFrequencySemiannually => to_semi_annu_end(date),
        ClFrequency::ClFrequencyAnnually => to_year_end(date),
        _ => (),
    }
}

pub fn to_period_begin_int(freq: ClFrequency, n_date: u64) -> u64 {
    let mut naive_date = NaiveDate::default();
    from_int(n_date, &mut naive_date);
    to_period_begin(freq, &mut naive_date);
    to_int(&naive_date)
}

pub fn is_weekend(n_date: u64) -> bool {
    let mut naive_date = NaiveDate::default();
    from_int(n_date, &mut naive_date);
    naive_date.weekday().number_from_sunday() == 7 || naive_date.weekday().number_from_sunday() == 1
}

pub fn to_period_end_int(freq: ClFrequency, n_date: u64) -> u64 {
    let mut naive_date = NaiveDate::default();
    from_int(n_date, &mut naive_date);
    to_period_end(freq, &mut naive_date);
    to_int(&naive_date)
}

pub fn to_n_period_begin_int(freq: ClFrequency, n: i32, n_date: u64) -> u64 {
    let mut naive_date = NaiveDate::default();
    from_int(n_date, &mut naive_date);
    to_n_period(freq, n, ClDateMoveAction::ClMoveToBegin, &mut naive_date);
    to_int(&naive_date)
}

pub fn to_n_period_end_int(freq: ClFrequency, n: i32, n_date: u64) -> u64 {
    let mut naive_date = NaiveDate::default();
    from_int(n_date, &mut naive_date);
    to_n_period(freq, n, ClDateMoveAction::ClMoveToEnd, &mut naive_date);
    to_int(&naive_date)
}

pub fn to_n_period_int(freq: ClFrequency, n: i32, n_date: u64) -> u64 {
    let mut naive_date = NaiveDate::default();
    from_int(n_date, &mut naive_date);
    to_n_period(freq, n, ClDateMoveAction::ClNotMove, &mut naive_date);
    to_int(&naive_date)
}

pub fn to_n_period(
    freq: ClFrequency,
    n: i32,
    action: ClDateMoveAction,
    date: &mut NaiveDate,
) -> bool {
    match freq {
        ClFrequency::ClFrequencyDaily => {
            if n > 0 {
                match date.checked_add_days(Days::new(n as u64)) {
                    Some(new_date) => *date = new_date,
                    _ => return false,
                }
            } else {
                let m = n * (-1);
                match date.checked_sub_days(Days::new(m as u64)) {
                    Some(new_date) => *date = new_date,
                    _ => return false,
                }
            }
        }
        ClFrequency::ClFrequencyWeekly => {
            if n > 0 {
                match date.checked_add_days(Days::new(n as u64 * 7)) {
                    Some(new_date) => *date = new_date,
                    _ => return false,
                }
            } else {
                let m = n * (-1);
                match date.checked_sub_days(Days::new(m as u64 * 7)) {
                    Some(new_date) => *date = new_date,
                    _ => return false,
                }
            }
            match action {
                ClDateMoveAction::ClMoveToEnd => to_week_end(date),
                ClDateMoveAction::ClMoveToBegin => to_week_begin(date),
                _ => (),
            }
        }
        ClFrequency::ClFrequencyMonthly => {
            if n > 0 {
                match date.checked_add_months(Months::new(n as u32)) {
                    Some(new_date) => *date = new_date,
                    _ => return false,
                }
            } else {
                let m = n * (-1);
                match date.checked_sub_months(Months::new(m as u32)) {
                    Some(new_date) => *date = new_date,
                    _ => return false,
                }
            }
            match action {
                ClDateMoveAction::ClMoveToEnd => to_month_end(date),
                ClDateMoveAction::ClMoveToBegin => to_month_begin(date),
                _ => (),
            }
        }
        ClFrequency::ClFrequencyQuarterly => {
            if n > 0 {
                match date.checked_add_months(Months::new(n as u32 * 3)) {
                    Some(new_date) => *date = new_date,
                    _ => return false,
                }
            } else {
                let m = n * (-1);
                match date.checked_sub_months(Months::new(m as u32 * 3)) {
                    Some(new_date) => *date = new_date,
                    _ => return false,
                }
            }

            match action {
                ClDateMoveAction::ClMoveToEnd => to_quarter_end(date),
                ClDateMoveAction::ClMoveToBegin => to_quarter_begin(date),
                _ => (),
            }
        }
        ClFrequency::ClFrequencySemiannually => {
            if n > 0 {
                match date.checked_add_months(Months::new(n as u32 * 6)) {
                    Some(new_date) => *date = new_date,
                    _ => return false,
                }
            } else {
                let m = n * (-1);
                match date.checked_sub_months(Months::new(m as u32 * 6)) {
                    Some(new_date) => *date = new_date,
                    _ => return false,
                }
            }
            match action {
                ClDateMoveAction::ClMoveToEnd => to_semi_annu_end(date),
                ClDateMoveAction::ClMoveToBegin => to_semi_annu_begin(date),
                _ => (),
            }
        }
        ClFrequency::ClFrequencyAnnually => {
            if n > 0 {
                match date.checked_add_months(Months::new(n as u32 * 12)) {
                    Some(new_date) => *date = new_date,
                    _ => return false,
                }
            } else {
                let m = n * (-1);
                match date.checked_sub_months(Months::new(m as u32 * 12)) {
                    Some(new_date) => *date = new_date,
                    _ => return false,
                }
            }
            match action {
                ClDateMoveAction::ClMoveToEnd => to_year_end(date),
                ClDateMoveAction::ClMoveToBegin => to_year_begin(date),
                _ => (),
            }
        }
        _ => (),
    }
    return true;
}

#[cfg(test)]
mod test {
    use chrono::{Datelike, NaiveDate};

    use crate::{
        date_util::{
            self, to_month_end, to_period_end, to_quarter_end, to_semi_annu_end, to_week_end,
            to_year_end,
        },
        enums::{ClDateMoveAction, ClFrequency},
    };

    use super::to_n_period;
    #[test]
    fn should_correct_from_int() {
        let mut dt = NaiveDate::default();
        date_util::from_int(44743, &mut dt);
        assert_eq!(dt.year() == 2022 && dt.month() == 7 && dt.day() == 1, true);
    }
    #[test]
    fn should_correct_to_int() {
        let dt = NaiveDate::from_ymd_opt(2022, 10, 1).unwrap();
        assert_eq!(date_util::to_int(&dt) == 44835, true);
    }
    #[test]
    fn should_to_weekend() {
        let mut dt = NaiveDate::from_ymd_opt(2022, 11, 1).unwrap();
        to_week_end(&mut dt);
        assert_eq!(dt.year() == 2022 && dt.month() == 11 && dt.day() == 5, true);
    }
    #[test]
    fn should_to_quarterend() {
        let mut dt = NaiveDate::from_ymd_opt(2022, 7, 1).unwrap();
        to_quarter_end(&mut dt);
        assert_eq!(dt.year() == 2022 && dt.month() == 9 && dt.day() == 30, true);
    }
    #[test]
    fn should_to_monthend() {
        let mut dt = NaiveDate::from_ymd_opt(2022, 11, 1).unwrap();
        to_month_end(&mut dt);
        assert_eq!(
            dt.year() == 2022 && dt.month() == 11 && dt.day() == 30,
            true
        );
    }
    #[test]
    fn should_to_yearend() {
        let mut dt = NaiveDate::from_ymd_opt(2022, 11, 1).unwrap();
        to_year_end(&mut dt);
        assert_eq!(
            dt.year() == 2022 && dt.month() == 12 && dt.day() == 31,
            true
        );
    }
    #[test]
    fn should_to_semi_annu_end() {
        let mut dt = NaiveDate::from_ymd_opt(2022, 1, 1).unwrap();
        to_semi_annu_end(&mut dt);
        assert_eq!(dt.year() == 2022 && dt.month() == 6 && dt.day() == 30, true);
    }

    #[test]
    fn should_to_period_end() {
        let mut dt_week = NaiveDate::from_ymd_opt(2022, 11, 1).unwrap();
        to_period_end(ClFrequency::ClFrequencyWeekly, &mut dt_week);
        assert_eq!(
            dt_week.year() == 2022 && dt_week.month() == 11 && dt_week.day() == 5,
            true
        );

        let mut dt_month = NaiveDate::from_ymd_opt(2022, 11, 1).unwrap();
        to_period_end(ClFrequency::ClFrequencyMonthly, &mut dt_month);
        assert_eq!(
            dt_month.year() == 2022 && dt_month.month() == 11 && dt_month.day() == 30,
            true
        );

        let mut dt_quarter = NaiveDate::from_ymd_opt(2022, 7, 1).unwrap();
        to_period_end(ClFrequency::ClFrequencyQuarterly, &mut dt_quarter);
        assert_eq!(
            dt_quarter.year() == 2022 && dt_quarter.month() == 9 && dt_quarter.day() == 30,
            true
        );

        let mut dt_semi_annu = NaiveDate::from_ymd_opt(2022, 1, 1).unwrap();
        to_period_end(ClFrequency::ClFrequencySemiannually, &mut dt_semi_annu);
        assert_eq!(
            dt_semi_annu.year() == 2022 && dt_semi_annu.month() == 6 && dt_semi_annu.day() == 30,
            true
        );

        let mut dt_year = NaiveDate::from_ymd_opt(2022, 11, 1).unwrap();
        to_period_end(ClFrequency::ClFrequencyAnnually, &mut dt_year);
        assert_eq!(
            dt_year.year() == 2022 && dt_year.month() == 12 && dt_year.day() == 31,
            true
        );
    }

    #[test]
    fn should_to_weekbegin() {
        let mut dt = NaiveDate::from_ymd_opt(2022, 11, 1).unwrap();
        date_util::to_week_begin(&mut dt);
        assert_eq!(
            dt.year() == 2022 && dt.month() == 10 && dt.day() == 30,
            true
        );
    }
    #[test]
    fn should_to_monthbegin() {
        let mut dt = NaiveDate::from_ymd_opt(2022, 11, 15).unwrap();
        date_util::to_month_begin(&mut dt);
        assert_eq!(dt.year() == 2022 && dt.month() == 11 && dt.day() == 1, true);
    }
    #[test]
    fn should_to_quarterbegin() {
        let mut dt = NaiveDate::from_ymd_opt(2022, 9, 15).unwrap();
        date_util::to_quarter_begin(&mut dt);
        assert_eq!(dt.year() == 2022 && dt.month() == 7 && dt.day() == 1, true);
    }

    #[test]
    fn should_to_semi_annu_begin() {
        let mut dt = NaiveDate::from_ymd_opt(2022, 11, 1).unwrap();
        date_util::to_semi_annu_begin(&mut dt);
        assert_eq!(dt.year() == 2022 && dt.month() == 7 && dt.day() == 1, true);
    }

    #[test]
    fn should_to_yearbegin() {
        let mut dt = NaiveDate::from_ymd_opt(2022, 11, 1).unwrap();
        date_util::to_year_begin(&mut dt);
        assert_eq!(dt.year() == 2022 && dt.month() == 1 && dt.day() == 1, true);
    }

    #[test]
    fn should_to_period_begin() {
        let mut dt_week = NaiveDate::from_ymd_opt(2022, 11, 1).unwrap();
        date_util::to_period_begin(ClFrequency::ClFrequencyWeekly, &mut dt_week);
        assert_eq!(
            dt_week.year() == 2022 && dt_week.month() == 10 && dt_week.day() == 30,
            true
        );

        let mut dt_month = NaiveDate::from_ymd_opt(2022, 11, 15).unwrap();
        date_util::to_period_begin(ClFrequency::ClFrequencyMonthly, &mut dt_month);
        assert_eq!(
            dt_month.year() == 2022 && dt_month.month() == 11 && dt_month.day() == 1,
            true
        );

        let mut dt_quarter = NaiveDate::from_ymd_opt(2022, 9, 15).unwrap();
        date_util::to_period_begin(ClFrequency::ClFrequencyQuarterly, &mut dt_quarter);
        assert_eq!(
            dt_quarter.year() == 2022 && dt_quarter.month() == 7 && dt_quarter.day() == 1,
            true
        );

        let mut dt_semi_annu = NaiveDate::from_ymd_opt(2022, 11, 1).unwrap();
        date_util::to_period_begin(ClFrequency::ClFrequencySemiannually, &mut dt_semi_annu);
        assert_eq!(
            dt_semi_annu.year() == 2022 && dt_semi_annu.month() == 7 && dt_semi_annu.day() == 1,
            true
        );

        let mut dt_year = NaiveDate::from_ymd_opt(2022, 11, 1).unwrap();
        date_util::to_period_begin(ClFrequency::ClFrequencyAnnually, &mut dt_year);
        assert_eq!(
            dt_year.year() == 2022 && dt_year.month() == 1 && dt_year.day() == 1,
            true
        );
    }
    #[test]
    fn should_handle_exception_to_n_period() {
        let mut dt = NaiveDate::from_ymd_opt(2022, 12, 1).unwrap();
        let ret = to_n_period(
            ClFrequency::ClFrequencyMonthly,
            -10000000,
            ClDateMoveAction::ClMoveToEnd,
            &mut dt,
        );
        assert_eq!(ret, false)
    }

    #[test]
    fn should_handle_leap_year_top_n_period() {
        let mut dt = NaiveDate::from_ymd_opt(2020, 1, 31).unwrap();
        to_n_period(
            ClFrequency::ClFrequencyMonthly,
            1,
            ClDateMoveAction::ClNotMove,
            &mut dt,
        );
        assert_eq!(dt.year() == 2020 && dt.month() == 2 && dt.day() == 29, true)
    }
    #[test]
    fn should_to_n_period() {
        let mut dt = NaiveDate::from_ymd_opt(2022, 7, 1).unwrap();
        let mut ret = to_n_period(
            ClFrequency::ClFrequencyDaily,
            63,
            ClDateMoveAction::ClMoveToEnd,
            &mut dt,
        );
        assert_eq!(
            ret == true && dt.year() == 2022 && dt.month() == 9 && dt.day() == 2,
            true
        );

        dt = NaiveDate::from_ymd_opt(2022, 7, 1).unwrap();
        ret = to_n_period(
            ClFrequency::ClFrequencyWeekly,
            -3,
            ClDateMoveAction::ClMoveToEnd,
            &mut dt,
        );
        assert_eq!(
            ret == true && dt.year() == 2022 && dt.month() == 6 && dt.day() == 11,
            true
        );

        dt = NaiveDate::from_ymd_opt(2022, 7, 1).unwrap();
        to_n_period(
            ClFrequency::ClFrequencyWeekly,
            3,
            ClDateMoveAction::ClMoveToBegin,
            &mut dt,
        );
        assert_eq!(dt.year() == 2022 && dt.month() == 7 && dt.day() == 17, true);

        dt = NaiveDate::from_ymd_opt(2022, 7, 1).unwrap();
        to_n_period(
            ClFrequency::ClFrequencyWeekly,
            3,
            ClDateMoveAction::ClNotMove,
            &mut dt,
        );
        assert_eq!(dt.year() == 2022 && dt.month() == 7 && dt.day() == 22, true);

        dt = NaiveDate::from_ymd_opt(2022, 7, 15).unwrap();
        to_n_period(
            ClFrequency::ClFrequencyMonthly,
            3,
            ClDateMoveAction::ClMoveToBegin,
            &mut dt,
        );
        assert_eq!(dt.year() == 2022 && dt.month() == 10 && dt.day() == 1, true);

        dt = NaiveDate::from_ymd_opt(2022, 7, 15).unwrap();
        to_n_period(
            ClFrequency::ClFrequencyMonthly,
            -3,
            ClDateMoveAction::ClMoveToEnd,
            &mut dt,
        );
        assert_eq!(dt.year() == 2022 && dt.month() == 4 && dt.day() == 30, true);

        dt = NaiveDate::from_ymd_opt(2022, 7, 15).unwrap();
        to_n_period(
            ClFrequency::ClFrequencyMonthly,
            -3,
            ClDateMoveAction::ClNotMove,
            &mut dt,
        );
        assert_eq!(dt.year() == 2022 && dt.month() == 4 && dt.day() == 15, true);

        dt = NaiveDate::from_ymd_opt(2022, 7, 15).unwrap();
        to_n_period(
            ClFrequency::ClFrequencyQuarterly,
            -3,
            ClDateMoveAction::ClMoveToEnd,
            &mut dt,
        );
        assert_eq!(
            dt.year() == 2021 && dt.month() == 12 && dt.day() == 31,
            true
        );

        dt = NaiveDate::from_ymd_opt(2022, 7, 15).unwrap();
        to_n_period(
            ClFrequency::ClFrequencyQuarterly,
            1,
            ClDateMoveAction::ClMoveToBegin,
            &mut dt,
        );
        assert_eq!(dt.year() == 2022 && dt.month() == 10 && dt.day() == 1, true);

        dt = NaiveDate::from_ymd_opt(2022, 7, 15).unwrap();
        to_n_period(
            ClFrequency::ClFrequencyQuarterly,
            -1,
            ClDateMoveAction::ClNotMove,
            &mut dt,
        );
        assert_eq!(dt.year() == 2022 && dt.month() == 4 && dt.day() == 15, true);

        dt = NaiveDate::from_ymd_opt(2022, 7, 15).unwrap();
        to_n_period(
            ClFrequency::ClFrequencySemiannually,
            -2,
            ClDateMoveAction::ClMoveToBegin,
            &mut dt,
        );
        assert_eq!(dt.year() == 2021 && dt.month() == 7 && dt.day() == 1, true);

        dt = NaiveDate::from_ymd_opt(2020, 7, 15).unwrap();
        to_n_period(
            ClFrequency::ClFrequencySemiannually,
            2,
            ClDateMoveAction::ClMoveToEnd,
            &mut dt,
        );
        assert_eq!(
            dt.year() == 2021 && dt.month() == 12 && dt.day() == 31,
            true
        );
        dt = NaiveDate::from_ymd_opt(2020, 7, 15).unwrap();
        to_n_period(
            ClFrequency::ClFrequencySemiannually,
            1,
            ClDateMoveAction::ClNotMove,
            &mut dt,
        );
        assert_eq!(dt.year() == 2021 && dt.month() == 1 && dt.day() == 15, true);

        dt = NaiveDate::from_ymd_opt(2020, 7, 15).unwrap();
        to_n_period(
            ClFrequency::ClFrequencyAnnually,
            2,
            ClDateMoveAction::ClMoveToEnd,
            &mut dt,
        );
        assert_eq!(
            dt.year() == 2022 && dt.month() == 12 && dt.day() == 31,
            true
        );

        dt = NaiveDate::from_ymd_opt(2020, 7, 15).unwrap();
        to_n_period(
            ClFrequency::ClFrequencyAnnually,
            -2,
            ClDateMoveAction::ClMoveToBegin,
            &mut dt,
        );
        assert_eq!(dt.year() == 2018 && dt.month() == 1 && dt.day() == 1, true);

        dt = NaiveDate::from_ymd_opt(2020, 7, 15).unwrap();
        to_n_period(
            ClFrequency::ClFrequencyAnnually,
            -2,
            ClDateMoveAction::ClNotMove,
            &mut dt,
        );
        assert_eq!(dt.year() == 2018 && dt.month() == 7 && dt.day() == 15, true);
    }
}
