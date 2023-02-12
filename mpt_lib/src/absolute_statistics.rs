use crate::{
    common::{
        annualize_return, get_annual_multiplier, is_sorted_array, is_valid_frequency, DataGroup,
    },
    date_util,
    enums::{self, Errors},
    MPTCalculator,
};
use std::ops::ControlFlow;

impl<'a> MPTCalculator<'a> {
    ///calculate the average value of an array not include NAN/INF values
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![10.0, 20.0, 30.0];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err = mpt.average(&mut res);
    ///assert_eq!(err == Errors::ClErrorCodeNoError && res==20.0,true)
    ///```
    pub fn average(&self, avg: &mut f64) -> Errors {
        *avg = self
            .values
            .iter()
            .filter(|x| (**x).is_finite())
            .sum::<f64>()
            / self.values.iter().filter(|x| (**x).is_finite()).count() as f64;
        return Errors::ClErrorCodeNoError;
    }

    ///calculate the standard deviation value of an array，if the array has NAN/INF values,the result will be NAN
    ///
    ///# Arguments
    ///freq: the frequence of source data.
    ///
    ///is_annu: the flag of annualize.
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///    -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
    ///    1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
    ///    1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
    ///    1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err = mpt.standard_deviation(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 15.99317),
    ///    true
    ///);
    ///```
    pub fn standard_deviation(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        standard_deviation_result: &mut f64,
    ) -> Errors {
        return Self::standard_deviation_internal(
            self.values,
            freq,
            is_annu,
            standard_deviation_result,
        );
    }
    ///calculate the harmonic mean value of an array, if the array has NAN/INF values,the result will be NAN
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///   -1.5,2.3,4.5
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err = mpt.mean_harmonic(&mut res);
    ///assert_eq!(
    ///   err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, -310.5),
    ///   true
    ///);
    ///```
    pub fn mean_harmonic(&self, mean_res: &mut f64) -> Errors {
        *mean_res = f64::NAN;

        let mut sum = 0.0;

        if self
            .values
            .iter()
            .try_for_each(|x| {
                if !x.is_finite() {
                    return ControlFlow::Break(());
                }
                sum += 1.0 / x;
                ControlFlow::Continue(())
            })
            .is_break()
        {
            return Errors::ClErrorCodeNoError;
        }

        *mean_res = self.values.len() as f64 / sum;

        return Errors::ClErrorCodeNoError;
    }

    ///calculate the weighted arithmetic mean value of an array not include NAN/INF values,if the array or weights has NAN/INF values,the result will be NAN
    ///# Arguments
    ///weights: the weights for the values
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![-1.5, 2.3, 4.5];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let weights = vec![0.1, 0.2, 0.3];
    ///let err = mpt.weighted_mean_arithmetic(&weights, &mut res);
    ///assert_eq!(
    ///   err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 2.76666667),
    ///   true
    ///);
    ///```
    pub fn weighted_mean_arithmetic(&self, weights: &[f64], mean_res: &mut f64) -> Errors {
        *mean_res = f64::NAN;
        if weights.len() != self.values.len() {
            return Errors::ClErrorCodeInvalidPara;
        }

        let mut sum = 0.0;
        let mut weight_sum = 0.0;
        if self
            .values
            .iter()
            .enumerate()
            .try_for_each(|v| {
                if !v.1.is_finite() || !weights[v.0].is_finite() {
                    return ControlFlow::Break(());
                }
                sum += v.1 * weights[v.0];
                weight_sum += weights[v.0];
                ControlFlow::Continue(())
            })
            .is_break()
        {
            return Errors::ClErrorCodeNoError;
        }

        if weight_sum != 0.0 {
            *mean_res = sum / weight_sum
        }

        return Errors::ClErrorCodeNoError;
    }
    ///calculate the weighted geometric mean value of an array,if the array or weights has NAN/INF values,the result will be NAN
    ///# Arguments
    ///weights: the weights for values
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///
    ///let data = vec![
    ///   1.22072, 0.0668, 2.20588, 0.91563, 0.76766, 1.21429, 3.43456, 4.99825, 3.89481,
    ///   1.59564, 0.86793, 2.41477, 1.80305, 0.6709, 3.57769, 4.77481, 0.37317, 3.52713,
    ///   1.88831, 1.73502, 1.20155, 3.36542, 2.03551, 5.6145, 2.71663, 0.04815, 3.99807,
    ///   1.66744, 9.68658, 0.46681, 4.22095, 6.7, 15.27331, 8.46123, 0.76369, 10.32347,
    ///];
    ///let weighting = vec![
    ///       3.683070486,2.698835031,2.615091784,2.829245119,4.197477687,
    ///       3.747731115,1.428980992,1.490970258,3.776323531,1.126182408,
    ///       4.589706355,2.213203472,3.290841193,1.574023637,2.7073515,
    ///       2.067657476,2.715387407,3.782522676,4.737767273,3.587905857,
    ///       1.00234693,3.598129659,2.182956354,2.399354298,0.893462788,
    ///       1.636175797,1.182474797,4.58802791,3.983018253,4.741795995,
    ///       2.837587798,2.613364024,4.084667264,0.443121313,1.119531868,
    ///       3.833709695,
    ///   ];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err = mpt.weighted_mean_geometric(&weighting,&mut res);
    ///assert_eq!(
    ///   err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 1.9433672988),
    ///   true
    ///);
    ///```
    pub fn weighted_mean_geometric(&self, weights: &[f64], mean_res: &mut f64) -> Errors {
        *mean_res = f64::NAN;
        if weights.len() != self.values.len() {
            return Errors::ClErrorCodeInvalidPara;
        }

        let mut sum = 0.0;
        let mut weight_sum = 0.0;
        if self
            .values
            .iter()
            .enumerate()
            .try_for_each(|v| {
                if !v.1.is_finite() || !weights[v.0].is_finite() || *v.1 < 0.0 {
                    return ControlFlow::Break(());
                }
                sum += v.1.ln() * weights[v.0];
                weight_sum += weights[v.0];
                ControlFlow::Continue(())
            })
            .is_break()
        {
            return Errors::ClErrorCodeNoError;
        }

        if weight_sum != 0.0 {
            *mean_res = (sum / weight_sum).exp();
        }

        return Errors::ClErrorCodeNoError;
    }

    ///calculate the weighted harmonic mean value of an array,if the array or weights has NAN/INF values,the result will be NAN
    ///# Arguments
    ///weights: the weights for values
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///   1.22072, 0.0668, 2.20588, 0.91563, 0.76766, 1.21429, 3.43456, 4.99825, 3.89481,
    ///   1.59564, 0.86793, 2.41477, 1.80305, 0.6709, 3.57769, 4.77481, 0.37317, 3.52713,
    ///   1.88831, 1.73502, 1.20155, 3.36542, 2.03551, 5.6145, 2.71663, 0.04815, 3.99807,
    ///   1.66744, 9.68658, 0.46681, 4.22095, 6.7, 15.27331, 8.46123, 0.76369, 10.32347,
    ///];
    ///let weighting = vec![
    ///       3.683070486,2.698835031,2.615091784,2.829245119,4.197477687,
    ///       3.747731115,1.428980992,1.490970258,3.776323531,1.126182408,
    ///       4.589706355,2.213203472,3.290841193,1.574023637,2.7073515,
    ///       2.067657476,2.715387407,3.782522676,4.737767273,3.587905857,
    ///       1.00234693,3.598129659,2.182956354,2.399354298,0.893462788,
    ///       1.636175797,1.182474797,4.58802791,3.983018253,4.741795995,
    ///       2.837587798,2.613364024,4.084667264,0.443121313,1.119531868,
    ///       3.833709695,
    ///   ];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err = mpt.weighted_mean_harmonic(&weighting, &mut res);
    ///assert_eq!(
    ///   err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 0.726329928),
    ///   true
    ///);
    ///```
    pub fn weighted_mean_harmonic(&self, weights: &[f64], mean_res: &mut f64) -> Errors {
        *mean_res = f64::NAN;
        if weights.len() != self.values.len() {
            return Errors::ClErrorCodeInvalidPara;
        }

        let mut sum = 0.0;
        let mut weight_sum = 0.0;
        if self
            .values
            .iter()
            .enumerate()
            .try_for_each(|v| {
                if !v.1.is_finite() || !weights[v.0].is_finite() || *v.1 == 0.0 {
                    return ControlFlow::Break(());
                }
                sum += weights[v.0] / v.1;
                weight_sum += weights[v.0];
                ControlFlow::Continue(())
            })
            .is_break()
        {
            return Errors::ClErrorCodeNoError;
        }

        if weight_sum != 0.0 {
            *mean_res = weight_sum / sum;
        }

        return Errors::ClErrorCodeNoError;
    }
    ///calculate the geometric mean value of an array,if the array has NAN/INF values,the result will be NAN
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///
    ///let data = vec![
    ///   1.22072, 0.0668, 2.20588, 0.91563, 0.76766, 1.21429, 3.43456, 4.99825, 3.89481,
    ///   1.59564, 0.86793, 2.41477, 1.80305, 0.6709, 3.57769, 4.77481, 0.37317, 3.52713,
    ///   1.88831, 1.73502, 1.20155, 3.36542, 2.03551, 5.6145, 2.71663, 0.04815, 3.99807,
    ///   1.66744, 9.68658, 0.46681, 4.22095, 6.7, 15.27331, 8.46123, 0.76369, 10.32347,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err = mpt.mean_geometric(&mut res);
    ///assert_eq!(
    ///   err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 1.920852518),
    ///   true
    ///);
    ///```
    pub fn mean_geometric(&self, mean_res: &mut f64) -> Errors {
        if self.values.len() == 0 {
            return Errors::ClErrorCodeInvalidPara;
        }
        *mean_res = 1.0;

        let is_even = self.values.len() % 2 == 0;
        let mut negative_product = 1.0;
        let mut negative_num = 0;
        let value_array_size = self.values.len();
        self.values.iter().enumerate().try_for_each(|v| {
            if !(*v.1).is_finite() {
                *mean_res = f64::NAN;
                ControlFlow::Break(())
            } else if MPTCalculator::is_eq_double(*v.1, 0.0) {
                *mean_res = 0.0;
                ControlFlow::Break(())
            } else if *v.1 < 0.0 && is_even {
                negative_product *= v.1;
                negative_num += 1;
                if negative_num == 2 {
                    *mean_res *= negative_product.powf(1.0 / value_array_size as f64);
                    negative_product = 1.0;
                    negative_num = 0;
                    ControlFlow::Continue(())
                } else {
                    ControlFlow::Continue(())
                }
            } else if *v.1 < 0.0 {
                *mean_res *= -1.0 * ((-1.0) * v.1).powf(1.0 / value_array_size as f64);
                ControlFlow::Continue(())
            } else {
                *mean_res *= v.1.powf(1.0 / value_array_size as f64);
                ControlFlow::Continue(())
            }
        });

        if negative_num % 2 != 0 {
            *mean_res = f64::NAN;
        }

        return Errors::ClErrorCodeNoError;
    }
    ///calculate the arithmetic mean value of an array,if the array has NAN/INF values,the result will be NAN
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///
    ///let data = vec![
    ///   -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
    ///   1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
    ///   1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
    ///   1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err = mpt.mean_arithmetic(&mut res);
    ///assert_eq!(
    ///   err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, -0.85194),
    ///   true
    ///);
    ///```
    pub fn mean_arithmetic(&self, mean_res: &mut f64) -> Errors {
        *mean_res = f64::NAN;

        let mut sum = 0.0;
        let mut count = 0;
        if self
            .values
            .iter()
            .try_for_each(|x| {
                if !x.is_finite() {
                    return ControlFlow::Break(());
                }
                sum += x;
                count += 1;
                ControlFlow::Continue(())
            })
            .is_break()
        {
            return Errors::ClErrorCodeNoError;
        }
        if count > 0 {
            *mean_res = sum / count as f64
        }

        return Errors::ClErrorCodeNoError;
    }
    ///calculate the annulized arithmetic mean value of an array, if the array has NAN/INF values,the result will be NAN
    ///# Arguments
    ///freq: the frequence of source data.
    ///
    ///is_annu: the flag of annualize.
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///
    ///let data = vec![
    ///   -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
    ///   1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
    ///   1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
    ///   1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err = mpt.mean_arithmetic_annu(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
    ///assert_eq!(
    ///   err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, -10.223263),
    ///   true
    ///);
    ///```
    pub fn mean_arithmetic_annu(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        mean_res: &mut f64,
    ) -> Errors {
        *mean_res = f64::NAN;

        self.mean_arithmetic(mean_res);

        if is_annu {
            *mean_res *= get_annual_multiplier(freq, false);
        }
        return Errors::ClErrorCodeNoError;
    }

    fn loss_gain_standard_deviation(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        cmp_fn: fn(f64, f64) -> bool,
        loss_standard_deviation: &mut f64,
    ) -> Errors {
        if self.values.len() == 0 || is_annu && !is_valid_frequency(freq) {
            return Errors::ClErrorCodeInvalidPara;
        }
        *loss_standard_deviation = f64::NAN;
        let mut filter_values = Vec::with_capacity(self.values.len());

        if self
            .values
            .iter()
            .try_for_each(|x| {
                if !x.is_finite() {
                    return ControlFlow::Break(());
                }
                if cmp_fn(*x, 0.0) {
                    filter_values.push(*x);
                }
                ControlFlow::Continue(())
            })
            .is_break()
        {
            return Errors::ClErrorCodeNoError;
        }

        return Self::standard_deviation_internal(
            &filter_values,
            freq,
            is_annu,
            loss_standard_deviation,
        );
    }
    ///calculate the gain standard deviation value of an array, if the array has NAN/INF values,the result will be NAN
    ///
    ///# Arguments
    ///freq: the frequence of source data.
    ///
    ///is_annu: the flag of annuize.
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///   -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
    ///   1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
    ///   1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
    ///   1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err =
    ///mpt.gain_standard_deviation(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
    ///assert_eq!(
    ///err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 5.03185),
    ///true
    ///);
    ///```
    pub fn gain_standard_deviation(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        dev_res: &mut f64,
    ) -> Errors {
        return self.loss_gain_standard_deviation(freq, is_annu, |a, b| a > b, dev_res);
    }

    ///calculate the loss standard deviation value of an array, if the array has NAN/INF values,the result will be NAN
    ///
    ///# Arguments
    ///freq: the frequence of source data.
    ///
    ///is_annu: the flag of annuize.
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///   -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
    ///   1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
    ///   1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
    ///   1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err = mpt.loss_standard_deviation(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
    ///assert_eq!(
    ///   err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 14.88251),
    ///   true
    ///);
    ///```
    pub fn loss_standard_deviation(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        dev_res: &mut f64,
    ) -> Errors {
        return self.loss_gain_standard_deviation(freq, is_annu, |a, b| a < b, dev_res);
    }

    ///calculate the semi standard deviation value of an array, if the array has NAN/INF values,the result will be NAN
    ///
    ///# Arguments
    ///freq: the frequence of source data.
    ///
    ///is_annu: the flag of annuize.
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///   -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
    ///   1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
    ///   1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
    ///   1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err =
    ///mpt.semi_standard_deviation(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
    ///assert_eq!(
    ///err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 13.22398),
    ///true
    ///);
    ///```
    pub fn semi_standard_deviation(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        dev_res: &mut f64,
    ) -> Errors {
        if self.values.len() == 0 || is_annu && !is_valid_frequency(freq) {
            return Errors::ClErrorCodeInvalidPara;
        }
        *dev_res = f64::NAN;
        let mut mean_res = f64::NAN;
        let ret = self.mean_arithmetic(&mut mean_res);
        if ret != Errors::ClErrorCodeNoError {
            return Errors::ClErrorCodeNoError;
        }
        let mut sum_return = 0.0;

        if self
            .values
            .iter()
            .try_for_each(|x| {
                if !x.is_finite() {
                    return ControlFlow::Break(());
                }
                if *x < mean_res {
                    sum_return += (*x - mean_res) * (*x - mean_res);
                }
                ControlFlow::Continue(())
            })
            .is_break()
        {
            return Errors::ClErrorCodeNoError;
        }

        *dev_res = (sum_return / (self.values.len() - 1) as f64).sqrt();
        if is_annu {
            *dev_res *= (get_annual_multiplier(freq, false)).sqrt();
        }
        return Errors::ClErrorCodeNoError;
    }

    ///calculate the weighted standard deviation value of an array，if the array or weights has NAN/INF values,the result will be NAN
    ///# Arguments
    ///weights: the weights for values
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///
    ///let data = vec![
    ///   1.22072, 0.0668, 2.20588, 0.91563, 0.76766, 1.21429, 3.43456, 4.99825, 3.89481,
    ///   1.59564, 0.86793, 2.41477, 1.80305, 0.6709, 3.57769, 4.77481, 0.37317, 3.52713,
    ///   1.88831, 1.73502, 1.20155, 3.36542, 2.03551, 5.6145, 2.71663, 0.04815, 3.99807,
    ///   1.66744, 9.68658, 0.46681, 4.22095, 6.7, 15.27331, 8.46123, 0.76369, 10.32347,
    ///];
    ///let weighting = vec![
    ///       3.683070486,2.698835031,2.615091784,2.829245119,4.197477687,
    ///       3.747731115,1.428980992,1.490970258,3.776323531,1.126182408,
    ///       4.589706355,2.213203472,3.290841193,1.574023637,2.7073515,
    ///       2.067657476,2.715387407,3.782522676,4.737767273,3.587905857,
    ///       1.00234693,3.598129659,2.182956354,2.399354298,0.893462788,
    ///       1.636175797,1.182474797,4.58802791,3.983018253,4.741795995,
    ///       2.837587798,2.613364024,4.084667264,0.443121313,1.119531868,
    ///       3.833709695,
    ///   ];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err = mpt.weighted_standard_deviation(&weighting,&mut res);
    ///assert_eq!(
    ///   err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 3.586653428),
    ///   true
    ///);
    ///```
    pub fn weighted_standard_deviation(&self, weights: &[f64], dev_res: &mut f64) -> Errors {
        if self.values.len() == 0 || weights.len() == 0 || self.values.len() != weights.len() {
            return Errors::ClErrorCodeInvalidPara;
        }
        *dev_res = f64::NAN;

        let sum_weight: f64 = weights.iter().filter(|x| (**x).is_finite()).sum();

        let mut mean_res = 0.0;
        let res = self.weighted_mean_arithmetic(weights, &mut mean_res);
        if res != Errors::ClErrorCodeNoError || !mean_res.is_finite() {
            return Errors::ClErrorCodeNoError;
        }

        let excess_sum = self.values.iter().enumerate().fold(0.0, |acc, v| {
            acc + weights[v.0] * (v.1 - mean_res) * (v.1 - mean_res)
        });

        if sum_weight != 0.0 {
            *dev_res = (excess_sum / sum_weight).sqrt();
        }
        return Errors::ClErrorCodeNoError;
    }
    ///calculate the skewness value of an array, if the array has NAN/INF values,the result will be NAN
    ///
    ///# Arguments
    ///freq: the frequence of source data.
    ///
    ///is_annu: the flag of annuize.
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///   -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
    ///   1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
    ///   1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
    ///   1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err =
    ///mpt.skewness(&mut res);
    ///assert_eq!(
    ///err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, -1.31604),
    ///true
    ///);
    ///```
    pub fn skewness(&self, skewness: &mut f64) -> Errors {
        if self.values.len() == 0 {
            return Errors::ClErrorCodeInvalidPara;
        }
        *skewness = f64::NAN;

        let mut mean_res = 0.0;
        let res = self.average(&mut mean_res);
        if res != Errors::ClErrorCodeNoError || !mean_res.is_finite() {
            return Errors::ClErrorCodeNoError;
        }
        struct SkewnessData {
            count: i32,
            sum: f64,
            sum_distance: f64,
        }

        let dis_sum = self.values.iter().fold(
            SkewnessData {
                sum: 0.0,
                count: 0,
                sum_distance: 0.0,
            },
            |acc, v| {
                let dis = v - mean_res;
                SkewnessData {
                    count: acc.count + 1,
                    sum: acc.sum + dis * dis,
                    sum_distance: acc.sum_distance + dis * dis * dis,
                }
            },
        );

        if dis_sum.count <= 2 {
            *skewness = f64::NAN;
        } else {
            let std_dev = (dis_sum.sum / (dis_sum.count - 1) as f64).sqrt();
            if !std_dev.is_finite() {
                *skewness = f64::NAN;
            } else {
                *skewness = dis_sum.sum_distance
                    / (dis_sum.count - 1) as f64
                    / (dis_sum.count - 2) as f64
                    / std_dev
                    / std_dev
                    / std_dev
                    * dis_sum.count as f64;
            }
        }
        return Errors::ClErrorCodeNoError;
    }

