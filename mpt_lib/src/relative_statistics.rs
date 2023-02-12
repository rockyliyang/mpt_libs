use std::ops::ControlFlow;

use crate::{
    common::{
        annualize_return, get_annual_multiplier, CaptureData, InformationRatioData, RatioData,
        TreynorRatioData,
    },
    enums::{self, ClFrequency, Errors},
    MPTCalculator,
};
struct XYData {
    x_sum: f64,
    y_sum: f64,
    xx_sum: f64,
    yy_sum: f64,
    xy_sum: f64,
    count: usize,
}

struct BearBullXYData {
    bear_x_sum: f64,
    bear_y_sum: f64,
    bear_xx_sum: f64,
    bear_yy_sum: f64,
    bear_xy_sum: f64,
    bear_count: usize,

    bull_x_sum: f64,
    bull_y_sum: f64,
    bull_xx_sum: f64,
    bull_yy_sum: f64,
    bull_xy_sum: f64,
    bull_count: usize,
    valid_total: usize,
}

fn gather_bear_bull_xy(
    values: &[f64],
    benchmark: &[f64],
    value_array_size: usize,
) -> BearBullXYData {
    let mut xy_data = BearBullXYData {
        bear_x_sum: 0.0,
        bear_y_sum: 0.0,
        bear_xx_sum: 0.0,
        bear_yy_sum: 0.0,
        bear_xy_sum: 0.0,
        bear_count: 0,

        bull_x_sum: 0.0,
        bull_y_sum: 0.0,
        bull_xx_sum: 0.0,
        bull_yy_sum: 0.0,
        bull_xy_sum: 0.0,
        bull_count: 0,

        valid_total: 0,
    };

    for i in 0..value_array_size {
        if values[i].is_finite() && benchmark[i].is_finite() {
            if benchmark[i] < 0.0 {
                xy_data.bear_xy_sum += values[i] * benchmark[i];
                xy_data.bear_xx_sum += benchmark[i] * benchmark[i];
                xy_data.bear_yy_sum += values[i] * benchmark[i];
                xy_data.bear_y_sum += values[i];
                xy_data.bear_x_sum += benchmark[i];
                xy_data.bear_count += 1;
            } else if benchmark[i] > 0.0 {
                xy_data.bull_xy_sum += values[i] * benchmark[i];
                xy_data.bull_xx_sum += benchmark[i] * benchmark[i];
                xy_data.bull_yy_sum += values[i] * benchmark[i];
                xy_data.bull_y_sum += values[i];
                xy_data.bull_x_sum += benchmark[i];
                xy_data.bull_count += 1;
            }
            xy_data.valid_total += 1;
        }
    }

    xy_data
}

fn gather_xy(values: &[f64], benchmark: &[f64], value_array_size: usize) -> XYData {
    let mut xy_data = XYData {
        x_sum: 0.0,
        y_sum: 0.0,
        xx_sum: 0.0,
        yy_sum: 0.0,
        xy_sum: 0.0,
        count: 0,
    };

    for i in 0..value_array_size {
        if values[i].is_finite() && benchmark[i].is_finite() {
            xy_data.xy_sum += values[i] * benchmark[i];
            xy_data.xx_sum += benchmark[i] * benchmark[i];
            xy_data.yy_sum += values[i] * values[i];
            xy_data.y_sum += values[i];
            xy_data.x_sum += benchmark[i];
            xy_data.count += 1;
        }
    }

    xy_data
}
impl<'a> MPTCalculator<'a> {
    ///calculate the beta value of an array if the array has NAN/INF values,the result will be NAN.
    ///
    ///# Arguments
    ///freq: the frequence of source data
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
    ///let bmk_data = vec![
    ///    0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
    ///    1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
    ///    3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
    ///    -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
    ///let err = mpt.beta(&mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 0.97364),
    ///    true
    ///);
    ///```
    pub fn beta(&self, beta: &mut f64) -> Errors {
        let xy_data = gather_xy(self.values, self.benchmark, self.values.len());

        let stdev = xy_data.xx_sum - xy_data.x_sum * xy_data.x_sum / xy_data.count as f64;
        if xy_data.count > 0 && stdev != 0.0 {
            *beta = (xy_data.xy_sum - xy_data.x_sum * xy_data.y_sum / xy_data.count as f64) / stdev;
        } else {
            *beta = f64::NAN;
        }

        return Errors::ClErrorCodeNoError;
    }
    ///calculate the alpha value of an array if the array has NAN/INF values,the result will be NAN.
    ///
    ///# Arguments
    ///freq: the frequence of source data
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
    ///let bmk_data = vec![
    ///    0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
    ///    1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
    ///    3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
    ///    -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
    ///let err = mpt.alpha(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 0.66313),
    ///    true
    ///);
    ///```
    pub fn alpha(&self, freq: enums::ClFrequency, is_annu: bool, alpha_result: &mut f64) -> Errors {
        *alpha_result = f64::NAN;
        let xy_data = gather_xy(self.values, self.benchmark, self.values.len());

        if xy_data.count > 0 {
            let mut beta_value = 0.0;
            let ret = self.beta(&mut beta_value);
            if ret == Errors::ClErrorCodeNoError {
                let y_mean = xy_data.y_sum / xy_data.count as f64;
                let x_mean = xy_data.x_sum / xy_data.count as f64;
                *alpha_result = y_mean - x_mean * beta_value;
                if is_annu {
                    *alpha_result *= get_annual_multiplier(freq, false);
                }
            }
        }
        return Errors::ClErrorCodeNoError;
    }

    ///calculate the tracking value of an array if the array has NAN/INF values,the result will be NAN.
    ///
    ///# Arguments
    ///freq: the frequence of source data
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
    ///let bmk_data = vec![
    ///    0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
    ///    1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
    ///    3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
    ///    -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
    ///let err = mpt.tracking_error(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 4.37063),
    ///    true
    ///);
    ///```
    pub fn tracking_error(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        tracking_error_result: &mut f64,
    ) -> Errors {
        let mut excess_vec = vec![f64::NAN; self.values.len()];
        let ret = Self::array_subtraction_internal(self.values, self.benchmark, &mut excess_vec);
        if ret != Errors::ClErrorCodeNoError {
            return ret;
        }

        return Self::standard_deviation_internal(
            &excess_vec,
            freq,
            is_annu,
            tracking_error_result,
        );
    }

    fn information_ratio_calc(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        information_ratio_data_res: &mut InformationRatioData,
    ) -> Errors {
        if self
            .values
            .iter()
            .enumerate()
            .try_for_each(|v| {
                if !v.1.is_finite() || !self.benchmark[v.0].is_finite() {
                    return ControlFlow::Break(());
                }
                (*information_ratio_data_res).total_return *= 1.0 + v.1 / 100.0;
                (*information_ratio_data_res).bmk_total_return *= 1.0 + self.benchmark[v.0] / 100.0;
                (*information_ratio_data_res).sum += v.1 - self.benchmark[v.0];
                (*information_ratio_data_res).count += 1;
                ControlFlow::Continue(())
            })
            .is_break()
        {
            return Errors::ClErrorCodeNoError;
        }
        if (*information_ratio_data_res).count == 0 {
            return Errors::ClErrorCodeNoError;
        }

        let ret = self.tracking_error(
            freq,
            is_annu,
            &mut information_ratio_data_res.tracking_error,
        );

        if ret != Errors::ClErrorCodeNoError {
            return ret;
        }
        return Errors::ClErrorCodeNoError;
    }

