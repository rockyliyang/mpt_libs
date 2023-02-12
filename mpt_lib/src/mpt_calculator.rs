use core::slice;

use crate::{common::InputDatas, enums::Errors};
pub struct MPTCalculator<'a> {
    pub values: &'a [f64],
    pub benchmark: &'a [f64],
    pub riskfree: &'a [f64],
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
