//!The full name of MPT is modern portfolio theory.
//! It is an economic framework through which investors try to take minimal market risks and achieve maximum returns for a given investment portfolio
//!</br>
//! The target of lib is provide all MPT calculation method
//!
mod absolute_statistics;

pub mod common;
mod date_util;
pub mod enums;
pub use self::common::check_and_convert;
pub use self::common::MPTCalculator;
