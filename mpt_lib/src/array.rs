use std::cmp::Ordering;

use chrono::NaiveDate;

use crate::{
    date_util,
    enums::{self, Errors},
    MPTCalculator,
};
//use chrono::{Duration, NaiveDate};
use std::ops::ControlFlow;

impl<'a> MPTCalculator<'a> {
    ///calc the max values in an arrays not include NA/INF values
    ///
    ///# Examples
    ///```
    /// use mpt_lib::MPTCalculator;
    /// use mpt_lib::enums::{self, Errors};
    ///
    ///let data = vec![
    ///    4.487083398,1.651309218,0.912195263,2.995680315,9.98454554,
    ///    2.32625796,8.175311666,7.566763025,5.010273527,1.479493882,
    ///];
    ///let mpt = MPTCalculator::from_v(&data);
    ///let mut res = f64::NAN;
    ///let err = mpt.max(&mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 9.98454554),
    ///    true
    ///);
    ///```
    pub fn max(&self, max_value: &mut f64) -> enums::Errors {
        *max_value =
            self.values.iter().fold(
                f64::NEG_INFINITY,
                |a, &x| if x.is_finite() { x.max(a) } else { a },
            );
        return Errors::ClErrorCodeNoError;
    }

    fn find_double(
        &self,
        start_date: i32,
        end_date: i32,
        dates: &[i32],
        pos: &mut i32,
        cmp_fn: fn(i32, i32) -> bool,
        init_fn: fn() -> i32,
    ) -> enums::Errors {
        *pos = -1;
        let mut benchmark = init_fn();
        self.values.iter().enumerate().for_each(|v| {
            if v.1.is_finite()
                && dates[v.0] >= start_date
                && dates[v.0] <= end_date
                && cmp_fn(dates[v.0], benchmark)
            {
                benchmark = dates[v.0];
                *pos = v.0 as i32;
            }
        });
        return Errors::ClErrorCodeNoError;
    }

    /// find first valid double and its date in an array between start date and end date not include NA/INF values
    /// if not find anything, pos is -1
    /// else pos is the first pos
    ///
    ///# Arguments
    /// start_date: the start date for calculation period
    ///
    /// end_end: the end date for calculation period
    ///
    /// dates: the dates array, the every date mapping to the value in the values
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///f64::NAN,1.651309218,0.912195263,2.995680315,9.98454554,
    ///2.32625796,8.175311666,7.566763025,5.010273527,1.479493882,
    ///];
    ///let dates = vec![
    ///    39445, 39801, 39817, 39522, 39514, 39602, 39760, 39493, 39686, 39752,
    ///];
    ///let mpt = MPTCalculator::from_v(&data);
    ///let mut res = 0;
    ///let err = mpt.first_double(39445, 39752, &dates, &mut res);
    ///assert_eq!(err == Errors::ClErrorCodeNoError && res == 7, true);
    ///```
    pub fn first_double(
        &self,
        start_date: i32,
        end_date: i32,
        dates: &[i32],
        pos: &mut i32,
    ) -> enums::Errors {
        *pos = -1;
        let mut benchmark = end_date;
        self.values.iter().enumerate().for_each(|v| {
            if v.1.is_finite()
                && dates[v.0] >= start_date
                && dates[v.0] <= end_date
                && dates[v.0] <= benchmark
            {
                benchmark = dates[v.0];
                *pos = v.0 as i32;
            }
        });
        return Errors::ClErrorCodeNoError;
    }

    /// find last valid double and its date in an array between start date and end date
    /// if not find anything, pos is -1
    /// else pos is the first pos
    ///
    ///# Arguments
    /// start_date: the start date for calculation period
    ///
    /// end_end: the end date for calculation period
    ///
    /// dates: the dates array, the every date mapping to the value in the values
    ///
    /// # Examples
    ///```
    /// use mpt_lib::MPTCalculator;
    /// use mpt_lib::enums::{self, Errors};
    ///
    ///let data = vec![
    ///    4.487083398,1.651309218,0.912195263,2.995680315,9.98454554,
    ///    2.32625796,8.175311666,7.566763025,5.010273527,1.479493882,
    ///];
    ///let dates = vec![
    ///    39445, 39801, 39817, 39522, 39514, 39602, 39760, 39493, 39686, 39752,
    ///];
    ///let mpt = MPTCalculator::from_v(&data);
    ///let mut res = 0;
    ///let err = mpt.last_double(39445, 39752, &dates, &mut res);
    ///assert_eq!(err == Errors::ClErrorCodeNoError && res == 9, true);
    ///```
    pub fn last_double(
        &self,
        start_date: i32,
        end_date: i32,
        dates: &[i32],
        pos: &mut i32,
    ) -> enums::Errors {
        *pos = -1;
        let mut benchmark = start_date;
        self.values.iter().enumerate().for_each(|v| {
            if v.1.is_finite()
                && dates[v.0] >= start_date
                && dates[v.0] <= end_date
                && dates[v.0] >= benchmark
            {
                benchmark = dates[v.0];
                *pos = v.0 as i32;
            }
        });
        return Errors::ClErrorCodeNoError;
    }

    pub fn rate_of_change(&self, rate_of_change: &mut [f64]) -> Errors {
        let mut ret: Vec<f64> = self
            .values
            .iter()
            .scan(0.0, |st, &x| {
                let res = (*st - x).abs();
                *st = x;
                Some(res)
            })
            .skip(1)
            .collect();

        ret.swap_with_slice(rate_of_change);
        return Errors::ClErrorCodeNoError;
    }

    pub fn count(
        values1: &[f64],
        values2: &[f64],
        values3: &[f64],
        value_array_size: i32,
        count: &mut i32,
    ) -> Errors {
        let mut count_inter = 0 as usize;
        for i in 0..value_array_size as usize {
            if (values1.len() == 0 || values1[i].is_finite())
                && (values2.len() == 0 || values2[i].is_finite())
                && (values3.len() == 0 || values3[i].is_finite())
            {
                count_inter = count_inter + 1;
            }
        }
        *count = count_inter as i32;

        return Errors::ClErrorCodeNoError;
    }
    fn calc_in_period<T>(
        &self,
        start_date: i32,
        end_date: i32,
        dates: &[i32],
        calc_fn: fn(&mut T, f64) -> T,
        int_fn: fn() -> T,
        res: &mut T,
    ) -> enums::Errors {
        *res = int_fn();
        self.values.iter().enumerate().for_each(|v| {
            if v.1.is_finite() && dates[v.0] >= start_date && dates[v.0] <= end_date {
                calc_fn(&mut *res, *v.1);
            }
        });

        return Errors::ClErrorCodeNoError;
    }