    ///calculate the information ratio arithmetic value of an array if the array has NAN/INF values,the result will be NAN.
    ///
    ///# Arguments
    ///freq: the frequence of source data
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
    ///let bmk_data = vec![
    ///    0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
    ///    1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
    ///    3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
    ///    -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
    ///let err = mpt.information_ratio_arithmetic(
    ///    enums::ClFrequency::ClFrequencyMonthly,
    ///    true,
    ///    &mut res,
    ///);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 0.19228),
    ///    true
    ///);
    ///```
    pub fn information_ratio_arithmetic(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        information_ratio_arithmetic_res: &mut f64,
    ) -> Errors {
        *information_ratio_arithmetic_res = f64::NAN;

        let mut information_ratio_data = InformationRatioData {
            total_return: 1.0,
            bmk_total_return: 1.0,
            sum: 0.0,
            tracking_error: f64::NAN,
            count: 1,
        };

        let ret = self.information_ratio_calc(freq, is_annu, &mut information_ratio_data);

        if ret != Errors::ClErrorCodeNoError {
            return ret;
        }

        if information_ratio_data.tracking_error.is_nan()
            || information_ratio_data.tracking_error == 0.0
        {
            return Errors::ClErrorCodeNoError;
        }

        if is_annu {
            *information_ratio_arithmetic_res = f64::NAN;
            let mutiplier = get_annual_multiplier(freq, false);
            let ann_return = 100.0
                * (information_ratio_data
                    .total_return
                    .powf(mutiplier / self.values.len() as f64)
                    - 1.0);
            let ann_rf_return = 100.0
                * (information_ratio_data
                    .bmk_total_return
                    .powf(mutiplier / self.values.len() as f64)
                    - 1.0);

            *information_ratio_arithmetic_res =
                (ann_return - ann_rf_return) / information_ratio_data.tracking_error;
        } else {
            *information_ratio_arithmetic_res = (information_ratio_data.sum
                / self.values.len() as f64)
                / information_ratio_data.tracking_error;
        }

        return Errors::ClErrorCodeNoError;
    }
    ///calculate the information ratio geometric of an array if the array has NAN/INF values,the result will be NAN.
    ///
    ///# Arguments
    ///freq: the frequence of source data
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
    ///let bmk_data = vec![
    ///    0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
    ///    1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
    ///    3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
    ///    -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
    ///let err =
    ///mpt.information_ratio_geometric(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
    ///assert_eq!(
    ///err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 0.21796),
    ///true
    ///);
    ///```
    pub fn information_ratio_geometric(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        information_ratio_geometric_res: &mut f64,
    ) -> Errors {
        let mut information_ratio_data = InformationRatioData {
            total_return: 1.0,
            bmk_total_return: 1.0,
            sum: 0.0,
            tracking_error: f64::NAN,
            count: 1,
        };
        let ret = self.information_ratio_calc(freq, is_annu, &mut information_ratio_data);
        if ret != Errors::ClErrorCodeNoError {
            return ret;
        }
        if information_ratio_data.tracking_error.is_nan()
            || information_ratio_data.tracking_error == 0.0
        {
            return Errors::ClErrorCodeNoError;
        }

        if is_annu {
            let mutiplier = get_annual_multiplier(freq, false);
            let ann_return = 100.0
                * (information_ratio_data
                    .total_return
                    .powf(mutiplier / self.values.len() as f64)
                    - 1.0);
            let ann_rf_return = 100.0
                * (information_ratio_data
                    .bmk_total_return
                    .powf(mutiplier / self.values.len() as f64)
                    - 1.0);

            *information_ratio_geometric_res =
                ((100.0 + ann_return) / (100.0 + ann_rf_return) - 1.0) * 100.0
                    / information_ratio_data.tracking_error;
        } else {
            *information_ratio_geometric_res = (information_ratio_data.total_return
                / information_ratio_data.bmk_total_return
                - 1.0)
                * 100.0
                / information_ratio_data.tracking_error;
        }

        return Errors::ClErrorCodeNoError;
    }
    ///calculate the excess return geometric value of an array if the array has NAN/INF values,the result will be NAN.
    ///
    ///# Arguments
    ///freq: the frequence of source data
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
    ///let bmk_data = vec![
    ///    0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
    ///    1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
    ///    3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
    ///    -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
    ///let err =
    ///mpt.excess_return_geometric(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
    ///assert_eq!(
    ///err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 0.95263),
    ///true
    ///);
    ///```
    pub fn excess_return_geometric(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        excess: &mut f64,
    ) -> Errors {
        let mut port_ret = 1.0;
        let mut bmk_ret = 1.0;

        if self
            .values
            .iter()
            .enumerate()
            .try_for_each(|v| {
                if !v.1.is_finite() || !self.benchmark[v.0].is_finite() {
                    return ControlFlow::Break(());
                }
                port_ret *= 1.0 + v.1 / 100.0;
                bmk_ret *= 1.0 + self.benchmark[v.0] / 100.0;
                ControlFlow::Continue(())
            })
            .is_break()
        {
            return Errors::ClErrorCodeNoError;
        }

        *excess = (port_ret / bmk_ret - 1.0) * 100.0;
        if is_annu {
            *excess = annualize_return(*excess, freq, self.values.len() as f64, true);
        }

        return Errors::ClErrorCodeNoError;
    }
    ///calculate the excess return arithmetic value of an array if the array has NAN/INF values,the result will be NAN.
    ///
    ///# Arguments
    ///freq: the frequence of source data
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
    ///let bmk_data = vec![
    ///    0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
    ///    1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
    ///    3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
    ///    -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
    ///let err =
    ///mpt.excess_return_arithmetic(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
    ///assert_eq!(
    ///err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 0.84039),
    ///true
    ///);
    ///```
    pub fn excess_return_arithmetic(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        excess: &mut f64,
    ) -> Errors {
        let mut port_ret = 1.0;
        let mut bmk_ret = 1.0;
        *excess = f64::NAN;
        if self
            .values
            .iter()
            .enumerate()
            .try_for_each(|v| {
                if !v.1.is_finite() || !self.benchmark[v.0].is_finite() {
                    return ControlFlow::Break(());
                }
                port_ret *= 1.0 + v.1 / 100.0;
                bmk_ret *= 1.0 + self.benchmark[v.0] / 100.0;
                ControlFlow::Continue(())
            })
            .is_break()
        {
            return Errors::ClErrorCodeNoError;
        }

        *excess = (port_ret - bmk_ret) * 100.0;
        if is_annu {
            let annu_acct_return = annualize_return(
                (port_ret - 1.0) * 100.0,
                freq,
                self.values.len() as f64,
                true,
            );
            let bmk_acct_return = annualize_return(
                (bmk_ret - 1.0) * 100.0,
                freq,
                self.values.len() as f64,
                true,
            );

            *excess = annu_acct_return - bmk_acct_return;
        }

        return Errors::ClErrorCodeNoError;
    }
    ///calculate the excess return arithmetic value of an array if the array has NAN/INF values,the result will be NAN.
    ///
    ///# Arguments
    ///freq: the frequence of source data
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
    ///let bmk_data = vec![
    ///    0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
    ///    1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
    ///    3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
    ///    -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
    ///let err = mpt.excess_return_relative_percentage(
    ///    enums::ClFrequency::ClFrequencyMonthly,
    ///    true,
    ///    &mut res,
    ///);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 24.6508),
    ///    true
    ///);
    ///```
    pub fn excess_return_relative_percentage(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        excess: &mut f64,
    ) -> Errors {
        let mut port_ret = 1.0;
        let mut bmk_ret = 1.0;

        if self
            .values
            .iter()
            .enumerate()
            .try_for_each(|v| {
                if !v.1.is_finite() || !self.benchmark[v.0].is_finite() {
                    return ControlFlow::Break(());
                }
                port_ret *= 1.0 + v.1 / 100.0;
                bmk_ret *= 1.0 + self.benchmark[v.0] / 100.0;
                ControlFlow::Continue(())
            })
            .is_break()
        {
            return Errors::ClErrorCodeNoError;
        }

        *excess = (port_ret - 1.0) / (bmk_ret - 1.0) * 100.0;
        if is_annu {
            *excess = annualize_return(*excess, freq, self.values.len() as f64, true);
        }

        return Errors::ClErrorCodeNoError;
    }

    pub fn up_downside_standard_deviation(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        cmp_fn: fn(f64, f64) -> bool,
        up_downside_standard_deviation: &mut f64,
    ) -> Errors {
        *up_downside_standard_deviation = f64::NAN;
        let mut excess_return: Vec<f64> = Vec::with_capacity(self.values.len());
        if self
            .values
            .iter()
            .enumerate()
            .try_for_each(|v| {
                if !v.1.is_finite() || !self.benchmark[v.0].is_finite() {
                    return ControlFlow::Break(());
                }
                if cmp_fn(*v.1, self.benchmark[v.0]) {
                    excess_return.push(v.1 - self.benchmark[v.0]);
                }
                ControlFlow::Continue(())
            })
            .is_break()
        {
            return Errors::ClErrorCodeNoError;
        }

        return Self::standard_deviation_internal(
            &excess_return,
            freq,
            is_annu,
            up_downside_standard_deviation,
        );
    }
    ///calculate the downside standard deviation value of an array if the array has NAN/INF values,the result will be NAN.
    ///
    ///# Arguments
    ///freq: the frequence of source data
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
    ///let bmk_data = vec![
    ///    0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
    ///    1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
    ///    3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
    ///    -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
    ///let err = mpt.downside_standard_deviation(
    ///    enums::ClFrequency::ClFrequencyMonthly,
    ///    true,
    ///    &mut res,
    ///);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 2.00029),
    ///    true
    ///);
    ///```
    pub fn downside_standard_deviation(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        downside_standard_deviation: &mut f64,
    ) -> Errors {
        return self.up_downside_standard_deviation(
            freq,
            is_annu,
            |a, b| a < b,
            downside_standard_deviation,
        );
    }
    ///calculate the upside standard deviation value of an array if the array has NAN/INF values,the result will be NAN.
    ///
    ///# Arguments
    ///freq: the frequence of source data
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
    ///let bmk_data = vec![
    ///    0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
    ///    1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
    ///    3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
    ///    -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
    ///let err =
    ///mpt.upside_standard_deviation(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
    ///assert_eq!(
    ///err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 2.70293),
    ///true
    ///);
    ///```
    pub fn upside_standard_deviation(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        upside_standard_deviation: &mut f64,
    ) -> Errors {
        return self.up_downside_standard_deviation(
            freq,
            is_annu,
            |a, b| a > b,
            upside_standard_deviation,
        );
    }

    fn get_s(&self, s_result: &mut f64) -> Errors {
        let mut alpha_result = f64::NAN;
        let mut ret = self.alpha(ClFrequency::ClFrequencyMonthly, false, &mut alpha_result);

        if ret != Errors::ClErrorCodeNoError {
            *s_result = f64::NAN;
            return ret;
        }
        let mut beta_result = f64::NAN;
        ret = self.beta(&mut beta_result);
        if ret != Errors::ClErrorCodeNoError {
            *s_result = f64::NAN;
            return ret;
        }

        if self.values.iter().find(|x| !x.is_finite()) != None
            || self.benchmark.iter().find(|x| !x.is_finite()) != None
        {
            return Errors::ClErrorCodeNoError;
        }
        *s_result = self.values.iter().enumerate().fold(0.0, |acc, x| {
            acc + (x.1 - alpha_result - self.benchmark[x.0] * beta_result).powf(2.0)
        });
        (*s_result) = ((*s_result) / (self.values.len() - 2) as f64).sqrt();

        return Errors::ClErrorCodeNoError;
    }

