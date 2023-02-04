use num_enum::TryFromPrimitive;
use std::{error::Error, fmt, fmt::Display};
#[derive(TryFromPrimitive)]
#[repr(u32)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Errors {
    ClErrorCodeNoError,
    ClErrorCodeInvalidPara,
    ClErrorCodeInvalidOutput,
    ClErrorCodeInvalidValue,
    ClErrorCodeInputLenTooShort,
    ClErrorCodeInvalidDate,
    ClErrorCodeCcFaild,
    ClErrorCodeDidNotSetHoliday,
    ClErrorCodeUnsortedByDate,
    ClErrorCodeJni,
    ClErrorCodeUnknown = 1000,
    ClErrorMleCodeLogStalbeVar = 1001,
    ClErrorCodeFtqLogStalbeVar = 1002,
    ClErrorCodeInvLogStalbeVar = 1003,
    ClErrorCodeEcfLogStalbeVar = 1004,
    ClErrorCodeCurHergeException = 1005,
}
impl Error for Errors {
    fn description(&self) -> &str {
        "Calculation failed:"
    }
}

impl Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "calculaton failed:{}", self)
    }
}

#[derive(TryFromPrimitive)]
#[repr(i16)]
#[derive(PartialEq, Clone, Copy)]
pub enum ClFrequency {
    ClFrequencyUnknown = -1,
    ClFrequencyDaily,        //= 0,
    ClFrequencyWeekly,       //1
    ClFrequencyMonthly,      //2
    ClFrequencyQuarterly,    //3
    ClFrequencyAnnually,     //4
    ClFrequencySemiannually, //5
}

#[derive(TryFromPrimitive)]
#[repr(i16)]
#[derive(PartialEq, Clone, Copy)]
pub enum ClDateMoveAction {
    ClMoveToEnd,   //= 0,
    ClMoveToBegin, //1
    ClNotMove,     //2
}

#[derive(TryFromPrimitive)]
#[repr(i16)]
#[derive(PartialEq)]
pub enum ClRankType {
    ClRankTypeNoRank = 0,
    ClRankTypeRaw = 1,
    ClRankTypeAsc = 2,
    ClRankTypeDec = 3,
    ClRankTypePercAsc = 4,
    ClRankTypePercDec = 5,
    ClRankTypeDecAsc = 6,
    ClRankTypeDecDec = 7,
    ClRankTypeQuinAsc = 8,
    ClRankTypeQuinDec = 9,
    ClRankTypeQuartAsc = 10,
    ClRankTypeQuartDec = 11,
}

#[derive(TryFromPrimitive)]
#[repr(i16)]
#[derive(PartialEq, Clone, Copy)]
pub enum ClCreditQualityRating {
    CLGradeNotRated = 0,
    CLGradeBelowB = 1,
    CLGradeB = 2,
    CLGradeBB = 3,
    CLGradeBBB = 4,
    CLGradeA = 5,
    CLGradeAA = 6,
    CLGradeAAA = 7,
}
