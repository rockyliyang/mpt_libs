use std::{collections::HashSet, ops::ControlFlow};

use float_cmp::approx_eq;

use crate::{
    date_util,
    enums::{self, Errors},
    MPTCalculator,
};

pub struct AvgCreditQualityCalculator {
    pub a0: [f64; 3],
    pub a1: [f64; 3],
    pub a2: [f64; 3],
    pub y_prime: [f64; 2],
}

pub struct OwnershipZone {
    pub sum_weight: f64,
    pub centroid_x: f64,
    pub centroid_y: f64,
    pub dispersion_x: f64,
    pub dispersion_y: f64,
    pub tilt: f64,
    pub ozone75: f64,
}

pub struct InputDatas<'a> {
    pub values: &'a [f64],
    pub benchmark: &'a [f64],
    pub riskfree: &'a [f64],
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct DataGroup {
    pub start: usize,
    pub end: usize,
    pub data: f64,
}

impl DataGroup {
    pub fn new() -> DataGroup {
        DataGroup {
            start: 0,
            end: 0,
            data: 0.0,
        }
    }
    pub fn from(old: &DataGroup) -> DataGroup {
        DataGroup {
            start: old.start,
            end: old.end,
            data: old.data,
        }
    }
}

#[derive(Clone)]
pub(crate) struct TreynorRatioData {
    pub total_return: f64,
    pub rf_total_return: f64,
    pub excess_beta: f64,
    pub sum: f64,
    pub count: usize,
}

#[derive(Clone)]
pub(crate) struct InformationRatioData {
    pub total_return: f64,
    pub bmk_total_return: f64,
    pub tracking_error: f64,
    pub sum: f64,
    pub count: usize,
}
pub(crate) struct CaptureData {
    pub count: i32,
    pub accu_y: f64,
    pub accu_x: f64,
}

pub(crate) struct RatioData {
    pub count: i32,
    pub ratio: i32,
}

pub fn get_annual_multiplier(freq: enums::ClFrequency, is_fd: bool) -> f64 {
    let mut multiplier = f64::NAN;
    if freq == enums::ClFrequency::ClFrequencyDaily {
        if is_fd {
            multiplier = 250.0;
        } else {
            multiplier = 365.25;
        }
    } else if freq == enums::ClFrequency::ClFrequencyWeekly {
        multiplier = 52.0;
    } else if freq == enums::ClFrequency::ClFrequencyMonthly {
        multiplier = 12.0;
    } else if freq == enums::ClFrequency::ClFrequencyQuarterly {
        multiplier = 4.0;
    } else if freq == enums::ClFrequency::ClFrequencySemiannually {
        multiplier = 2.0;
    } else if freq == enums::ClFrequency::ClFrequencyAnnually {
        multiplier = 1.0;
    }
    return multiplier;
}

pub(crate) fn is_valid_frequency(freq: enums::ClFrequency) -> bool {
    if freq == enums::ClFrequency::ClFrequencyDaily
        || freq == enums::ClFrequency::ClFrequencyWeekly
        || freq == enums::ClFrequency::ClFrequencyMonthly
        || freq == enums::ClFrequency::ClFrequencyQuarterly
        || freq == enums::ClFrequency::ClFrequencySemiannually
        || freq == enums::ClFrequency::ClFrequencyAnnually
    {
        return true;
    } else {
        return false;
    }
}

pub(crate) fn annualize_return(
    return_value: f64,
    freq: enums::ClFrequency,
    periods: f64,
    is_geometric: bool,
) -> f64 {
    if !return_value.is_finite() || periods < 0.0 || !is_valid_frequency(freq) {
        return f64::NAN;
    }
    let mutipler = get_annual_multiplier(freq, false);
    if periods == mutipler {
        return return_value;
    } else {
        if is_geometric {
            return ((return_value / 100.0 + 1.0).powf(mutipler / periods) - 1.0) * 100.0;
        } else {
            return return_value * mutipler / periods;
        }
    }
}
pub fn is_sorted_array<T: std::cmp::PartialOrd>(data: &[T]) -> bool {
    if data.len() < 2 {
        return false;
    }
    let is_asc = data[1] > data[0];
    for i in 2..data.len() {
        if (data[i] > data[i - 1]) != is_asc {
            return false;
        }
    }
    true
}

impl<'a> MPTCalculator<'a> {
    pub fn from(values: &'a [f64], benchmark: &'a [f64], riskfree: &'a [f64]) -> MPTCalculator<'a> {
        MPTCalculator {
            values: values,
            benchmark: benchmark,
            riskfree: riskfree,
        }
    }
    pub fn from_v(values: &'a [f64]) -> MPTCalculator<'a> {
        MPTCalculator {
            values: values,
            benchmark: &[f64::NAN; 0],
            riskfree: &[f64::NAN; 0],
        }
    }
    pub fn from_v_b(values: &'a [f64], benchmark: &'a [f64]) -> MPTCalculator<'a> {
        MPTCalculator {
            values: values,
            benchmark: benchmark,
            riskfree: &[f64::NAN; 0],
        }
    }
    pub fn from_v_r(values: &'a [f64], riskfree: &'a [f64]) -> MPTCalculator<'a> {
        MPTCalculator {
            values: values,
            benchmark: &[f64::NAN; 0],
            riskfree: riskfree,
        }
    }