    ///calculate the kurtosis value of an array, if the array has NAN/INF values,the result will be NAN
    ///
    ///# Arguments
    ///freq: the frequence of source data.
    ///
    ///is_annu: the flag of annuize.
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///   -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
    ///   1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
    ///   1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
    ///   1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err =
    ///mpt.kurtosis(&mut res);
    ///assert_eq!(
    ///err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 1.76946),
    ///true
    ///);
    ///```
    pub fn kurtosis(&self, kurtosis: &mut f64) -> Errors {
        if self.values.len() == 0 {
            return Errors::ClErrorCodeInvalidPara;
        }
        *kurtosis = f64::NAN;

        let mut mean_res = 0.0;
        let res = self.average(&mut mean_res);
        if res != Errors::ClErrorCodeNoError || !mean_res.is_finite() {
            return Errors::ClErrorCodeNoError;
        }
        struct KurtosisData {
            count: i32,
            sum: f64,
            sum_distance: f64,
        }

        let dis_sum = self.values.iter().fold(
            KurtosisData {
                sum: 0.0,
                count: 0,
                sum_distance: 0.0,
            },
            |acc, v| {
                let dis = (v - mean_res) * (v - mean_res);
                KurtosisData {
                    count: acc.count + 1,
                    sum: acc.sum + dis,
                    sum_distance: acc.sum_distance + dis * dis,
                }
            },
        );

        if dis_sum.count <= 3 {
            *kurtosis = f64::NAN;
        } else {
            let std_dev = (dis_sum.sum / (dis_sum.count - 1) as f64).sqrt();
            if !std_dev.is_finite() {
                *kurtosis = f64::NAN;
            } else {
                *kurtosis = dis_sum.sum_distance
                    / (dis_sum.count - 1) as f64
                    / (dis_sum.count - 2) as f64
                    / (dis_sum.count - 3) as f64
                    / std_dev
                    / std_dev
                    / std_dev
                    / std_dev
                    * dis_sum.count as f64
                    * (dis_sum.count + 1) as f64;

                *kurtosis -= 3.0 * (dis_sum.count - 1) as f64 * (dis_sum.count - 1) as f64
                    / ((dis_sum.count - 2) as f64 * (dis_sum.count - 3) as f64);
            }
        }
        return Errors::ClErrorCodeNoError;
    }

    fn calc_sharpe_ratio(
        is_annu: bool,
        total_return: f64,
        std_dev: f64,
        freq: enums::ClFrequency,
        is_israelsen: bool,
    ) -> f64 {
        let mut sharpe_ratio_result = f64::NAN;
        if is_israelsen {
            if std_dev != 0.0 {
                if total_return > 0.0 {
                    sharpe_ratio_result = total_return * std_dev;
                } else {
                    sharpe_ratio_result = total_return / std_dev;
                }

                if is_annu {
                    sharpe_ratio_result =
                        (sharpe_ratio_result) * get_annual_multiplier(freq, false).sqrt()
                }
            }
        } else {
            sharpe_ratio_result = total_return / std_dev;
            if is_annu {
                sharpe_ratio_result =
                    (sharpe_ratio_result) * get_annual_multiplier(freq, false).sqrt()
            }
        }
        sharpe_ratio_result
    }

    fn sharpe_ratio_common(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        is_israelsen: bool,
        sharpe_ratio_result: &mut f64,
    ) -> Errors {
        *sharpe_ratio_result = f64::NAN;

        let mut avg_excess_return = f64::NAN;
        self.calc_avg_excess_return(&mut avg_excess_return);
        let mut excess_vec = vec![f64::NAN; self.values.len()];
        let mut ret = Self::array_subtraction_internal(self.values, self.riskfree, &mut excess_vec);
        if ret != Errors::ClErrorCodeNoError {
            return ret;
        }
        let mut excess_dev = 0.0;
        ret = Self::standard_deviation_internal(excess_vec.as_ref(), freq, false, &mut excess_dev);

        if ret != Errors::ClErrorCodeNoError {
            return ret;
        }
        *sharpe_ratio_result =
            Self::calc_sharpe_ratio(is_annu, avg_excess_return, excess_dev, freq, is_israelsen);
        return Errors::ClErrorCodeNoError;
    }

    ///calculate the sharpe ratio value of an array,it need riskfree data, if the array and riskfree have NAN/INF values,the result will be NAN
    ///
    ///# Arguments
    ///freq: the frequence of source data.
    ///
    ///is_annu: the flag of annuize.
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///   -1.76334, -3.7317, -0.49068, 11.83432, 9.08289, 3.39531, 0.70368, 0.89286, -0.76953,
    ///  6.39783, 1.38484, 2.33645, 2.80998, 0.5808, -0.61141, -0.20506, -0.47945, -0.13765,
    ///   -3.4459, -0.85653, 1.83585, 0.84836, 3.61024, 3.99188, -1.7892, 2.02054, -0.81169,
    ///   -1.40753, 3.02125, -0.67676, 1.07073, -2.21509, 0.29547, -2.65139, 2.62273, -0.65557,
    ///   0.76463, -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825,
    ///   3.89481, 1.59564, 0.86793,
    ///];
    ///let rf_data = vec![
    ///   0.10075, 0.0999, 0.09735, 0.0982, 0.09311, 0.08124, 0.07785, 0.08209, 0.08124, 0.07955,
    ///   0.0804, 0.07701, 0.07701, 0.07955, 0.0804, 0.0804, 0.08887, 0.10923, 0.11602, 0.12791,
    ///   0.14235, 0.15085, 0.17806, 0.19083, 0.20105, 0.21894, 0.23855, 0.24111, 0.24708,
    ///   0.25903, 0.27868, 0.30004, 0.3009, 0.32143, 0.34026, 0.33884, 0.36586, 0.38497,
    ///   0.39406, 0.40057, 0.41237, 0.41911, 0.43358, 0.43548, 0.42107, 0.42743, 0.43278,
    ///   0.4235,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v_r(&data, &rf_data);
    ///let err =
    ///mpt.sharpe_ratio(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
    ///assert_eq!(
    ///err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 0.94596),
    ///true
    ///);

    ///```
    pub fn sharpe_ratio(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        sharpe_ratio_result: &mut f64,
    ) -> Errors {
        return self.sharpe_ratio_common(freq, is_annu, false, sharpe_ratio_result);
    }

    fn calc_sharpe_ratio_arithmetic(
        is_annu: bool,
        total_return: f64,
        rf_total_return: f64,
        std_dev: f64,
        is_israelsen: bool,
    ) -> f64 {
        let mut sharpe_ratio_result = f64::NAN;
        if is_israelsen {
            if is_annu {
                if std_dev != 0.0 {
                    if total_return < rf_total_return {
                        sharpe_ratio_result = (total_return - rf_total_return) * std_dev;
                    } else {
                        sharpe_ratio_result = (total_return - rf_total_return) / std_dev;
                    }
                }
            } else {
                if std_dev != 0.0 {
                    if total_return < 0.0 {
                        sharpe_ratio_result = total_return * std_dev;
                    } else {
                        sharpe_ratio_result = total_return / std_dev;
                    }
                }
            }
        } else {
            if is_annu {
                if std_dev != 0.0 {
                    sharpe_ratio_result = (total_return - rf_total_return) / std_dev;
                }
            } else {
                if std_dev != 0.0 {
                    sharpe_ratio_result = total_return / std_dev;
                }
            }
        }
        sharpe_ratio_result
    }

    fn sharpe_ratio_arithmetic_common(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        is_israelsen: bool,
        sharpe_ratio_arithmetic: &mut f64,
    ) -> Errors {
        if self.values.len() == 0
            || self.riskfree.len() == 0
            || is_annu && !is_valid_frequency(freq)
        {
            return Errors::ClErrorCodeInvalidPara;
        }
        *sharpe_ratio_arithmetic = f64::NAN;

        if is_annu {
            let mut annu_total_return = f64::NAN;
            let mut annu_rf_total_return = f64::NAN;

            if Self::calc_annu_total_return(
                self.values,
                self.riskfree,
                freq,
                &mut annu_total_return,
                &mut annu_rf_total_return,
            ) != Errors::ClErrorCodeNoError
            {
                return Errors::ClErrorCodeNoError;
            }
            let mut annu_std_dev = f64::NAN;
            self.standard_deviation(freq, true, &mut annu_std_dev);

            *sharpe_ratio_arithmetic = Self::calc_sharpe_ratio_arithmetic(
                is_annu,
                annu_total_return,
                annu_rf_total_return,
                annu_std_dev,
                is_israelsen,
            );
        } else {
            let mut avg_excess_return = f64::NAN;
            if self.calc_avg_excess_return(&mut avg_excess_return) != Errors::ClErrorCodeNoError {
                return Errors::ClErrorCodeNoError;
            }
            let mut annu_std_dev = f64::NAN;
            self.standard_deviation(freq, true, &mut annu_std_dev);
            *sharpe_ratio_arithmetic = Self::calc_sharpe_ratio_arithmetic(
                is_annu,
                avg_excess_return,
                f64::NAN,
                annu_std_dev,
                is_israelsen,
            );
        }

        return Errors::ClErrorCodeNoError;
    }
    ///calculate the sharpe ratio arithmetic value of an array, it need riskfree data, if the array and riskfree have NAN/INF values,the result will be NAN
    ///
    ///# Arguments
    ///freq: the frequence of source data.
    ///
    ///is_annu: the flag of annuize.
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///   -1.76334, -3.7317, -0.49068, 11.83432, 9.08289, 3.39531, 0.70368, 0.89286, -0.76953,
    ///  6.39783, 1.38484, 2.33645, 2.80998, 0.5808, -0.61141, -0.20506, -0.47945, -0.13765,
    ///   -3.4459, -0.85653, 1.83585, 0.84836, 3.61024, 3.99188, -1.7892, 2.02054, -0.81169,
    ///   -1.40753, 3.02125, -0.67676, 1.07073, -2.21509, 0.29547, -2.65139, 2.62273, -0.65557,
    ///   0.76463, -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825,
    ///   3.89481, 1.59564, 0.86793,
    ///];
    ///let rf_data = vec![
    ///   0.10075, 0.0999, 0.09735, 0.0982, 0.09311, 0.08124, 0.07785, 0.08209, 0.08124, 0.07955,
    ///   0.0804, 0.07701, 0.07701, 0.07955, 0.0804, 0.0804, 0.08887, 0.10923, 0.11602, 0.12791,
    ///   0.14235, 0.15085, 0.17806, 0.19083, 0.20105, 0.21894, 0.23855, 0.24111, 0.24708,
    ///   0.25903, 0.27868, 0.30004, 0.3009, 0.32143, 0.34026, 0.33884, 0.36586, 0.38497,
    ///   0.39406, 0.40057, 0.41237, 0.41911, 0.43358, 0.43548, 0.42107, 0.42743, 0.43278,
    ///   0.4235,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v_r(&data, &rf_data);
    ///let err =
    ///mpt.sharpe_ratio_arithmetic(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
    ///assert_eq!(
    ///err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 0.96502),
    ///true
    ///);
    ///```
    pub fn sharpe_ratio_arithmetic(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        sharpe_ratio_arithmetic: &mut f64,
    ) -> Errors {
        return self.sharpe_ratio_arithmetic_common(freq, is_annu, false, sharpe_ratio_arithmetic);
    }

    fn calc_sharpe_ratio_geometric(
        total_return: f64,
        rf_total_return: f64,
        std_dev: f64,
        is_israelsen: bool,
    ) -> f64 {
        let mut share_ration_res = f64::NAN;
        if is_israelsen {
            if std_dev != 0.0 {
                let ret = (100.0 + total_return) / (100.0 + rf_total_return) - 1.0;
                if ret < 0.0 {
                    share_ration_res = ret * 100.0 * std_dev;
                } else {
                    share_ration_res = ret * 100.0 / std_dev;
                }
            }
        } else {
            if std_dev != 0.0 {
                share_ration_res =
                    ((100.0 + total_return) / (100.0 + rf_total_return) - 1.0) * 100.0 / std_dev;
            }
        }
        share_ration_res
    }
    fn sharpe_ratio_geometric_common(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        is_israelsen: bool,
        sharpe_ratio_result: &mut f64,
    ) -> Errors {
        if self.values.len() == 0
            || self.riskfree.len() == 0
            || is_annu && !is_valid_frequency(freq)
        {
            return Errors::ClErrorCodeInvalidPara;
        }
        *sharpe_ratio_result = f64::NAN;

        let mut total_return = f64::NAN;
        let mut rf_total_return = f64::NAN;
        Self::total_return_accumulat(self.values, &mut total_return);
        Self::total_return_accumulat(self.riskfree, &mut rf_total_return);

        if !total_return.is_finite() || !rf_total_return.is_finite() {
            return Errors::ClErrorCodeCcFaild;
        }
        if is_annu {
            total_return = annualize_return(total_return, freq, self.values.len() as f64, true);
            rf_total_return =
                annualize_return(rf_total_return, freq, self.values.len() as f64, true);
        }
        let mut std_dev = f64::NAN;
        self.standard_deviation(freq, is_annu, &mut std_dev);
        *sharpe_ratio_result =
            Self::calc_sharpe_ratio_geometric(total_return, rf_total_return, std_dev, is_israelsen);
        return Errors::ClErrorCodeNoError;
    }

    ///calculate the sharpe ratio geometric value of an array, it need riskfree data, if the array and riskfree have NAN/INF values,the result will be NAN
    ///
    ///# Arguments
    ///freq: the frequence of source data.
    ///
    ///is_annu: the flag of annuize.
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///    -1.76334, -3.7317, -0.49068, 11.83432, 9.08289, 3.39531, 0.70368, 0.89286, -0.76953,
    ///   6.39783, 1.38484, 2.33645, 2.80998, 0.5808, -0.61141, -0.20506, -0.47945, -0.13765,
    ///    -3.4459, -0.85653, 1.83585, 0.84836, 3.61024, 3.99188, -1.7892, 2.02054, -0.81169,
    ///    -1.40753, 3.02125, -0.67676, 1.07073, -2.21509, 0.29547, -2.65139, 2.62273, -0.65557,
    ///    0.76463, -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825,
    ///    3.89481, 1.59564, 0.86793,
    ///];
    ///let rf_data = vec![
    ///    0.10075, 0.0999, 0.09735, 0.0982, 0.09311, 0.08124, 0.07785, 0.08209, 0.08124, 0.07955,
    ///    0.0804, 0.07701, 0.07701, 0.07955, 0.0804, 0.0804, 0.08887, 0.10923, 0.11602, 0.12791,
    ///    0.14235, 0.15085, 0.17806, 0.19083, 0.20105, 0.21894, 0.23855, 0.24111, 0.24708,
    ///    0.25903, 0.27868, 0.30004, 0.3009, 0.32143, 0.34026, 0.33884, 0.36586, 0.38497,
    ///    0.39406, 0.40057, 0.41237, 0.41911, 0.43358, 0.43548, 0.42107, 0.42743, 0.43278,
    ///    0.4235,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v_r(&data, &rf_data);
    ///let err =
    ///mpt.sharpe_ratio_geometric(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
    ///assert_eq!(
    ///err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 0.93957),
    ///true
    ///);
    ///```
    pub fn sharpe_ratio_geometric(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        sharpe_ratio_result: &mut f64,
    ) -> Errors {
        return self.sharpe_ratio_geometric_common(freq, is_annu, false, sharpe_ratio_result);
    }

    fn up_downside_deviation(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        cmp_fn: fn(f64, f64) -> bool,
        downside_deviation: &mut f64,
    ) -> Errors {
        if self.values.len() == 0
            || self.benchmark.len() == 0
            || is_annu && !is_valid_frequency(freq)
        {
            return Errors::ClErrorCodeInvalidPara;
        }

        *downside_deviation = f64::NAN;
        let mut sum_return = 0.0;
        let mut count = 0;
        if self
            .values
            .iter()
            .enumerate()
            .try_for_each(|v| {
                if !v.1.is_finite() || !self.benchmark[v.0].is_finite() {
                    return ControlFlow::Break(());
                }
                sum_return += if cmp_fn(*v.1, self.benchmark[v.0]) {
                    (*v.1 - self.benchmark[v.0]) * (*v.1 - self.benchmark[v.0])
                } else {
                    0.0
                };
                count += 1;
                ControlFlow::Continue(())
            })
            .is_break()
        {
            return Errors::ClErrorCodeNoError;
        }

        if count > 0 {
            *downside_deviation = (sum_return / count as f64).sqrt();
            if is_annu {
                *downside_deviation *= get_annual_multiplier(freq, false).sqrt();
            }
        }
        return Errors::ClErrorCodeNoError;
    }

    pub fn downside_deviation(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        downside_deviation: &mut f64,
    ) -> Errors {
        return self.up_downside_deviation(freq, is_annu, |a, b| a < b, downside_deviation);
    }

    pub fn upside_deviation(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        downside_deviation: &mut f64,
    ) -> Errors {
        return self.up_downside_deviation(freq, is_annu, |a, b| a > b, downside_deviation);
    }

    fn calc_sortino_ratio(
        is_annu: bool,
        total_return: f64,
        down_side_stddev: f64,
        freq: enums::ClFrequency,
    ) -> f64 {
        let mut downside_ratio_result = f64::NAN;
        if down_side_stddev.is_finite() && down_side_stddev != 0.0 {
            downside_ratio_result = total_return / down_side_stddev;

            if is_annu {
                downside_ratio_result =
                    (downside_ratio_result) * get_annual_multiplier(freq, false).sqrt()
            }
        }

        downside_ratio_result
    }
    ///calculate the sortino ratio value of an array, it need riskfree data, if the array and riskfree have NAN/INF values,the result will be NAN
    ///
    ///# Arguments
    ///freq: the frequence of source data.
    ///
    ///is_annu: the flag of annuize.
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///    -1.76334, -3.7317, -0.49068, 11.83432, 9.08289, 3.39531, 0.70368, 0.89286, -0.76953,
    ///    6.39783, 1.38484, 2.33645, 2.80998, 0.5808, -0.61141, -0.20506, -0.47945, -0.13765,
    ///    -3.4459, -0.85653, 1.83585, 0.84836, 3.61024, 3.99188, -1.7892, 2.02054, -0.81169,
    ///    -1.40753, 3.02125, -0.67676, 1.07073, -2.21509, 0.29547, -2.65139, 2.62273, -0.65557,
    ///    0.76463, -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825,
    ///    3.89481, 1.59564, 0.86793,
    ///];
    ///let rf_data = vec![
    ///    0.10075, 0.0999, 0.09735, 0.0982, 0.09311, 0.08124, 0.07785, 0.08209, 0.08124, 0.07955,
    ///    0.0804, 0.07701, 0.07701, 0.07955, 0.0804, 0.0804, 0.08887, 0.10923, 0.11602, 0.12791,
    ///    0.14235, 0.15085, 0.17806, 0.19083, 0.20105, 0.21894, 0.23855, 0.24111, 0.24708,
    ///    0.25903, 0.27868, 0.30004, 0.3009, 0.32143, 0.34026, 0.33884, 0.36586, 0.38497,
    ///    0.39406, 0.40057, 0.41237, 0.41911, 0.43358, 0.43548, 0.42107, 0.42743, 0.43278,
    ///    0.4235,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v_r(&data, &rf_data);
    ///let err = mpt.sortino_ratio(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 2.37108),
    ///    true
    ///);
    ///```
    pub fn sortino_ratio(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        sortino_ratio_result: &mut f64,
    ) -> Errors {
        *sortino_ratio_result = f64::NAN;

        let mut avg_excess_return = f64::NAN;
        self.calc_avg_excess_return(&mut avg_excess_return);
        let mut down_side_dev = 0.0;
        let ret = MPTCalculator::from_v_b(self.values, self.riskfree).downside_deviation(
            freq,
            false,
            &mut down_side_dev,
        );

        if ret != Errors::ClErrorCodeNoError {
            return ret;
        }
        *sortino_ratio_result =
            Self::calc_sortino_ratio(is_annu, avg_excess_return, down_side_dev, freq);
        return Errors::ClErrorCodeNoError;
    }