    fn standard_error_alpha_beta(
        &self,
        standard_error_alpha_result: &mut f64,
        standard_error_beta_result: &mut f64,
    ) -> Errors {
        let mut avg_result = f64::NAN;
        let mut s_result = f64::NAN;
        let mut ret = MPTCalculator::from_v(self.benchmark).average(&mut avg_result);
        if ret != Errors::ClErrorCodeNoError {
            return ret;
        }

        ret = self.get_s(&mut s_result);
        if ret != Errors::ClErrorCodeNoError {
            return ret;
        }
        struct StandardErrorData {
            sum: f64,
            excess: f64,
        }

        let accumalte = self.benchmark.iter().filter(|x| x.is_finite()).fold(
            StandardErrorData {
                sum: 0.0,
                excess: 0.0,
            },
            |acc, x| StandardErrorData {
                sum: acc.sum + x * x,
                excess: acc.excess + (x - avg_result) * (x - avg_result),
            },
        );
        if accumalte.sum == 0.0 {
            *standard_error_alpha_result = f64::NAN;
        } else {
            *standard_error_alpha_result =
                s_result * (accumalte.sum / (accumalte.excess * self.values.len() as f64)).sqrt();
        }

        if accumalte.excess == 0.0 {
            *standard_error_beta_result = f64::NAN;
        } else {
            *standard_error_beta_result = s_result / accumalte.excess.sqrt();
        }

        return Errors::ClErrorCodeNoError;
    }
    ///calculate the standard error alpha value of an array if the array has NAN/INF values,the result will be NAN.
    ///
    ///# Arguments
    ///freq: the frequence of source data
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
    ///let bmk_data = vec![
    ///    0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
    ///    1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
    ///    3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
    ///    -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
    ///let err = mpt.standard_error_alpha(&mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 0.21689),
    ///    true
    ///);
    ///```
    pub fn standard_error_alpha(&self, standard_error_alpha_result: &mut f64) -> Errors {
        let mut standard_error_beta_result = 0.0;
        return self.standard_error_alpha_beta(
            standard_error_alpha_result,
            &mut standard_error_beta_result,
        );
    }
    ///calculate the standard error beta value of an array if the array has NAN/INF values,the result will be NAN.
    ///
    ///# Arguments
    ///freq: the frequence of source data
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
    ///let bmk_data = vec![
    ///    0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
    ///    1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
    ///    3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
    ///    -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
    ///let err =
    ///mpt.standard_error_beta(&mut res);
    ///assert_eq!(
    ///err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 0.04720),
    ///true
    ///);
    ///```
    pub fn standard_error_beta(&self, standard_error_beta_result: &mut f64) -> Errors {
        let mut standard_error_alpha_result = 0.0;
        return self.standard_error_alpha_beta(
            &mut standard_error_alpha_result,
            standard_error_beta_result,
        );
    }

    fn treynor_ratio_calc(&self, treynor_ratio_data: &mut TreynorRatioData) -> Errors {
        if self
            .values
            .iter()
            .enumerate()
            .try_for_each(|v| {
                if !v.1.is_finite() || !self.benchmark[v.0].is_finite() {
                    return ControlFlow::Break(());
                }
                (*treynor_ratio_data).total_return *= 1.0 + v.1 / 100.0;
                (*treynor_ratio_data).rf_total_return *= 1.0 + self.riskfree[v.0] / 100.0;
                (*treynor_ratio_data).sum += v.1 - self.riskfree[v.0];
                (*treynor_ratio_data).count += 1;
                ControlFlow::Continue(())
            })
            .is_break()
        {
            return Errors::ClErrorCodeNoError;
        }

        if treynor_ratio_data.count == 0 {
            return Errors::ClErrorCodeNoError;
        }

        let mut excess_return = vec![f64::NAN; self.values.len()];
        let mut bmk_excess_return = vec![f64::NAN; self.values.len()];

        let mut ret =
            Self::array_subtraction_internal(self.values, self.riskfree, &mut excess_return);

        if ret != Errors::ClErrorCodeNoError {
            return ret;
        }
        ret =
            Self::array_subtraction_internal(self.benchmark, self.riskfree, &mut bmk_excess_return);

        if ret != Errors::ClErrorCodeNoError {
            return ret;
        }

        ret = MPTCalculator::from_v_b(&excess_return, &bmk_excess_return)
            .beta(&mut treynor_ratio_data.excess_beta);

        if ret != Errors::ClErrorCodeNoError {
            return ret;
        }

        return Errors::ClErrorCodeNoError;
    }

    ///calculate the treynor ratio arithmetic value of an array if the array has NAN/INF values,the result will be NAN.
    ///
    ///# Arguments
    ///freq: the frequence of source data
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
    ///let bmk_data = vec![
    ///    0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
    ///    1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
    ///    3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
    ///    -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
    ///];
    ///let rf_data = vec![
    ///    0.38497, 0.39406, 0.40057, 0.41237, 0.41911, 0.43358, 0.43548, 0.42107, 0.42743,
    ///    0.43278, 0.4235, 0.43403, 0.4394, 0.43558, 0.42739, 0.41784, 0.40578, 0.42384, 0.41252,
    ///    0.35001, 0.34617, 0.30686, 0.26785, 0.2483, 0.19164, 0.1187, 0.11352, 0.14765, 0.16356,
    ///    0.1443, 0.15408, 0.11971, 0.06686, 0.0254, 0.00313, 0.00321,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from(&data, &bmk_data, &rf_data);
    ///let err =
    ///    mpt.treynor_ratio_arithmetic(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, -14.9971),
    ///    true
    ///);
    ///```
    pub fn treynor_ratio_arithmetic(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        treynor_ratio_arithmetic_result: &mut f64,
    ) -> Errors {
        *treynor_ratio_arithmetic_result = f64::NAN;
        let mut treynor_ratio_data = TreynorRatioData {
            total_return: 1.0,
            rf_total_return: 1.0,
            sum: 0.0,
            excess_beta: f64::NAN,
            count: 0,
        };
        self.treynor_ratio_calc(&mut treynor_ratio_data);
        if treynor_ratio_data.excess_beta.is_nan() || treynor_ratio_data.excess_beta == 0.0 {
            return Errors::ClErrorCodeNoError;
        }

        if is_annu {
            let mutiplier = get_annual_multiplier(freq, false);
            let ann_return = 100.0
                * (treynor_ratio_data
                    .total_return
                    .powf(mutiplier / self.values.len() as f64)
                    - 1.0);
            let ann_rf_return = 100.0
                * (treynor_ratio_data
                    .rf_total_return
                    .powf(mutiplier / self.values.len() as f64)
                    - 1.0);

            *treynor_ratio_arithmetic_result =
                (ann_return - ann_rf_return) / treynor_ratio_data.excess_beta;
        } else {
            *treynor_ratio_arithmetic_result = (treynor_ratio_data.sum / self.values.len() as f64)
                / treynor_ratio_data.excess_beta;
        }

        return Errors::ClErrorCodeNoError;
    }

    ///calculate the treynor ratio geometric value of an array if the array has NAN/INF values,the result will be NAN.
    ///
    ///# Arguments
    ///freq: the frequence of source data
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
    ///let bmk_data = vec![
    ///    0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
    ///    1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
    ///    3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
    ///    -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
    ///];
    ///let rf_data = vec![
    ///    0.38497, 0.39406, 0.40057, 0.41237, 0.41911, 0.43358, 0.43548, 0.42107, 0.42743,
    ///    0.43278, 0.4235, 0.43403, 0.4394, 0.43558, 0.42739, 0.41784, 0.40578, 0.42384, 0.41252,
    ///    0.35001, 0.34617, 0.30686, 0.26785, 0.2483, 0.19164, 0.1187, 0.11352, 0.14765, 0.16356,
    ///    0.1443, 0.15408, 0.11971, 0.06686, 0.0254, 0.00313, 0.00321,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from(&data, &bmk_data, &rf_data);
    ///let err =
    ///    mpt.treynor_ratio_geometric(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, -14.47007),
    ///    true
    ///);
    ///```
    pub fn treynor_ratio_geometric(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        treynor_ratio_geometric_result: &mut f64,
    ) -> Errors {
        *treynor_ratio_geometric_result = 0.0;
        let mut treynor_ratio_data = TreynorRatioData {
            total_return: 1.0,
            rf_total_return: 1.0,
            sum: 0.0,
            excess_beta: f64::NAN,
            count: 0,
        };
        self.treynor_ratio_calc(&mut treynor_ratio_data);

        if treynor_ratio_data.excess_beta == 0.0 {
            return Errors::ClErrorCodeNoError;
        }

        if is_annu {
            let mutiplier = get_annual_multiplier(freq, false);
            let ann_return = 100.0
                * (treynor_ratio_data
                    .total_return
                    .powf(mutiplier / self.values.len() as f64)
                    - 1.0);
            let ann_rf_return = 100.0
                * (treynor_ratio_data
                    .rf_total_return
                    .powf(mutiplier / self.values.len() as f64)
                    - 1.0);

            *treynor_ratio_geometric_result =
                ((100.0 + ann_return) / (100.0 + ann_rf_return) - 1.0) * 100.0
                    / treynor_ratio_data.excess_beta;
        } else {
            *treynor_ratio_geometric_result =
                (treynor_ratio_data.total_return / treynor_ratio_data.rf_total_return - 1.0)
                    * 100.0
                    / treynor_ratio_data.excess_beta;
        }

        return Errors::ClErrorCodeNoError;
    }