    /// get the numbers in an array between start date and end date and not inculde NA/INF
    /// # Examples
    ///```
    /// use mpt_lib::MPTCalculator;
    /// use mpt_lib::enums::{self, Errors};
    ///
    ///let data = vec![
    ///    8.6397889, 7.9137227, 10.847360, 11.727434, 12.136135, 8.0826517, 12.006847, 13.789638,
    ///    9.4216315, 9.3328426, 9.9971608, 7.2958290,
    ///];
    ///let dates = vec![
    ///    39478, 39507, 39538, 39568, 39599, 39629, 39660, 39691, 39721, 39752, 39782, 39813,
    ///];
    ///let mpt = MPTCalculator::from_v(&data);
    ///let mut res = 0;
    ///let err = mpt.count_in_period(39478, 39813, &dates, &mut res);
    ///assert_eq!(err == Errors::ClErrorCodeNoError && res == 12, true);
    ///```
    pub fn count_in_period(
        &self,
        start_date: i32,
        end_date: i32,
        dates: &[i32],
        count: &mut i32,
    ) -> enums::Errors {
        return self.calc_in_period::<i32>(
            start_date,
            end_date,
            dates,
            |a, _| {
                *a += 1;
                *a
            },
            || 0,
            &mut *count,
        );
    }
    /// get the sum for an array between start date and end date and not inculde NA/INF
    /// # Arguments
    /// start_date: the start date for calculation period
    ///
    /// end_end: the end date for calculation period
    ///
    /// dates: the dates array, the every date mapping to the value in the values
    ///
    /// # Examples
    ///```
    /// use mpt_lib::MPTCalculator;
    /// use mpt_lib::enums::{self, Errors};
    ///
    ///let data = vec![
    ///    8.6397889, 7.9137227, 10.847360, 11.727434, 12.136135, 8.0826517, 12.006847, 13.789638,
    ///    9.4216315, 9.3328426, 9.9971608, 7.2958290,
    ///];
    ///let dates = vec![
    ///    39478, 39507, 39538, 39568, 39599, 39629, 39660, 39691, 39721, 39752, 39782, 39813,
    ///];
    ///let mpt = MPTCalculator::from_v(&data);
    ///let mut res = f64::NAN;
    ///let err = mpt.sum_in_period(39478, 39813, &dates, &mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 121.1910412),
    ///    true
    ///);
    ///```
    pub fn sum_in_period(
        &self,
        start_date: i32,
        end_date: i32,
        dates: &[i32],
        sum: &mut f64,
    ) -> enums::Errors {
        return self.calc_in_period::<f64>(
            start_date,
            end_date,
            dates,
            |a, b| {
                (*a) += b;
                *a
            },
            || 0.0,
            &mut *sum,
        );
    }
    /// get the max of values and its date in an array between start date and end date and not inculde NA/INF
    /// # Arguments
    /// start_date: the start date for calculation period
    ///
    /// end_end: the end date for calculation period
    ///
    /// dates: the dates array, the every date mapping to the value in the values
    /// # Examples
    ///```
    /// use mpt_lib::MPTCalculator;
    /// use mpt_lib::enums::{self, Errors};
    ///
    ///let data = vec![
    ///    8.6397889, 7.9137227, 10.847360, 11.727434, 12.136135, 8.0826517, 12.006847, 13.789638,
    ///    9.4216315, 9.3328426, 9.9971608, 7.2958290,
    ///];
    ///let dates = vec![
    ///    39478, 39507, 39538, 39568, 39599, 39629, 39660, 39691, 39721, 39752, 39782, 39813,
    ///];
    ///let mpt = MPTCalculator::from_v(&data);
    ///let mut res = f64::NAN;
    ///let mut max_date = 0;
    ///let err = mpt.max_in_period(39478, 39813, &dates, &mut res, &mut max_date);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && res == 13.789638 && max_date == 39691,
    ///    true
    ///);
    ///```
    pub fn max_in_period(
        &self,
        start_date: i32,
        end_date: i32,
        dates: &[i32],
        max: &mut f64,
        date: &mut i32,
    ) -> enums::Errors {
        *max = f64::NEG_INFINITY;

        self.values.iter().enumerate().for_each(|v| {
            if v.1.is_finite() && dates[v.0] >= start_date && dates[v.0] <= end_date {
                if *v.1 > *max {
                    *max = *v.1;
                    *date = dates[v.0];
                }
            }
        });

        return Errors::ClErrorCodeNoError;
    }
    /// get the minimum in an array and its date between start date and end date and not inculde NA/INF
    /// # Arguments
    /// start_date: the start date for calculation period
    ///
    /// end_end: the end date for calculation period
    ///
    /// dates: the dates array, the every date mapping to the value in the values
    /// # Examples
    ///```
    /// use mpt_lib::MPTCalculator;
    /// use mpt_lib::enums::{self, Errors};
    ///
    ///let data = vec![
    ///    8.6397889, 7.9137227, 10.847360, 11.727434, 12.136135, 8.0826517, 12.006847, 13.789638,
    ///    9.4216315, 9.3328426, 9.9971608, 7.2958290,
    ///];
    ///let dates = vec![
    ///    39478, 39507, 39538, 39568, 39599, 39629, 39660, 39691, 39721, 39752, 39782, 39813,
    ///];
    ///let mpt = MPTCalculator::from_v(&data);
    ///let mut res = f64::NAN;
    ///let mut min_date = 0;
    ///let err = mpt.min_in_period(39478, 39813, &dates, &mut res, &mut min_date);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 7.2958290) && min_date == 39813,
    ///    true
    ///);
    ///```
    pub fn min_in_period(
        &self,
        start_date: i32,
        end_date: i32,
        dates: &[i32],
        min: &mut f64,
        date: &mut i32,
    ) -> enums::Errors {
        *min = f64::INFINITY;
        self.values.iter().enumerate().for_each(|v| {
            if v.1.is_finite() && dates[v.0] >= start_date && dates[v.0] <= end_date {
                if *v.1 < *min {
                    *min = *v.1;
                    *date = dates[v.0];
                }
            }
        });
        return Errors::ClErrorCodeNoError;
    }

    /// get the minimum in an array and its date not inculde NA/INF
    /// # Examples
    ///```
    /// use mpt_lib::MPTCalculator;
    /// use mpt_lib::enums::{self, Errors};
    ///
    ///let data = vec![
    ///    8.6397889, 7.9137227, 10.847360, 11.727434, 12.136135, 8.0826517, 12.006847, 13.789638,
    ///    9.4216315, 9.3328426, 9.9971608, 7.2958290,
    ///];
    ///let mpt = MPTCalculator::from_v(&data);
    ///let mut res = f64::NAN;
    ///let err = mpt.min(&mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 7.2958290),
    ///    true
    ///);
    ///```
    pub fn min(&self, min_res: &mut f64) -> Errors {
        *min_res = self
            .values
            .iter()
            .filter(|x| (**x).is_finite())
            .fold(f64::INFINITY, |a, &x| x.min(a));

        return Errors::ClErrorCodeNoError;
    }

    /// get the sum for an array not inculde NA/INF
    /// # Examples
    ///```
    /// use mpt_lib::MPTCalculator;
    /// use mpt_lib::enums::{self, Errors};
    ///
    ///let data = vec![
    ///    8.6397889, 7.9137227, 10.847360, 11.727434, 12.136135, 8.0826517, 12.006847, 13.789638,
    ///    9.4216315, 9.3328426, 9.9971608, 7.2958290,
    ///];
    ///let mpt = MPTCalculator::from_v(&data);
    ///let mut res = f64::NAN;
    ///let err = mpt.sum(&mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res,  121.1910412),
    ///    true
    ///);
    ///```
    pub fn sum(&self, sum_res: &mut f64) -> Errors {
        *sum_res = self.values.iter().filter(|x| (**x).is_finite()).sum();
        return Errors::ClErrorCodeNoError;
    }