    ///calculate the sortino ratio arithmetic value of an array, it need riskfree data, if the array and riskfree have NAN/INF values,the result will be NAN
    ///
    ///# Arguments
    ///freq: the frequence of source data.
    ///
    ///is_annu: the flag of annuize.
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///    -1.76334, -3.7317, -0.49068, 11.83432, 9.08289, 3.39531, 0.70368, 0.89286, -0.76953,
    ///    6.39783, 1.38484, 2.33645, 2.80998, 0.5808, -0.61141, -0.20506, -0.47945, -0.13765,
    ///    -3.4459, -0.85653, 1.83585, 0.84836, 3.61024, 3.99188, -1.7892, 2.02054, -0.81169,
    ///    -1.40753, 3.02125, -0.67676, 1.07073, -2.21509, 0.29547, -2.65139, 2.62273, -0.65557,
    ///    0.76463, -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825,
    ///    3.89481, 1.59564, 0.86793,
    ///];
    ///let rf_data = vec![
    ///    0.10075, 0.0999, 0.09735, 0.0982, 0.09311, 0.08124, 0.07785, 0.08209, 0.08124, 0.07955,
    ///    0.0804, 0.07701, 0.07701, 0.07955, 0.0804, 0.0804, 0.08887, 0.10923, 0.11602, 0.12791,
    ///    0.14235, 0.15085, 0.17806, 0.19083, 0.20105, 0.21894, 0.23855, 0.24111, 0.24708,
    ///    0.25903, 0.27868, 0.30004, 0.3009, 0.32143, 0.34026, 0.33884, 0.36586, 0.38497,
    ///    0.39406, 0.40057, 0.41237, 0.41911, 0.43358, 0.43548, 0.42107, 0.42743, 0.43278,
    ///    0.4235,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v_r(&data, &rf_data);
    ///let err =
    ///    mpt.sortino_ratio_arithmetic(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 0.96502248),
    ///    true
    ///);
    ///```
    pub fn sortino_ratio_arithmetic(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        sortino_ratio_res: &mut f64,
    ) -> Errors {
        if self.values.len() == 0
            || self.riskfree.len() == 0
            || is_annu && !is_valid_frequency(freq)
        {
            return Errors::ClErrorCodeInvalidPara;
        }
        *sortino_ratio_res = f64::NAN;

        if is_annu {
            let mut annu_total_return = f64::NAN;
            let mut annu_rf_total_return = f64::NAN;

            if Self::calc_annu_total_return(
                self.values,
                self.riskfree,
                freq,
                &mut annu_total_return,
                &mut annu_rf_total_return,
            ) != Errors::ClErrorCodeNoError
            {
                return Errors::ClErrorCodeNoError;
            }
            let mut std_dev = f64::NAN;
            self.standard_deviation(freq, true, &mut std_dev);

            if std_dev != 0.0 {
                *sortino_ratio_res = (annu_total_return - annu_rf_total_return) / std_dev;
            }
        } else {
            let mut avg_excess_return = f64::NAN;
            self.calc_avg_excess_return(&mut avg_excess_return);
            let mut std_dev = f64::NAN;
            self.standard_deviation(freq, false, &mut std_dev);
            if std_dev != 0.0 {
                *sortino_ratio_res = avg_excess_return / std_dev;
            }
        }

        return Errors::ClErrorCodeNoError;
    }

    ///calculate the sortino ratio geometric value of an array, it need riskfree data, if the array and riskfree have NAN/INF values,the result will be NAN
    ///
    ///# Arguments
    ///freq: the frequence of source data.
    ///
    ///is_annu: the flag of annuize.
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///    -1.76334, -3.7317, -0.49068, 11.83432, 9.08289, 3.39531, 0.70368, 0.89286, -0.76953,
    ///    6.39783, 1.38484, 2.33645, 2.80998, 0.5808, -0.61141, -0.20506, -0.47945, -0.13765,
    ///    -3.4459, -0.85653, 1.83585, 0.84836, 3.61024, 3.99188, -1.7892, 2.02054, -0.81169,
    ///    -1.40753, 3.02125, -0.67676, 1.07073, -2.21509, 0.29547, -2.65139, 2.62273, -0.65557,
    ///    0.76463, -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825,
    ///    3.89481, 1.59564, 0.86793,
    ///];
    ///let rf_data = vec![
    ///    0.10075, 0.0999, 0.09735, 0.0982, 0.09311, 0.08124, 0.07785, 0.08209, 0.08124, 0.07955,
    ///    0.0804, 0.07701, 0.07701, 0.07955, 0.0804, 0.0804, 0.08887, 0.10923, 0.11602, 0.12791,
    ///    0.14235, 0.15085, 0.17806, 0.19083, 0.20105, 0.21894, 0.23855, 0.24111, 0.24708,
    ///    0.25903, 0.27868, 0.30004, 0.3009, 0.32143, 0.34026, 0.33884, 0.36586, 0.38497,
    ///    0.39406, 0.40057, 0.41237, 0.41911, 0.43358, 0.43548, 0.42107, 0.42743, 0.43278,
    ///    0.4235,
    /// ];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v_r(&data, &rf_data);
    ///let err =
    ///    mpt.sortino_ratio_geometric(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 2.34312),
    ///    true
    ///);
    ///```
    pub fn sortino_ratio_geometric(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        sortino_ratio_result: &mut f64,
    ) -> Errors {
        if self.values.len() == 0
            || self.riskfree.len() == 0
            || is_annu && !is_valid_frequency(freq)
        {
            return Errors::ClErrorCodeInvalidPara;
        }
        *sortino_ratio_result = f64::NAN;

        let mut total_return = f64::NAN;
        let mut rf_total_return = f64::NAN;
        Self::total_return_accumulat(self.values, &mut total_return);
        Self::total_return_accumulat(self.riskfree, &mut rf_total_return);

        if !total_return.is_finite() || !rf_total_return.is_finite() {
            return Errors::ClErrorCodeCcFaild;
        }
        if is_annu {
            total_return = annualize_return(total_return, freq, self.values.len() as f64, true);
            rf_total_return =
                annualize_return(rf_total_return, freq, self.values.len() as f64, true);
        }
        let mut std_dev = f64::NAN;
        MPTCalculator::from_v_b(self.values, self.riskfree).downside_deviation(
            freq,
            is_annu,
            &mut std_dev,
        );
        *sortino_ratio_result =
            Self::calc_sharpe_ratio_geometric(total_return, rf_total_return, std_dev, false);
        return Errors::ClErrorCodeNoError;
    }

    fn calc_lpm(values: &[f64], riskfree: &[f64], rank: f64) -> f64 {
        let mut result = f64::NAN;
        let mut lpms = Vec::with_capacity(values.len());
        if values
            .iter()
            .enumerate()
            .try_for_each(|v| {
                if !v.1.is_finite() || !riskfree[v.0].is_finite() {
                    return ControlFlow::Break(());
                }

                if riskfree[v.0] > *v.1 {
                    lpms.push(riskfree[v.0] - v.1);
                } else {
                    lpms.push(0.0);
                }
                ControlFlow::Continue(())
            })
            .is_break()
        {
            return result;
        }

        result = lpms.iter().fold(0.0, |acc, x| acc + x.powf(rank));
        result /= values.len() as f64;
        result
    }

    fn excess_mean(
        values: &[f64],
        riskfree: &[f64],
        excess_mean_res: &mut f64,
        count: &mut i32,
    ) -> Errors {
        *excess_mean_res = 0.0;
        *count = 0;

        if values
            .iter()
            .enumerate()
            .try_for_each(|v| {
                if !v.1.is_finite() || !riskfree[v.0].is_finite() {
                    return ControlFlow::Break(());
                }
                *excess_mean_res += v.1 - riskfree[v.0];
                *count += 1;
                ControlFlow::Continue(())
            })
            .is_break()
        {
            return Errors::ClErrorCodeNoError;
        }
        return Errors::ClErrorCodeNoError;
    }
    ///calculate the omega value of an array, it need riskfree data, if the array and riskfree have NAN/INF values,the result will be NAN
    ///
    ///# Arguments
    ///freq: the frequence of source data.
    ///
    ///is_annu: the flag of annuize.
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///    -1.76334, -3.7317, -0.49068, 11.83432, 9.08289, 3.39531, 0.70368, 0.89286, -0.76953,
    ///    6.39783, 1.38484, 2.33645, 2.80998, 0.5808, -0.61141, -0.20506, -0.47945, -0.13765,
    ///    -3.4459, -0.85653, 1.83585, 0.84836, 3.61024, 3.99188, -1.7892, 2.02054, -0.81169,
    ///    -1.40753, 3.02125, -0.67676, 1.07073, -2.21509, 0.29547, -2.65139, 2.62273, -0.65557,
    ///    0.76463, -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825,
    ///    3.89481, 1.59564, 0.86793,
    ///];
    ///let rf_data = vec![
    ///    0.10075, 0.0999, 0.09735, 0.0982, 0.09311, 0.08124, 0.07785, 0.08209, 0.08124, 0.07955,
    ///    0.0804, 0.07701, 0.07701, 0.07955, 0.0804, 0.0804, 0.08887, 0.10923, 0.11602, 0.12791,
    ///    0.14235, 0.15085, 0.17806, 0.19083, 0.20105, 0.21894, 0.23855, 0.24111, 0.24708,
    ///    0.25903, 0.27868, 0.30004, 0.3009, 0.32143, 0.34026, 0.33884, 0.36586, 0.38497,
    ///    0.39406, 0.40057, 0.41237, 0.41911, 0.43358, 0.43548, 0.42107, 0.42743, 0.43278,
    ///    0.4235,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v_r(&data, &rf_data);
    ///let err = mpt.omega(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 2.2412239894355674),
    ///    true
    ///);
    ///```
    pub fn omega(&self, freq: enums::ClFrequency, is_annu: bool, omega_res: &mut f64) -> Errors {
        if self.values.len() == 0
            || self.riskfree.len() == 0
            || is_annu && !is_valid_frequency(freq)
        {
            return Errors::ClErrorCodeInvalidPara;
        }

        let lpm = Self::calc_lpm(self.values, self.riskfree, 1.0);
        *omega_res = f64::NAN;

        if !lpm.is_finite() || lpm == 0.0 {
            return Errors::ClErrorCodeNoError;
        }

        if is_annu {
            let mut annu_total_return = f64::NAN;
            let mut annu_rf_total_return = f64::NAN;

            if Self::calc_annu_total_return(
                self.values,
                self.riskfree,
                freq,
                &mut annu_total_return,
                &mut annu_rf_total_return,
            ) != Errors::ClErrorCodeNoError
            {
                return Errors::ClErrorCodeNoError;
            }

            *omega_res = (annu_total_return - annu_rf_total_return)
                / (lpm * get_annual_multiplier(freq, false))
                + 1.0;
        } else {
            let mut count = 0;
            let mut excess_mean_res = 0.0;
            if Self::excess_mean(self.values, self.riskfree, &mut excess_mean_res, &mut count)
                != Errors::ClErrorCodeNoError
            {
                return Errors::ClErrorCodeNoError;
            }

            *omega_res = excess_mean_res / count as f64 / lpm + 1.0;
        }

        return Errors::ClErrorCodeNoError;
    }
    ///calculate the kapp3 value of an array,it need riskfree data, if the array and riskfree have NAN/INF values,the result will be NAN
    ///
    ///# Arguments
    ///freq: the frequence of source data.
    ///
    ///is_annu: the flag of annuize.
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///    -1.76334, -3.7317, -0.49068, 11.83432, 9.08289, 3.39531, 0.70368, 0.89286, -0.76953,
    ///    6.39783, 1.38484, 2.33645, 2.80998, 0.5808, -0.61141, -0.20506, -0.47945, -0.13765,
    ///    -3.4459, -0.85653, 1.83585, 0.84836, 3.61024, 3.99188, -1.7892, 2.02054, -0.81169,
    ///    -1.40753, 3.02125, -0.67676, 1.07073, -2.21509, 0.29547, -2.65139, 2.62273, -0.65557,
    ///    0.76463, -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825,
    ///    3.89481, 1.59564, 0.86793,
    ///];
    ///let rf_data = vec![
    ///    0.10075, 0.0999, 0.09735, 0.0982, 0.09311, 0.08124, 0.07785, 0.08209, 0.08124, 0.07955,
    ///    0.0804, 0.07701, 0.07701, 0.07955, 0.0804, 0.0804, 0.08887, 0.10923, 0.11602, 0.12791,
    ///    0.14235, 0.15085, 0.17806, 0.19083, 0.20105, 0.21894, 0.23855, 0.24111, 0.24708,
    ///    0.25903, 0.27868, 0.30004, 0.3009, 0.32143, 0.34026, 0.33884, 0.36586, 0.38497,
    ///    0.39406, 0.40057, 0.41237, 0.41911, 0.43358, 0.43548, 0.42107, 0.42743, 0.43278,
    ///    0.4235,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v_r(&data, &rf_data);
    ///let err = mpt.kappa3(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 2.77311069),
    ///    true
    ///);
    ///```
    pub fn kappa3(&self, freq: enums::ClFrequency, is_annu: bool, kappa3_res: &mut f64) -> Errors {
        if self.values.len() == 0
            || self.riskfree.len() == 0
            || is_annu && !is_valid_frequency(freq)
        {
            return Errors::ClErrorCodeInvalidPara;
        }

        let lpm = Self::calc_lpm(self.values, self.riskfree, 3.0);
        *kappa3_res = f64::NAN;

        if !lpm.is_finite() || MPTCalculator::is_eq_double(lpm, 0.0) {
            return Errors::ClErrorCodeNoError;
        }

        if is_annu {
            let mut annu_total_return = f64::NAN;
            let mut annu_rf_total_return = f64::NAN;

            if Self::calc_annu_total_return(
                self.values,
                self.riskfree,
                freq,
                &mut annu_total_return,
                &mut annu_rf_total_return,
            ) != Errors::ClErrorCodeNoError
            {
                return Errors::ClErrorCodeNoError;
            }

            *kappa3_res = (annu_total_return - annu_rf_total_return)
                / (lpm * get_annual_multiplier(freq, false)).powf(1.0 / 3.0);
        } else {
            let mut count = 0;
            let mut excess_mean_res = 0.0;
            if Self::excess_mean(self.values, self.riskfree, &mut excess_mean_res, &mut count)
                != Errors::ClErrorCodeNoError
            {
                return Errors::ClErrorCodeNoError;
            }

            *kappa3_res = excess_mean_res / count as f64 / lpm.powf(1.0 / 3.0);
        }

        return Errors::ClErrorCodeNoError;
    }
    ///calculate the gain loss ratio value of an array, if the array has NAN/INF values,the result will be NAN
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///    -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
    ///    1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
    ///    1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
    ///    1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err = mpt.gain_loss_ratio(&mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 0.58877),
    ///    true
    ///);
    ///```
    pub fn gain_loss_ratio(&self, gain_loss_res: &mut f64) -> Errors {
        if self.values.len() == 0 {
            return Errors::ClErrorCodeInvalidPara;
        }
        *gain_loss_res = f64::NAN;
        let mut sum_gain = 0.0;
        let mut sum_loss = 0.0;
        if self
            .values
            .iter()
            .try_for_each(|x| {
                if !x.is_finite() {
                    return ControlFlow::Break(());
                }
                if *x > 0.0 {
                    sum_gain += *x;
                }
                if *x < 0.0 {
                    sum_loss += *x;
                }
                ControlFlow::Continue(())
            })
            .is_break()
        {
            return Errors::ClErrorCodeNoError;
        }

        if sum_loss != 0.0 {
            *gain_loss_res = -sum_gain / sum_loss;
        }

        return Errors::ClErrorCodeNoError;
    }
    ///calculate the coefficeient viaiantion value of an array, if the array has NAN/INF values,the result will be NAN
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///    -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
    ///    1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
    ///    1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
    ///    1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err = mpt.coefficeient_viaiantion(&mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, -5.41921),
    ///    true
    ///);
    ///```
    pub fn coefficeient_viaiantion(&self, coefficeient_viaiantion_res: &mut f64) -> Errors {
        if self.values.len() == 0 {
            return Errors::ClErrorCodeInvalidPara;
        }

        *coefficeient_viaiantion_res = f64::NAN;
        let mut mean_res = 0.0;
        let mut res = self.mean_arithmetic(&mut mean_res);
        if res != Errors::ClErrorCodeNoError {
            return res;
        }

        let mut std_dev = f64::NAN;
        res = self.standard_deviation(enums::ClFrequency::ClFrequencyMonthly, false, &mut std_dev);
        if res != Errors::ClErrorCodeNoError {
            return res;
        }

        if !std_dev.is_finite()
            || !mean_res.is_finite()
            || MPTCalculator::is_eq_double(mean_res, 0.0)
        {
            *coefficeient_viaiantion_res = f64::NAN;
        } else {
            *coefficeient_viaiantion_res = std_dev / mean_res;
        }
        return Errors::ClErrorCodeNoError;
    }
    ///calculate the efficiency ratio arthmetic value of an array, if the array has NAN/INF values,the result will be NAN
    ///
    ///# Arguments
    ///freq: the frequence of source data.
    ///
    ///is_annu: the flag of annuize.
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///    2.8709, -1.6506, 0.8281, 4.8182, 4.0484, -0.4246, -1.8230, 1.1619, 6.2151, 5.3158,
    ///   -3.7904, 0.3500, -8.9486, -1.6029, -2.1879, 6.5159, 3.0498, -8.3762, -3.9341, -0.0780,
    ///    -17.9807, -21.5895, -11.3292, 4.8884, -7.5447, -7.5943, 13.9102, 13.6679, 6.2313,
    ///    -1.3755, 8.7637, 2.1660, 5.3087, -5.4276, 5.4496, 4.3492,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err =
    ///    mpt.efficiency_ratio_arthmetic(enums::ClFrequency::ClFrequencyMonthly, false, &mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, -0.020986),
    ///    true
    ///);
    ///```
    pub fn efficiency_ratio_arthmetic(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        result: &mut f64,
    ) -> Errors {
        if self.values.len() == 0 {
            return Errors::ClErrorCodeInvalidPara;
        }

        *result = f64::NAN;
        let mut mean_res = 0.0;
        let mut res = self.mean_arithmetic(&mut mean_res);
        if res != Errors::ClErrorCodeNoError {
            return res;
        }

        let mut std_dev = f64::NAN;
        res = self.standard_deviation(enums::ClFrequency::ClFrequencyMonthly, false, &mut std_dev);
        if res != Errors::ClErrorCodeNoError {
            return res;
        }

        if !std_dev.is_finite() || !mean_res.is_finite() || mean_res == 0.0 {
            *result = f64::NAN;
        } else {
            *result = mean_res / std_dev;
        }

        if is_annu {
            *result *= get_annual_multiplier(freq, false).sqrt();
        }

        return Errors::ClErrorCodeNoError;
    }
    ///calculate the jarque_bera value of an array, if the array has NAN/INF values,the result will be NAN
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///    -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
    ///    1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
    ///    1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
    ///    1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err = mpt.jarque_bera(&mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 15.08823),
    ///    true
    ///);
    ///```
    pub fn jarque_bera(&self, jarque_bera: &mut f64) -> Errors {
        if self.values.len() == 0 {
            return Errors::ClErrorCodeInvalidPara;
        }
        *jarque_bera = f64::NAN;
        let mut skewness = f64::NAN;
        let mut kurtosis = f64::NAN;
        let mut ret = self.skewness(&mut skewness);
        if ret != Errors::ClErrorCodeNoError {
            return ret;
        }

        ret = self.kurtosis(&mut kurtosis);
        if ret != Errors::ClErrorCodeNoError {
            return ret;
        }
        *jarque_bera =
            self.values.len() as f64 * (skewness * skewness / 6.0 + kurtosis * kurtosis / 24.0);
        return Errors::ClErrorCodeNoError;
    }
    ///calculate the median value of an array, if the array has NAN/INF values,the result will be NAN
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///    -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
    ///    1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
    ///    1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
    ///    1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err = mpt.median(&mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, -0.057475),
    ///    true
    ///);
    ///```
    pub fn median(&self, result: &mut f64) -> Errors {
        if self.values.len() == 0 {
            return Errors::ClErrorCodeInvalidPara;
        }
        if self.values.iter().find(|x| !x.is_finite()) != None {
            return Errors::ClErrorCodeNoError;
        }
        *result = f64::NAN;

        let mut data = vec![0.0; self.values.len()];
        data.copy_from_slice(&self.values);
        data.sort_by(|a, b| a.total_cmp(b));

        *result = (data[data.len() / 2] + data[(data.len() - 1) / 2]) / 2.0;

        return Errors::ClErrorCodeNoError;
    }
    ///calculate the median weighted value of an array, if the array has NAN/INF values,the result will be NAN
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///    -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
    ///    1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
    ///    1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
    ///    1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err = mpt.median_weighted(&mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, -0.057475),
    ///    true
    ///);
    ///```
    pub fn median_weighted(&self, result: &mut f64) -> Errors {
        if self.values.len() == 0 {
            return Errors::ClErrorCodeInvalidPara;
        }
        if self.values.iter().find(|x| !x.is_finite()) != None {
            return Errors::ClErrorCodeNoError;
        }
        let mut data = vec![0.0; self.values.len()];
        data.copy_from_slice(&self.values);
        data.sort_by(|a, b| a.total_cmp(b));

        *result = f64::NAN;
        if data.len() % 2 == 0 {
            let mut i = (data.len() / 2) - 1;
            let mut sum = data[i] + data[i + 1];
            let mut count = 2;
            while i > 0 && MPTCalculator::is_eq_double(data[i], data[i - 1]) {
                sum += data[i - 1];
                count += 1;
            }

            i = data.len() / 2;
            while (i + 1) < data.len() && MPTCalculator::is_eq_double(data[i], data[i + 1]) {
                sum += data[i + 1];
                i += 1;
                count += 1;
            }

            *result = sum / count as f64;
        } else {
            *result = data[(data.len() + 1) / 2 - 1];
        }

        return Errors::ClErrorCodeNoError;
    }