    pub(crate) fn standard_deviation_internal(
        values: &[f64],
        freq: enums::ClFrequency,
        is_annu: bool,
        standard_deviation_result: &mut f64,
    ) -> Errors {
        let mut mean = f64::NAN;
        let ret = MPTCalculator::from_v(values).average(&mut mean);
        if ret != Errors::ClErrorCodeNoError {
            return ret;
        }

        if values.iter().find(|x| !x.is_finite()) != None {
            return Errors::ClErrorCodeNoError;
        }

        let accumalte = values
            .iter()
            .filter(|x| x.is_finite())
            .fold(0.0, |acc, x| acc + (x - mean) * (x - mean));

        if values.len() == 0 {
            *standard_deviation_result = 0.0
        } else {
            *standard_deviation_result = (accumalte / (values.len() as f64 - 1.0)).sqrt()
        }

        if is_annu {
            *standard_deviation_result =
                *standard_deviation_result * get_annual_multiplier(freq, false).sqrt()
        }

        return Errors::ClErrorCodeNoError;
    }

    pub(crate) fn array_subtraction_internal(
        values1: &[f64],
        values2: &[f64],
        output: &mut [f64],
    ) -> Errors {
        values1.iter().enumerate().for_each(|v| {
            if v.1.is_finite() && values2[v.0].is_finite() {
                output[v.0] = v.1 - values2[v.0]
            } else {
                output[v.0] = f64::NAN
            }
        });
        return Errors::ClErrorCodeNoError;
    }

    pub(crate) fn convert_discreet_return_by_dri(
        &self,
        res: &mut Vec<f64>,
        bmk_res: &mut Vec<f64>,
        rf_res: &mut Vec<f64>,
    ) {
        if self.values.len() > 0 {
            self.values
                .iter()
                .enumerate()
                .scan(
                    (f64::NAN, f64::NAN, f64::NAN, 0),
                    |state: &mut (f64, f64, f64, i32), x: (usize, &f64)| {
                        if state.3 == 0 {
                            state.0 = *x.1;
                            state.3 += 1;
                        } else if state.0 != *x.1 {
                            if !state.0.is_finite() || !x.1.is_finite() {
                                res.push(f64::NAN);
                            } else {
                                res.push((x.1 / (state.0)).ln() * (1.0 / state.3 as f64));
                            }
                            state.0 = *x.1;

                            if self.benchmark.len() > 0 && x.0 < self.benchmark.len() {
                                if !state.1.is_finite() || !self.benchmark[x.0].is_finite() {
                                    bmk_res.push(f64::NAN);
                                } else {
                                    bmk_res.push(
                                        (self.benchmark[x.0] / (state.1)).ln()
                                            * (1.0 / state.3 as f64),
                                    );
                                }
                                state.1 = self.benchmark[x.0];
                            }
                            if self.riskfree.len() > 0 && x.0 < self.riskfree.len() {
                                if !state.2.is_finite() || !self.riskfree[x.0].is_finite() {
                                    rf_res.push(f64::NAN);
                                } else {
                                    rf_res.push(
                                        (self.riskfree[x.0] / (state.2)).ln()
                                            * (1.0 / state.3 as f64),
                                    );
                                }
                                state.2 = self.riskfree[x.0];
                            }

                            state.3 = 1;
                        } else {
                            state.3 += 1;
                        }
                        Some(())
                    },
                )
                .count();
        }
    }

    pub(crate) fn convert_discreet_return_by_holiday(
        &self,
        start_date: i32,
        holiday: &[i32],
        res: &mut Vec<f64>,
        bmk_res: &mut Vec<f64>,
        rf_res: &mut Vec<f64>,
    ) {
        if self.values.len() > 0 {
            res.reserve(self.values.len());
            bmk_res.reserve(self.values.len());
            rf_res.reserve(self.values.len());

            let holiday_set: HashSet<&i32> = holiday.iter().collect();
            let mut trade_days: Vec<usize> = Vec::with_capacity(self.values.len());
            trade_days.push(0);
            for i in 1..self.values.len() {
                if date_util::is_weekend((i as i32 + start_date - 1) as u64)
                    || holiday_set.get(&(i as i32 + start_date - 1)) != None
                {
                    trade_days.push(i);
                }
            }

            trade_days
                .iter()
                .scan(0, |state, &x| {
                    if *state != 0 {
                        if !self.values[x].is_finite() || !self.values[*state].is_finite() {
                            res.push(f64::NAN);
                        } else {
                            res.push(
                                (self.values[x] / self.values[*state]).ln()
                                    * (1.0 / (x - *state) as f64),
                            );
                        }

                        if self.benchmark.len() > 0
                            && x < self.benchmark.len()
                            && *state < self.benchmark.len()
                        {
                            if !self.benchmark[x].is_finite() || !self.benchmark[*state].is_finite()
                            {
                                bmk_res.push(f64::NAN);
                            } else {
                                bmk_res.push(
                                    (self.benchmark[x] / self.benchmark[*state]).ln()
                                        * (1.0 / (x - *state) as f64),
                                );
                            }
                        }

                        if self.riskfree.len() > 0
                            && x < self.riskfree.len()
                            && *state < self.riskfree.len()
                        {
                            if !self.riskfree[x].is_finite() || !self.riskfree[*state].is_finite() {
                                rf_res.push(f64::NAN);
                            } else {
                                rf_res.push(
                                    (self.riskfree[x] / self.riskfree[*state]).ln()
                                        * (1.0 / (x - *state) as f64),
                                );
                            }
                        }
                    }
                    *state = x;
                    Some(())
                })
                .count();
        }
    }

