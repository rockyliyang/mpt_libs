//! The full name of MPT is modern portfolio theory.
//! It is an economic framework through which investors try to take minimal market risks and achieve maximum returns for a given investment portfolio
//! </br>
//! The target of lib is provide all MPT calculation method
//!
//! ## Key parameter explaintion
//! date : all date in this lib is a integer value from 1970-01-01
//!
//! freq: the frequence include bellow
//!
//! ```
//! pub enum ClFrequency {
//! ClFrequencyUnknown = -1,
//! ClFrequencyDaily,        //= 0,
//! ClFrequencyWeekly,       //1
//! ClFrequencyMonthly,      //2
//! ClFrequencyQuarterly,    //3
//! ClFrequencyAnnually,     //4
//! ClFrequencySemiannually, //5
//! }
//!```
//!rank type:
//!```
//! pub enum ClRankType {
//! ClRankTypeNoRank = 0,
//! ClRankTypeRaw = 1,
//! ClRankTypeAsc = 2,
//! ClRankTypeDec = 3,
//! ClRankTypePercAsc = 4,
//! ClRankTypePercDec = 5,
//! ClRankTypeDecAsc = 6,
//! ClRankTypeDecDec = 7,
//! ClRankTypeQuinAsc = 8,
//! ClRankTypeQuinDec = 9,
//! ClRankTypeQuartAsc = 10,
//! ClRankTypeQuartDec = 11,
//! }
//!
//!
//!
mod absolute_statistics;
mod array;
mod common;
mod date_util;
mod rank;
mod relative_statistics;

pub mod enums;
pub mod mpt_calculator;
pub use self::mpt_calculator::check_and_convert;
pub use self::mpt_calculator::MPTCalculator;