    fn up_down_month_percent(
        &self,
        cmp_fn: fn(f64, f64) -> bool,
        up_number_res: &mut f64,
    ) -> Errors {
        if self.values.len() == 0 {
            return Errors::ClErrorCodeInvalidPara;
        }
        *up_number_res = f64::NAN;
        let mut count = 0;
        if self
            .values
            .iter()
            .try_for_each(|x| {
                if !x.is_finite() {
                    return ControlFlow::Break(());
                }
                if cmp_fn(*x, 0.0) {
                    count += 1;
                }

                ControlFlow::Continue(())
            })
            .is_break()
        {
            return Errors::ClErrorCodeNoError;
        }
        *up_number_res = count as f64 / self.values.len() as f64 * 100.0;
        return Errors::ClErrorCodeNoError;
    }
    ///calculate the up month percent value of an array, if the array has NAN/INF values,the result will be NAN
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///    -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
    ///    1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
    ///    1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
    ///    1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err = mpt.up_month_percent(&mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 47.22222),
    ///    true
    ///);
    ///```
    pub fn up_month_percent(&self, up_number_res: &mut f64) -> Errors {
        return self.up_down_month_percent(|a, b| a >= b, up_number_res);
    }

    ///calculate the up month percent value of an array, if the array has NAN/INF values,the result will be NAN
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///    -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
    ///    1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
    ///    1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
    ///    1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err = mpt.down_month_percent(&mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 52.77778),
    ///    true
    ///);
    ///```
    pub fn down_month_percent(&self, up_number_res: &mut f64) -> Errors {
        return self.up_down_month_percent(|a, b| a < b, up_number_res);
    }
    ///calculate the average gain and loss value of an array, if the array has NAN/INF values,the result will be NAN
    ///
    ///# Arguments
    ///freq: the frequence of source data.
    ///
    ///is_annu: the flag of annuize.
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///    -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
    ///    1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
    ///    1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
    ///    1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
    ///];
    ///let mut avg_gain = 0.0;
    ///let mut avg_loss = 0.0;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err = mpt.average_gain_loss(&mut avg_gain, &mut avg_loss);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(avg_gain, 2.57330),
    ///    true
    ///);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(avg_loss, -4.01982),
    ///    true
    ///);
    ///```
    pub fn average_gain_loss(&self, avg_gain: &mut f64, avg_loss: &mut f64) -> Errors {
        if self.values.len() == 0 {
            return Errors::ClErrorCodeInvalidPara;
        }
        *avg_gain = f64::NAN;
        *avg_loss = f64::NAN;

        let mut gain_accu_return = 1.0;
        let mut gain_count = 0.0;

        let mut loss_accu_return = 1.0;
        let mut loss_count = 0.0;

        if self
            .values
            .iter()
            .try_for_each(|x| {
                if !x.is_finite() {
                    return ControlFlow::Break(());
                }
                if *x >= 0.0 {
                    gain_accu_return *= 1.0 + *x / 100.0;
                    gain_count += 1.0;
                }

                if *x <= 0.0 {
                    loss_accu_return *= 1.0 + *x / 100.0;
                    loss_count += 1.0;
                }

                ControlFlow::Continue(())
            })
            .is_break()
        {
            return Errors::ClErrorCodeNoError;
        }

        if gain_count != 0.0 {
            *avg_gain = (gain_accu_return.powf(1.0 / gain_count) - 1.0) * 100.0
        }

        if loss_count != 0.0 {
            *avg_loss = (loss_accu_return.powf(1.0 / loss_count) - 1.0) * 100.0;
        }
        return Errors::ClErrorCodeNoError;
    }

    fn get_max_draw_down(values: &[f64], start: usize, end: usize, dg: &mut DataGroup) -> Errors {
        if values.len() == 0 || end >= values.len() {
            return Errors::ClErrorCodeInvalidPara;
        }
        let mut start = start;
        for i in start..end + 1 {
            if values[i] != 0.0 {
                start = if i > 0 { i - 1 } else { i };
                break;
            }
        }

        let mut total_max_index = start;
        let mut total_min_index = start;
        for i in start..end + 1 {
            if values[i] > values[total_max_index] {
                total_max_index = i;
            }
            if values[i] < values[total_min_index] {
                total_min_index = i;
            }
        }

        if total_max_index < total_min_index {
            dg.start = total_max_index;
            dg.end = total_min_index;
            dg.data = values[dg.start] - values[dg.end];
            return Errors::ClErrorCodeInvalidPara;
        }

        if total_max_index == total_min_index {
            dg.start = 0;
            dg.end = 0;
            dg.data = 0.0;
            return Errors::ClErrorCodeInvalidPara;
        }

        let mut maxindex_before_min = start;
        for i in start..total_min_index {
            if values[i] > values[maxindex_before_min] {
                maxindex_before_min = i;
            }
        }
        let down_value_befor_min = values[maxindex_before_min] - values[total_min_index];

        let mut minindex_after_max = total_max_index;
        for i in total_max_index..end + 1 {
            if values[i] < values[minindex_after_max] {
                minindex_after_max = i;
            }
        }
        let down_value_after_max = values[total_max_index] - values[minindex_after_max];

        let mut first_inflexion_after_min = total_min_index;
        for i in total_min_index..total_max_index {
            if values[i + 1] > values[i] {
                first_inflexion_after_min = i + 1;
            } else {
                break;
            }
        }

        let mut first_inflexion_before_max = total_max_index;
        for i in (total_min_index..total_max_index).rev() {
            if values[i - 1] < values[i] {
                first_inflexion_before_max = i - 1;
            } else {
                break;
            }
        }

        let mut max_down_extern = DataGroup::new();
        if down_value_befor_min < down_value_after_max {
            max_down_extern.start = total_max_index;
            max_down_extern.end = minindex_after_max;
            max_down_extern.data = down_value_after_max;
        } else {
            max_down_extern.start = maxindex_before_min;
            max_down_extern.end = total_min_index;
            max_down_extern.data = down_value_befor_min;
        }

        if first_inflexion_after_min > first_inflexion_before_max {
            *dg = max_down_extern;
            return Errors::ClErrorCodeNoError;
        }
        let mut max_down_between = DataGroup::new();
        Self::get_max_draw_down(
            values,
            first_inflexion_after_min,
            first_inflexion_before_max,
            &mut max_down_between,
        );

        if max_down_between.data > max_down_extern.data {
            *dg = max_down_between;
        } else {
            *dg = max_down_extern;
        }
        return Errors::ClErrorCodeNoError;
    }

    ///calculate the max draw down value,peek date,valley date,recover month and recover date of an array, if the array has NAN/INF values,the result will be NAN
    ///freq: the frequence of source data.
    ///
    ///dates: the date of value
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///    -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
    ///    1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
    ///    1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
    ///    1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
    ///];

    ///let dates = vec![
    ///   38776, 38807, 38837, 38868, 38898, 38929, 38960, 38990, 39021, 39051, 39082, 39113,
    ///    39141, 39172, 39202, 39233, 39263, 39294, 39325, 39355, 39386, 39416, 39447, 39478,
    ///    39507, 39538, 39568, 39599, 39629, 39660, 39691, 39721, 39752, 39782, 39813, 39844,
    ///];
    ///let mut max_draw_down = f64::NAN;
    ///let mut max_draw_down_peek_date = 0;
    ///let mut max_draw_down_valley_date = 0;
    ///let mut max_draw_down_month = 0;
    ///let mut recovery_month = 0;
    ///let mut recovery_date = 0;

    ///let mpt = MPTCalculator::from_v(&data);
    ///let err = mpt.max_draw_down(
    ///    &dates,
    ///    enums::ClFrequency::ClFrequencyMonthly,
    ///    &mut max_draw_down,
    ///    &mut max_draw_down_peek_date,
    ///    &mut max_draw_down_valley_date,
    ///    &mut max_draw_down_month,
    ///    &mut recovery_month,
    ///    &mut recovery_date,
    ///);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(max_draw_down, -43.72595),
    ///    true
    ///);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && max_draw_down_peek_date == 39387,
    ///    true
    ///);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && max_draw_down_valley_date == 39844,
    ///    true
    ///);
    ///assert_eq!(
    ///   err == Errors::ClErrorCodeNoError && max_draw_down_month == 15,
    ///   true
    ///);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && recovery_month == 0,
    ///    true
    ///);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && recovery_date == 0,
    ///   true
    ///);
    ///```
    pub fn max_draw_down(
        &self,
        dates: &[i32],
        freq: enums::ClFrequency,
        max_draw_down: &mut f64,
        max_draw_down_peek_date: &mut i32,
        max_draw_down_valley_date: &mut i32,
        max_draw_down_month: &mut i32,
        recovery_month: &mut i32,
        recovery_date: &mut i32,
    ) -> Errors {
        if self.values.len() == 0 {
            return Errors::ClErrorCodeInvalidPara;
        }

        *max_draw_down = f64::NAN;
        *max_draw_down_peek_date = 0;
        *max_draw_down_valley_date = 0;
        *max_draw_down_month = 0;
        *recovery_month = 0;
        *recovery_date = 0;
        let mut log_accum_series = vec![f64::NAN; self.values.len() + 1];
        log_accum_series[0] = 0.0;
        if !self.values[0].is_finite() {
            return Errors::ClErrorCodeNoError;
        } else {
            log_accum_series[1] = (1.0 + self.values[0] / 100.0).ln();
        }

        for i in 1..self.values.len() {
            if !self.values[i].is_finite() {
                return Errors::ClErrorCodeNoError;
            }
            log_accum_series[i + 1] = (1.0 + self.values[i] / 100.0).ln() + log_accum_series[i];
        }

        let mut max_draw_down_dg = DataGroup::new();
        Self::get_max_draw_down(
            &log_accum_series,
            0,
            log_accum_series.len() - 1,
            &mut max_draw_down_dg,
        );

        if max_draw_down_dg.start < max_draw_down_dg.end && max_draw_down_dg.data != 0.0 {
            *max_draw_down = ((-max_draw_down_dg.data).exp() - 1.0) * 100.0;
            *max_draw_down_peek_date =
                date_util::to_period_begin_int(freq, dates[max_draw_down_dg.start] as u64) as i32;
            *max_draw_down_valley_date = dates[max_draw_down_dg.end - 1];
            *max_draw_down_month = (max_draw_down_dg.end - max_draw_down_dg.start) as i32;

            let mut recovery_pos = 0;
            for i in max_draw_down_dg.end..log_accum_series.len() {
                if log_accum_series[i] >= log_accum_series[max_draw_down_dg.start] {
                    recovery_pos = i;
                    break;
                }
            }
            if recovery_pos != 0 {
                *recovery_month = (recovery_pos - max_draw_down_dg.end) as i32;
                *recovery_date = dates[recovery_pos - 1];
            }
        }

        return Errors::ClErrorCodeNoError;
    }