    pub(crate) fn convert_discreet_return(&self, is_dri: bool, result: &mut Vec<f64>) {
        if self.values.len() > 0 {
            result.reserve(self.values.len());
            if is_dri {
                self.values
                    .iter()
                    .scan((f64::NAN, 0), |state, &x| {
                        if state.1 == 0 {
                            state.0 = x;
                            state.1 += 1;
                        } else if state.0 != x {
                            if !state.0.is_finite() || !x.is_finite() {
                                result.push(f64::NAN);
                            } else {
                                result.push((x / (state.0)).ln() * (1.0 / state.1 as f64));
                            }
                            state.0 = x;
                            state.1 = 1;
                        } else {
                            state.1 += 1;
                        }
                        Some(())
                    })
                    .count();
            } else {
                self.values
                    .iter()
                    .for_each(|v| result.push((1.0 + v / 100.0).ln()));
            }
        }
    }

    pub(crate) fn calc_avg_excess_return(&self, avg_excess_return: &mut f64) -> Errors {
        let mut sum_excess_return = 0.0;
        let mut count = 0;
        if self
            .values
            .iter()
            .enumerate()
            .try_for_each(|v| {
                if !v.1.is_finite() || !self.riskfree[v.0].is_finite() {
                    return ControlFlow::Break(());
                } else {
                    sum_excess_return += v.1 - self.riskfree[v.0];
                    count += 1;
                    return ControlFlow::Continue(());
                }
            })
            .is_break()
        {
            return Errors::ClErrorCodeCcFaild;
        }

        if count == 0 {
            return Errors::ClErrorCodeCcFaild;
        }
        *avg_excess_return = sum_excess_return / count as f64;
        return Errors::ClErrorCodeNoError;
    }

    pub(crate) fn total_return_accumulat(values: &[f64], result: &mut f64) -> Errors {
        if values.len() == 0 {
            return Errors::ClErrorCodeInvalidPara;
        }
        *result = f64::NAN;
        let mut acct_return = 1.0;
        if values
            .iter()
            .try_for_each(|v| {
                if !v.is_finite() {
                    return ControlFlow::Break(());
                } else {
                    acct_return *= 1.0 + v / 100.0;
                    return ControlFlow::Continue(());
                }
            })
            .is_break()
        {
            return Errors::ClErrorCodeNoError;
        }
        *result = (acct_return - 1.0) * 100.0;

        return Errors::ClErrorCodeNoError;
    }

    pub(crate) fn calc_annu_total_return(
        values: &[f64],
        riskfree: &[f64],
        freq: enums::ClFrequency,
        annu_total_return: &mut f64,
        annu_rf_total_return: &mut f64,
    ) -> Errors {
        let mut total_return = f64::NAN;
        let mut rf_total_return = f64::NAN;
        Self::total_return_accumulat(values, &mut total_return);
        Self::total_return_accumulat(riskfree, &mut rf_total_return);

        if !total_return.is_finite() || !rf_total_return.is_finite() {
            return Errors::ClErrorCodeCcFaild;
        }

        *annu_total_return = annualize_return(total_return, freq, values.len() as f64, true);
        *annu_rf_total_return = annualize_return(rf_total_return, freq, values.len() as f64, true);
        return Errors::ClErrorCodeNoError;
    }

    pub const MIN_DOUBLE: f64 = 0.000009;
    pub fn is_eq_double(a: f64, b: f64) -> bool {
        //return (a - b).abs() < MIN_DOUBLE;
        return approx_eq!(f64, a, b, epsilon = Self::MIN_DOUBLE);
    }

    pub fn is_eq_double_array(a: &[f64], b: &[f64]) -> bool {
        if a.len() != b.len() {
            return false;
        }
        for i in 0..a.len() {
            if !Self::is_eq_double(a[i], b[i]) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::is_sorted_array;
    #[test]
    fn should_correct_sorted_order() {
        assert_eq!(is_sorted_array(&[1, 2, 3, 4, 5, 6]), true);
        assert_eq!(is_sorted_array(&[1.0, 2.0, 3.0, 4.0, 5.0, 1.0]), false);
    }
}