    /// calculate the weigted average value for an array not inculde NA/INF
    ///
    /// # Arguments
    /// weights: the weights which mapping to every value
    ///
    /// # Examples
    ///```
    /// use mpt_lib::MPTCalculator;
    /// use mpt_lib::enums::{self, Errors};
    ///
    ///let data = vec![
    ///    0.1018520000,0.1379310000,0.1322920000,-0.0384620000,0.0041670000,
    ///    0.1724510000,0.4257430000,0.2689870000,0.0540540000,1.0000000000,
    ///    0.3307080000,f64::NAN,f64::NAN,0.0884350000,-0.0467840000,
    ///    f64::NAN,0.3450980000,-0.0566040000,0.1000000000,
    ///];
    ///
    ///let weights = vec![
    ///    26035.5600000000,118.7985000000,4012.4600000000,1.4294000000,422.7000120000,
    ///    142074.0300000000,10214.1000000000,711.3405150000,35.5523990000,1.4975000000,
    ///    36.8974990000,f64::NAN,f64::NAN,1609.7000000000,197.4942930000,
    ///    f64::NAN,1618.0400000000,34.0756990000,7.9369000000,
    ///];
    ///let mpt = MPTCalculator::from_v(&data);
    ///let mut res = f64::NAN;
    ///let err = mpt.weighted_average(&weights, &mut res);
    ///assert_eq!(err, Errors::ClErrorCodeNoError);
    ///assert_eq!(MPTCalculator::is_eq_double(res, 0.1760653727), true);
    ///```
    pub fn weighted_average(&self, weights: &[f64], avg_res: &mut f64) -> Errors {
        let mut total = 0.0;
        let mut sum = 0.0;
        self.values.iter().enumerate().for_each(|v| {
            if v.1.is_finite() && weights[v.0].is_finite() {
                sum += v.1 * weights[v.0];
                total = total + weights[v.0];
            }
        });

        *avg_res = sum / total;
        // avg_vec.iter_mut().for_each(|x| *x = *x / total);
        return Errors::ClErrorCodeNoError;
    }

    pub fn average_for_ts(
        &self,
        start_date: i32,
        end_date: i32,
        dates: &[i32],
        avg_res: &mut f64,
    ) -> enums::Errors {
        *avg_res = f64::NEG_INFINITY;

        let mut total = 0.0;
        let mut count = 0;

        self.values.iter().enumerate().for_each(|v| {
            if v.1.is_finite() && dates[v.0] >= start_date && dates[v.0] <= end_date {
                total = total + v.1;
                count += 1;
            }
        });

        *avg_res = total / count as f64;
        return Errors::ClErrorCodeNoError;
    }