    pub fn up_down_side_capture(
        &self,
        cmp_fn: fn(f64, f64) -> bool,
        upside_capture_ratio: &mut f64,
        upside_capture_return: &mut f64,
    ) -> Errors {
        let mut capture_data = CaptureData {
            count: 0,
            accu_y: 1.0,
            accu_x: 1.0,
        };
        if self
            .values
            .iter()
            .enumerate()
            .try_for_each(|v| {
                if !v.1.is_finite() || !self.benchmark[v.0].is_finite() {
                    return ControlFlow::Break(());
                }
                if cmp_fn(self.benchmark[v.0], 0.0) {
                    capture_data.accu_y *= 1.0 + v.1 / 100.0;
                    capture_data.accu_x *= 1.0 + self.benchmark[v.0] / 100.0;
                    capture_data.count += 1;
                }
                ControlFlow::Continue(())
            })
            .is_break()
        {
            return Errors::ClErrorCodeNoError;
        }

        if capture_data.count > 0 {
            *upside_capture_return =
                (capture_data.accu_y.powf(1.0 / capture_data.count as f64) - 1.0) * 100.0;

            *upside_capture_ratio = (capture_data.accu_y.powf(1.0 / capture_data.count as f64)
                - 1.0)
                / (capture_data.accu_x.powf(1.0 / capture_data.count as f64) - 1.0)
                * 100.0;
        }

        return Errors::ClErrorCodeNoError;
    }
    ///calculate the upside capture ration value of an array if the array has NAN/INF values,the result will be NAN.
    ///
    ///# Arguments
    ///freq: the frequence of source data
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
    ///let bmk_data = vec![
    ///    -0.34902, 4.72157, -0.07781, -5.69315, 0.50715, -3.32714, 2.85004, 0.70329, 5.68503,
    ///    2.51328, 0.19667, 1.60986, -0.88035, 0.93436, 1.73095, 3.99893, -1.58709, -6.90567,
    ///    2.15374, 1.58778, 2.80202, -7.2765, -0.22549, -6.8847, -3.80168, 0.26042, 4.1003,
    ///    4.48299, -7.8344, 3.60549, 3.49499, -8.10192, -20.90433, -11.9778, 5.56181, -11.19773,
    ///];
    ///let mut upside_capture_ratio = f64::NAN;
    ///let mut upside_capture_return = f64::NAN;
    ///let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
    ///let err = mpt.upside_capture(&mut upside_capture_ratio, &mut upside_capture_return);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(upside_capture_ratio, 75.25659),
    ///    true
    ///);
    ///
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(upside_capture_return, 2.00265),
    ///    true
    ///);
    ///```
    pub fn upside_capture(
        &self,
        upside_capture_ratio: &mut f64,
        upside_capture_return: &mut f64,
    ) -> Errors {
        return self.up_down_side_capture(
            |a, b| a >= b,
            upside_capture_ratio,
            upside_capture_return,
        );
    }
    ///calculate the downside capture ration value of an array if the array has NAN/INF values,the result will be NAN.
    ///
    ///# Arguments
    ///freq: the frequence of source data
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
    ///let bmk_data = vec![
    ///    -0.34902, 4.72157, -0.07781, -5.69315, 0.50715, -3.32714, 2.85004, 0.70329, 5.68503,
    ///    2.51328, 0.19667, 1.60986, -0.88035, 0.93436, 1.73095, 3.99893, -1.58709, -6.90567,
    ///    2.15374, 1.58778, 2.80202, -7.2765, -0.22549, -6.8847, -3.80168, 0.26042, 4.1003,
    ///    4.48299, -7.8344, 3.60549, 3.49499, -8.10192, -20.90433, -11.9778, 5.56181, -11.19773,
    ///];
    ///let mut down_capture_ratio = f64::NAN;
    ///let mut down_capture_return = f64::NAN;
    ///let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
    ///let err = mpt.downside_capture(&mut down_capture_ratio, &mut down_capture_return);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(down_capture_ratio, 73.03436),
    ///    true
    ///);
    ///
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(down_capture_return, -4.54466),
    ///    true
    ///);
    ///```
    pub fn downside_capture(
        &self,
        down_capture_ratio: &mut f64,
        down_capture_return: &mut f64,
    ) -> Errors {
        return self.up_down_side_capture(|a, b| a < b, down_capture_ratio, down_capture_return);
    }
    ///calculate the bear bull beta value of an array if the array has NAN/INF values,the result will be NAN.
    ///
    ///# Arguments
    ///freq: the frequence of source data
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
    ///let bmk_data = vec![
    ///    0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
    ///    1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
    ///    3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
    ///    -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
    ///];
    ///let mut bear_beta = f64::NAN;
    ///let mut bull_beta = f64::NAN;
    ///let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
    ///let err = mpt.bear_bull_beta(&mut bear_beta, &mut bull_beta);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError
    ///        && MPTCalculator::is_eq_double(bear_beta, 0.97732)
    ///        && MPTCalculator::is_eq_double(bull_beta, 1.07004),
    ///    true
    ///);
    ///```
    pub fn bear_bull_beta(&self, bear_beta: &mut f64, bull_beta: &mut f64) -> Errors {
        let xy_data = gather_bear_bull_xy(self.values, self.benchmark, self.values.len());

        *bear_beta = f64::NAN;
        let bear_divisor = xy_data.bear_count as f64 * xy_data.bear_xx_sum
            - xy_data.bear_x_sum * xy_data.bear_x_sum;
        if bear_divisor != 0.0 {
            *bear_beta = (xy_data.bear_count as f64 * xy_data.bear_xy_sum
                - xy_data.bear_x_sum * xy_data.bear_y_sum)
                / bear_divisor
        }

        *bull_beta = f64::NAN;
        let bull_divisor = xy_data.bull_count as f64 * xy_data.bull_xx_sum
            - xy_data.bull_x_sum * xy_data.bull_x_sum;
        if bull_divisor != 0.0 {
            *bull_beta = (xy_data.bull_count as f64 * xy_data.bull_xy_sum
                - xy_data.bull_x_sum * xy_data.bull_y_sum)
                / bull_divisor
        }

        return Errors::ClErrorCodeNoError;
    }
    ///calculate the bear bull colleation value of an array if the array has NAN/INF values,the result will be NAN.
    ///
    ///# Arguments
    ///freq: the frequence of source data
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
    ///let bmk_data = vec![
    ///    0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
    ///    1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
    ///    3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
    ///    -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
    ///];
    ///
    ///let mut bear_colleantion_res = f64::NAN;
    ///let mut bull_colleantion_res = f64::NAN;
    ///let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
    ///let err = mpt.bear_bull_colleation(&mut bear_colleantion_res, &mut bull_colleantion_res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError
    ///        && MPTCalculator::is_eq_double(bear_colleantion_res, 0.96025)
    ///        && MPTCalculator::is_eq_double(bull_colleantion_res, 0.73785),
    ///    true
    ///);
    ///```
    pub fn bear_bull_colleation(
        &self,
        bear_colleantion_res: &mut f64,
        bull_colleantion_res: &mut f64,
    ) -> Errors {
        let xy_data = gather_bear_bull_xy(self.values, self.benchmark, self.values.len());

        let bear_mean_x = xy_data.bear_x_sum / xy_data.bear_count as f64;
        let bear_mean_y = xy_data.bear_y_sum / xy_data.bear_count as f64;
        let bull_mean_x = xy_data.bull_x_sum / xy_data.bull_count as f64;
        let bull_mean_y = xy_data.bull_y_sum / xy_data.bull_count as f64;

        let mut bear_sum_mean_xy = 0.0;
        let mut bear_sum_mean_xx = 0.0;
        let mut bear_sum_mean_yy = 0.0;
        let mut bull_sum_mean_xy = 0.0;
        let mut bull_sum_mean_xx = 0.0;
        let mut bull_sum_mean_yy = 0.0;

        for i in 0..self.values.len() {
            if self.benchmark[i] < 0.0 {
                bear_sum_mean_xy +=
                    (self.benchmark[i] - bear_mean_x) * (self.values[i] - bear_mean_y);
                bear_sum_mean_xx +=
                    (self.benchmark[i] - bear_mean_x) * (self.benchmark[i] - bear_mean_x);
                bear_sum_mean_yy += (self.values[i] - bear_mean_y) * (self.values[i] - bear_mean_y)
            } else if self.benchmark[i] > 0.0 {
                bull_sum_mean_xy +=
                    (self.benchmark[i] - bull_mean_x) * (self.values[i] - bull_mean_y);
                bull_sum_mean_xx +=
                    (self.benchmark[i] - bull_mean_x) * (self.benchmark[i] - bull_mean_x);
                bull_sum_mean_yy += (self.values[i] - bull_mean_y) * (self.values[i] - bull_mean_y)
            }
        }

        *bear_colleantion_res = f64::NAN;
        let bear_divisor = bear_sum_mean_xx * bear_sum_mean_yy;
        if bear_divisor > 0.0 {
            *bear_colleantion_res = bear_sum_mean_xy / bear_divisor.sqrt();
        }

        *bull_colleantion_res = f64::NAN;
        let bull_divisor = bull_sum_mean_xx * bull_sum_mean_yy;
        if bull_divisor > 0.0 {
            *bull_colleantion_res = bull_sum_mean_xy / bull_divisor.sqrt();
        }

        return Errors::ClErrorCodeNoError;
    }

