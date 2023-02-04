use crate::{
    common::{
        annualize_return, get_annual_multiplier, is_sorted_array, is_valid_frequency, DataGroup,
        MPTCalculator,
    },
    date_util,
    enums::{self, ClFrequency, Errors},
};
use std::{collections::HashSet, ops::ControlFlow};

impl<'a> MPTCalculator<'a> {
    ///calculate the average value of a series not include NAN/INF values
    ///# Examples

    ///```

    ///use calclib::common::{get_annual_multiplier, MPTCalculator};
    ///use calclib::enums::{self, Errors};

    ///let data = vec![10.0, 20.0, 30.0];
    ///let mut res = 0.0;
    ///let mpt = MPTCalculator::from_v(&data);
    ///let err = mpt.average(&mut res);
    ///assert_eq!(err == Errors::ClErrorCodeNoError && res==20.0,true)
    ///```
    pub fn average(&self, avg: &mut f64) -> Errors {
        return Self::average_internal(self.values, avg);
    }

    ///calculate the standard deviation value of a series not include NAN/INF values
    /// Arguments
    /// freq is the frequence of source data
    /// is_annu is the flag of annuize
    ///# Examples

    ///```

    /// use calclib::common::{is_eq_double,get_annual_multiplier, MPTCalculator};
    /// use calclib::enums::{self, Errors};

    /// let data = vec![
    ///     -1.22072, -0.0668, 2.20588, -0.91563, -0.76766, -1.21429, 3.43456, 4.99825, 3.89481,
    ///     1.59564, 0.86793, 2.41477, -1.80305, 0.6709, 3.57769, 4.77481, -0.37317, -3.52713,
    ///     1.88831, 1.73502, 1.20155, -3.36542, -2.03551, -5.6145, -2.71663, -0.04815, 3.99807,
    ///     1.66744, -9.68658, -0.46681, 4.22095, -6.7, -15.27331, -8.46123, 0.76369, -10.32347,
    /// ];
    /// let mut res = 0.0;
    /// let mpt = MPTCalculator::from_v(&data);
    /// let err = mpt.standard_deviation(enums::ClFrequency::ClFrequencyMonthly, true, &mut res);
    /// assert_eq!(
    ///     err == Errors::ClErrorCodeNoError && is_eq_double(res, 15.99317),
    ///     true
    /// );

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
    ///calculate the standard deviation value of a series not include NAN/INF values
    /// Arguments
    /// freq is the frequence of source data
    /// is_annu is the flag of annuize
    ///# Examples

    ///```

    /// use calclib::common::{is_eq_double,get_annual_multiplier, MPTCalculator};
    /// use calclib::enums::{self, Errors};

    /// let data = vec![
    ///     -1.5,2.3,4.5
    /// ];
    /// let mut res = 0.0;
    /// let mpt = MPTCalculator::from_v(&data);
    /// let err = mpt.mean_harmonic(&mut res);
    /// assert_eq!(
    ///     err == Errors::ClErrorCodeNoError && is_eq_double(res, -310.5),
    ///     true
    /// );

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

    ///calculate the standard deviation value of a series not include NAN/INF values
    /// Arguments
    /// freq is the frequence of source data
    /// is_annu is the flag of annuize
    ///# Examples

    ///```

    /// use calclib::common::{is_eq_double,get_annual_multiplier, MPTCalculator};
    /// use calclib::enums::{self, Errors};

    /// let data = vec![
    ///     -1.5,2.3,4.5
    /// ];
    ///
    /// let weights = vec![0.1,0.2,0.3];
    /// 
    /// let mut res = 0.0;
    /// let mpt = MPTCalculator::from_v(&data);
    /// let err = mpt.weighted_mean_arithmetic(weights,&mut res);
    /// assert_eq!(
    ///     err == Errors::ClErrorCodeNoError && is_eq_double(res, -310.5),
    ///     true
    /// );

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
}

#[cfg(test)]
mod test {
    use crate::{
        common::{is_eq_double, MPTCalculator},
        enums::{self, Errors},
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
            err == Errors::ClErrorCodeNoError && is_eq_double(res, 15.99317),
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
            err == Errors::ClErrorCodeNoError && is_eq_double(res, -310.5),
            true
        );
    }
}