    /// get the average value for the difference frequence between start date and end date
    /// # Arguments
    /// start_date: the start date for calculation period
    ///
    /// end_end: the end date for calculation period
    ///
    /// freq: the caculation frequece
    ///
    /// dates: the date mapping to every value
    /// # Examples
    /// ```
    /// use mpt_lib::MPTCalculator;
    /// use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///    8.169969704, 17.53634366,5.344482043, 8.452863347,10.33126183,
    ///    12.7683848,10.67265227,0.973782619,17.43341129,7.793032244,
    ///    5.839265786,0.589181628,19.80643643,0.319166174,5.887448511,
    ///    12.20015787,13.45983609,10.02752314,14.23095689,6.187832093,
    ///    14.82583437,14.75803915,9.90970724,12.95576028,11.02894738,
    ///    7.56902044,7.865732487,12.08801651,13.4391796,11.06652432,
    ///    14.73725253,19.29441213,8.767635516,0.900297309,14.3543903,
    ///    7.172694944,3.227678772,16.11482084,6.041044308,6.189942603,
    ///    11.16458034,13.50079637,17.17207264,10.1869313,18.52252921,
    ///    16.34204845,12.98478245,16.96478255,17.00365216,13.34776382,
    ///    3.630459975,16.20671779,12.70671665,8.410486911,4.581037552,
    ///    6.045530479,1.850320217,17.61496548,13.61665776, 10.14668818,
    ///    9.9238987, 18.61926739,3.407462739,3.147958377,1.312209162,
    ///    17.77142914,4.835527897,11.21452525,15.90649149,11.14175699,
    ///    10.12361377,18.02892583,13.52113804,2.467934258,5.844192095,
    ///    11.06558362,1.964113557,1.100482004,3.83922735,11.447917,
    ///    0.545897803,15.3561911, 13.04722015,13.16691716,
    ///];
    ///let dates = vec![
    ///    39441, 39441, 39445, 39452, 39461, 39461, 39467, 39476, 39484, 39485, 39493, 39500,
    ///    39508, 39514, 39522, 39529, 39533, 39536, 39537, 39546, 39552, 39557, 39563, 39567,
    ///    39569, 39570, 39576, 39581, 39583, 39591, 39595, 39595, 39602, 39609, 39613, 39615,
    ///    39615, 39618, 39625, 39633, 39642, 39650, 39657, 39666, 39667, 39668, 39668, 39673,
    ///    39681, 39681, 39686, 39688, 39691, 39692, 39693, 39693, 39694, 39700, 39703, 39706,
    ///    39706, 39710, 39719, 39728, 39734, 39735, 39737, 39740, 39747, 39752, 39755, 39760,
    ///    39762, 39770, 39779, 39785, 39787, 39792, 39792, 39801, 39802, 39810, 39812, 39817,
    ///];
    ///
    ///let excpted_values = vec![
    ///    8.639788972, 7.913722736,10.84736073,11.72743463,12.13613567,
    ///    8.082651712,12.00684799,13.78963844,9.421631542,9.332842614,
    ///    9.9971608, 7.295829073,
    ///];
    ///let excpted_dates = vec![
    ///    39478, 39507, 39538, 39568, 39599, 39629, 39660, 39691, 39721, 39752, 39782, 39813,
    ///];
    ///let mut avg_values = Vec::new();
    ///let mut avg_dates = Vec::new();
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err = mpt.average_freq_for_ts(
    ///    39478,
    ///    39813,
    ///    enums::ClFrequency::ClFrequencyMonthly,
    ///    &dates,
    ///    &mut avg_values,
    ///    &mut avg_dates,
    ///);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && avg_dates == excpted_dates,
    ///    true
    ///);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double_array(&avg_values, &excpted_values),
    ///    true
    ///);
    ///```
    pub fn average_freq_for_ts(
        &self,
        start_date: i32,
        end_date: i32,
        freq: enums::ClFrequency,
        dates: &[i32],
        avg_values: &mut Vec<f64>,
        avg_dates: &mut Vec<i32>,
    ) -> enums::Errors {
        if dates.len() == 0
            || start_date > *dates.last().unwrap()
            || end_date < *dates.first().unwrap()
        {
            return Errors::ClErrorCodeInvalidPara;
        }

        let mut start_naive_date = NaiveDate::default();
        let mut end_naive_date = NaiveDate::default();
        if !date_util::from_int(start_date as u64, &mut start_naive_date)
            || !date_util::from_int(end_date as u64, &mut end_naive_date)
        {
            return Errors::ClErrorCodeInvalidPara;
        }

        let mut n_freq_start = start_date;
        n_freq_start = date_util::to_period_begin_int(freq, n_freq_start as u64) as i32;
        let mut freq_end = start_naive_date;
        date_util::to_period_end(freq, &mut freq_end);
        let mut n_freq_end = date_util::to_int(&freq_end) as i32;

        let mut freq_sum = 0.0;
        let mut freq_count = 0;
        for i in 0..self.values.len() {
            if dates[i] > end_date {
                break;
            }
            if dates[i] >= n_freq_start && dates[i] <= n_freq_end {
                freq_sum += self.values[i];
                freq_count += 1;
            }
            if dates[i] > n_freq_end {
                avg_values.push(freq_sum / freq_count as f64);
                avg_dates.push(n_freq_end);
                freq_count = 0;
                loop {
                    n_freq_start = n_freq_end + 1;
                    n_freq_end = date_util::to_period_end_int(freq, (n_freq_end + 1) as u64) as i32;
                    if dates[i] >= n_freq_start && dates[i] <= n_freq_end {
                        break;
                    } else {
                        avg_values.push(0.0);
                        avg_dates.push(n_freq_end);
                    }
                }
                if n_freq_end > end_date {
                    break;
                }
                freq_sum = self.values[i];
                freq_count = 1;
            }
        }
        //push the latest period
        if end_date == n_freq_end {
            avg_values.push(freq_sum / freq_count as f64);
            avg_dates.push(n_freq_end);
        }

        //start_navie_date -
        return Errors::ClErrorCodeNoError;
    }

    ///Add the values in same position for two arrays, the output is same size array
    ///Array = valueArray1 + valueArray2
    ///
    ///#Arguments:
    ///values2		    [in]	value array
    ///output		    [out]	added array, the array size is equal to valueArraySize
    ///
    ///# Examples
    ///```
    /// use mpt_lib::MPTCalculator;
    /// use mpt_lib::enums::{self, Errors};
    ///
    ///let data = vec![
    ///    4.487083398,1.651309218,0.912195263,2.995680315,9.98454554,
    ///    2.32625796,8.175311666,7.566763025,5.010273527,1.479493882,
    ///];
    ///
    ///let data2 = vec![
    ///    6.184582992,1.145629713,0.923955492,7.367172708,1.438963969,
    ///    2.163394174,0.20035163,1.022927912, 9.43590199,1.056781513,
    ///];
    ///
    ///const EXPECTED_RES: [f64; 10] = [
    ///    10.67166639,2.796938931,1.836150755,10.36285302,11.42350951,
    ///    4.489652134,8.375663296,8.589690937,14.44617552,2.536275396,
    ///];
    ///let mpt = MPTCalculator::from_v(&data);
    ///let mut res = [f64::NAN; 10];
    ///let err = mpt.array_addition(&data2, &mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double_array(&res, &EXPECTED_RES),
    ///    true
    ///);
    ///```
    pub fn array_addition(&self, values2: &[f64], output: &mut [f64]) -> Errors {
        self.values.iter().enumerate().for_each(|v| {
            if v.1.is_finite() && values2[v.0].is_finite() {
                output[v.0] = v.1 + values2[v.0];
            } else {
                output[v.0] = f64::NAN;
            }
        });
        return Errors::ClErrorCodeNoError;
    }

    ///Subtract the values in same position for two arrays, the output is same size array
    ///Array = valueArray1 - valueArray2
    ///
    ///#Arguments:
    ///values2		    [in]	value array
    ///output		    [out]	added array, the array size is equal to valueArraySize
    ///
    ///# Examples
    ///```
    /// use mpt_lib::MPTCalculator;
    /// use mpt_lib::enums::{self, Errors};
    ///
    ///let data = vec![
    ///    4.487083398,1.651309218,0.912195263,2.995680315,9.98454554,
    ///    2.32625796,8.175311666,7.566763025,5.010273527,1.479493882,
    ///];
    ///
    ///let data2 = vec![
    ///    6.184582992,1.145629713,0.923955492,7.367172708,1.438963969,
    ///    2.163394174,0.20035163,1.022927912, 9.43590199,1.056781513,
    ///];
    ///
    ///const EXPECTED_RES: [f64; 10] = [
    ///    -1.697499594,0.505679505,-0.011760229,-4.371492393,8.545581571,
    ///    0.162863785,7.974960036,6.543835114,-4.425628463,0.422712369,
    ///];
    ///let mpt = MPTCalculator::from_v(&data);
    ///let mut res = [f64::NAN; 10];
    ///let err = mpt.array_subtraction(&data2, &mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double_array(&res, &EXPECTED_RES),
    ///    true
    ///);
    ///```
    pub fn array_subtraction(&self, values2: &[f64], output: &mut [f64]) -> Errors {
        self.values.iter().enumerate().for_each(|v| {
            if v.1.is_finite() && values2[v.0].is_finite() {
                output[v.0] = v.1 - values2[v.0];
            } else {
                output[v.0] = f64::NAN;
            }
        });
        return Errors::ClErrorCodeNoError;
    }

    ///Multiply the values in same position for two arrays, the output is same size array
    ///Array = valueArray1 * valueArray2
    ///
    ///#Arguments:
    ///values2		    [in]	value array
    ///output		    [out]	added array, the array size is equal to valueArraySize
    ///
    ///Return Value:
    ///Return the error code
    ///
    ///# Examples
    ///```
    /// use mpt_lib::MPTCalculator;
    /// use mpt_lib::enums::{self, Errors};
    ///
    ///let data = vec![
    ///    4.487083398,1.651309218,0.912195263,2.995680315,9.98454554,
    ///    2.32625796,8.175311666,7.566763025,5.010273527,1.479493882,
    ///];
    ///
    ///let data2 = vec![
    ///    6.184582992,1.145629713,0.923955492,7.367172708,1.438963969,
    ///    2.163394174,0.20035163,1.022927912, 9.43590199,1.056781513,
    ///];
    ///
    ///const EXPECTED_RES: [f64; 10] = [
    ///    27.75073967,1.891788905,0.842827823,22.06969426,14.36740127,
    ///    5.032612918,1.637937015,7.740253099,47.27644994,1.563501784,
    ///];
    ///let mpt = MPTCalculator::from_v(&data);
    ///let mut res = [f64::NAN; 10];
    ///let err = mpt.array_multiplication(&data2, &mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double_array(&res, &EXPECTED_RES),
    ///    true
    ///);
    ///```
    pub fn array_multiplication(&self, values2: &[f64], output: &mut [f64]) -> Errors {
        self.values.iter().enumerate().for_each(|v| {
            if v.1.is_finite() && values2[v.0].is_finite() {
                output[v.0] = v.1 * values2[v.0];
            } else {
                output[v.0] = f64::NAN;
            }
        });
        return Errors::ClErrorCodeNoError;
    }

    ///Div the values in same position for two arrays, the output is same size array
    ///Array = valueArray1 / valueArray2
    ///
    ///# Examples
    ///```
    /// use mpt_lib::MPTCalculator;
    /// use mpt_lib::enums::{self, Errors};
    ///
    ///let data = vec![
    ///    4.487083398,1.651309218,0.912195263,2.995680315,9.98454554,
    ///    2.32625796,8.175311666,7.566763025,5.010273527,1.479493882,
    ///];
    ///
    ///let data2 = vec![
    ///    6.184582992,1.145629713,0.923955492,7.367172708,1.438963969,
    ///    2.163394174,0.20035163,1.022927912, 9.43590199,1.056781513,
    ///];
    ///
    ///const EXPECTED_RES: [f64; 10] = [
    ///    0.725527235,1.441398735,0.987271866,0.406625504,6.938704344,
    ///    1.075281605,40.80481741,7.397161559,0.530979819,1.399999777,
    ///];
    ///let mpt = MPTCalculator::from_v(&data);
    ///let mut res = [f64::NAN; 10];
    ///let err = mpt.array_division(&data2, &mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double_array(&res, &EXPECTED_RES),
    ///    true
    ///);
    ///```
    pub fn array_division(&self, values2: &[f64], output: &mut [f64]) -> Errors {
        self.values.iter().enumerate().for_each(|v| {
            if v.1.is_finite() && values2[v.0].is_finite() {
                output[v.0] = v.1 / values2[v.0];
            } else {
                output[v.0] = f64::NAN;
            }
        });
        return Errors::ClErrorCodeNoError;
    }
    ///calculate accumulate the value for an array, the output is same size array
    ///
    ///# Examples
    ///```
    /// use mpt_lib::MPTCalculator;
    /// use mpt_lib::enums::{self, Errors};
    ///
    ///let data = vec![
    ///    4.487083398,1.651309218,0.912195263,2.995680315,9.98454554,
    ///    2.32625796,8.175311666,7.566763025,5.010273527,1.479493882,
    ///];
    ///
    ///const EXPECTED_RES: [f64; 10] = [
    ///    4.487083398,6.138392616,7.050587879,10.04626819,20.03081373,
    ///    22.35707169,30.53238336,38.09914639,43.10941991,44.58891379,
    ///];
    ///let mpt = MPTCalculator::from_v(&data);
    ///let mut res = [f64::NAN; 10];
    ///let err = mpt.array_additive_accumulation(&mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double_array(&res, &EXPECTED_RES),
    ///    true
    ///);
    ///```
    pub fn array_additive_accumulation(&self, output: &mut [f64]) -> Errors {
        self.values.iter().enumerate().fold(0.0 as f64, |a, x| {
            let v = a + x.1;
            output[x.0] = v;
            v
        });

        return Errors::ClErrorCodeNoError;
    }
    ///calulate rescale the value for an array, the output is an same size array
    ///#Arguments:
    ///rescale_factor:	the rescale factor
    ///
    ///# Examples
    ///```
    /// use mpt_lib::MPTCalculator;
    /// use mpt_lib::enums::{self, Errors};
    ///
    ///let data = vec![
    ///    4.487083398,1.651309218,0.912195263,2.995680315,9.98454554,
    ///    2.32625796,8.175311666,7.566763025,5.010273527,1.479493882,
    ///];
    ///
    ///const EXPECTED_RES: [f64; 10] = [
    ///    22.43541699,8.25654609, 4.560976314, 14.97840158, 49.9227277,
    ///    11.6312898,40.87655833,37.83381513,25.05136763,7.397469412,
    ///];
    ///let mpt = MPTCalculator::from_v(&data);
    ///let mut res = [f64::NAN; 10];
    ///let err = mpt.rescale_array(5.0, &mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double_array(&res, &EXPECTED_RES),
    ///    true
    ///);
    ///```
    pub fn rescale_array(&self, rescale_factor: f64, output: &mut [f64]) -> Errors {
        self.values.iter().enumerate().for_each(|x| {
            output[x.0] = x.1 * rescale_factor;
        });

        return Errors::ClErrorCodeNoError;
    }

    fn top_or_botoom_n_vec(
        &self,
        in_n: i32,
        top_values: &mut Vec<f64>,
        top_pos: &mut Vec<i32>,
        cmp_fn: fn(f64, f64) -> Ordering,
    ) -> Errors {
        top_values.reserve(self.values.len());
        top_pos.reserve(self.values.len());
        let mut num_values: Vec<(usize, &f64)> = self.values.iter().enumerate().collect();

        num_values.sort_by(|a, b| {
            if a.1.is_finite() && b.1.is_finite() {
                cmp_fn(*a.1, *b.1)
            } else if !b.1.is_finite() {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });

        let mut i = 0;
        num_values.iter().try_for_each(|x| {
            if x.1.is_finite() {
                top_values.push(*x.1);
                top_pos.push(x.0 as i32);
                i += 1;
                if i >= in_n.try_into().unwrap() {
                    return ControlFlow::Break(x.1);
                }
            }
            ControlFlow::Continue(())
        });

        return Errors::ClErrorCodeNoError;
    }
    ///get the top values for an array, the output is an array include n bottom numbers
    ///#Arguments:
    ///in_n:the bottom numbers
    ///
    ///# Examples
    ///```
    /// use mpt_lib::MPTCalculator;
    /// use mpt_lib::enums::{self, Errors};
    ///
    ///let data = vec![10.0, 11.0, 15.0, f64::NAN, 4.0, 13.24, f64::NAN, 9.45];
    ///let excepted_values = vec![4.0, 9.45, 10.0];
    ///let excepted_pos = vec![4, 7, 0];
    ///let mpt = MPTCalculator::from_v(&data);
    ///let mut top_pos = Vec::new();
    ///let mut top_values = Vec::new();
    ///let err = mpt.bottom_n(3, &mut top_values, &mut top_pos);
    ///assert_eq!(err, Errors::ClErrorCodeNoError);
    ///assert_eq!(MPTCalculator::is_eq_double_array(&top_values, &excepted_values), true);
    ///assert_eq!(excepted_pos, top_pos);
    ///```
    pub fn bottom_n(&self, in_n: i32, top_values: &mut Vec<f64>, top_pos: &mut Vec<i32>) -> Errors {
        return self.top_or_botoom_n_vec(in_n, top_values, top_pos, |a, b| a.total_cmp(&b));
    }
    ///get the top values for an array, the output is an array include n top numbers    
    ///#Arguments:
    ///in_n:the top numbers
    ///
    ///
    ///# Examples
    ///```
    /// use mpt_lib::MPTCalculator;
    /// use mpt_lib::enums::{self, Errors};
    ///
    ///let data = vec![10.0, 11.0, 15.0, f64::NAN, 4.0, 13.24, f64::NAN, 9.45];
    ///let excepted_values = vec![15.0, 13.24, 11.0];
    ///let excepted_pos = vec![2, 5, 1];
    ///let mpt = MPTCalculator::from_v(&data);
    ///let mut top_pos = Vec::new();
    ///let mut top_values = Vec::new();
    ///let err = mpt.top_n(3, &mut top_values, &mut top_pos);
    ///assert_eq!(err, Errors::ClErrorCodeNoError);
    ///assert_eq!(MPTCalculator::is_eq_double_array(&top_values, &excepted_values), true);
    ///assert_eq!(excepted_pos, top_pos);
    ///```
    pub fn top_n(&self, in_n: i32, top_values: &mut Vec<f64>, top_pos: &mut Vec<i32>) -> Errors {
        return self.top_or_botoom_n_vec(in_n, top_values, top_pos, |a, b| b.total_cmp(&a));
    }

    fn get_top_numbers(&self, percentage: f64, min_n: i32, numbers: &mut i32) -> Errors {
        let mut values_count = 0;
        let ret = Self::count(
            self.values,
            &[f64::NAN; 0],
            &[f64::NAN; 0],
            self.values.len() as i32,
            &mut values_count,
        );

        if ret != Errors::ClErrorCodeNoError {
            return ret;
        }

        if values_count < min_n {
            *numbers = values_count;
        } else if ((values_count as f64 * percentage / 100.0) as i32) < min_n {
            *numbers = min_n;
        } else {
            *numbers = (values_count as f64 * percentage / 100.0) as i32;
        }
        return Errors::ClErrorCodeNoError;
    }
    ///get the top values for an array base on the percent, the output is an array includ the top values
    ///#Arguments:
    ///percentage: the output percentage  
    /// min_n: the minumn output numbers
    ///
    ///# Examples
    ///```
    /// use mpt_lib::MPTCalculator;
    /// use mpt_lib::enums::{self, Errors};
    ///
    ///let data = vec![10.0, 11.0, 15.0, f64::NAN, 4.0, 13.24, f64::NAN, 9.45];
    ///let excepted_values = vec![15.0, 13.24, 11.0];
    ///let excepted_pos = vec![2, 5, 1];
    ///let mpt = MPTCalculator::from_v(&data);
    ///let mut top_pos = Vec::new();
    ///let mut top_values = Vec::new();
    ///let err = mpt.top_percent(10.0, 3, &mut top_values, &mut top_pos);
    ///assert_eq!(err, Errors::ClErrorCodeNoError);
    ///assert_eq!(MPTCalculator::is_eq_double_array(&top_values, &excepted_values), true);
    ///assert_eq!(excepted_pos, top_pos);
    ///```
    pub fn top_percent(
        &self,
        percentage: f64,
        min_n: i32,
        top_values: &mut Vec<f64>,
        top_pos: &mut Vec<i32>,
    ) -> Errors {
        let mut numbers = 0;
        let ret = self.get_top_numbers(percentage, min_n, &mut numbers);

        if ret != Errors::ClErrorCodeNoError {
            return ret;
        }

        return self.top_n(numbers, top_values, top_pos);
    }
    ///get the bottoms values for an array base on the percent, the output is an array includ the bottom values
    ///#Arguments:
    ///percentage: the output percentage  
    /// min_n: the minumn output numbers
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///
    ///let data = vec![10.0, 11.0, 15.0, f64::NAN, 4.0, 13.24, f64::NAN, 9.45];
    ///let excepted_values = vec![4.0, 9.45, 10.0];
    ///let excepted_pos = vec![4, 7, 0];
    ///let mpt = MPTCalculator::from_v(&data);
    ///let mut top_pos = Vec::new();
    ///let mut top_values = Vec::new();
    ///let err = mpt.bottom_percent(10.0, 3, &mut top_values, &mut top_pos);
    ///assert_eq!(err, Errors::ClErrorCodeNoError);
    ///assert_eq!(MPTCalculator::is_eq_double_array(&top_values, &excepted_values), true);
    ///assert_eq!(excepted_pos, top_pos);
    ///```
    pub fn bottom_percent(
        &self,
        percentage: f64,
        min_n: i32,
        top_values: &mut Vec<f64>,
        top_pos: &mut Vec<i32>,
    ) -> Errors {
        let mut numbers = 0;
        let ret = self.get_top_numbers(percentage, min_n, &mut numbers);

        if ret != Errors::ClErrorCodeNoError {
            return ret;
        }
        return self.bottom_n(numbers, top_values, top_pos);
    }
    ///calulate the percentile value for an array,
    ///#Arguments:
    ///nth: the percent for the array
    ///# Examples
    ///```
    /// use mpt_lib::MPTCalculator;
    /// use mpt_lib::enums::{self, Errors};
    ///
    ///let data = vec![
    ///    0.08010, 0.24430, 0.15230, -0.62630, -0.00830, 0.06930, -0.01550, 0.07080, 0.08270,
    ///    0.17080, 1.10610, -0.79380, 0.05430, 0.13480, 0.13430, 0.08520, 0.09350, -0.19120,
    ///    0.21420,
    ///];
    ///let mpt = MPTCalculator::from_v(&data);
    ///let mut res = f64::NAN;
    ///let err = mpt.percentile(75, &mut res);
    ///assert_eq!(err, Errors::ClErrorCodeNoError);
    ///assert_eq!(MPTCalculator::is_eq_double(res, 0.14355), true);
    ///```
    pub fn percentile(&self, nth: i32, percentile: &mut f64) -> Errors {
        let mut values_sort = vec![f64::NAN; self.values.len()];
        values_sort.copy_from_slice(self.values);
        values_sort.sort_by(|a, b| {
            if a.is_finite() && b.is_finite() {
                a.total_cmp(&b)
            } else if !b.is_finite() {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });

        let mut count = 0;
        Self::count(
            self.values,
            &[f64::NAN; 0],
            &[f64::NAN; 0],
            self.values.len() as i32,
            &mut count,
        );

        let remain = nth as f64 * (count as f64 - 1.0) / 100.0;
        let n = remain.floor() as i32;
        let remain = remain.fract();

        *percentile = values_sort[n as usize];
        if n + 1 < count {
            *percentile += remain * (values_sort[(n + 1) as usize] - values_sort[n as usize]);
        }
        return Errors::ClErrorCodeNoError;
    }
}

#[cfg(test)]
mod test {
    use crate::{
        enums::{ClFrequency, Errors},
        MPTCalculator,
    };

    #[test]
    fn should_correct_array_additione() {
        let data = vec![
            4.487083398,
            1.651309218,
            0.912195263,
            2.995680315,
            9.98454554,
            2.32625796,
            8.175311666,
            7.566763025,
            5.010273527,
            1.479493882,
        ];

        let data2 = vec![
            6.184582992,
            1.145629713,
            0.923955492,
            7.367172708,
            1.438963969,
            2.163394174,
            0.20035163,
            1.022927912,
            9.43590199,
            1.056781513,
        ];

        const EXPECTED_RES: [f64; 10] = [
            10.67166639,
            2.796938931,
            1.836150755,
            10.3628530,
            11.423509509,
            4.489652134,
            8.375663296,
            8.589690937,
            14.446175517,
            2.536275395,
        ];
        let mpt = MPTCalculator::from_v(&data);
        let mut res = [f64::NAN; 10];
        let err = mpt.array_addition(&data2, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError
                && MPTCalculator::is_eq_double_array(&res, &EXPECTED_RES),
            true
        );
    }

    #[test]
    fn should_correct_array_subtraction() {
        let data = vec![
            4.487083398,
            1.651309218,
            0.912195263,
            2.995680315,
            9.98454554,
            2.32625796,
            8.175311666,
            7.566763025,
            5.010273527,
            1.479493882,
        ];

        let data2 = vec![
            6.184582992,
            1.145629713,
            0.923955492,
            7.367172708,
            1.438963969,
            2.163394174,
            0.20035163,
            1.022927912,
            9.43590199,
            1.056781513,
        ];

        const EXPECTED_RES: [f64; 10] = [
            -1.697499594,
            0.505679505,
            -0.011760229,
            -4.371492393,
            8.545581571,
            0.162863785,
            7.974960036,
            6.543835114,
            -4.425628463,
            0.422712369,
        ];
        let mpt = MPTCalculator::from_v(&data);
        let mut res = [f64::NAN; 10];
        let err = mpt.array_subtraction(&data2, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError
                && MPTCalculator::is_eq_double_array(&res, &EXPECTED_RES),
            true
        );
    }

    #[test]
    fn should_correct_array_multiplication() {
        let data = vec![
            4.487083398,
            1.651309218,
            0.912195263,
            2.995680315,
            9.98454554,
            2.32625796,
            8.175311666,
            7.566763025,
            5.010273527,
            1.479493882,
        ];

        let data2 = vec![
            6.184582992,
            1.145629713,
            0.923955492,
            7.367172708,
            1.438963969,
            2.163394174,
            0.20035163,
            1.022927912,
            9.43590199,
            1.056781513,
        ];

        const EXPECTED_RES: [f64; 10] = [
            27.75073967,
            1.891788905,
            0.842827823,
            22.06969426,
            14.36740127,
            5.032612918,
            1.637937015,
            7.740253099,
            47.27644994,
            1.563501784,
        ];
        let mpt = MPTCalculator::from_v(&data);
        let mut res = [f64::NAN; 10];
        let err = mpt.array_multiplication(&data2, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError
                && MPTCalculator::is_eq_double_array(&res, &EXPECTED_RES),
            true
        );
    }

    #[test]
    fn should_correct_array_division() {
        let data = vec![
            4.487083398,
            1.651309218,
            0.912195263,
            2.995680315,
            9.98454554,
            2.32625796,
            8.175311666,
            7.566763025,
            5.010273527,
            1.479493882,
        ];

        let data2 = vec![
            6.184582992,
            1.145629713,
            0.923955492,
            7.367172708,
            1.438963969,
            2.163394174,
            0.20035163,
            1.022927912,
            9.43590199,
            1.056781513,
        ];

        const EXPECTED_RES: [f64; 10] = [
            0.725527235,
            1.441398735,
            0.987271866,
            0.406625504,
            6.938704344,
            1.075281605,
            40.80481741,
            7.397161559,
            0.530979819,
            1.399999777,
        ];
        let mpt = MPTCalculator::from_v(&data);
        let mut res = [f64::NAN; 10];
        let err = mpt.array_division(&data2, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError
                && MPTCalculator::is_eq_double_array(&res, &EXPECTED_RES),
            true
        );
    }

    #[test]
    fn should_correct_additive_accumulation() {
        let data = vec![
            4.487083398,
            1.651309218,
            0.912195263,
            2.995680315,
            9.98454554,
            2.32625796,
            8.175311666,
            7.566763025,
            5.010273527,
            1.479493882,
        ];

        const EXPECTED_RES: [f64; 10] = [
            4.487083398,
            6.138392616,
            7.050587879,
            10.04626819,
            20.03081373,
            22.35707169,
            30.53238336,
            38.09914639,
            43.10941991,
            44.58891379,
        ];
        let mpt = MPTCalculator::from_v(&data);
        let mut res = [f64::NAN; 10];
        let err = mpt.array_additive_accumulation(&mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError
                && MPTCalculator::is_eq_double_array(&res, &EXPECTED_RES),
            true
        );
    }

    #[test]
    fn should_correct_rescale_array() {
        let data = vec![
            4.487083398,
            1.651309218,
            0.912195263,
            2.995680315,
            9.98454554,
            2.32625796,
            8.175311666,
            7.566763025,
            5.010273527,
            1.479493882,
        ];

        const EXPECTED_RES: [f64; 10] = [
            22.43541699,
            8.25654609,
            4.560976314,
            14.97840158,
            49.9227277,
            11.6312898,
            40.87655833,
            37.83381513,
            25.05136763,
            7.397469412,
        ];
        let mpt = MPTCalculator::from_v(&data);
        let mut res = [f64::NAN; 10];
        let err = mpt.rescale_array(5.0, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError
                && MPTCalculator::is_eq_double_array(&res, &EXPECTED_RES),
            true
        );
    }

    #[test]
    fn should_correct_max() {
        let data = vec![
            4.487083398,
            1.651309218,
            0.912195263,
            2.995680315,
            9.98454554,
            2.32625796,
            8.175311666,
            7.566763025,
            5.010273527,
            1.479493882,
        ];

        let mpt = MPTCalculator::from_v(&data);
        let mut res = f64::NAN;
        let err = mpt.max(&mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 9.98454554),
            true
        );
    }

    #[test]
    fn should_correct_first_double() {
        let data = vec![
            f64::NAN,
            1.651309218,
            0.912195263,
            2.995680315,
            9.98454554,
            2.32625796,
            8.175311666,
            7.566763025,
            5.010273527,
            1.479493882,
        ];
        let dates = vec![
            39445, 39801, 39817, 39522, 39514, 39602, 39760, 39493, 39686, 39752,
        ];
        let mpt = MPTCalculator::from_v(&data);
        let mut res = 0;
        let err = mpt.first_double(39445, 39752, &dates, &mut res);
        assert_eq!(err == Errors::ClErrorCodeNoError && res == 7, true);
    }

    #[test]
    fn should_correct_last_double() {
        let data = vec![
            f64::NAN,
            1.651309218,
            0.912195263,
            2.995680315,
            9.98454554,
            2.32625796,
            8.175311666,
            7.566763025,
            5.010273527,
            1.479493882,
        ];
        let dates = vec![
            39445, 39801, 39817, 39522, 39514, 39602, 39760, 39493, 39686, 39752,
        ];
        let mpt = MPTCalculator::from_v(&data);
        let mut res = 0;
        let err = mpt.last_double(39445, 39752, &dates, &mut res);
        assert_eq!(err == Errors::ClErrorCodeNoError && res == 9, true);
    }

    #[test]
    fn should_correct_top_percent() {
        let data = vec![10.0, 11.0, 15.0, f64::NAN, 4.0, 13.24, f64::NAN, 9.45];
        let excepted_values = vec![15.0, 13.24, 11.0];
        let excepted_pos = vec![2, 5, 1];
        let mpt = MPTCalculator::from_v(&data);
        let mut top_pos = Vec::new();
        let mut top_values = Vec::new();
        let err = mpt.top_percent(10.0, 3, &mut top_values, &mut top_pos);
        assert_eq!(err, Errors::ClErrorCodeNoError);
        assert_eq!(
            MPTCalculator::is_eq_double_array(&top_values, &excepted_values),
            true
        );
        assert_eq!(excepted_pos, top_pos);
    }

    #[test]
    fn should_correct_bottom_percent() {
        let data = vec![10.0, 11.0, 15.0, f64::NAN, 4.0, 13.24, f64::NAN, 9.45];
        let excepted_values = vec![4.0, 9.45, 10.0];
        let excepted_pos = vec![4, 7, 0];
        let mpt = MPTCalculator::from_v(&data);
        let mut top_pos = Vec::new();
        let mut top_values = Vec::new();
        let err = mpt.bottom_percent(10.0, 3, &mut top_values, &mut top_pos);
        assert_eq!(err, Errors::ClErrorCodeNoError);
        assert_eq!(
            MPTCalculator::is_eq_double_array(&top_values, &excepted_values),
            true
        );
        assert_eq!(excepted_pos, top_pos);
    }

    #[test]
    fn should_correct_top_numbers() {
        let data = vec![10.0, 11.0, 15.0, f64::NAN, 4.0, 13.24, f64::NAN, 9.45];
        let excepted_values = vec![15.0, 13.24, 11.0];
        let excepted_pos = vec![2, 5, 1];
        let mpt = MPTCalculator::from_v(&data);
        let mut top_pos = Vec::new();
        let mut top_values = Vec::new();
        let err = mpt.top_n(3, &mut top_values, &mut top_pos);
        assert_eq!(err, Errors::ClErrorCodeNoError);
        assert_eq!(
            MPTCalculator::is_eq_double_array(&top_values, &excepted_values),
            true
        );
        assert_eq!(excepted_pos, top_pos);
    }

    #[test]
    fn should_correct_bottom_numbers() {
        let data = vec![10.0, 11.0, 15.0, f64::NAN, 4.0, 13.24, f64::NAN, 9.45];
        let excepted_values = vec![4.0, 9.45, 10.0];
        let excepted_pos = vec![4, 7, 0];
        let mpt = MPTCalculator::from_v(&data);
        let mut top_pos = Vec::new();
        let mut top_values = Vec::new();
        let err = mpt.bottom_n(3, &mut top_values, &mut top_pos);
        assert_eq!(err, Errors::ClErrorCodeNoError);
        assert_eq!(
            MPTCalculator::is_eq_double_array(&top_values, &excepted_values),
            true
        );
        assert_eq!(excepted_pos, top_pos);
    }

    #[test]
    fn should_correct_percentile() {
        let data = vec![
            0.08010, 0.24430, 0.15230, -0.62630, -0.00830, 0.06930, -0.01550, 0.07080, 0.08270,
            0.17080, 1.10610, -0.79380, 0.05430, 0.13480, 0.13430, 0.08520, 0.09350, -0.19120,
            0.21420,
        ];
        let mpt = MPTCalculator::from_v(&data);
        let mut res = f64::NAN;
        let err = mpt.percentile(75, &mut res);
        assert_eq!(err, Errors::ClErrorCodeNoError);
        assert_eq!(MPTCalculator::is_eq_double(res, 0.14355), true);
    }

    #[test]
    fn should_correct_weighted_average() {
        let data = vec![
            0.1018520000,
            0.1379310000,
            0.1322920000,
            -0.0384620000,
            0.0041670000,
            0.1724510000,
            0.4257430000,
            0.2689870000,
            0.0540540000,
            1.0000000000,
            0.3307080000,
            f64::NAN,
            f64::NAN,
            0.0884350000,
            -0.0467840000,
            f64::NAN,
            0.3450980000,
            -0.0566040000,
            0.1000000000,
        ];

        let weights = vec![
            26035.5600000000,
            118.7985000000,
            4012.4600000000,
            1.4294000000,
            422.7000120000,
            142074.0300000000,
            10214.1000000000,
            711.3405150000,
            35.5523990000,
            1.4975000000,
            36.8974990000,
            f64::NAN,
            f64::NAN,
            1609.7000000000,
            197.4942930000,
            f64::NAN,
            1618.0400000000,
            34.0756990000,
            7.9369000000,
        ];
        let mpt = MPTCalculator::from_v(&data);
        let mut res = f64::NAN;
        let err = mpt.weighted_average(&weights, &mut res);
        assert_eq!(err, Errors::ClErrorCodeNoError);
        assert_eq!(MPTCalculator::is_eq_double(res, 0.1760653727), true);
    }

    #[test]
    fn should_correct_average_for_ts() {
        let data = vec![
            8.169969704,
            17.53634366,
            5.344482043,
            8.452863347,
            10.33126183,
            12.7683848,
            10.67265227,
            0.973782619,
            17.43341129,
            7.793032244,
            5.839265786,
            0.589181628,
            19.80643643,
            0.319166174,
            5.887448511,
            12.20015787,
            13.45983609,
            10.02752314,
            14.23095689,
            6.187832093,
            14.82583437,
            14.75803915,
            9.90970724,
            12.95576028,
            11.02894738,
            7.56902044,
            7.865732487,
            12.08801651,
            13.4391796,
            11.06652432,
            14.73725253,
            19.29441213,
            8.767635516,
            0.900297309,
            14.3543903,
            7.172694944,
            3.227678772,
            16.11482084,
            6.041044308,
            6.189942603,
            11.16458034,
            13.50079637,
            17.17207264,
            10.1869313,
            18.52252921,
            16.34204845,
            12.98478245,
            16.96478255,
            17.00365216,
            13.34776382,
            3.630459975,
            16.20671779,
            12.70671665,
            8.410486911,
            4.581037552,
            6.045530479,
            1.850320217,
            17.61496548,
            13.61665776,
            10.14668818,
            9.9238987,
            18.61926739,
            3.407462739,
            3.147958377,
            1.312209162,
            17.77142914,
            4.835527897,
            11.21452525,
            15.90649149,
            11.14175699,
            10.12361377,
            18.02892583,
            13.52113804,
            2.467934258,
            5.844192095,
            11.06558362,
            1.964113557,
            1.100482004,
            3.83922735,
            11.447917,
            0.545897803,
            15.3561911,
            13.04722015,
            13.16691716,
        ];
        let dates = vec![
            39441, 39441, 39445, 39452, 39461, 39461, 39467, 39476, 39484, 39485, 39493, 39500,
            39508, 39514, 39522, 39529, 39533, 39536, 39537, 39546, 39552, 39557, 39563, 39567,
            39569, 39570, 39576, 39581, 39583, 39591, 39595, 39595, 39602, 39609, 39613, 39615,
            39615, 39618, 39625, 39633, 39642, 39650, 39657, 39666, 39667, 39668, 39668, 39673,
            39681, 39681, 39686, 39688, 39691, 39692, 39693, 39693, 39694, 39700, 39703, 39706,
            39706, 39710, 39719, 39728, 39734, 39735, 39737, 39740, 39747, 39752, 39755, 39760,
            39762, 39770, 39779, 39785, 39787, 39792, 39792, 39801, 39802, 39810, 39812, 39817,
        ];

        let excpted_values = vec![
            8.639788972,
            7.913722736,
            10.84736073,
            11.72743463,
            12.13613567,
            8.082651712,
            12.00684799,
            13.78963844,
            9.421631542,
            9.332842614,
            9.9971608,
            7.295829073,
        ];
        let excpted_dates = vec![
            39478, 39507, 39538, 39568, 39599, 39629, 39660, 39691, 39721, 39752, 39782, 39813,
        ];
        let mut avg_values = Vec::new();
        let mut avg_dates = Vec::new();
        let mpt = MPTCalculator::from_v(&data);
        let err = mpt.average_freq_for_ts(
            39478,
            39813,
            ClFrequency::ClFrequencyMonthly,
            &dates,
            &mut avg_values,
            &mut avg_dates,
        );
        assert_eq!(
            err == Errors::ClErrorCodeNoError && avg_dates == excpted_dates,
            true
        );
        assert_eq!(
            err == Errors::ClErrorCodeNoError
                && MPTCalculator::is_eq_double_array(&avg_values, &excpted_values),
            true
        );
    }

    #[test]
    fn should_correct_sum_in_period() {
        let data = vec![
            8.6397889, 7.9137227, 10.847360, 11.727434, 12.136135, 8.0826517, 12.006847, 13.789638,
            9.4216315, 9.3328426, 9.9971608, 7.2958290,
        ];
        let dates = vec![
            39478, 39507, 39538, 39568, 39599, 39629, 39660, 39691, 39721, 39752, 39782, 39813,
        ];
        let mpt = MPTCalculator::from_v(&data);
        let mut res = f64::NAN;
        let err = mpt.sum_in_period(39478, 39813, &dates, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 121.1910412),
            true
        );
    }

    #[test]
    fn should_correct_count_in_period() {
        let data = vec![
            8.6397889, 7.9137227, 10.847360, 11.727434, 12.136135, 8.0826517, 12.006847, 13.789638,
            9.4216315, 9.3328426, 9.9971608, 7.2958290,
        ];
        let dates = vec![
            39478, 39507, 39538, 39568, 39599, 39629, 39660, 39691, 39721, 39752, 39782, 39813,
        ];
        let mpt = MPTCalculator::from_v(&data);
        let mut res = 0;
        let err = mpt.count_in_period(39478, 39813, &dates, &mut res);
        assert_eq!(err == Errors::ClErrorCodeNoError && res == 12, true);
    }

    #[test]
    fn should_correct_max_in_period() {
        let data = vec![
            8.6397889, 7.9137227, 10.847360, 11.727434, 12.136135, 8.0826517, 12.006847, 13.789638,
            9.4216315, 9.3328426, 9.9971608, 7.2958290,
        ];
        let dates = vec![
            39478, 39507, 39538, 39568, 39599, 39629, 39660, 39691, 39721, 39752, 39782, 39813,
        ];
        let mpt = MPTCalculator::from_v(&data);
        let mut res = f64::NAN;
        let mut max_date = 0;
        let err = mpt.max_in_period(39478, 39813, &dates, &mut res, &mut max_date);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && res == 13.789638 && max_date == 39691,
            true
        );
    }