    ///calculate the r_squared value of an array if the array has NAN/INF values,the result will be NAN.
    ///
    ///# Arguments
    ///freq: the frequence of source data
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
    ///let bmk_data = vec![
    ///    0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
    ///    1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
    ///    3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
    ///    -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
    ///];
    ///
    ///let mut result = f64::NAN;
    ///let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
    ///let err = mpt.r_squared(&mut result);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError
    ///        && MPTCalculator::is_eq_double(result,92.59959),
    ///    true
    ///);
    ///```
    pub fn r_squared(&self, r_squard_result: &mut f64) -> Errors {
        let xy_data = gather_xy(self.values, self.benchmark, self.values.len());

        let cov_xy = xy_data.xy_sum - xy_data.x_sum * xy_data.y_sum / xy_data.count as f64;
        let y_std = xy_data.yy_sum - xy_data.y_sum * xy_data.y_sum / xy_data.count as f64;
        let x_std = xy_data.xx_sum - xy_data.x_sum * xy_data.x_sum / xy_data.count as f64;
        if x_std != 0.0 && y_std != 0.0 {
            *r_squard_result = cov_xy * cov_xy / (y_std * x_std) * 100.0;
        } else {
            *r_squard_result = f64::NAN;
        }
        return Errors::ClErrorCodeNoError;
    }

    ///calculate the batting average value of an array if the array has NAN/INF values,the result will be NAN.
    ///
    ///# Arguments
    ///freq: the frequence of source data
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
    ///let bmk_data = vec![
    ///    0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
    ///    1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
    ///    3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
    ///    -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
    ///];
    ///
    ///let mut result = f64::NAN;
    ///let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
    ///let err = mpt.batting_average(&mut result);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError
    ///        && MPTCalculator::is_eq_double(result,52.77778),
    ///    true
    ///);
    ///```
    pub fn batting_average(&self, batting: &mut f64) -> Errors {
        let mut sum = 0.0;
        let mut valid_count = 0;

        if self
            .values
            .iter()
            .enumerate()
            .try_for_each(|v| {
                if !v.1.is_finite() || !self.benchmark[v.0].is_finite() {
                    return ControlFlow::Break(());
                }
                valid_count += 1;
                if *v.1 > self.benchmark[v.0] {
                    sum += 1.0;
                }
                ControlFlow::Continue(())
            })
            .is_break()
        {
            return Errors::ClErrorCodeNoError;
        }

        if valid_count > 0 {
            *batting = (sum / valid_count as f64) * 100.0;
        }
        return Errors::ClErrorCodeNoError;
    }
    ///calculate the correlation value of an array if the array has NAN/INF values,the result will be NAN.
    ///
    ///# Arguments
    ///freq: the frequence of source data
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
    ///let bmk_data = vec![
    ///    0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
    ///    1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
    ///    3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
    ///    -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
    ///];
    ///
    ///let mut result = f64::NAN;
    ///let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
    ///let err = mpt.correlation(&mut result);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError
    ///        && MPTCalculator::is_eq_double(result,0.96229),
    ///    true
    ///);
    ///```
    pub fn correlation(&self, correlation_result: &mut f64) -> Errors {
        let xy_data = gather_xy(self.values, self.benchmark, self.values.len());

        if xy_data.count > 0 {
            let cov_xy = xy_data.xy_sum - xy_data.x_sum * xy_data.y_sum / xy_data.count as f64;
            let std_y = xy_data.yy_sum - xy_data.y_sum * xy_data.y_sum / xy_data.count as f64;
            let std_x = xy_data.xx_sum - xy_data.x_sum * xy_data.x_sum / xy_data.count as f64;

            if std_y != 0.0 && std_x != 0.0 {
                if cov_xy > 0.0 {
                    *correlation_result = ((cov_xy * cov_xy) / (std_x * std_y)).sqrt()
                } else {
                    *correlation_result = -((cov_xy * cov_xy) / (std_x * std_y)).sqrt()
                }
            }
        }
        return Errors::ClErrorCodeNoError;
    }
    ///calculate the appraisal ratio value of an array if the array has NAN/INF values,the result will be NAN.
    ///
    ///# Arguments
    ///freq: the frequence of source data
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
    ///let bmk_data = vec![
    ///    0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
    ///    1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
    ///    3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
    ///    -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
    ///];
    ///
    ///let mut result = f64::NAN;
    ///let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
    ///let err = mpt.appraisal_ratio(&mut result);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError
    ///        && MPTCalculator::is_eq_double(result,0.04337),
    ///    true
    ///);
    ///```
    pub fn appraisal_ratio(&self, appraisal_ratio_result: &mut f64) -> Errors {
        let mut alpha_result = f64::NAN;
        let mut s_result = f64::NAN;

        let mut ret = self.alpha(ClFrequency::ClFrequencyMonthly, false, &mut alpha_result);

        if ret != Errors::ClErrorCodeNoError {
            return ret;
        }

        ret = self.get_s(&mut s_result);
        if ret != Errors::ClErrorCodeNoError {
            return ret;
        }

        if alpha_result.is_finite() && s_result.is_finite() {
            *appraisal_ratio_result = alpha_result / s_result;
        } else {
            *appraisal_ratio_result = f64::NAN
        }

        return Errors::ClErrorCodeNoError;
    }
    ///calculate the relative risk value of an array if the array has NAN/INF values,the result will be NAN.
    ///
    ///# Arguments
    ///freq: the frequence of source data
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
    ///let bmk_data = vec![
    ///    0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
    ///    1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
    ///    3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
    ///    -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
    ///];
    ///
    ///let mut result = f64::NAN;
    ///let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
    ///let err = mpt.relative_risk(&mut result);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError
    ///        && MPTCalculator::is_eq_double(result,1.01180),
    ///    true
    ///);
    ///```
    pub fn relative_risk(&self, relative_risk_res: &mut f64) -> Errors {
        *relative_risk_res = f64::NAN;

        let mut stddev = f64::NAN;
        let mut bmk_stddev = f64::NAN;
        let mut ret = Self::standard_deviation_internal(
            self.values,
            ClFrequency::ClFrequencyMonthly,
            false,
            &mut stddev,
        );

        if ret != Errors::ClErrorCodeNoError {
            return ret;
        }
        ret = Self::standard_deviation_internal(
            self.benchmark,
            ClFrequency::ClFrequencyMonthly,
            false,
            &mut bmk_stddev,
        );

        if ret != Errors::ClErrorCodeNoError {
            return ret;
        }

        if !stddev.is_nan() && !bmk_stddev.is_nan() && bmk_stddev != 0.0 {
            *relative_risk_res = stddev / bmk_stddev;
        }

        return Errors::ClErrorCodeNoError;
    }

    ///calculate the up number value of an array if the array has NAN/INF values,the result will be NAN.
    ///
    ///# Arguments
    ///freq: the frequence of source data
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
    ///let bmk_data = vec![
    ///    -0.34902, 4.72157, -0.07781, -5.69315, 0.50715, -3.32714, 2.85004, 0.70329, 5.68503,
    ///    2.51328, 0.19667, 1.60986, -0.88035, 0.93436, 1.73095, 3.99893, -1.58709, -6.90567,
    ///    2.15374, 1.58778, 2.80202, -7.2765, -0.22549, -6.8847, -3.80168, 0.26042, 4.1003,
    ///    4.48299, -7.8344, 3.60549, 3.49499, -8.10192, -20.90433, -11.9778, 5.56181, -11.19773,
    ///];
    ///let mut result = f64::NAN;
    ///let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
    ///let err = mpt.up_number_ratio(&mut result);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(result, 0.80000),
    ///    true
    ///);
    ///```
    pub fn up_number_ratio(&self, up_number_ratio_result: &mut f64) -> Errors {
        let mut ratio_data = RatioData { count: 0, ratio: 0 };

        if self
            .values
            .iter()
            .enumerate()
            .try_for_each(|v| {
                if !v.1.is_finite() || !self.benchmark[v.0].is_finite() {
                    return ControlFlow::Break(());
                }
                if self.benchmark[v.0] > 0.0 {
                    if *v.1 > 0.0 {
                        ratio_data.ratio += 1;
                    }

                    ratio_data.count += 1;
                }
                ControlFlow::Continue(())
            })
            .is_break()
        {
            return Errors::ClErrorCodeNoError;
        }

        *up_number_ratio_result = f64::NAN;
        if ratio_data.count > 0 {
            *up_number_ratio_result = ratio_data.ratio as f64 / ratio_data.count as f64
        }

        return Errors::ClErrorCodeNoError;
    }
    ///calculate the down number value of an array if the array has NAN/INF values,the result will be NAN.
    ///
    ///# Arguments
    ///freq: the frequence of source data
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
    ///let bmk_data = vec![
    ///    -0.34902, 4.72157, -0.07781, -5.69315, 0.50715, -3.32714, 2.85004, 0.70329, 5.68503,
    ///    2.51328, 0.19667, 1.60986, -0.88035, 0.93436, 1.73095, 3.99893, -1.58709, -6.90567,
    ///    2.15374, 1.58778, 2.80202, -7.2765, -0.22549, -6.8847, -3.80168, 0.26042, 4.1003,
    ///    4.48299, -7.8344, 3.60549, 3.49499, -8.10192, -20.90433, -11.9778, 5.56181, -11.19773,
    ///];
    ///let mut result = f64::NAN;
    ///let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
    ///let err = mpt.down_number_ratio(&mut result);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(result, 0.93750),
    ///    true
    ///);
    ///```
    pub fn down_number_ratio(&self, down_number_ratio_result: &mut f64) -> Errors {
        let mut ratio_data = RatioData { count: 0, ratio: 0 };

        if self
            .values
            .iter()
            .enumerate()
            .try_for_each(|v| {
                if !v.1.is_finite() || !self.benchmark[v.0].is_finite() {
                    return ControlFlow::Break(());
                }
                if self.benchmark[v.0] < 0.0 {
                    if *v.1 < 0.0 {
                        ratio_data.ratio += 1;
                    }

                    ratio_data.count += 1;
                }
                ControlFlow::Continue(())
            })
            .is_break()
        {
            return Errors::ClErrorCodeNoError;
        }

        *down_number_ratio_result = f64::NAN;
        if ratio_data.count > 0 {
            *down_number_ratio_result = ratio_data.ratio as f64 / ratio_data.count as f64
        }

        return Errors::ClErrorCodeNoError;
    }
    pub fn up_down_percent(
        &self,
        cmp_fn: fn(f64, f64) -> bool,
        up_percent_result: &mut f64,
    ) -> Errors {
        let mut ratio_data = RatioData { count: 0, ratio: 0 };
        if self
            .values
            .iter()
            .enumerate()
            .try_for_each(|v| {
                if !v.1.is_finite() || !self.benchmark[v.0].is_finite() {
                    return ControlFlow::Break(());
                }
                if cmp_fn(self.benchmark[v.0], 0.0) {
                    if *v.1 > self.benchmark[v.0] {
                        ratio_data.ratio += 1;
                    }

                    ratio_data.count += 1;
                }
                ControlFlow::Continue(())
            })
            .is_break()
        {
            return Errors::ClErrorCodeNoError;
        }

        *up_percent_result = f64::NAN;
        if ratio_data.count > 0 {
            *up_percent_result = ratio_data.ratio as f64 / ratio_data.count as f64
        }

        return Errors::ClErrorCodeNoError;
    }

