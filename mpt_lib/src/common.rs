use core::slice;

use crate::enums::{self, Errors};
pub struct MPTCalculator<'a> {
    pub values: &'a [f64],
    pub benchmark: &'a [f64],
    pub riskfree: &'a [f64],
}

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

#[derive(Clone, Copy)]
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
pub fn check_and_convert<'a>(
    values: *const f64,
    bmk_values: *const f64,
    riskfree_values: *const f64,
    value_array_size: usize,
    check_values: bool,
    check_bmk: bool,
    check_rf: bool,
) -> Result<InputDatas<'a>, Errors> {
    if values.is_null() || value_array_size == 0 {
        return Err(Errors::ClErrorCodeInvalidPara);
    }
    if check_bmk && bmk_values.is_null() {
        return Err(Errors::ClErrorCodeInvalidPara);
    }

    if check_rf && riskfree_values.is_null() {
        return Err(Errors::ClErrorCodeInvalidPara);
    }

    let input = InputDatas {
        values: unsafe { slice::from_raw_parts(values, value_array_size) },
        benchmark: unsafe { slice::from_raw_parts(bmk_values, value_array_size) },
        riskfree: unsafe { slice::from_raw_parts(riskfree_values, value_array_size) },
    };

    if check_values && input.values.iter().find(|x| !x.is_finite()) != None {
        return Err(Errors::ClErrorCodeNoError);
    }

    if check_bmk && input.benchmark.iter().find(|x| !x.is_finite()) != None {
        return Err(Errors::ClErrorCodeNoError);
    }

    if check_rf && input.riskfree.iter().find(|x| !x.is_finite()) != None {
        return Err(Errors::ClErrorCodeNoError);
    }
    Ok(input)
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

const MIN_DOUBLE: f64 = 0.00001;
pub fn is_eq_double(a: f64, b: f64) -> bool {
    return (a - b).abs() < MIN_DOUBLE;
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

    pub fn average_internal(values: &[f64], avg: &mut f64) -> Errors {
        *avg = values.iter().filter(|x| (**x).is_finite()).sum::<f64>()
            / values.iter().filter(|x| (**x).is_finite()).count() as f64;
        return Errors::ClErrorCodeNoError;
    }

    pub(crate) fn standard_deviation_internal(
        values: &[f64],
        freq: enums::ClFrequency,
        is_annu: bool,
        standard_deviation_result: &mut f64,
    ) -> Errors {
        let mut mean = f64::NAN;
        let ret = Self::average_internal(values, &mut mean);
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