    #[test]
    fn should_correct_min_in_period() {
        let data = vec![
            8.6397889, 7.9137227, 10.847360, 11.727434, 12.136135, 8.0826517, 12.006847, 13.789638,
            9.4216315, 9.3328426, 9.9971608, 7.2958290,
        ];
        let dates = vec![
            39478, 39507, 39538, 39568, 39599, 39629, 39660, 39691, 39721, 39752, 39782, 39813,
        ];
        let mpt = MPTCalculator::from_v(&data);
        let mut res = f64::NAN;
        let mut min_date = 0;
        let err = mpt.min_in_period(39478, 39813, &dates, &mut res, &mut min_date);
        assert_eq!(
            err == Errors::ClErrorCodeNoError
                && MPTCalculator::is_eq_double(res, 7.2958290)
                && min_date == 39813,
            true
        );
    }

    #[test]
    fn should_correct_min() {
        let data = vec![
            8.6397889, 7.9137227, 10.847360, 11.727434, 12.136135, 8.0826517, 12.006847, 13.789638,
            9.4216315, 9.3328426, 9.9971608, 7.2958290,
        ];
        let mpt = MPTCalculator::from_v(&data);
        let mut res = f64::NAN;
        let err = mpt.min(&mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 7.2958290),
            true
        );
    }

    #[test]
    fn should_correct_sum() {
        let data = vec![
            8.6397889, 7.9137227, 10.847360, 11.727434, 12.136135, 8.0826517, 12.006847, 13.789638,
            9.4216315, 9.3328426, 9.9971608, 7.2958290,
        ];
        let mpt = MPTCalculator::from_v(&data);
        let mut res = f64::NAN;
        let err = mpt.sum(&mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 121.1910412),
            true
        );
    }
}