    ///calculate the up percent value of an array if the array has NAN/INF values,the result will be NAN.
    ///
    ///# Arguments
    ///freq: the frequence of source data
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
    ///let bmk_data = vec![
    ///    -0.34902, 4.72157, -0.07781, -5.69315, 0.50715, -3.32714, 2.85004, 0.70329, 5.68503,
    ///    2.51328, 0.19667, 1.60986, -0.88035, 0.93436, 1.73095, 3.99893, -1.58709, -6.90567,
    ///    2.15374, 1.58778, 2.80202, -7.2765, -0.22549, -6.8847, -3.80168, 0.26042, 4.1003,
    ///    4.48299, -7.8344, 3.60549, 3.49499, -8.10192, -20.90433, -11.9778, 5.56181, -11.19773,
    ///];
    ///let mut result = f64::NAN;
    ///let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
    ///let err = mpt.up_percent(&mut result);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(result, 0.40000),
    ///    true
    ///);
    ///```
    pub fn up_percent(&self, up_percent_result: &mut f64) -> Errors {
        return self.up_down_percent(|a, b| a > b, up_percent_result);
    }

    ///calculate the down percent value of an array if the array has NAN/INF values,the result will be NAN.
    ///
    ///# Arguments
    ///freq: the frequence of source data
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
    ///let bmk_data = vec![
    ///    -0.34902, 4.72157, -0.07781, -5.69315, 0.50715, -3.32714, 2.85004, 0.70329, 5.68503,
    ///    2.51328, 0.19667, 1.60986, -0.88035, 0.93436, 1.73095, 3.99893, -1.58709, -6.90567,
    ///    2.15374, 1.58778, 2.80202, -7.2765, -0.22549, -6.8847, -3.80168, 0.26042, 4.1003,
    ///    4.48299, -7.8344, 3.60549, 3.49499, -8.10192, -20.90433, -11.9778, 5.56181, -11.19773,
    ///];
    ///let mut result = f64::NAN;
    ///let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
    ///let err = mpt.down_percent(&mut result);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(result, 0.75000),
    ///    true
    ///);
    ///```
    pub fn down_percent(&self, up_percent_result: &mut f64) -> Errors {
        return self.up_down_percent(|a, b| a < b, up_percent_result);
    }

    ///calculate the m_squared value of an array if the array has NAN/INF values,the result will be NAN.
    ///
    ///# Arguments
    ///freq: the frequence of source data
    ///
    ///is_annu: the flag of annualize.
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;

    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///    2.4037, 1.1276, 2.9127, 2.5981, -0.5162, 2.8709, -1.6506, 0.8281, 4.8182, 4.0484,
    ///    -0.4246, -1.8230, 1.1619, 6.2151, 5.3158, -3.7904, 0.3500, -8.9486, -1.6029, -2.1879,
    ///    6.5159, 3.0498, -8.3762, -3.9341, -0.0780, -17.9807, -21.5895, -11.3292, 4.8884,
    ///    -7.5447, -7.5943, 13.9102, 13.6679, 6.2313, -1.3755, 8.7637,
    ///];
    ///let bmk_data = vec![
    ///    2.3793, 2.5770, 3.2586, 1.9016, 1.4028, 1.5123, -1.9559, 1.1185, 4.4295, 3.4895,
    ///    -1.6613, -3.1005, 1.4990, 3.7399, 1.5907, -4.1807, -0.6938, -5.9982, -3.2486, -0.4318,
    ///    4.8703, 1.2953, -8.4304, -0.8406, 1.4465, -8.9107, -16.7948, -7.1755, 1.0640, -8.4286,
    ///    -10.6478, 8.7595, 9.5709, 5.5933, 0.1984, 7.5637,
    ///];
    ///let rf_data = vec![
    ///    0.4355, 0.4211, 0.4274, 0.4328, 0.4235, 0.4340, 0.4394, 0.4356, 0.4274, 0.4178, 0.4058,
    ///    0.4238, 0.4125, 0.3500, 0.3462, 0.3069, 0.2679, 0.2483, 0.1916, 0.1187, 0.1135, 0.1477,
    ///    0.1636, 0.1443, 0.1541, 0.1197, 0.0669, 0.0254, 0.0031, 0.0105, 0.0265, 0.0213, 0.0145,
    ///    0.0160, 0.0149, 0.0162,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from(&data, &bmk_data, &rf_data);
    ///let err = mpt.m_squared(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, -1.60415),
    ///    true
    ///);
    ///```
    pub fn m_squared(
        &self,
        freq: enums::ClFrequency,
        is_annu: bool,
        m_squared_res: &mut f64,
    ) -> Errors {
        *m_squared_res = f64::NAN;
        let mut rf_mean = f64::NAN;
        MPTCalculator::from_v(self.riskfree).mean_arithmetic(&mut rf_mean);
        if !rf_mean.is_finite() {
            return Errors::ClErrorCodeNoError;
        }
        let mut bmk_excess_return = vec![f64::NAN; self.values.len()];
        let ret =
            Self::array_subtraction_internal(self.benchmark, self.riskfree, &mut bmk_excess_return);

        if ret != Errors::ClErrorCodeNoError {
            return ret;
        }

        let mut bmk_stddev = f64::NAN;
        Self::standard_deviation_internal(
            &bmk_excess_return,
            ClFrequency::ClFrequencyMonthly,
            is_annu,
            &mut bmk_stddev,
        );

        if !bmk_stddev.is_finite() {
            return Errors::ClErrorCodeNoError;
        }

        let mut port_sharpe_ration = f64::NAN;
        self.sharpe_ratio(freq, is_annu, &mut port_sharpe_ration);

        if !port_sharpe_ration.is_finite() {
            return Errors::ClErrorCodeNoError;
        }

        if is_annu {
            *m_squared_res =
                rf_mean * get_annual_multiplier(freq, false) + port_sharpe_ration * bmk_stddev;
        } else {
            *m_squared_res = rf_mean + port_sharpe_ration * bmk_stddev;
        }

        return Errors::ClErrorCodeNoError;
    }
    ///calculate the market risk value of an array if the array has NAN/INF values,the result will be NAN.
    ///
    ///# Arguments
    ///freq: the frequence of source data
    ///
    ///is_annu: the flag of annualize.
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;

    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///    -3.64603, 8.80598, -1.4281, -10.48221, -2.80462, 2.98743, -12.2534, -13.75641,
    ///    -3.99425, 14.01812, -6.33636, -8.40185,
    ///];
    ///let bmk_data = vec![
    ///    -5.444254163,
    ///    7.678667961,
    ///    -0.037850949,
    ///    -9.800858913,
    ///    -3.001103688,
    ///    2.626641337,
    ///    -12.56062862,
    ///    -11.84056062,
    ///    1.146646578,
    ///    15.55676217,
    ///    -7.580194297,
    ///    -8.479793853,
    ///];
    ///let mut result = f64::NAN;
    ///let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
    ///let err = mpt.market_risk(&mut result);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(result, 6.47447029),
    ///    true
    ///);
    ///```
    pub fn market_risk(&self, market_risk_res: &mut f64) -> Errors {
        *market_risk_res = f64::NAN;
        let mut beta_res = f64::NAN;
        let mut mean_res = f64::NAN;

        self.beta(&mut beta_res);
        if !beta_res.is_finite() {
            return Errors::ClErrorCodeNoError;
        }
        MPTCalculator::from_v(self.benchmark).mean_arithmetic(&mut mean_res);
        if !mean_res.is_finite() {
            return Errors::ClErrorCodeNoError;
        }

        *market_risk_res = beta_res * mean_res * beta_res * mean_res;
        return Errors::ClErrorCodeNoError;
    }
    ///calculate the stock risk value of an array if the array has NAN/INF values,the result will be NAN.
    ///
    ///# Arguments
    ///freq: the frequence of source data
    ///
    ///is_annu: the flag of annualize.
    ///
    ///# Examples
    ///```
    ///use mpt_lib::MPTCalculator;