    fn get_max_gain(values: &[f64], start: usize, end: usize, dg: &mut DataGroup) -> Errors {
        if values.len() == 0 || end >= values.len() {
            return Errors::ClErrorCodeInvalidPara;
        }
        let mut start = start;
        for i in start..end + 1 {
            if values[i] != 0.0 {
                start = if i > 0 { i - 1 } else { i };
                break;
            }
        }

        let mut total_max_index = start;
        let mut total_min_index = start;
        for i in start..end + 1 {
            if values[i] > values[total_max_index] {
                total_max_index = i;
            }
            if values[i] < values[total_min_index] {
                total_min_index = i;
            }
        }
        //the max is at right, min is at left, mean it is a increase series.
        if total_max_index > total_min_index {
            dg.start = total_min_index;
            dg.end = total_max_index;
            dg.data = values[total_max_index] - values[total_min_index];
            return Errors::ClErrorCodeInvalidPara;
        }

        if total_max_index == total_min_index {
            dg.start = 0;
            dg.end = 0;
            dg.data = 0.0;
            return Errors::ClErrorCodeInvalidPara;
        }
        //the max is at left, min is at right, mean it is a decrease series.
        let mut minindex_before_max = start;
        for i in start..total_max_index {
            if values[i] < values[minindex_before_max] {
                minindex_before_max = i;
            }
        }
        let gain_value_befor_max = values[total_max_index] - values[minindex_before_max];

        let mut maxindex_after_min = total_min_index;
        for i in total_min_index..end + 1 {
            if values[i] > values[maxindex_after_min] {
                maxindex_after_min = i;
            }
        }
        let gain_value_after_min = values[maxindex_after_min] - values[total_min_index];

        let mut first_inflexion_after_max = total_max_index;
        for i in total_max_index..total_min_index {
            if values[i + 1] < values[i] {
                first_inflexion_after_max = i + 1;
            } else {
                break;
            }
        }

        let mut first_inflexion_before_min = total_min_index;
        for i in (total_max_index..total_min_index).rev() {
            if values[i - 1] > values[i] {
                first_inflexion_before_min = i - 1;
            } else {
                break;
            }
        }

        let mut max_gain_extern = DataGroup::new();
        if gain_value_befor_max < gain_value_after_min {
            max_gain_extern.start = total_min_index;
            max_gain_extern.end = maxindex_after_min;
            max_gain_extern.data = gain_value_after_min;
        } else {
            max_gain_extern.start = minindex_before_max;
            max_gain_extern.end = total_max_index;
            max_gain_extern.data = gain_value_befor_max;
        }

        if first_inflexion_after_max > first_inflexion_before_min {
            *dg = max_gain_extern;
            return Errors::ClErrorCodeNoError;
        }
        let mut max_gain_between = DataGroup::new();
        Self::get_max_gain(
            values,
            first_inflexion_after_max,
            first_inflexion_before_min,
            &mut max_gain_between,
        );

        if max_gain_between.data > max_gain_extern.data {
            *dg = max_gain_between;
        } else {
            *dg = max_gain_extern;
        }
        return Errors::ClErrorCodeNoError;
    }
    ///calculate the max gain value,start date,end date,max gain month of an array, if the array has NAN/INF values,the result will be NAN
    ///freq: the frequence of source data.
    ///
    ///dates: the date of value
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///   -2.57909, 0.0353, 3.56387, -3.88416, 0.0, -9.81106, -7.70466, -0.04348, -9.65637,
    ///   3.37025, 7.68514, -6.79066, -1.76334, -3.7317, -0.49068, 11.83432, 9.08289, 3.39531,
    ///   0.70368, 0.89286, -0.76953, 6.39783, 1.38484, 2.33645, 2.80998, 0.5808, -0.61141,
    ///   -0.20506, -0.47945, -0.13765, -3.4459, -0.85653, 1.83585, 0.84836, 3.61024, 3.99188,
    ///   -1.7892, 2.02054, -0.81169, -1.40753, 3.02125, -0.67676, 1.07073, -2.21509, 0.29547,
    ///   -2.65139, 2.62273, -0.65557, 0.76463, -1.22072, -0.0668, 2.20588, -0.91563, -0.76766,
    ///   -1.21429, 3.43456, 4.99825, 3.89481, 1.59564, 0.86793, 2.41477, -1.80305, 0.6709,
    ///   3.57769, 4.77481, -0.37317, -3.52713, 1.88831, 1.73502, 1.20155, -3.36542, -2.03551,
    ///   -5.6145, -2.71663, -0.04815, 3.99807, 1.66744, -9.68658, -0.46681, 4.22095, -6.7,
    ///   -15.27331, -8.46123, 0.76369,
    ///];
    ///
    ///let dates = vec![
    ///   37287, 37315, 37346, 37376, 37407, 37437, 37468, 37499, 37529, 37560, 37590, 37621,
    ///   37652, 37680, 37711, 37741, 37772, 37802, 37833, 37864, 37894, 37925, 37955, 37986,
    ///   38017, 38046, 38077, 38107, 38138, 38168, 38199, 38230, 38260, 38291, 38321, 38352,
    ///   38383, 38411, 38442, 38472, 38503, 38533, 38564, 38595, 38625, 38656, 38686, 38717,
    ///   38748, 38776, 38807, 38837, 38868, 38898, 38929, 38960, 38990, 39021, 39051, 39082,
    ///   39113, 39141, 39172, 39202, 39233, 39263, 39294, 39325, 39355, 39386, 39416, 39447,
    ///   39478, 39507, 39538, 39568, 39599, 39629, 39660, 39691, 39721, 39752, 39782, 39813,
    ///];
    ///let mut max_gain = f64::NAN;
    ///let mut start_date = 0;
    ///let mut end_date = 0;
    ///let mut max_gain_month = 0;
    ///
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err = mpt.max_gain(
    ///   &dates,
    ///   enums::ClFrequency::ClFrequencyMonthly,
    ///   &mut max_gain,
    ///   &mut start_date,
    ///   &mut end_date,
    ///   &mut max_gain_month,
    ///);
    ///assert_eq!(
    ///   err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(max_gain, 89.10075),
    ///   true
    ///);
    ///assert_eq!(
    ///   err == Errors::ClErrorCodeNoError && start_date == 37712,
    ///   true
    ///);
    ///assert_eq!(err == Errors::ClErrorCodeNoError && end_date == 39386, true);
    ///assert_eq!(
    ///   err == Errors::ClErrorCodeNoError && max_gain_month == 55,
    ///   true
    ///);
    ///```
    pub fn max_gain(
        &self,
        dates: &[i32],
        freq: enums::ClFrequency,
        max_gain: &mut f64,
        start_date: &mut i32,
        end_date: &mut i32,
        max_gain_month: &mut i32,
    ) -> Errors {
        if self.values.len() == 0 {
            return Errors::ClErrorCodeInvalidPara;
        }

        *max_gain = f64::NAN;
        *start_date = 0;
        *end_date = 0;
        *max_gain_month = 0;

        let mut log_accum_series = vec![f64::NAN; self.values.len() + 1];
        log_accum_series[0] = 0.0;
        if !self.values[0].is_finite() {
            return Errors::ClErrorCodeNoError;
        } else {
            log_accum_series[1] = (1.0 + self.values[0] / 100.0).ln();
        }

        for i in 1..self.values.len() {
            if !self.values[i].is_finite() {
                return Errors::ClErrorCodeNoError;
            }
            log_accum_series[i + 1] = (1.0 + self.values[i] / 100.0).ln() + log_accum_series[i];
        }

        let mut max_gain_dg = DataGroup::new();
        Self::get_max_gain(
            &log_accum_series,
            0,
            log_accum_series.len() - 1,
            &mut max_gain_dg,
        );
        *max_gain = (max_gain_dg.data.exp() - 1.0) * 100.0;
        if max_gain_dg.start < max_gain_dg.end && max_gain_dg.data != 0.0 {
            *start_date =
                date_util::to_period_begin_int(freq, dates[max_gain_dg.start] as u64) as i32;
            *end_date = dates[max_gain_dg.end - 1];
            *max_gain_month = (max_gain_dg.end - max_gain_dg.start) as i32;
        }

        return Errors::ClErrorCodeNoError;
    }
    ///calculate the calmar ratio value of an array, the input data should sort by date,and should has not NA/INF, otherwrise result will be NAN
    ///
    ///# Arguments
    ///freq: the frequence of source data.
    ///
    ///dates: the date of value
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016, 1.40278, 1.51232,
    ///-1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901, 3.73988, 1.59068,
    ///-4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526, -8.43036, -0.84062,
    ///1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864, -10.64778, 8.75952,
    ///];
    ///let dates = vec![
    ///38837, 38868, 38898, 38929, 38960, 38990, 39021, 39051, 39082, 39113, 39141, 39172,
    ///39202, 39233, 39263, 39294, 39325, 39355, 39386, 39416, 39447, 39478, 39507, 39538,
    ///39568, 39599, 39629, 39660, 39691, 39721, 39752, 39782, 39813, 39844, 39872, 39903,
    ///];
    ///let mut result = f64::NAN;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err = mpt.calmar_ratio(&dates, enums::ClFrequency::ClFrequencyMonthly, &mut result);
    ///assert_eq!(
    ///   err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(result, -0.2562775),
    ///   true
    ///);
    ///```
    pub fn calmar_ratio(
        &self,
        dates: &[i32],
        freq: enums::ClFrequency,
        calmar_ratio: &mut f64,
    ) -> Errors {
        if self.values.len() == 0 || !is_valid_frequency(freq) {
            return Errors::ClErrorCodeInvalidPara;
        }
        *calmar_ratio = f64::NAN;
        if !is_sorted_array(dates) {
            return Errors::ClErrorCodeUnsortedByDate;
        }

        let mut max_draw_down = f64::NAN;
        let mut max_draw_down_peek_date = 0;
        let mut max_draw_down_valley_date = 0;
        let mut max_draw_down_month = 0;
        let mut recovery_month = 0;
        let mut recovery_date = 0;

        self.max_draw_down(
            dates,
            freq,
            &mut max_draw_down,
            &mut max_draw_down_peek_date,
            &mut max_draw_down_valley_date,
            &mut max_draw_down_month,
            &mut recovery_month,
            &mut recovery_date,
        );

        if max_draw_down != 0.0 {
            let total_return = (self
                .values
                .iter()
                .fold(1.0, |acc, v| acc * (1.0 + v / 100.0))
                - 1.0)
                * 100.0;

            let annu_total_return =
                annualize_return(total_return, freq, self.values.len() as f64, true);

            if annu_total_return.is_finite() {
                *calmar_ratio = annu_total_return / max_draw_down.abs();
            }
        }
        return Errors::ClErrorCodeNoError;
    }
    ///calculate the average draw down value of an array, the input data should sort by date,and should has not NA/INF,otherwrisethe result will be NAN
    ///
    ///# Arguments
    ///freq: the frequence of source data.
    ///
    ///dates: the date of value
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///1.52768, 4.04616, 3.40287, -2.43748, 2.1044, -1.7708, -1.89656, 3.18186, 0.14197,
    ///3.71883, -0.9124, 0.80994, -1.66708, 3.78221, 0.03481, 2.64778, 0.27133, 1.24475,
    ///1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016, 1.40278, 1.51232,
    ///-1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901, 3.73988, 1.59068,
    ///-4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526, -8.43036, -0.84062,
    ///1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864, -10.64778, 8.75952,
    ///];
    ///let dates = vec![
    ///38291, 38321, 38352, 38383, 38411, 38442, 38472, 38503, 38533, 38564, 38595, 38625,
    ///38656, 38686, 38717, 38748, 38776, 38807, 38837, 38868, 38898, 38929, 38960, 38990,
    /// 39021, 39051, 39082, 39113, 39141, 39172, 39202, 39233, 39263, 39294, 39325, 39355,
    /// 39386, 39416, 39447, 39478, 39507, 39538, 39568, 39599, 39629, 39660, 39691, 39721,
    /// 39752, 39782, 39813, 39844, 39872, 39903,
    ///];
    ///let mut result = f64::NAN;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err =
    ///    mpt.average_draw_down(&dates, enums::ClFrequency::ClFrequencyMonthly, &mut result);
    ///assert_eq!(
    ///   err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(result, -15.76075),
    ///   true
    ///);
    ///```
    pub fn average_draw_down(
        &self,
        dates: &[i32],
        freq: enums::ClFrequency,
        avg_draw_down: &mut f64,
    ) -> Errors {
        if self.values.len() == 0 || !is_valid_frequency(freq) {
            return Errors::ClErrorCodeInvalidPara;
        }
        if !is_sorted_array(dates) {
            return Errors::ClErrorCodeUnsortedByDate;
        }

        if self.values.iter().find(|x| !x.is_finite()) != None {
            return Errors::ClErrorCodeNoError;
        }
        *avg_draw_down = 0.0;

        let annu_mutiplier = get_annual_multiplier(freq, false);
        let mut begin_date = dates[0];
        let mut end_date =
            date_util::to_n_period_end_int(freq, annu_mutiplier as i32 - 1, begin_date as u64)
                as i32;

        let mut start_pos = 0;
        let mut end_pos = 0;

        let mut max_draw_down = f64::NAN;
        let mut max_draw_down_peek_date = 0;
        let mut max_draw_down_valley_date = 0;
        let mut max_draw_down_month = 0;
        let mut recovery_month = 0;
        let mut recovery_date = 0;
        while end_pos < self.values.len() - 1 {
            for i in start_pos..self.values.len() {
                if dates[i] > end_date {
                    break;
                }
                end_pos = i;
            }
            let mpt = MPTCalculator::from_v(&self.values[start_pos..end_pos + 1]);
            mpt.max_draw_down(
                dates,
                freq,
                &mut max_draw_down,
                &mut max_draw_down_peek_date,
                &mut max_draw_down_valley_date,
                &mut max_draw_down_month,
                &mut recovery_month,
                &mut recovery_date,
            );

            if max_draw_down.is_finite() {
                *avg_draw_down += max_draw_down;
            }

            if end_pos < self.values.len() - 1 {
                start_pos = end_pos + 1;
                begin_date = dates[start_pos];
                end_date = date_util::to_n_period_end_int(
                    freq,
                    annu_mutiplier as i32 - 1,
                    begin_date as u64,
                ) as i32;
            }
        }

        *avg_draw_down *= annu_mutiplier / self.values.len() as f64;
        return Errors::ClErrorCodeNoError;
    }
    ///calculate the sterling ratio value of an array, the input data should sort by date,and should has not NA/INF,otherwrise the result will be NAN
    ///
    ///# Arguments
    ///freq: the frequence of source data.
    ///
    ///dates: the date of value.
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///1.52768, 4.04616, 3.40287, -2.43748, 2.1044, -1.7708, -1.89656, 3.18186, 0.14197,
    ///3.71883, -0.9124, 0.80994, -1.66708, 3.78221, 0.03481, 2.64778, 0.27133, 1.24475,
    ///1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016, 1.40278, 1.51232,
    ///-1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901, 3.73988, 1.59068,
    ///-4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526, -8.43036, -0.84062,
    ///1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864, -10.64778, 8.75952,
    ///];
    ///let dates = vec![
    ///38291, 38321, 38352, 38383, 38411, 38442, 38472, 38503, 38533, 38564, 38595, 38625,
    ///38656, 38686, 38717, 38748, 38776, 38807, 38837, 38868, 38898, 38929, 38960, 38990,
    /// 39021, 39051, 39082, 39113, 39141, 39172, 39202, 39233, 39263, 39294, 39325, 39355,
    /// 39386, 39416, 39447, 39478, 39507, 39538, 39568, 39599, 39629, 39660, 39691, 39721,
    /// 39752, 39782, 39813, 39844, 39872, 39903,
    ///];
    ///let mut result = f64::NAN;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err = mpt.sterling_ratio(&dates, enums::ClFrequency::ClFrequencyMonthly, &mut result);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(result, -0.2034894),
    ///    true
    ///);
    ///```
    pub fn sterling_ratio(
        &self,
        dates: &[i32],
        freq: enums::ClFrequency,
        sterling_ration: &mut f64,
    ) -> Errors {
        if self.values.len() == 0 || !is_valid_frequency(freq) {
            return Errors::ClErrorCodeInvalidPara;
        }
        *sterling_ration = f64::NAN;
        let mut avg_draw_down = f64::NAN;

        self.average_draw_down(dates, freq, &mut avg_draw_down);

        if avg_draw_down - 10.0 != 0.0 {
            let total_return = (self
                .values
                .iter()
                .fold(1.0, |acc, v| acc * (1.0 + v / 100.0))
                - 1.0)
                * 100.0;

            let annu_total_return =
                annualize_return(total_return, freq, self.values.len() as f64, true);

            if annu_total_return.is_finite() {
                *sterling_ration = annu_total_return / (avg_draw_down - 10.0).abs();
            }
        }
        return Errors::ClErrorCodeNoError;
    }

    fn best_worth_rolling_month(
        &self,
        dates: &[i32],
        best_months_num: i32,
        cmp_fn: fn(f64, f64) -> bool,
        best_rolling_month_date: &mut i32,
        best_rolling_month_value: &mut f64,
    ) -> Errors {
        if self.values.len() == 0 || best_months_num as usize > self.values.len() {
            return Errors::ClErrorCodeInvalidPara;
        }

        if !is_sorted_array(dates) {
            return Errors::ClErrorCodeInvalidOutput;
        }

        *best_rolling_month_date = 0;
        *best_rolling_month_value = f64::NAN;
        if self
            .values
            .iter()
            .enumerate()
            .try_for_each(|x| {
                if !x.1.is_finite() {
                    return ControlFlow::Break(());
                }
                if x.0 >= (best_months_num - 1) as usize {
                    let mut f = 1.0;
                    let mut pos = 0;
                    while pos < best_months_num {
                        f *= 1.0 + self.values[x.0 - pos as usize] / 100.0;
                        pos += 1;
                    }
                    f = (f - 1.0) * 100.0;

                    if !(*best_rolling_month_value).is_finite()
                        || cmp_fn(f, *best_rolling_month_value)
                    {
                        *best_rolling_month_date = dates[x.0];
                        *best_rolling_month_value = f;
                    }
                }
                ControlFlow::Continue(())
            })
            .is_break()
        {
            return Errors::ClErrorCodeNoError;
        }

        return Errors::ClErrorCodeNoError;
    }
    ///calculate the best rolling month value of an array, the input data should sort by date,and should has not NA/INF,the result will be NAN
    ///
    ///# Arguments
    ///best_months_num: the best month number
    ///
    ///dates: the date of value
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///    -2.57909, 0.0353, 3.56387, -3.88416, 0.0, -9.81106, -7.70466, -0.04348, -9.65637,
    ///    3.37025, 7.68514, -6.79066, -1.76334, -3.7317, -0.49068, 11.83432, 9.08289, 3.39531,
    ///    0.70368, 0.89286, -0.76953, 6.39783, 1.38484, 2.33645, 2.80998, 0.5808, -0.61141,
    ///    -0.20506, -0.47945, -0.13765, -3.4459, -0.85653, 1.83585, 0.84836, 3.61024, 3.99188,
    ///    -1.7892, 2.02054, -0.81169, -1.40753, 3.02125, -0.67676, 1.07073, -2.21509, 0.29547,
    ///    -2.65139, 2.62273, -0.65557, 0.76463, -1.22072, -0.0668, 2.20588, -0.91563, -0.76766,
    ///    -1.21429, 3.43456, 4.99825, 3.89481, 1.59564, 0.86793, 2.41477, -1.80305, 0.6709,
    ///    3.57769, 4.77481, -0.37317, -3.52713, 1.88831, 1.73502, 1.20155, -3.36542, -2.03551,
    ///    -5.6145, -2.71663, -0.04815, 3.99807, 1.66744, -9.68658, -0.46681, 4.22095, -6.7,
    ///    -15.27331, -8.46123, 0.76369,
    ///];

    ///let dates = vec![
    ///    37287, 37315, 37346, 37376, 37407, 37437, 37468, 37499, 37529, 37560, 37590, 37621,
    ///    37652, 37680, 37711, 37741, 37772, 37802, 37833, 37864, 37894, 37925, 37955, 37986,
    ///    38017, 38046, 38077, 38107, 38138, 38168, 38199, 38230, 38260, 38291, 38321, 38352,
    ///    38383, 38411, 38442, 38472, 38503, 38533, 38564, 38595, 38625, 38656, 38686, 38717,
    ///    38748, 38776, 38807, 38837, 38868, 38898, 38929, 38960, 38990, 39021, 39051, 39082,
    ///    39113, 39141, 39172, 39202, 39233, 39263, 39294, 39325, 39355, 39386, 39416, 39447,
    ///    39478, 39507, 39538, 39568, 39599, 39629, 39660, 39691, 39721, 39752, 39782, 39813,
    ///];

    ///let mut best_rolling_month_date = 0;
    ///let mut best_rolling_month_value = f64::NAN;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err = mpt.best_rolling_month(
    ///    &dates,
    ///    3,
    ///    &mut best_rolling_month_date,
    ///    &mut best_rolling_month_value,
    ///);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError
    ///        && MPTCalculator::is_eq_double(best_rolling_month_value, 26.13411852),
    ///    true
    ///);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && best_rolling_month_date == 37802,
    ///    true
    ///);
    ///```
    pub fn best_rolling_month(
        &self,
        dates: &[i32],
        best_months_num: i32,
        best_rolling_month_date: &mut i32,
        best_rolling_month_value: &mut f64,
    ) -> Errors {
        return self.best_worth_rolling_month(
            dates,
            best_months_num,
            |a, b| a > b,
            best_rolling_month_date,
            best_rolling_month_value,
        );
    }
    ///calculate the worst rolling month value of an array, the input data should sort by date,and should has not NA/INF,the result will be NAN
    ///
    ///# Arguments
    ///worst_months_num: the best month number
    ///
    ///dates: the date of value
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///    -2.57909, 0.0353, 3.56387, -3.88416, 0.0, -9.81106, -7.70466, -0.04348, -9.65637,
    ///    3.37025, 7.68514, -6.79066, -1.76334, -3.7317, -0.49068, 11.83432, 9.08289, 3.39531,
    ///   0.70368, 0.89286, -0.76953, 6.39783, 1.38484, 2.33645, 2.80998, 0.5808, -0.61141,
    ///   -0.20506, -0.47945, -0.13765, -3.4459, -0.85653, 1.83585, 0.84836, 3.61024, 3.99188,
    ///   -1.7892, 2.02054, -0.81169, -1.40753, 3.02125, -0.67676, 1.07073, -2.21509, 0.29547,
    ///   -2.65139, 2.62273, -0.65557, 0.76463, -1.22072, -0.0668, 2.20588, -0.91563, -0.76766,
    ///   -1.21429, 3.43456, 4.99825, 3.89481, 1.59564, 0.86793, 2.41477, -1.80305, 0.6709,
    ///   3.57769, 4.77481, -0.37317, -3.52713, 1.88831, 1.73502, 1.20155, -3.36542, -2.03551,
    ///   -5.6145, -2.71663, -0.04815, 3.99807, 1.66744, -9.68658, -0.46681, 4.22095, -6.7,
    ///   -15.27331, -8.46123, 0.76369,
    ///];
    ///
    ///let dates = vec![
    ///   37287, 37315, 37346, 37376, 37407, 37437, 37468, 37499, 37529, 37560, 37590, 37621,
    ///   37652, 37680, 37711, 37741, 37772, 37802, 37833, 37864, 37894, 37925, 37955, 37986,
    ///   38017, 38046, 38077, 38107, 38138, 38168, 38199, 38230, 38260, 38291, 38321, 38352,
    ///   38383, 38411, 38442, 38472, 38503, 38533, 38564, 38595, 38625, 38656, 38686, 38717,
    ///   38748, 38776, 38807, 38837, 38868, 38898, 38929, 38960, 38990, 39021, 39051, 39082,
    ///   39113, 39141, 39172, 39202, 39233, 39263, 39294, 39325, 39355, 39386, 39416, 39447,
    ///   39478, 39507, 39538, 39568, 39599, 39629, 39660, 39691, 39721, 39752, 39782, 39813,
    ///];
    ///
    ///let mut worst_rolling_month_date = 0;
    ///let mut worst_rolling_month_value = f64::NAN;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err = mpt.worst_rolling_month(
    ///   &dates,
    ///   3,
    ///   &mut worst_rolling_month_date,
    ///   &mut worst_rolling_month_value,
    ///);
    ///assert_eq!(
    ///   err == Errors::ClErrorCodeNoError
    ///       && MPTCalculator::is_eq_double(worst_rolling_month_value, -27.63860069),
    ///   true
    ///);
    ///assert_eq!(
    ///   err == Errors::ClErrorCodeNoError && worst_rolling_month_date == 39782,
    ///   true
    ///);
    ///```
    pub fn worst_rolling_month(
        &self,
        dates: &[i32],
        worst_months_num: i32,
        worst_rolling_month_date: &mut i32,
        worst_rolling_month_value: &mut f64,
    ) -> Errors {
        return self.best_worth_rolling_month(
            dates,
            worst_months_num,
            |a, b| a < b,
            worst_rolling_month_date,
            worst_rolling_month_value,
        );
    }

    fn get_last_up_down_streak(
        values: &[f64],
        start: usize,
        end: usize,
        cmp_fn: fn(f64, f64) -> bool,
    ) -> DataGroup {
        let mut final_streak = DataGroup {
            start: start,
            end: start,
            data: 1.0,
        };
        for mut i in start..end {
            if !values[i].is_finite() {
                continue;
            }

            let mut streak = DataGroup {
                start: i,
                end: i,
                data: 1.0,
            };
            while i < end {
                if !values[i].is_finite() {
                    i += 1;
                    continue;
                } else if cmp_fn(values[i], 0.0) {
                    streak.data *= values[i] / 100.0 + 1.0;
                    streak.end = i;
                } else {
                    break;
                }
                i += 1;
            }
            if streak.data != 1.0
                && (streak.end - streak.start) >= (final_streak.end - final_streak.start)
            {
                final_streak = streak;
            }
        }

        final_streak.data = (final_streak.data - 1.0) * 100.0;
        final_streak
    }

    fn longest_up_down_streak(
        &self,
        dates: &[i32],
        freq: enums::ClFrequency,
        is_up: bool,
        longest_up_down_streak: &mut f64,
        longest_up_down_start_date: &mut i32,
        longest_up_down_end_date: &mut i32,
        longest_up_down_periods: &mut i32,
    ) -> Errors {
        if self.values.len() == 0 {
            return Errors::ClErrorCodeInvalidPara;
        }
        *longest_up_down_streak = f64::NAN;
        *longest_up_down_start_date = 0;
        *longest_up_down_end_date = 0;
        *longest_up_down_periods = 0;

        let cmp_fn = if is_up { |a, b| a > b } else { |a, b| a < b };

        let longest_up_down_group =
            Self::get_last_up_down_streak(self.values, 0, self.values.len(), cmp_fn);
        if longest_up_down_group.data == 0.0
            && longest_up_down_group.start == longest_up_down_group.end
        {
            return Errors::ClErrorCodeNoError;
        }

        *longest_up_down_start_date =
            date_util::to_period_begin_int(freq, dates[longest_up_down_group.start] as u64) as i32;
        *longest_up_down_end_date =
            date_util::to_period_end_int(freq, dates[longest_up_down_group.end] as u64) as i32;
        *longest_up_down_streak = longest_up_down_group.data;
        *longest_up_down_periods =
            (longest_up_down_group.end - longest_up_down_group.start) as i32 + 1;

        return Errors::ClErrorCodeNoError;
    }

    ///calculate the longest up streak value,longest up streak start date,end date,month numbers of an array, the input data should sort by date,and should has not NA/INF,the result will be NAN
    ///
    ///# Arguments
    ///freq: the frequence of source data.
    ///
    ///dates: the date of value
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///
    ///let data = vec![
    ///   -2.57909, 0.0353, 3.56387, -3.88416, 0.0, -9.81106, -7.70466, -0.04348, -9.65637,
    ///   3.37025, 7.68514, -6.79066, -1.76334, -3.7317, -0.49068, 11.83432, 9.08289, 3.39531,
    ///   0.70368, 0.89286, -0.76953, 6.39783, 1.38484, 2.33645, 2.80998, 0.5808, -0.61141,
    ///   -0.20506, -0.47945, -0.13765, -3.4459, -0.85653, 1.83585, 0.84836, 3.61024, 3.99188,
    ///   -1.7892, 2.02054, -0.81169, -1.40753, 3.02125, -0.67676, 1.07073, -2.21509, 0.29547,
    ///   -2.65139, 2.62273, -0.65557, 0.76463, -1.22072, -0.0668, 2.20588, -0.91563, -0.76766,
    ///   -1.21429, 3.43456, 4.99825, 3.89481, 1.59564, 0.86793, 2.41477, -1.80305, 0.6709,
    ///   3.57769, 4.77481, -0.37317, -3.52713, 1.88831, 1.73502, 1.20155, -3.36542, -2.03551,
    ///   -5.6145, -2.71663, -0.04815, 3.99807, 1.66744, -9.68658, -0.46681, 4.22095, -6.7,
    ///   -15.27331, -8.46123, 0.76369,
    ///];
    ///
    ///let dates = vec![
    ///   37287, 37315, 37346, 37376, 37407, 37437, 37468, 37499, 37529, 37560, 37590, 37621,
    ///   37652, 37680, 37711, 37741, 37772, 37802, 37833, 37864, 37894, 37925, 37955, 37986,
    ///   38017, 38046, 38077, 38107, 38138, 38168, 38199, 38230, 38260, 38291, 38321, 38352,
    ///   38383, 38411, 38442, 38472, 38503, 38533, 38564, 38595, 38625, 38656, 38686, 38717,
    ///   38748, 38776, 38807, 38837, 38868, 38898, 38929, 38960, 38990, 39021, 39051, 39082,
    ///   39113, 39141, 39172, 39202, 39233, 39263, 39294, 39325, 39355, 39386, 39416, 39447,
    ///   39478, 39507, 39538, 39568, 39599, 39629, 39660, 39691, 39721, 39752, 39782, 39813,
    ///];
    ///
    ///let mut longest_up_down_streak = f64::NAN;
    ///let mut longest_up_down_start_date = 0;
    ///let mut longest_up_down_end_date = 0;
    ///let mut longest_up_down_periods = 0;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err = mpt.longest_down_streak(
    ///   &dates,
    ///   enums::ClFrequency::ClFrequencyMonthly,
    ///   &mut longest_up_down_streak,
    ///   &mut longest_up_down_start_date,
    ///   &mut longest_up_down_end_date,
    ///   &mut longest_up_down_periods,
    ///);
    ///assert_eq!(
    ///   err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(longest_up_down_streak, -5.63859),
    ///   true
    ///);
    ///assert_eq!(
    ///   err == Errors::ClErrorCodeNoError && longest_up_down_start_date == 38047,
    ///   true
    ///);
    ///assert_eq!(
    ///   err == Errors::ClErrorCodeNoError && longest_up_down_end_date == 38230,
    ///   true
    ///);
    ///assert_eq!(
    ///   err == Errors::ClErrorCodeNoError && longest_up_down_periods == 6,
    ///   true
    ///);
    ///```
    pub fn longest_up_streak(
        &self,
        dates: &[i32],
        freq: enums::ClFrequency,
        longest_up_down_streak: &mut f64,
        longest_up_down_start_date: &mut i32,
        longest_up_down_end_date: &mut i32,
        longest_up_down_periods: &mut i32,
    ) -> Errors {
        return self.longest_up_down_streak(
            dates,
            freq,
            true,
            longest_up_down_streak,
            longest_up_down_start_date,
            longest_up_down_end_date,
            longest_up_down_periods,
        );
    }

    ///calculate the longest down streak value,longest up streak start date,end date,month numbersof an array, if the array has NAN/INF values,the result will be NAN
    ///
    ///# Arguments
    ///freq: the frequence of source data.
    ///
    ///dates: the date of value
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///
    ///let data = vec![
    ///   -2.57909, 0.0353, 3.56387, -3.88416, 0.0, -9.81106, -7.70466, -0.04348, -9.65637,
    ///   3.37025, 7.68514, -6.79066, -1.76334, -3.7317, -0.49068, 11.83432, 9.08289, 3.39531,
    ///   0.70368, 0.89286, -0.76953, 6.39783, 1.38484, 2.33645, 2.80998, 0.5808, -0.61141,
    ///   -0.20506, -0.47945, -0.13765, -3.4459, -0.85653, 1.83585, 0.84836, 3.61024, 3.99188,
    ///   -1.7892, 2.02054, -0.81169, -1.40753, 3.02125, -0.67676, 1.07073, -2.21509, 0.29547,
    ///   -2.65139, 2.62273, -0.65557, 0.76463, -1.22072, -0.0668, 2.20588, -0.91563, -0.76766,
    ///   -1.21429, 3.43456, 4.99825, 3.89481, 1.59564, 0.86793, 2.41477, -1.80305, 0.6709,
    ///   3.57769, 4.77481, -0.37317, -3.52713, 1.88831, 1.73502, 1.20155, -3.36542, -2.03551,
    ///   -5.6145, -2.71663, -0.04815, 3.99807, 1.66744, -9.68658, -0.46681, 4.22095, -6.7,
    ///   -15.27331, -8.46123, 0.76369,
    ///];
    ///
    ///let dates = vec![
    ///   37287, 37315, 37346, 37376, 37407, 37437, 37468, 37499, 37529, 37560, 37590, 37621,
    ///   37652, 37680, 37711, 37741, 37772, 37802, 37833, 37864, 37894, 37925, 37955, 37986,
    ///   38017, 38046, 38077, 38107, 38138, 38168, 38199, 38230, 38260, 38291, 38321, 38352,
    ///   38383, 38411, 38442, 38472, 38503, 38533, 38564, 38595, 38625, 38656, 38686, 38717,
    ///   38748, 38776, 38807, 38837, 38868, 38898, 38929, 38960, 38990, 39021, 39051, 39082,
    ///   39113, 39141, 39172, 39202, 39233, 39263, 39294, 39325, 39355, 39386, 39416, 39447,
    ///   39478, 39507, 39538, 39568, 39599, 39629, 39660, 39691, 39721, 39752, 39782, 39813,
    ///];
    ///
    ///let mut longest_up_down_streak = f64::NAN;
    ///let mut longest_up_down_start_date = 0;
    ///let mut longest_up_down_end_date = 0;
    ///let mut longest_up_down_periods = 0;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err = mpt.longest_down_streak(
    ///   &dates,
    ///   enums::ClFrequency::ClFrequencyMonthly,
    ///   &mut longest_up_down_streak,
    ///   &mut longest_up_down_start_date,
    ///   &mut longest_up_down_end_date,
    ///   &mut longest_up_down_periods,
    ///);
    ///assert_eq!(
    ///   err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(longest_up_down_streak, -5.63859),
    ///   true
    ///);
    ///assert_eq!(
    ///   err == Errors::ClErrorCodeNoError && longest_up_down_start_date == 38047,
    ///   true
    ///);
    ///assert_eq!(
    ///   err == Errors::ClErrorCodeNoError && longest_up_down_end_date == 38230,
    ///   true
    ///);
    ///assert_eq!(
    ///   err == Errors::ClErrorCodeNoError && longest_up_down_periods == 6,
    ///   true
    ///);
    ///```
    pub fn longest_down_streak(
        &self,
        dates: &[i32],
        freq: enums::ClFrequency,
        longest_up_down_streak: &mut f64,
        longest_up_down_start_date: &mut i32,
        longest_up_down_end_date: &mut i32,
        longest_up_down_periods: &mut i32,
    ) -> Errors {
        return self.longest_up_down_streak(
            dates,
            freq,
            false,
            longest_up_down_streak,
            longest_up_down_start_date,
            longest_up_down_end_date,
            longest_up_down_periods,
        );
    }
    ///calculate the volatity value of an array, if the array has NAN/INF values,the result will be NAN
    ///
    ///# Arguments
    ///freq: the frequence of source data.
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///   210.69, 195.58, 190.08, 179.72, 179.72, 165.24, 163.12, 160.8, 148.96, 153.29, 169.47,
    ///   181.52, 174.86, 184.9, 174.12, 166.82, 167.46, 165.24, 150.86, 143.88, 151.07, 150.65,
    ///   141.13,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err = mpt.volatity(enums::ClFrequency::ClFrequencyDaily, &mut res);
    ///assert_eq!(
    ///   err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 83.388666),
    ///   true
    ///);
    ///```
    pub fn volatity(&self, freq: enums::ClFrequency, result: &mut f64) -> Errors {
        if self.values.len() == 0 {
            return Errors::ClErrorCodeInvalidPara;
        }
        if self.values.iter().find(|x| !x.is_finite()) != None {
            return Errors::ClErrorCodeNoError;
        }
        *result = f64::NAN;
        let mut relative_return = Vec::with_capacity(self.values.len() - 1);
        if self
            .values
            .iter()
            .enumerate()
            .try_for_each(|x| {
                if x.0 > 0 {
                    if !x.1.is_finite()
                        || !self.values[x.0 - 1].is_finite()
                        || MPTCalculator::is_eq_double(self.values[x.0 - 1], 0.0)
                    {
                        return ControlFlow::Break(());
                    }
                    relative_return.push((x.1 / self.values[x.0 - 1]).ln() * 100.0);
                }

                ControlFlow::Continue(())
            })
            .is_break()
        {
            return Errors::ClErrorCodeNoError;
        }

        let mut standard_deviation_result = f64::NAN;
        let ret = Self::standard_deviation_internal(
            &relative_return,
            freq,
            false,
            &mut standard_deviation_result,
        );
        if ret != Errors::ClErrorCodeNoError {
            return ret;
        }
        *result = standard_deviation_result * get_annual_multiplier(freq, true).sqrt();
        return Errors::ClErrorCodeNoError;
    }

    ///calculate the volatity value of an array, if the array has NAN/INF values,the result will be NAN
    ///
    ///# Arguments
    ///freq: the frequence of source data.
    ///observerd_value: the observerd value
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;
    ///use mpt_lib::enums::{self, Errors};
    ///
    ///let data = vec![
    ///   210.69, 195.58, 190.08, 179.72, 179.72, 165.24, 163.12, 160.8, 148.96, 153.29, 169.47,
    ///   181.52, 174.86, 184.9, 174.12, 166.82, 167.46, 165.24, 150.86, 143.88, 151.07, 150.65,
    ///   141.13,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err = mpt.zscore(200.0, &mut res);
    ///assert_eq!(
    ///   err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 1.813224655535872),
    ///   true
    ///);
    ///```
    pub fn zscore(&self, observerd_value: f64, result: &mut f64) -> Errors {
        if self.values.len() == 0 {
            return Errors::ClErrorCodeInvalidPara;
        }
        *result = f64::NAN;

        let mut mean_res = f64::NAN;
        let mut stddev = f64::NAN;

        let mut ret = self.mean_arithmetic(&mut mean_res);
        if ret != Errors::ClErrorCodeNoError || !mean_res.is_finite() {
            return ret;
        }

        ret = self.standard_deviation(enums::ClFrequency::ClFrequencyMonthly, false, &mut stddev);
        if ret != Errors::ClErrorCodeNoError || !stddev.is_finite() || stddev == 0.0 {
            return ret;
        }

        *result = (observerd_value - mean_res) / stddev;
        return Errors::ClErrorCodeNoError;
    }
}

#[cfg(test)]
mod test {
    use crate::{
        enums::{self, Errors},
        MPTCalculator,
    };