    ///use mpt_lib::enums::{self, Errors};
    ///let data = vec![
    ///    -3.64603, 8.80598, -1.4281, -10.48221, -2.80462, 2.98743, -12.2534, -13.75641,
    ///    -3.99425, 14.01812, -6.33636, -8.40185,
    ///];
    ///let bmk_data = vec![
    ///    -5.444254163,
    ///    7.678667961,
    ///    -0.037850949,
    ///    -9.800858913,
    ///    -3.001103688,
    ///    2.626641337,
    ///    -12.56062862,
    ///    -11.84056062,
    ///    1.146646578,
    ///    15.55676217,
    ///    -7.580194297,
    ///    -8.479793853,
    ///];
    ///let mut result = f64::NAN;
    ///let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
    ///let err = mpt.stock_risk(&mut result);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(result, 3.182966438),
    ///    true
    ///);
    ///```
    pub fn stock_risk(&self, stock_risk_res: &mut f64) -> Errors {
        *stock_risk_res = f64::NAN;
        let mut market_risk_res = 0.0;
        let mut ret = self.market_risk(&mut market_risk_res);

        if ret != Errors::ClErrorCodeNoError {
            return ret;
        }
        let mut mean_res = 0.0;

        ret = self.mean_arithmetic(&mut mean_res);
        if ret != Errors::ClErrorCodeNoError {
            return ret;
        }
        *stock_risk_res = mean_res * mean_res - market_risk_res;
        return Errors::ClErrorCodeNoError;
    }
    ///calculate the stock risk value of an array if the array has NAN/INF values,the result will be NAN.
    ///
    ///# Arguments
    ///freq: the frequence of source data
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
    ///let bmk_data = vec![
    ///    0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
    ///    1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
    ///    3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
    ///    -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
    ///];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
    ///let err = mpt.covariance(&mut res);
    ///assert_eq!(
    ///    err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 20.2720342),
    ///    true
    ///);
    ///```
    pub fn covariance(&self, covariance: &mut f64) -> Errors {
        let xy_data = gather_xy(self.values, self.benchmark, self.values.len());

        if xy_data.count > 0 {
            *covariance = (xy_data.xy_sum - xy_data.x_sum * xy_data.y_sum / xy_data.count as f64)
                / (xy_data.count - 1) as f64;
        } else {
            *covariance = f64::NAN;
        }

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
    fn should_correct_alpha() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let bmk_data = vec![
            0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
            1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
            3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
            -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
        let err = mpt.alpha(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 0.66313),
            true
        );
    }