    #[test]
    fn should_correct_average() {
        let data = vec![10.0, 20.0, 30.0];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v(&data);
        let err = mpt.average(&mut res);
        assert_eq!(err == Errors::ClErrorCodeNoError && res == 20.0, true);
    }
    #[test]
    fn should_correct_stddev() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v(&data);
        let err = mpt.standard_deviation(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 15.99317),
            true
        );
    }

    #[test]
    fn should_correct_gain_stddev() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v(&data);
        let err =
            mpt.gain_standard_deviation(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 5.03185),
            true
        );
    }
    #[test]
    fn should_correct_loss_stddev() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v(&data);
        let err =
            mpt.loss_standard_deviation(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 14.88251),
            true
        );
    }

    #[test]
    fn should_correct_semin_stddev() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v(&data);
        let err =
            mpt.semi_standard_deviation(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 13.22398),
            true
        );
    }
    #[test]
    fn should_correct_mean_harmonic() {
        let data = vec![-1.5, 2.3, 4.5];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v(&data);
        let err = mpt.mean_harmonic(&mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, -310.5),
            true
        );
    }
    #[test]
    fn should_correct_weighted_mean_arithmetic() {
        let data = vec![-1.5, 2.3, 4.5];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v(&data);
        let weights = vec![0.1, 0.2, 0.3];
        let err = mpt.weighted_mean_arithmetic(&weights, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 2.76666667),
            true
        );
    }
    #[test]
    fn should_correct_weighted_mean_geometic() {
        let data = vec![
            1.22072, 0.0668, 2.20588, 0.91563, 0.76766, 1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, 1.80305, 0.6709, 3.57769, 4.77481, 0.37317, 3.52713,
            1.88831, 1.73502, 1.20155, 3.36542, 2.03551, 5.6145, 2.71663, 0.04815, 3.99807,
            1.66744, 9.68658, 0.46681, 4.22095, 6.7, 15.27331, 8.46123, 0.76369, 10.32347,
        ];

        let weighting = vec![
            3.683070486,
            2.698835031,
            2.615091784,
            2.829245119,
            4.197477687,
            3.747731115,
            1.428980992,
            1.490970258,
            3.776323531,
            1.126182408,
            4.589706355,
            2.213203472,
            3.290841193,
            1.574023637,
            2.7073515,
            2.067657476,
            2.715387407,
            3.782522676,
            4.737767273,
            3.587905857,
            1.00234693,
            3.598129659,
            2.182956354,
            2.399354298,
            0.893462788,
            1.636175797,
            1.182474797,
            4.58802791,
            3.983018253,
            4.741795995,
            2.837587798,
            2.613364024,
            4.084667264,
            0.443121313,
            1.119531868,
            3.833709695,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v(&data);
        let err = mpt.weighted_mean_geometric(&weighting, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 1.943367298),
            true
        );
    }
    #[test]
    fn should_correct_weighted_mean_harmonic() {
        let data = vec![
            1.22072, 0.0668, 2.20588, 0.91563, 0.76766, 1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, 1.80305, 0.6709, 3.57769, 4.77481, 0.37317, 3.52713,
            1.88831, 1.73502, 1.20155, 3.36542, 2.03551, 5.6145, 2.71663, 0.04815, 3.99807,
            1.66744, 9.68658, 0.46681, 4.22095, 6.7, 15.27331, 8.46123, 0.76369, 10.32347,
        ];

        let weighting = vec![
            3.683070486,
            2.698835031,
            2.615091784,
            2.829245119,
            4.197477687,
            3.747731115,
            1.428980992,
            1.490970258,
            3.776323531,
            1.126182408,
            4.589706355,
            2.213203472,
            3.290841193,
            1.574023637,
            2.7073515,
            2.067657476,
            2.715387407,
            3.782522676,
            4.737767273,
            3.587905857,
            1.00234693,
            3.598129659,
            2.182956354,
            2.399354298,
            0.893462788,
            1.636175797,
            1.182474797,
            4.58802791,
            3.983018253,
            4.741795995,
            2.837587798,
            2.613364024,
            4.084667264,
            0.443121313,
            1.119531868,
            3.833709695,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v(&data);
        let err = mpt.weighted_mean_harmonic(&weighting, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 0.726329928),
            true
        );
    }

    #[test]
    fn should_correct_mean_geometric() {
        let data = vec![
            1.22072, 0.0668, 2.20588, 0.91563, 0.76766, 1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, 1.80305, 0.6709, 3.57769, 4.77481, 0.37317, 3.52713,
            1.88831, 1.73502, 1.20155, 3.36542, 2.03551, 5.6145, 2.71663, 0.04815, 3.99807,
            1.66744, 9.68658, 0.46681, 4.22095, 6.7, 15.27331, 8.46123, 0.76369, 10.32347,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v(&data);
        let err = mpt.mean_geometric(&mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 1.920852518),
            true
        );
    }

    #[test]
    fn should_correct_arithmetic_mean() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v(&data);
        let err = mpt.mean_arithmetic(&mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, -0.85194),
            true
        );
    }

    #[test]
    fn should_correct_arithmetic_mean_annu() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v(&data);
        let err = mpt.mean_arithmetic_annu(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, -10.223263),
            true
        );
    }

    #[test]
    fn should_correct_weighted_standard_deviation() {
        let data = vec![
            1.22072, 0.0668, 2.20588, 0.91563, 0.76766, 1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, 1.80305, 0.6709, 3.57769, 4.77481, 0.37317, 3.52713,
            1.88831, 1.73502, 1.20155, 3.36542, 2.03551, 5.6145, 2.71663, 0.04815, 3.99807,
            1.66744, 9.68658, 0.46681, 4.22095, 6.7, 15.27331, 8.46123, 0.76369, 10.32347,
        ];

        let weighting = vec![
            3.683070486,
            2.698835031,
            2.615091784,
            2.829245119,
            4.197477687,
            3.747731115,
            1.428980992,
            1.490970258,
            3.776323531,
            1.126182408,
            4.589706355,
            2.213203472,
            3.290841193,
            1.574023637,
            2.7073515,
            2.067657476,
            2.715387407,
            3.782522676,
            4.737767273,
            3.587905857,
            1.00234693,
            3.598129659,
            2.182956354,
            2.399354298,
            0.893462788,
            1.636175797,
            1.182474797,
            4.58802791,
            3.983018253,
            4.741795995,
            2.837587798,
            2.613364024,
            4.084667264,
            0.443121313,
            1.119531868,
            3.833709695,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v(&data);
        let err = mpt.weighted_standard_deviation(&weighting, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 3.586653428),
            true
        );
    }

    #[test]
    fn should_correct_skewness() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v(&data);
        let err = mpt.skewness(&mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, -1.31604),
            true
        );
    }

    #[test]
    fn should_correct_kurtosis() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v(&data);
        let err = mpt.kurtosis(&mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 1.76946),
            true
        );
    }

    #[test]
    fn should_correct_sharpe_ratio() {
        let data = vec![
            -1.76334, -3.7317, -0.49068, 11.83432, 9.08289, 3.39531, 0.70368, 0.89286, -0.76953,
            6.39783, 1.38484, 2.33645, 2.80998, 0.5808, -0.61141, -0.20506, -0.47945, -0.13765,
            -3.4459, -0.85653, 1.83585, 0.84836, 3.61024, 3.99188, -1.7892, 2.02054, -0.81169,
            -1.40753, 3.02125, -0.67676, 1.07073, -2.21509, 0.29547, -2.65139, 2.62273, -0.65557,
            0.76463, -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825,
            3.89481, 1.59564, 0.86793,
        ];
        let rf_data = vec![
            0.10075, 0.0999, 0.09735, 0.0982, 0.09311, 0.08124, 0.07785, 0.08209, 0.08124, 0.07955,
            0.0804, 0.07701, 0.07701, 0.07955, 0.0804, 0.0804, 0.08887, 0.10923, 0.11602, 0.12791,
            0.14235, 0.15085, 0.17806, 0.19083, 0.20105, 0.21894, 0.23855, 0.24111, 0.24708,
            0.25903, 0.27868, 0.30004, 0.3009, 0.32143, 0.34026, 0.33884, 0.36586, 0.38497,
            0.39406, 0.40057, 0.41237, 0.41911, 0.43358, 0.43548, 0.42107, 0.42743, 0.43278,
            0.4235,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v_r(&data, &rf_data);
        let err = mpt.sharpe_ratio(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 0.94596),
            true
        );
    }

    #[test]
    fn should_correct_sharpe_ratio_arithmetic() {
        let data = vec![
            -1.76334, -3.7317, -0.49068, 11.83432, 9.08289, 3.39531, 0.70368, 0.89286, -0.76953,
            6.39783, 1.38484, 2.33645, 2.80998, 0.5808, -0.61141, -0.20506, -0.47945, -0.13765,
            -3.4459, -0.85653, 1.83585, 0.84836, 3.61024, 3.99188, -1.7892, 2.02054, -0.81169,
            -1.40753, 3.02125, -0.67676, 1.07073, -2.21509, 0.29547, -2.65139, 2.62273, -0.65557,
            0.76463, -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825,
            3.89481, 1.59564, 0.86793,
        ];
        let rf_data = vec![
            0.10075, 0.0999, 0.09735, 0.0982, 0.09311, 0.08124, 0.07785, 0.08209, 0.08124, 0.07955,
            0.0804, 0.07701, 0.07701, 0.07955, 0.0804, 0.0804, 0.08887, 0.10923, 0.11602, 0.12791,
            0.14235, 0.15085, 0.17806, 0.19083, 0.20105, 0.21894, 0.23855, 0.24111, 0.24708,
            0.25903, 0.27868, 0.30004, 0.3009, 0.32143, 0.34026, 0.33884, 0.36586, 0.38497,
            0.39406, 0.40057, 0.41237, 0.41911, 0.43358, 0.43548, 0.42107, 0.42743, 0.43278,
            0.4235,
        ];
        let mut res = f64::NAN;
        let mpt = MPTCalculator::from_v_r(&data, &rf_data);
        let err =
            mpt.sharpe_ratio_arithmetic(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 0.96502),
            true
        );
    }

    #[test]
    fn should_correct_sharpe_ratio_geometric() {
        let data = vec![
            -1.76334, -3.7317, -0.49068, 11.83432, 9.08289, 3.39531, 0.70368, 0.89286, -0.76953,
            6.39783, 1.38484, 2.33645, 2.80998, 0.5808, -0.61141, -0.20506, -0.47945, -0.13765,
            -3.4459, -0.85653, 1.83585, 0.84836, 3.61024, 3.99188, -1.7892, 2.02054, -0.81169,
            -1.40753, 3.02125, -0.67676, 1.07073, -2.21509, 0.29547, -2.65139, 2.62273, -0.65557,
            0.76463, -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825,
            3.89481, 1.59564, 0.86793,
        ];
        let rf_data = vec![
            0.10075, 0.0999, 0.09735, 0.0982, 0.09311, 0.08124, 0.07785, 0.08209, 0.08124, 0.07955,
            0.0804, 0.07701, 0.07701, 0.07955, 0.0804, 0.0804, 0.08887, 0.10923, 0.11602, 0.12791,
            0.14235, 0.15085, 0.17806, 0.19083, 0.20105, 0.21894, 0.23855, 0.24111, 0.24708,
            0.25903, 0.27868, 0.30004, 0.3009, 0.32143, 0.34026, 0.33884, 0.36586, 0.38497,
            0.39406, 0.40057, 0.41237, 0.41911, 0.43358, 0.43548, 0.42107, 0.42743, 0.43278,
            0.4235,
        ];
        let mut res = f64::NAN;
        let mpt = MPTCalculator::from_v_r(&data, &rf_data);
        let err =
            mpt.sharpe_ratio_geometric(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 0.93957),
            true
        );
    }

    #[test]
    fn should_correct_sortino_ratio() {
        let data = vec![
            -1.76334, -3.7317, -0.49068, 11.83432, 9.08289, 3.39531, 0.70368, 0.89286, -0.76953,
            6.39783, 1.38484, 2.33645, 2.80998, 0.5808, -0.61141, -0.20506, -0.47945, -0.13765,
            -3.4459, -0.85653, 1.83585, 0.84836, 3.61024, 3.99188, -1.7892, 2.02054, -0.81169,
            -1.40753, 3.02125, -0.67676, 1.07073, -2.21509, 0.29547, -2.65139, 2.62273, -0.65557,
            0.76463, -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825,
            3.89481, 1.59564, 0.86793,
        ];
        let rf_data = vec![
            0.10075, 0.0999, 0.09735, 0.0982, 0.09311, 0.08124, 0.07785, 0.08209, 0.08124, 0.07955,
            0.0804, 0.07701, 0.07701, 0.07955, 0.0804, 0.0804, 0.08887, 0.10923, 0.11602, 0.12791,
            0.14235, 0.15085, 0.17806, 0.19083, 0.20105, 0.21894, 0.23855, 0.24111, 0.24708,
            0.25903, 0.27868, 0.30004, 0.3009, 0.32143, 0.34026, 0.33884, 0.36586, 0.38497,
            0.39406, 0.40057, 0.41237, 0.41911, 0.43358, 0.43548, 0.42107, 0.42743, 0.43278,
            0.4235,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v_r(&data, &rf_data);
        let err = mpt.sortino_ratio(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 2.37108),
            true
        );
    }

    #[test]
    fn should_correct_sortino_ratio_arithmetic() {
        let data = vec![
            -1.76334, -3.7317, -0.49068, 11.83432, 9.08289, 3.39531, 0.70368, 0.89286, -0.76953,
            6.39783, 1.38484, 2.33645, 2.80998, 0.5808, -0.61141, -0.20506, -0.47945, -0.13765,
            -3.4459, -0.85653, 1.83585, 0.84836, 3.61024, 3.99188, -1.7892, 2.02054, -0.81169,
            -1.40753, 3.02125, -0.67676, 1.07073, -2.21509, 0.29547, -2.65139, 2.62273, -0.65557,
            0.76463, -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825,
            3.89481, 1.59564, 0.86793,
        ];
        let rf_data = vec![
            0.10075, 0.0999, 0.09735, 0.0982, 0.09311, 0.08124, 0.07785, 0.08209, 0.08124, 0.07955,
            0.0804, 0.07701, 0.07701, 0.07955, 0.0804, 0.0804, 0.08887, 0.10923, 0.11602, 0.12791,
            0.14235, 0.15085, 0.17806, 0.19083, 0.20105, 0.21894, 0.23855, 0.24111, 0.24708,
            0.25903, 0.27868, 0.30004, 0.3009, 0.32143, 0.34026, 0.33884, 0.36586, 0.38497,
            0.39406, 0.40057, 0.41237, 0.41911, 0.43358, 0.43548, 0.42107, 0.42743, 0.43278,
            0.4235,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v_r(&data, &rf_data);
        let err =
            mpt.sortino_ratio_arithmetic(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 0.96502248),
            true
        );
    }

    #[test]
    fn should_correct_sortino_ratio_geometric() {
        let data = vec![
            -1.76334, -3.7317, -0.49068, 11.83432, 9.08289, 3.39531, 0.70368, 0.89286, -0.76953,
            6.39783, 1.38484, 2.33645, 2.80998, 0.5808, -0.61141, -0.20506, -0.47945, -0.13765,
            -3.4459, -0.85653, 1.83585, 0.84836, 3.61024, 3.99188, -1.7892, 2.02054, -0.81169,
            -1.40753, 3.02125, -0.67676, 1.07073, -2.21509, 0.29547, -2.65139, 2.62273, -0.65557,
            0.76463, -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825,
            3.89481, 1.59564, 0.86793,
        ];
        let rf_data = vec![
            0.10075, 0.0999, 0.09735, 0.0982, 0.09311, 0.08124, 0.07785, 0.08209, 0.08124, 0.07955,
            0.0804, 0.07701, 0.07701, 0.07955, 0.0804, 0.0804, 0.08887, 0.10923, 0.11602, 0.12791,
            0.14235, 0.15085, 0.17806, 0.19083, 0.20105, 0.21894, 0.23855, 0.24111, 0.24708,
            0.25903, 0.27868, 0.30004, 0.3009, 0.32143, 0.34026, 0.33884, 0.36586, 0.38497,
            0.39406, 0.40057, 0.41237, 0.41911, 0.43358, 0.43548, 0.42107, 0.42743, 0.43278,
            0.4235,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v_r(&data, &rf_data);
        let err =
            mpt.sortino_ratio_geometric(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 2.34312),
            true
        );
    }

    #[test]
    fn should_correct_omega() {
        let data = vec![
            -1.76334, -3.7317, -0.49068, 11.83432, 9.08289, 3.39531, 0.70368, 0.89286, -0.76953,
            6.39783, 1.38484, 2.33645, 2.80998, 0.5808, -0.61141, -0.20506, -0.47945, -0.13765,
            -3.4459, -0.85653, 1.83585, 0.84836, 3.61024, 3.99188, -1.7892, 2.02054, -0.81169,
            -1.40753, 3.02125, -0.67676, 1.07073, -2.21509, 0.29547, -2.65139, 2.62273, -0.65557,
            0.76463, -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825,
            3.89481, 1.59564, 0.86793,
        ];
        let rf_data = vec![
            0.10075, 0.0999, 0.09735, 0.0982, 0.09311, 0.08124, 0.07785, 0.08209, 0.08124, 0.07955,
            0.0804, 0.07701, 0.07701, 0.07955, 0.0804, 0.0804, 0.08887, 0.10923, 0.11602, 0.12791,
            0.14235, 0.15085, 0.17806, 0.19083, 0.20105, 0.21894, 0.23855, 0.24111, 0.24708,
            0.25903, 0.27868, 0.30004, 0.3009, 0.32143, 0.34026, 0.33884, 0.36586, 0.38497,
            0.39406, 0.40057, 0.41237, 0.41911, 0.43358, 0.43548, 0.42107, 0.42743, 0.43278,
            0.4235,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v_r(&data, &rf_data);
        let err = mpt.omega(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError
                && MPTCalculator::is_eq_double(res, 2.2412239894355674),
            true
        );
    }

    #[test]
    fn should_correct_kappa3() {
        let data = vec![
            -1.76334, -3.7317, -0.49068, 11.83432, 9.08289, 3.39531, 0.70368, 0.89286, -0.76953,
            6.39783, 1.38484, 2.33645, 2.80998, 0.5808, -0.61141, -0.20506, -0.47945, -0.13765,
            -3.4459, -0.85653, 1.83585, 0.84836, 3.61024, 3.99188, -1.7892, 2.02054, -0.81169,
            -1.40753, 3.02125, -0.67676, 1.07073, -2.21509, 0.29547, -2.65139, 2.62273, -0.65557,
            0.76463, -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825,
            3.89481, 1.59564, 0.86793,
        ];
        let rf_data = vec![
            0.10075, 0.0999, 0.09735, 0.0982, 0.09311, 0.08124, 0.07785, 0.08209, 0.08124, 0.07955,
            0.0804, 0.07701, 0.07701, 0.07955, 0.0804, 0.0804, 0.08887, 0.10923, 0.11602, 0.12791,
            0.14235, 0.15085, 0.17806, 0.19083, 0.20105, 0.21894, 0.23855, 0.24111, 0.24708,
            0.25903, 0.27868, 0.30004, 0.3009, 0.32143, 0.34026, 0.33884, 0.36586, 0.38497,
            0.39406, 0.40057, 0.41237, 0.41911, 0.43358, 0.43548, 0.42107, 0.42743, 0.43278,
            0.4235,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v_r(&data, &rf_data);
        let err = mpt.kappa3(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 2.77311069),
            true
        );
    }

    #[test]
    fn should_correct_gain_loss_ratio() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v(&data);
        let err = mpt.gain_loss_ratio(&mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 0.58877),
            true
        );
    }

    #[test]
    fn should_correct_coefficeient_viaiantion() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v(&data);
        let err = mpt.coefficeient_viaiantion(&mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, -5.41921),
            true
        );
    }

    #[test]
    fn should_correct_efficiency_ratio_arthmetic() {
        let data = vec![
            2.8709, -1.6506, 0.8281, 4.8182, 4.0484, -0.4246, -1.8230, 1.1619, 6.2151, 5.3158,
            -3.7904, 0.3500, -8.9486, -1.6029, -2.1879, 6.5159, 3.0498, -8.3762, -3.9341, -0.0780,
            -17.9807, -21.5895, -11.3292, 4.8884, -7.5447, -7.5943, 13.9102, 13.6679, 6.2313,
            -1.3755, 8.7637, 2.1660, 5.3087, -5.4276, 5.4496, 4.3492,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v(&data);
        let err =
            mpt.efficiency_ratio_arthmetic(enums::ClFrequency::ClFrequencyMonthly, false, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, -0.020986),
            true
        );
    }

    #[test]
    fn should_correct_jarque_bera() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v(&data);
        let err = mpt.jarque_bera(&mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 15.08823),
            true
        );
    }

    #[test]
    fn should_correct_median() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v(&data);
        let err = mpt.median(&mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, -0.057475),
            true
        );
    }

    #[test]
    fn should_correct_median_weighted() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v(&data);
        let err = mpt.median_weighted(&mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, -0.057475),
            true
        );
    }

    #[test]
    fn should_correct_down_month_percent() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v(&data);
        let err = mpt.down_month_percent(&mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 52.77778),
            true
        );
    }

    #[test]
    fn should_correct_up_month_percent() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v(&data);
        let err = mpt.up_month_percent(&mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 47.22222),
            true
        );
    }

    #[test]
    fn should_correct_average_gain_loss() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let mut avg_gain = 0.0;
        let mut avg_loss = 0.0;
        let mpt = MPTCalculator::from_v(&data);
        let err = mpt.average_gain_loss(&mut avg_gain, &mut avg_loss);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(avg_gain, 2.57330),
            true
        );
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(avg_loss, -4.01982),
            true
        );
    }

    #[test]
    fn should_correct_max_draw_down() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];

        let dates = vec![
            38776, 38807, 38837, 38868, 38898, 38929, 38960, 38990, 39021, 39051, 39082, 39113,
            39141, 39172, 39202, 39233, 39263, 39294, 39325, 39355, 39386, 39416, 39447, 39478,
            39507, 39538, 39568, 39599, 39629, 39660, 39691, 39721, 39752, 39782, 39813, 39844,
        ];
        let mut max_draw_down = f64::NAN;
        let mut max_draw_down_peek_date = 0;
        let mut max_draw_down_valley_date = 0;
        let mut max_draw_down_month = 0;
        let mut recovery_month = 0;
        let mut recovery_date = 0;

        let mpt = MPTCalculator::from_v(&data);
        let err = mpt.max_draw_down(
            &dates,
            enums::ClFrequency::ClFrequencyMonthly,
            &mut max_draw_down,
            &mut max_draw_down_peek_date,
            &mut max_draw_down_valley_date,
            &mut max_draw_down_month,
            &mut recovery_month,
            &mut recovery_date,
        );
        assert_eq!(
            err == Errors::ClErrorCodeNoError
                && MPTCalculator::is_eq_double(max_draw_down, -43.72595),
            true
        );
        assert_eq!(
            err == Errors::ClErrorCodeNoError && max_draw_down_peek_date == 39387,
            true
        );
        assert_eq!(
            err == Errors::ClErrorCodeNoError && max_draw_down_valley_date == 39844,
            true
        );
        assert_eq!(
            err == Errors::ClErrorCodeNoError && max_draw_down_month == 15,
            true
        );
        assert_eq!(
            err == Errors::ClErrorCodeNoError && recovery_month == 0,
            true
        );
        assert_eq!(
            err == Errors::ClErrorCodeNoError && recovery_date == 0,
            true
        );
    }

    #[test]
    fn should_correct_max_gain() {
        let data = vec![
            -2.57909, 0.0353, 3.56387, -3.88416, 0.0, -9.81106, -7.70466, -0.04348, -9.65637,
            3.37025, 7.68514, -6.79066, -1.76334, -3.7317, -0.49068, 11.83432, 9.08289, 3.39531,
            0.70368, 0.89286, -0.76953, 6.39783, 1.38484, 2.33645, 2.80998, 0.5808, -0.61141,
            -0.20506, -0.47945, -0.13765, -3.4459, -0.85653, 1.83585, 0.84836, 3.61024, 3.99188,
            -1.7892, 2.02054, -0.81169, -1.40753, 3.02125, -0.67676, 1.07073, -2.21509, 0.29547,
            -2.65139, 2.62273, -0.65557, 0.76463, -1.22072, -0.0668, 2.20588, -0.91563, -0.76766,
            -1.21429, 3.43456, 4.99825, 3.89481, 1.59564, 0.86793, 2.41477, -1.80305, 0.6709,
            3.57769, 4.77481, -0.37317, -3.52713, 1.88831, 1.73502, 1.20155, -3.36542, -2.03551,
            -5.6145, -2.71663, -0.04815, 3.99807, 1.66744, -9.68658, -0.46681, 4.22095, -6.7,
            -15.27331, -8.46123, 0.76369,
        ];

        let dates = vec![
            37287, 37315, 37346, 37376, 37407, 37437, 37468, 37499, 37529, 37560, 37590, 37621,
            37652, 37680, 37711, 37741, 37772, 37802, 37833, 37864, 37894, 37925, 37955, 37986,
            38017, 38046, 38077, 38107, 38138, 38168, 38199, 38230, 38260, 38291, 38321, 38352,
            38383, 38411, 38442, 38472, 38503, 38533, 38564, 38595, 38625, 38656, 38686, 38717,
            38748, 38776, 38807, 38837, 38868, 38898, 38929, 38960, 38990, 39021, 39051, 39082,
            39113, 39141, 39172, 39202, 39233, 39263, 39294, 39325, 39355, 39386, 39416, 39447,
            39478, 39507, 39538, 39568, 39599, 39629, 39660, 39691, 39721, 39752, 39782, 39813,
        ];
        let mut max_gain = f64::NAN;
        let mut start_date = 0;
        let mut end_date = 0;
        let mut max_gain_month = 0;

        let mpt = MPTCalculator::from_v(&data);
        let err = mpt.max_gain(
            &dates,
            enums::ClFrequency::ClFrequencyMonthly,
            &mut max_gain,
            &mut start_date,
            &mut end_date,
            &mut max_gain_month,
        );
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(max_gain, 89.10075),
            true
        );
        assert_eq!(
            err == Errors::ClErrorCodeNoError && start_date == 37712,
            true
        );
        assert_eq!(err == Errors::ClErrorCodeNoError && end_date == 39386, true);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && max_gain_month == 55,
            true
        );
    }
    #[test]
    fn should_correct_calmar_ratio() {
        let data = vec![
            1.52768, 4.04616, 3.40287, -2.43748, 2.1044, -1.7708, -1.89656, 3.18186, 0.14197,
            3.71883, -0.9124, 0.80994, -1.66708, 3.78221, 0.03481, 2.64778, 0.27133, 1.24475,
            1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016, 1.40278, 1.51232,
            -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901, 3.73988, 1.59068,
            -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526, -8.43036, -0.84062,
            1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864, -10.64778, 8.75952,
        ];

        let dates = vec![
            38291, 38321, 38352, 38383, 38411, 38442, 38472, 38503, 38533, 38564, 38595, 38625,
            38656, 38686, 38717, 38748, 38776, 38807, 38837, 38868, 38898, 38929, 38960, 38990,
            39021, 39051, 39082, 39113, 39141, 39172, 39202, 39233, 39263, 39294, 39325, 39355,
            39386, 39416, 39447, 39478, 39507, 39538, 39568, 39599, 39629, 39660, 39691, 39721,
            39752, 39782, 39813, 39844, 39872, 39903,
        ];
        let mut result = f64::NAN;
        let mpt = MPTCalculator::from_v(&data);
        let err =
            mpt.average_draw_down(&dates, enums::ClFrequency::ClFrequencyMonthly, &mut result);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(result, -15.76075),
            true
        );
    }

    #[test]
    fn should_correct_average_draw_down() {
        let data = vec![
            1.52768, 4.04616, 3.40287, -2.43748, 2.1044, -1.7708, -1.89656, 3.18186, 0.14197,
            3.71883, -0.9124, 0.80994, -1.66708, 3.78221, 0.03481, 2.64778, 0.27133, 1.24475,
            1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016, 1.40278, 1.51232,
            -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901, 3.73988, 1.59068,
            -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526, -8.43036, -0.84062,
            1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864, -10.64778, 8.75952,
        ];

        let dates = vec![
            38291, 38321, 38352, 38383, 38411, 38442, 38472, 38503, 38533, 38564, 38595, 38625,
            38656, 38686, 38717, 38748, 38776, 38807, 38837, 38868, 38898, 38929, 38960, 38990,
            39021, 39051, 39082, 39113, 39141, 39172, 39202, 39233, 39263, 39294, 39325, 39355,
            39386, 39416, 39447, 39478, 39507, 39538, 39568, 39599, 39629, 39660, 39691, 39721,
            39752, 39782, 39813, 39844, 39872, 39903,
        ];
        let mut result = f64::NAN;
        let mpt = MPTCalculator::from_v(&data);
        let err =
            mpt.average_draw_down(&dates, enums::ClFrequency::ClFrequencyMonthly, &mut result);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(result, -15.76075),
            true
        );
    }

    #[test]
    fn should_correct_sterling_ratio() {
        let data = vec![
            1.52768, 4.04616, 3.40287, -2.43748, 2.1044, -1.7708, -1.89656, 3.18186, 0.14197,
            3.71883, -0.9124, 0.80994, -1.66708, 3.78221, 0.03481, 2.64778, 0.27133, 1.24475,
            1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016, 1.40278, 1.51232,
            -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901, 3.73988, 1.59068,
            -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526, -8.43036, -0.84062,
            1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864, -10.64778, 8.75952,
        ];

        let dates = vec![
            38291, 38321, 38352, 38383, 38411, 38442, 38472, 38503, 38533, 38564, 38595, 38625,
            38656, 38686, 38717, 38748, 38776, 38807, 38837, 38868, 38898, 38929, 38960, 38990,
            39021, 39051, 39082, 39113, 39141, 39172, 39202, 39233, 39263, 39294, 39325, 39355,
            39386, 39416, 39447, 39478, 39507, 39538, 39568, 39599, 39629, 39660, 39691, 39721,
            39752, 39782, 39813, 39844, 39872, 39903,
        ];
        let mut result = f64::NAN;
        let mpt = MPTCalculator::from_v(&data);
        let err = mpt.sterling_ratio(&dates, enums::ClFrequency::ClFrequencyMonthly, &mut result);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(result, -0.2034894),
            true
        );
    }

    #[test]
    fn should_correct_best_rolling_month() {
        let data = vec![
            -2.57909, 0.0353, 3.56387, -3.88416, 0.0, -9.81106, -7.70466, -0.04348, -9.65637,
            3.37025, 7.68514, -6.79066, -1.76334, -3.7317, -0.49068, 11.83432, 9.08289, 3.39531,
            0.70368, 0.89286, -0.76953, 6.39783, 1.38484, 2.33645, 2.80998, 0.5808, -0.61141,
            -0.20506, -0.47945, -0.13765, -3.4459, -0.85653, 1.83585, 0.84836, 3.61024, 3.99188,
            -1.7892, 2.02054, -0.81169, -1.40753, 3.02125, -0.67676, 1.07073, -2.21509, 0.29547,
            -2.65139, 2.62273, -0.65557, 0.76463, -1.22072, -0.0668, 2.20588, -0.91563, -0.76766,
            -1.21429, 3.43456, 4.99825, 3.89481, 1.59564, 0.86793, 2.41477, -1.80305, 0.6709,
            3.57769, 4.77481, -0.37317, -3.52713, 1.88831, 1.73502, 1.20155, -3.36542, -2.03551,
            -5.6145, -2.71663, -0.04815, 3.99807, 1.66744, -9.68658, -0.46681, 4.22095, -6.7,
            -15.27331, -8.46123, 0.76369,
        ];

        let dates = vec![
            37287, 37315, 37346, 37376, 37407, 37437, 37468, 37499, 37529, 37560, 37590, 37621,
            37652, 37680, 37711, 37741, 37772, 37802, 37833, 37864, 37894, 37925, 37955, 37986,
            38017, 38046, 38077, 38107, 38138, 38168, 38199, 38230, 38260, 38291, 38321, 38352,
            38383, 38411, 38442, 38472, 38503, 38533, 38564, 38595, 38625, 38656, 38686, 38717,
            38748, 38776, 38807, 38837, 38868, 38898, 38929, 38960, 38990, 39021, 39051, 39082,
            39113, 39141, 39172, 39202, 39233, 39263, 39294, 39325, 39355, 39386, 39416, 39447,
            39478, 39507, 39538, 39568, 39599, 39629, 39660, 39691, 39721, 39752, 39782, 39813,
        ];

        let mut best_rolling_month_date = 0;
        let mut best_rolling_month_value = f64::NAN;
        let mpt = MPTCalculator::from_v(&data);
        let err = mpt.best_rolling_month(
            &dates,
            3,
            &mut best_rolling_month_date,
            &mut best_rolling_month_value,
        );
        assert_eq!(
            err == Errors::ClErrorCodeNoError
                && MPTCalculator::is_eq_double(best_rolling_month_value, 26.13411852),
            true
        );
        assert_eq!(
            err == Errors::ClErrorCodeNoError && best_rolling_month_date == 37802,
            true
        );
    }

    #[test]
    fn should_correct_worst_rolling_month() {
        let data = vec![
            -2.57909, 0.0353, 3.56387, -3.88416, 0.0, -9.81106, -7.70466, -0.04348, -9.65637,
            3.37025, 7.68514, -6.79066, -1.76334, -3.7317, -0.49068, 11.83432, 9.08289, 3.39531,
            0.70368, 0.89286, -0.76953, 6.39783, 1.38484, 2.33645, 2.80998, 0.5808, -0.61141,
            -0.20506, -0.47945, -0.13765, -3.4459, -0.85653, 1.83585, 0.84836, 3.61024, 3.99188,
            -1.7892, 2.02054, -0.81169, -1.40753, 3.02125, -0.67676, 1.07073, -2.21509, 0.29547,
            -2.65139, 2.62273, -0.65557, 0.76463, -1.22072, -0.0668, 2.20588, -0.91563, -0.76766,
            -1.21429, 3.43456, 4.99825, 3.89481, 1.59564, 0.86793, 2.41477, -1.80305, 0.6709,
            3.57769, 4.77481, -0.37317, -3.52713, 1.88831, 1.73502, 1.20155, -3.36542, -2.03551,
            -5.6145, -2.71663, -0.04815, 3.99807, 1.66744, -9.68658, -0.46681, 4.22095, -6.7,
            -15.27331, -8.46123, 0.76369,
        ];

        let dates = vec![
            37287, 37315, 37346, 37376, 37407, 37437, 37468, 37499, 37529, 37560, 37590, 37621,
            37652, 37680, 37711, 37741, 37772, 37802, 37833, 37864, 37894, 37925, 37955, 37986,
            38017, 38046, 38077, 38107, 38138, 38168, 38199, 38230, 38260, 38291, 38321, 38352,
            38383, 38411, 38442, 38472, 38503, 38533, 38564, 38595, 38625, 38656, 38686, 38717,
            38748, 38776, 38807, 38837, 38868, 38898, 38929, 38960, 38990, 39021, 39051, 39082,
            39113, 39141, 39172, 39202, 39233, 39263, 39294, 39325, 39355, 39386, 39416, 39447,
            39478, 39507, 39538, 39568, 39599, 39629, 39660, 39691, 39721, 39752, 39782, 39813,
        ];

        let mut worst_rolling_month_date = 0;
        let mut worst_rolling_month_value = f64::NAN;
        let mpt = MPTCalculator::from_v(&data);
        let err = mpt.worst_rolling_month(
            &dates,
            3,
            &mut worst_rolling_month_date,
            &mut worst_rolling_month_value,
        );
        assert_eq!(
            err == Errors::ClErrorCodeNoError
                && MPTCalculator::is_eq_double(worst_rolling_month_value, -27.63860069),
            true
        );
        assert_eq!(
            err == Errors::ClErrorCodeNoError && worst_rolling_month_date == 39782,
            true
        );
    }

    #[test]
    fn should_correct_longest_down_streak() {
        let data = vec![
            -2.57909, 0.0353, 3.56387, -3.88416, 0.0, -9.81106, -7.70466, -0.04348, -9.65637,
            3.37025, 7.68514, -6.79066, -1.76334, -3.7317, -0.49068, 11.83432, 9.08289, 3.39531,
            0.70368, 0.89286, -0.76953, 6.39783, 1.38484, 2.33645, 2.80998, 0.5808, -0.61141,
            -0.20506, -0.47945, -0.13765, -3.4459, -0.85653, 1.83585, 0.84836, 3.61024, 3.99188,
            -1.7892, 2.02054, -0.81169, -1.40753, 3.02125, -0.67676, 1.07073, -2.21509, 0.29547,
            -2.65139, 2.62273, -0.65557, 0.76463, -1.22072, -0.0668, 2.20588, -0.91563, -0.76766,
            -1.21429, 3.43456, 4.99825, 3.89481, 1.59564, 0.86793, 2.41477, -1.80305, 0.6709,
            3.57769, 4.77481, -0.37317, -3.52713, 1.88831, 1.73502, 1.20155, -3.36542, -2.03551,
            -5.6145, -2.71663, -0.04815, 3.99807, 1.66744, -9.68658, -0.46681, 4.22095, -6.7,
            -15.27331, -8.46123, 0.76369,
        ];

        let dates = vec![
            37287, 37315, 37346, 37376, 37407, 37437, 37468, 37499, 37529, 37560, 37590, 37621,
            37652, 37680, 37711, 37741, 37772, 37802, 37833, 37864, 37894, 37925, 37955, 37986,
            38017, 38046, 38077, 38107, 38138, 38168, 38199, 38230, 38260, 38291, 38321, 38352,
            38383, 38411, 38442, 38472, 38503, 38533, 38564, 38595, 38625, 38656, 38686, 38717,
            38748, 38776, 38807, 38837, 38868, 38898, 38929, 38960, 38990, 39021, 39051, 39082,
            39113, 39141, 39172, 39202, 39233, 39263, 39294, 39325, 39355, 39386, 39416, 39447,
            39478, 39507, 39538, 39568, 39599, 39629, 39660, 39691, 39721, 39752, 39782, 39813,
        ];

        let mut longest_up_down_streak = f64::NAN;
        let mut longest_up_down_start_date = 0;
        let mut longest_up_down_end_date = 0;
        let mut longest_up_down_periods = 0;
        let mpt = MPTCalculator::from_v(&data);
        let err = mpt.longest_down_streak(
            &dates,
            enums::ClFrequency::ClFrequencyMonthly,
            &mut longest_up_down_streak,
            &mut longest_up_down_start_date,
            &mut longest_up_down_end_date,
            &mut longest_up_down_periods,
        );
        assert_eq!(
            err == Errors::ClErrorCodeNoError
                && MPTCalculator::is_eq_double(longest_up_down_streak, -5.63859),
            true
        );
        assert_eq!(
            err == Errors::ClErrorCodeNoError && longest_up_down_start_date == 38047,
            true
        );
        assert_eq!(
            err == Errors::ClErrorCodeNoError && longest_up_down_end_date == 38230,
            true
        );
        assert_eq!(
            err == Errors::ClErrorCodeNoError && longest_up_down_periods == 6,
            true
        );
    }

    #[test]
    fn should_correct_longest_up_streak() {
        let data = vec![
            -2.57909, 0.0353, 3.56387, -3.88416, 0.0, -9.81106, -7.70466, -0.04348, -9.65637,
            3.37025, 7.68514, -6.79066, -1.76334, -3.7317, -0.49068, 11.83432, 9.08289, 3.39531,
            0.70368, 0.89286, -0.76953, 6.39783, 1.38484, 2.33645, 2.80998, 0.5808, -0.61141,
            -0.20506, -0.47945, -0.13765, -3.4459, -0.85653, 1.83585, 0.84836, 3.61024, 3.99188,
            -1.7892, 2.02054, -0.81169, -1.40753, 3.02125, -0.67676, 1.07073, -2.21509, 0.29547,
            -2.65139, 2.62273, -0.65557, 0.76463, -1.22072, -0.0668, 2.20588, -0.91563, -0.76766,
            -1.21429, 3.43456, 4.99825, 3.89481, 1.59564, 0.86793, 2.41477, -1.80305, 0.6709,
            3.57769, 4.77481, -0.37317, -3.52713, 1.88831, 1.73502, 1.20155, -3.36542, -2.03551,
            -5.6145, -2.71663, -0.04815, 3.99807, 1.66744, -9.68658, -0.46681, 4.22095, -6.7,
            -15.27331, -8.46123, 0.76369,
        ];

        let dates = vec![
            37287, 37315, 37346, 37376, 37407, 37437, 37468, 37499, 37529, 37560, 37590, 37621,
            37652, 37680, 37711, 37741, 37772, 37802, 37833, 37864, 37894, 37925, 37955, 37986,
            38017, 38046, 38077, 38107, 38138, 38168, 38199, 38230, 38260, 38291, 38321, 38352,
            38383, 38411, 38442, 38472, 38503, 38533, 38564, 38595, 38625, 38656, 38686, 38717,
            38748, 38776, 38807, 38837, 38868, 38898, 38929, 38960, 38990, 39021, 39051, 39082,
            39113, 39141, 39172, 39202, 39233, 39263, 39294, 39325, 39355, 39386, 39416, 39447,
            39478, 39507, 39538, 39568, 39599, 39629, 39660, 39691, 39721, 39752, 39782, 39813,
        ];

        let mut longest_up_down_streak = f64::NAN;
        let mut longest_up_down_start_date = 0;
        let mut longest_up_down_end_date = 0;
        let mut longest_up_down_periods = 0;
        let mpt = MPTCalculator::from_v(&data);
        let err = mpt.longest_up_streak(
            &dates,
            enums::ClFrequency::ClFrequencyMonthly,
            &mut longest_up_down_streak,
            &mut longest_up_down_start_date,
            &mut longest_up_down_end_date,
            &mut longest_up_down_periods,
        );
        assert_eq!(
            err == Errors::ClErrorCodeNoError
                && MPTCalculator::is_eq_double(longest_up_down_streak, 18.42199),
            true
        );
        assert_eq!(
            err == Errors::ClErrorCodeNoError && longest_up_down_start_date == 38930,
            true
        );
        assert_eq!(
            err == Errors::ClErrorCodeNoError && longest_up_down_end_date == 39113,
            true
        );
        assert_eq!(
            err == Errors::ClErrorCodeNoError && longest_up_down_periods == 6,
            true
        );
    }

    #[test]
    fn should_correct_volatity() {
        let data = vec![
            210.69, 195.58, 190.08, 179.72, 179.72, 165.24, 163.12, 160.8, 148.96, 153.29, 169.47,
            181.52, 174.86, 184.9, 174.12, 166.82, 167.46, 165.24, 150.86, 143.88, 151.07, 150.65,
            141.13,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v(&data);
        let err = mpt.volatity(enums::ClFrequency::ClFrequencyDaily, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 83.388666),
            true
        );
    }

    #[test]
    fn should_correct_zscore() {
        let data = vec![
            210.69, 195.58, 190.08, 179.72, 179.72, 165.24, 163.12, 160.8, 148.96, 153.29, 169.47,
            181.52, 174.86, 184.9, 174.12, 166.82, 167.46, 165.24, 150.86, 143.88, 151.07, 150.65,
            141.13,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v(&data);
        let err = mpt.zscore(200.0, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError
                && MPTCalculator::is_eq_double(res, 1.813224655535872),
            true
        );
    }
}