    #[test]
    fn should_correct_beta() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let bmk_data = vec![
            0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
            1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
            3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
            -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
        let err = mpt.beta(&mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 0.97364),
            true
        );
    }

    #[test]
    fn should_correct_tracking_error() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let bmk_data = vec![
            0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
            1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
            3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
            -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
        let err = mpt.tracking_error(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 4.37063),
            true
        );
    }

    #[test]
    fn should_correct_information_ratio_arithmetic() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let bmk_data = vec![
            0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
            1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
            3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
            -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
        let err = mpt.information_ratio_arithmetic(
            enums::ClFrequency::ClFrequencyMonthly,
            true,
            &mut res,
        );
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 0.19228),
            true
        );
    }

    #[test]
    fn should_correct_information_ratio_geometric() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let bmk_data = vec![
            0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
            1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
            3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
            -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
        let err =
            mpt.information_ratio_geometric(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 0.21796),
            true
        );
    }

    #[test]
    fn should_correct_excess_return_arithmetic() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let bmk_data = vec![
            0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
            1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
            3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
            -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
        let err =
            mpt.excess_return_arithmetic(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 0.84039),
            true
        );
    }

    #[test]
    fn should_correct_excess_return_geometric() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let bmk_data = vec![
            0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
            1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
            3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
            -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
        let err =
            mpt.excess_return_geometric(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 0.95263),
            true
        );
    }

    #[test]
    fn should_correct_excess_return_relative_percentage() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let bmk_data = vec![
            0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
            1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
            3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
            -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
        let err = mpt.excess_return_relative_percentage(
            enums::ClFrequency::ClFrequencyMonthly,
            true,
            &mut res,
        );
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 24.6508),
            true
        );
    }

    #[test]
    fn should_correct_upside_standard_deviation() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let bmk_data = vec![
            0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
            1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
            3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
            -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
        let err =
            mpt.upside_standard_deviation(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 2.70293),
            true
        );
    }

    #[test]
    fn should_correct_downside_standard_deviation() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let bmk_data = vec![
            0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
            1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
            3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
            -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
        let err =
            mpt.downside_standard_deviation(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 2.00029),
            true
        );
    }

    #[test]
    fn should_correct_standard_error_beta() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let bmk_data = vec![
            0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
            1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
            3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
            -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
        let err = mpt.standard_error_beta(&mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 0.04720),
            true
        );
    }

    #[test]
    fn should_correct_standard_error_alpha() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let bmk_data = vec![
            0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
            1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
            3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
            -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
        let err = mpt.standard_error_alpha(&mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 0.21689),
            true
        );
    }

    #[test]
    fn should_correct_treynor_ratio_geometric() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let bmk_data = vec![
            0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
            1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
            3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
            -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
        ];
        let rf_data = vec![
            0.38497, 0.39406, 0.40057, 0.41237, 0.41911, 0.43358, 0.43548, 0.42107, 0.42743,
            0.43278, 0.4235, 0.43403, 0.4394, 0.43558, 0.42739, 0.41784, 0.40578, 0.42384, 0.41252,
            0.35001, 0.34617, 0.30686, 0.26785, 0.2483, 0.19164, 0.1187, 0.11352, 0.14765, 0.16356,
            0.1443, 0.15408, 0.11971, 0.06686, 0.0254, 0.00313, 0.00321,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from(&data, &bmk_data, &rf_data);
        let err =
            mpt.treynor_ratio_geometric(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, -14.47007),
            true
        );
    }

    #[test]
    fn should_correct_treynor_ratio_arithmetic() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let bmk_data = vec![
            0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
            1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
            3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
            -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
        ];
        let rf_data = vec![
            0.38497, 0.39406, 0.40057, 0.41237, 0.41911, 0.43358, 0.43548, 0.42107, 0.42743,
            0.43278, 0.4235, 0.43403, 0.4394, 0.43558, 0.42739, 0.41784, 0.40578, 0.42384, 0.41252,
            0.35001, 0.34617, 0.30686, 0.26785, 0.2483, 0.19164, 0.1187, 0.11352, 0.14765, 0.16356,
            0.1443, 0.15408, 0.11971, 0.06686, 0.0254, 0.00313, 0.00321,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from(&data, &bmk_data, &rf_data);
        let err =
            mpt.treynor_ratio_arithmetic(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, -14.9971),
            true
        );
    }

    #[test]
    fn should_correct_upside_capture() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let bmk_data = vec![
            -0.34902, 4.72157, -0.07781, -5.69315, 0.50715, -3.32714, 2.85004, 0.70329, 5.68503,
            2.51328, 0.19667, 1.60986, -0.88035, 0.93436, 1.73095, 3.99893, -1.58709, -6.90567,
            2.15374, 1.58778, 2.80202, -7.2765, -0.22549, -6.8847, -3.80168, 0.26042, 4.1003,
            4.48299, -7.8344, 3.60549, 3.49499, -8.10192, -20.90433, -11.9778, 5.56181, -11.19773,
        ];
        let mut upside_capture_ratio = f64::NAN;
        let mut upside_capture_return = f64::NAN;
        let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
        let err = mpt.upside_capture(&mut upside_capture_ratio, &mut upside_capture_return);
        assert_eq!(
            err == Errors::ClErrorCodeNoError
                && MPTCalculator::is_eq_double(upside_capture_ratio, 75.25659),
            true
        );

        assert_eq!(
            err == Errors::ClErrorCodeNoError
                && MPTCalculator::is_eq_double(upside_capture_return, 2.00265),
            true
        );
    }

    #[test]
    fn should_correct_downside_capture() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let bmk_data = vec![
            -0.34902, 4.72157, -0.07781, -5.69315, 0.50715, -3.32714, 2.85004, 0.70329, 5.68503,
            2.51328, 0.19667, 1.60986, -0.88035, 0.93436, 1.73095, 3.99893, -1.58709, -6.90567,
            2.15374, 1.58778, 2.80202, -7.2765, -0.22549, -6.8847, -3.80168, 0.26042, 4.1003,
            4.48299, -7.8344, 3.60549, 3.49499, -8.10192, -20.90433, -11.9778, 5.56181, -11.19773,
        ];
        let mut down_capture_ratio = f64::NAN;
        let mut down_capture_return = f64::NAN;
        let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
        let err = mpt.downside_capture(&mut down_capture_ratio, &mut down_capture_return);
        assert_eq!(
            err == Errors::ClErrorCodeNoError
                && MPTCalculator::is_eq_double(down_capture_ratio, 73.03436),
            true
        );

        assert_eq!(
            err == Errors::ClErrorCodeNoError
                && MPTCalculator::is_eq_double(down_capture_return, -4.54466),
            true
        );
    }
    #[test]
    fn should_correct_bear_bull_beta() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let bmk_data = vec![
            0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
            1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
            3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
            -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
        ];
        let mut bear_beta = f64::NAN;
        let mut bull_beta = f64::NAN;
        let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
        let err = mpt.bear_bull_beta(&mut bear_beta, &mut bull_beta);
        assert_eq!(
            err == Errors::ClErrorCodeNoError
                && MPTCalculator::is_eq_double(bear_beta, 0.97732)
                && MPTCalculator::is_eq_double(bull_beta, 1.07004),
            true
        );
    }

    #[test]
    fn should_correct_bear_bull_colleation() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let bmk_data = vec![
            0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
            1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
            3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
            -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
        ];

        let mut bear_colleantion_res = f64::NAN;
        let mut bull_colleantion_res = f64::NAN;
        let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
        let err = mpt.bear_bull_colleation(&mut bear_colleantion_res, &mut bull_colleantion_res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError
                && MPTCalculator::is_eq_double(bear_colleantion_res, 0.96025)
                && MPTCalculator::is_eq_double(bull_colleantion_res, 0.73785),
            true
        );
    }

    #[test]
    fn should_correct_r_squared() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let bmk_data = vec![
            0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
            1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
            3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
            -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
        ];

        let mut result = f64::NAN;
        let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
        let err = mpt.r_squared(&mut result);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(result, 92.59959),
            true
        );
    }

    #[test]
    fn should_correct_batting_average() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let bmk_data = vec![
            0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
            1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
            3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
            -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
        ];

        let mut result = f64::NAN;
        let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
        let err = mpt.batting_average(&mut result);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(result, 52.77778),
            true
        );
    }

    #[test]
    fn should_correct_correlation() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let bmk_data = vec![
            0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
            1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
            3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
            -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
        ];

        let mut result = f64::NAN;
        let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
        let err = mpt.correlation(&mut result);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(result, 0.96229),
            true
        );
    }

    #[test]
    fn should_appraisal_ratio() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let bmk_data = vec![
            0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
            1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
            3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
            -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
        ];

        let mut result = f64::NAN;
        let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
        let err = mpt.appraisal_ratio(&mut result);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(result, 0.04337),
            true
        );
    }

    #[test]
    fn should_correct_relative_risk() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let bmk_data = vec![
            0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
            1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
            3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
            -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
        ];

        let mut result = f64::NAN;
        let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
        let err = mpt.relative_risk(&mut result);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(result, 1.01180),
            true
        );
    }

    #[test]
    fn should_correct_up_number_ratio() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let bmk_data = vec![
            -0.34902, 4.72157, -0.07781, -5.69315, 0.50715, -3.32714, 2.85004, 0.70329, 5.68503,
            2.51328, 0.19667, 1.60986, -0.88035, 0.93436, 1.73095, 3.99893, -1.58709, -6.90567,
            2.15374, 1.58778, 2.80202, -7.2765, -0.22549, -6.8847, -3.80168, 0.26042, 4.1003,
            4.48299, -7.8344, 3.60549, 3.49499, -8.10192, -20.90433, -11.9778, 5.56181, -11.19773,
        ];
        let mut result = f64::NAN;
        let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
        let err = mpt.up_number_ratio(&mut result);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(result, 0.80000),
            true
        );
    }

    #[test]
    fn should_correct_down_number_ratio() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let bmk_data = vec![
            -0.34902, 4.72157, -0.07781, -5.69315, 0.50715, -3.32714, 2.85004, 0.70329, 5.68503,
            2.51328, 0.19667, 1.60986, -0.88035, 0.93436, 1.73095, 3.99893, -1.58709, -6.90567,
            2.15374, 1.58778, 2.80202, -7.2765, -0.22549, -6.8847, -3.80168, 0.26042, 4.1003,
            4.48299, -7.8344, 3.60549, 3.49499, -8.10192, -20.90433, -11.9778, 5.56181, -11.19773,
        ];
        let mut result = f64::NAN;
        let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
        let err = mpt.down_number_ratio(&mut result);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(result, 0.93750),
            true
        );
    }

    #[test]
    fn should_correct_up_percent() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let bmk_data = vec![
            -0.34902, 4.72157, -0.07781, -5.69315, 0.50715, -3.32714, 2.85004, 0.70329, 5.68503,
            2.51328, 0.19667, 1.60986, -0.88035, 0.93436, 1.73095, 3.99893, -1.58709, -6.90567,
            2.15374, 1.58778, 2.80202, -7.2765, -0.22549, -6.8847, -3.80168, 0.26042, 4.1003,
            4.48299, -7.8344, 3.60549, 3.49499, -8.10192, -20.90433, -11.9778, 5.56181, -11.19773,
        ];
        let mut result = f64::NAN;
        let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
        let err = mpt.up_percent(&mut result);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(result, 0.40000),
            true
        );
    }

    #[test]
    fn should_correct_down_percent() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let bmk_data = vec![
            -0.34902, 4.72157, -0.07781, -5.69315, 0.50715, -3.32714, 2.85004, 0.70329, 5.68503,
            2.51328, 0.19667, 1.60986, -0.88035, 0.93436, 1.73095, 3.99893, -1.58709, -6.90567,
            2.15374, 1.58778, 2.80202, -7.2765, -0.22549, -6.8847, -3.80168, 0.26042, 4.1003,
            4.48299, -7.8344, 3.60549, 3.49499, -8.10192, -20.90433, -11.9778, 5.56181, -11.19773,
        ];
        let mut result = f64::NAN;
        let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
        let err = mpt.down_percent(&mut result);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(result, 0.75000),
            true
        );
    }

    #[test]
    fn should_correct_m_squared() {
        let data = vec![
            2.4037, 1.1276, 2.9127, 2.5981, -0.5162, 2.8709, -1.6506, 0.8281, 4.8182, 4.0484,
            -0.4246, -1.8230, 1.1619, 6.2151, 5.3158, -3.7904, 0.3500, -8.9486, -1.6029, -2.1879,
            6.5159, 3.0498, -8.3762, -3.9341, -0.0780, -17.9807, -21.5895, -11.3292, 4.8884,
            -7.5447, -7.5943, 13.9102, 13.6679, 6.2313, -1.3755, 8.7637,
        ];
        let bmk_data = vec![
            2.3793, 2.5770, 3.2586, 1.9016, 1.4028, 1.5123, -1.9559, 1.1185, 4.4295, 3.4895,
            -1.6613, -3.1005, 1.4990, 3.7399, 1.5907, -4.1807, -0.6938, -5.9982, -3.2486, -0.4318,
            4.8703, 1.2953, -8.4304, -0.8406, 1.4465, -8.9107, -16.7948, -7.1755, 1.0640, -8.4286,
            -10.6478, 8.7595, 9.5709, 5.5933, 0.1984, 7.5637,
        ];
        let rf_data = vec![
            0.4355, 0.4211, 0.4274, 0.4328, 0.4235, 0.4340, 0.4394, 0.4356, 0.4274, 0.4178, 0.4058,
            0.4238, 0.4125, 0.3500, 0.3462, 0.3069, 0.2679, 0.2483, 0.1916, 0.1187, 0.1135, 0.1477,
            0.1636, 0.1443, 0.1541, 0.1197, 0.0669, 0.0254, 0.0031, 0.0105, 0.0265, 0.0213, 0.0145,
            0.0160, 0.0149, 0.0162,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from(&data, &bmk_data, &rf_data);
        let err = mpt.m_squared(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, -1.60415),
            true
        );
    }

    #[test]
    fn should_correct_market_risk() {
        let data = vec![
            -3.64603, 8.80598, -1.4281, -10.48221, -2.80462, 2.98743, -12.2534, -13.75641,
            -3.99425, 14.01812, -6.33636, -8.40185,
        ];
        let bmk_data = vec![
            -5.444254163,
            7.678667961,
            -0.037850949,
            -9.800858913,
            -3.001103688,
            2.626641337,
            -12.56062862,
            -11.84056062,
            1.146646578,
            15.55676217,
            -7.580194297,
            -8.479793853,
        ];
        let mut result = f64::NAN;
        let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
        let err = mpt.market_risk(&mut result);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(result, 6.47447029),
            true
        );
    }

    #[test]
    fn should_correct_stock_risk() {
        let data = vec![
            -3.64603, 8.80598, -1.4281, -10.48221, -2.80462, 2.98743, -12.2534, -13.75641,
            -3.99425, 14.01812, -6.33636, -8.40185,
        ];
        let bmk_data = vec![
            -5.444254163,
            7.678667961,
            -0.037850949,
            -9.800858913,
            -3.001103688,
            2.626641337,
            -12.56062862,
            -11.84056062,
            1.146646578,
            15.55676217,
            -7.580194297,
            -8.479793853,
        ];
        let mut result = f64::NAN;
        let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
        let err = mpt.stock_risk(&mut result);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(result, 3.182966438),
            true
        );
    }

    #[test]
    fn should_correct_covariance() {
        let data = vec![
            -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
            1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
            1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
            1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
        ];
        let bmk_data = vec![
            0.27133, 1.24475, 1.34278, -2.87814, 0.13557, 0.61685, 2.37931, 2.577, 3.25861, 1.9016,
            1.40278, 1.51232, -1.95588, 1.1185, 4.42953, 3.48951, -1.66133, -3.10048, 1.49901,
            3.73988, 1.59068, -4.18066, -0.69376, -5.99816, -3.24858, -0.4318, 4.87031, 1.29526,
            -8.43036, -0.84062, 1.44647, -8.91073, -16.79479, -7.17546, 1.06403, -8.42864,
        ];
        let mut res = 0.0;
        let mpt = MPTCalculator::from_v_b(&data, &bmk_data);
        let err = mpt.covariance(&mut res);
        assert_eq!(
            err == Errors::ClErrorCodeNoError && MPTCalculator::is_eq_double(res, 20.2720342),
            true
        );
    }
}
