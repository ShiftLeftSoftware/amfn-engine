//! The core utility methods.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use chrono::{DateTime, Datelike, Local};
use math::round;
use rust_decimal::prelude::*;

use crate::core::ListDescriptor;
use crate::{ElemLevelType, ElemUpdateType, ListTrait};

pub struct CoreUtility {}

/// The core utility methods implementation.

impl CoreUtility {
    /// Crop letters from the beginning of a string and
    /// return a new string.
    ///
    /// # Arguments
    ///
    /// * `text` - The string to crop.
    /// * `pos` - The starting position.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn crop_letters(text: &str, pos: usize) -> &str {
        match text.char_indices().nth(pos) {
            None => "",
            Some((pos, _)) => &text[pos..],
        }
    }

    /// Calculates number of intervals between two dates.
    /// If intDate2 is greater than or equal to intDate1,
    /// the result will be positive, otherwise the result
    /// will be negative.
    ///
    /// # Arguments
    ///
    /// * `date1` - First date in YYYYMMDD format.
    /// * `date2` - Second date in YYYYMMDD format.
    /// * `frequency` - Date frequency.
    /// * `intervals` - Number of intervals of frequency.
    /// * `eom_param` - Adjust successive dates to end of month.
    ///
    /// # Return
    ///
    /// * Number of intervals (positive or negative).

    pub fn date_diff(
        mut date1: usize,
        mut date2: usize,
        frequency: crate::FrequencyType,
        intervals: usize,
        eom_param: bool,
    ) -> i32 {
        if intervals == 0 {
            return 0;
        }

        let mut interval_count = (CoreUtility::date_to_serial(date2) as i32)
            - (CoreUtility::date_to_serial(date1) as i32);
        if interval_count == 0 {
            return 0;
        }

        let mut sign: i32 = 1;
        if interval_count < 0 {
            date1 = std::mem::replace(&mut date2, date1);
            interval_count = -interval_count;
            sign = -1;
        }
        if (frequency as usize) > (crate::FrequencyType::StartFixed as usize) {
            return interval_count
                / (CoreUtility::days_in_frequency(frequency, crate::DEFAULT_DAYS_IN_YEAR) as i32)
                * sign;
        }

        let year1 = (date1 as i32) / 10000;
        let month1 = (date1 as i32) % 10000 / 100;
        let day1 = (date1 as i32) % 100;

        let year2 = (date2 as i32) / 10000;
        let month2 = (date2 as i32) % 10000 / 100;
        let mut day2 = (date2 as i32) % 100;

        interval_count = (year2 * 12 - year1 * 12) + (month2 - month1);
        let eom;
        if frequency == crate::FrequencyType::HalfMonth {
            eom = day2 >= 28 && eom_param;
        } else {
            eom = day2 == (CoreUtility::days_in_month(year2 as usize, month2 as usize) as i32);
        }
        if day1 > day2 && !eom {
            day2 += CoreUtility::days_in_month(year1 as usize, month1 as usize) as i32; // For semi-monthly calculation
            interval_count -= 1;
        }

        match frequency {
            crate::FrequencyType::OneYear => interval_count / 12 * sign,
            crate::FrequencyType::SixMonths => interval_count / 6 * sign,
            crate::FrequencyType::FourMonths => interval_count / 4 * sign,
            crate::FrequencyType::ThreeMonths => interval_count / 3 * sign,
            crate::FrequencyType::TwoMonths => interval_count / 2 * sign,
            crate::FrequencyType::HalfMonth => {
                interval_count *= 2;
                if day2 - day1 >= 15 {
                    interval_count += 1;
                }

                interval_count * sign
            }
            _ => interval_count / (intervals as i32) * sign,
        }
    }

    /// Calculates a new date based upon a given date and number of intervals.
    /// If intervals is positive, the resulting date will be greater
    /// than date, otherwise the resulting date will be less than
    /// date.
    ///
    /// # Arguments
    ///
    /// * `orig_date` - Optional original date in YYYYMMDD format,
    ///     otherwise zero. Used for the half-month (semi-monthly) frequency
    ///     and when bolEOM is true.
    /// * `date` - Date in YYYYMMDD format.
    /// * `frequency` - Date frequency.
    /// * `intervals` - Number of intervals of frequency.
    /// * `eom_param` - Adjust successive dates to end of month.
    ///
    /// # Return
    ///
    /// * New date in YYYYMMDD format.

    pub fn date_newi(
        orig_date: usize,
        date: usize,
        frequency: crate::FrequencyType,
        intervals: i32,
        eom_param: bool,
    ) -> usize {
        let mut year = (date / 10000) as i32;
        let mut month = (date % 10000 / 100) as i32;
        let mut day = date % 100;
        let mut orig_day = orig_date % 100;
        if intervals == 0 {
            return date;
        }

        if (frequency as usize) > (crate::FrequencyType::StartFixed as usize) {
            let serial = CoreUtility::date_to_serial(date) as i32;
            let days_in_freq =
                CoreUtility::days_in_frequency(frequency, crate::DEFAULT_DAYS_IN_YEAR) as i32;
            let new_serial = serial + (days_in_freq * intervals);

            return CoreUtility::serial_to_date(new_serial as usize);
        }

        if orig_day == 0 {
            orig_day = day;
        }

        match frequency {
            crate::FrequencyType::OneYear => {
                month += intervals * 12;
            }
            crate::FrequencyType::SixMonths => {
                month += intervals * 6;
            }
            crate::FrequencyType::FourMonths => {
                month += intervals * 4;
            }
            crate::FrequencyType::ThreeMonths => {
                month += intervals * 3;
            }
            crate::FrequencyType::TwoMonths => {
                month += intervals * 2;
            }
            crate::FrequencyType::HalfMonth => {
                month += intervals / 2;
            }
            _ => {
                month += intervals;
            }
        }

        if month > 0 {
            year += (month - 1) / 12;
            month = (month - 1) % 12 + 1;
        } else {
            year = year + (month / 12) - 1;
            month = 12 + (month % 12);
        }

        let mut days_in_month = CoreUtility::days_in_month(year as usize, month as usize);
        if day > days_in_month {
            day = days_in_month;
        }

        let half_intervals = intervals % 2;
        if frequency == crate::FrequencyType::HalfMonth && half_intervals != 0 {
            if half_intervals > 0 && day > 15 {
                month += 1;
                if month > 12 {
                    month = 1;
                    year += 1;
                }
            } else if half_intervals < 0 && day <= 15 {
                month -= 1;
                if month < 1 {
                    month = 12;
                    year -= 1;
                }
            }

            days_in_month = CoreUtility::days_in_month(year as usize, month as usize);

            let mut alt_day;
            if orig_day > 15 {
                if orig_day >= 28 && eom_param {
                    alt_day = 15;
                } else {
                    alt_day = orig_day - 15;
                }
            } else {
                alt_day = orig_day + 15;
            }

            if orig_day > days_in_month || (orig_day >= 28 && eom_param) {
                orig_day = days_in_month;
            }

            if alt_day > days_in_month || (alt_day >= 28 && eom_param) {
                alt_day = days_in_month;
            }

            if ((day as i32) - (alt_day as i32)).abs() > ((day as i32) - (orig_day as i32)).abs() {
                day = alt_day;
            } else {
                day = orig_day;
            }
        } else {
            let eom;
            if frequency == crate::FrequencyType::HalfMonth {
                eom = orig_day >= 28 && eom_param;
            } else {
                eom = orig_day
                    == CoreUtility::days_in_month(orig_date / 10000, orig_date % 10000 / 100);
            }

            if eom {
                // Check for EOM
                if orig_day > days_in_month || eom {
                    day = days_in_month;
                } else {
                    day = orig_day;
                }
            }
        }

        ((year * 10000) as usize) + ((month * 100) as usize) + day
    }

    /// Calculates a new date based upon a given date and number of intervals.
    ///
    /// # Arguments
    ///
    /// * `orig_date` - Optional original date in YYYYMMDD format,
    ///     otherwise zero. Used for the half-month (semi-monthly) frequency
    ///     and when bolEOM is true.
    /// * `date` - Date in YYYYMMDD format.
    /// * `frequency` - Date frequency.
    /// * `intervals` - Number of intervals of frequency.
    /// * `eom_param` - Adjust successive dates to end of month.
    ///
    /// # Return
    ///
    /// * New date in YYYYMMDD format.

    pub fn date_new(
        orig_date: usize,
        date: usize,
        frequency: crate::FrequencyType,
        intervals: usize,
        eom_param: bool,
    ) -> usize {
        CoreUtility::date_newi(orig_date, date, frequency, intervals as i32, eom_param)
    }

    /// Returns the current date in YYYYMMDD format.
    ///
    /// # Return
    ///
    /// * Current date in YYYYMMDD format.

    pub fn date_now() -> usize {
        let dt: DateTime<Local> = Local::now();
        (dt.year() as usize) * 10000 + (dt.month0() as usize) * 100 + (dt.day0() as usize)
    }

    /// Convert a date to a serial number.
    ///
    /// # Arguments
    ///
    /// * `intDate` - Date in YYYYMMDD format.
    ///
    /// # Return
    ///
    /// * Number of days since January 1, SERIAL_BASE_YEAR.

    pub fn date_to_serial(date: usize) -> usize {
        let year = date / 10000;
        let month = date % 10000 / 100;
        let day = date % 100;
        let val: usize;

        if month > 2 {
            val = 93 - CoreUtility::leap_year(year);
        } else {
            val = 91;
        }

        (round::ceil(((year - crate::SERIAL_BASE_YEAR) as f64) * 365.25, 0) as usize)
            + (3055 * (month + 2) / 100 - val + day)
    }

    /// Calculates the day count factor between two dates.
    /// (date2 - date1 must be less than or equal to 1 year.)
    ///
    /// # Arguments
    ///
    /// * `serial1` - First serial date.
    /// * `serial2` - Second serial date.
    /// * `day_count_basis` - Day count basis.
    /// * `days_in_year_param` - Number of days in the year.
    /// * `periods_in_year` - Number of periods in the year.
    ///
    /// # Return
    ///
    /// * Day count factor.

    pub fn day_count_factor(
        serial1: usize,
        serial2: usize,
        day_count_basis: crate::DayCountType,
        days_in_year_param: usize,
        periods_in_year: usize,
    ) -> Decimal {
        let mut days_in_year = days_in_year_param;
        let mut days: usize;

        if day_count_basis == crate::DayCountType::Periodic
            || day_count_basis == crate::DayCountType::RuleOf78
            || day_count_basis == crate::DayCountType::Actual
        {
            days = serial2 - serial1;
            return dec!(days) / dec!(days_in_year);
        }
        if day_count_basis == crate::DayCountType::ActualActualISMA {
            days = serial2 - serial1;
            days_in_year = days * periods_in_year;
            return dec!(days) / dec!(days_in_year);
        }
        let date1 = CoreUtility::serial_to_date(serial1);
        let year1 = (date1 / 10000) as i32;
        let month1 = (date1 % 10000 / 100) as i32;
        let mut day1 = (date1 % 100) as i32;

        let date2 = CoreUtility::serial_to_date(serial2);
        let mut year2 = (date2 / 10000) as i32;
        let mut month2 = (date2 % 10000 / 100) as i32;
        let mut day2 = (date2 % 100) as i32;

        match day_count_basis {
            crate::DayCountType::ActualActualAFB => {
                // AFB
                days = serial2 - serial1;
                days_in_year = 365
                    + CoreUtility::leap_year(year1 as usize)
                    + (if CoreUtility::leap_year(year2 as usize) > 0 {
                        1
                    } else {
                        0
                    });
                return dec!(days) / dec!(days_in_year);
            }
            crate::DayCountType::Actual365L => {
                // ISDA
                let serial = CoreUtility::date_to_serial((year2 as usize) * 10000 + 101); // January 1
                days = serial - serial1;
                days_in_year = 365 + CoreUtility::leap_year(year1 as usize);
                let mut dval = dec!(days) / dec!(days_in_year);
                days = serial2 - serial;
                days_in_year = 365 + CoreUtility::leap_year(year2 as usize);
                dval += dec!(days) / dec!(days_in_year);
                dval
            }
            crate::DayCountType::Dc30 => {
                day1 = if day1 > 30 { 30 } else { day1 };
                day2 = if day1 == 30 && day2 > 30 { 30 } else { day2 };
                days = ((year2 - year1) * 360 + (month2 - month1) * 30 + (day2 - day1)) as usize;
                return dec!(days) / dec!(days_in_year);
            }
            crate::DayCountType::Dc30E => {
                day1 = if day1 > 30 { 30 } else { day1 };
                day2 = if day2 > 30 { 30 } else { day2 };
                days = ((year2 - year1) * 360 + (month2 - month1) * 30 + (day2 - day1)) as usize;
                return dec!(days) / dec!(days_in_year);
            }
            crate::DayCountType::Dc30EP => {
                day1 = if day1 > 30 { 30 } else { day1 };
                if day2 > 30 {
                    day2 = 1;
                    month2 += 1;
                    if month2 > 12 {
                        month2 = 1;
                        year2 += 1;
                    }
                }
                days = ((year2 - year1) * 360 + (month2 - month1) * 30 + (day2 - day1)) as usize;
                return dec!(days) / dec!(days_in_year);
            }
            _ => {
                return dec!(0.0);
            }
        }
    }

    /// Returns the number of days in a frequency.
    ///
    /// # Arguments
    ///
    /// * `frequency` - Frequency value.
    /// * `days_in_year` - Number of days in the year.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn days_in_frequency(frequency: crate::FrequencyType, days_in_year: usize) -> usize {
        match frequency {
            crate::FrequencyType::OneYear
            | crate::FrequencyType::SixMonths
            | crate::FrequencyType::FourMonths
            | crate::FrequencyType::ThreeMonths
            | crate::FrequencyType::TwoMonths
            | crate::FrequencyType::HalfMonth => {
                days_in_year / CoreUtility::intervals_in_year(frequency, days_in_year)
            }
            crate::FrequencyType::FourWeeks => 28,
            crate::FrequencyType::TwoWeeks => 14,
            crate::FrequencyType::OneWeek => 7,
            crate::FrequencyType::OneDay | crate::FrequencyType::Continuous => 1,
            _ => 30, // Monthly
        }
    }

    /// Returns the number of days in a specific year and month.
    ///
    /// # Arguments
    ///
    /// * `year` - The year in YYYY format.
    /// * `month` - The month in MM format.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn days_in_month(year: usize, month: usize) -> usize {
        let days;
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => {
                days = 31;
            }
            2 => {
                if CoreUtility::leap_year(year) > 0 {
                    days = 29;
                } else {
                    days = 28;
                }
            }
            _ => {
                days = 30;
            }
        }
        days
    }

    /// Format and return an update message.
    ///
    /// # Arguments
    ///
    /// * `elem_update_type` - The update type.
    /// * `elem_update_level` - The update level.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn format_update(
        elem_update_type: ElemUpdateType,
        elem_update_level: ElemLevelType,
    ) -> String {
        let update_type: &str;
        match elem_update_type {
            crate::ElemUpdateType::Parameter => {
                update_type = "Parameter";
            }
            crate::ElemUpdateType::Preferences => {
                update_type = "Preferences";
            }
            crate::ElemUpdateType::Cashflow => {
                update_type = "Cashflow";
            }
            crate::ElemUpdateType::Event => {
                update_type = "Event";
            }
            crate::ElemUpdateType::Template => {
                update_type = "Template";
            }
            crate::ElemUpdateType::ExchangeRate => {
                update_type = "ExchangeRate";
            }
            _ => {
                update_type = "Descriptor";
            }
        }

        let update_level: &str;
        match elem_update_level {
            crate::ElemLevelType::Cashflow => {
                update_level = "Cashflow";
            }
            crate::ElemLevelType::Event => {
                update_level = "Event";
            }
            _ => {
                update_level = "Engine";
            }
        }
        format!("Update: {}, Level: {}", update_type, update_level)
    }

    /// Returns the enumerated value for a function number.
    ///
    /// # Arguments
    ///
    /// * `val` - The function number.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn get_col_name(val: usize) -> crate::ColumnType {
        match val {
            x if x == crate::ColumnType::Sequence as usize => crate::ColumnType::Sequence,
            x if x == crate::ColumnType::EventType as usize => crate::ColumnType::EventType,
            x if x == crate::ColumnType::Date as usize => crate::ColumnType::Date,
            x if x == crate::ColumnType::DateExpr as usize => crate::ColumnType::DateExpr,
            x if x == crate::ColumnType::Sort as usize => crate::ColumnType::Sort,
            x if x == crate::ColumnType::Value as usize => crate::ColumnType::Value,
            x if x == crate::ColumnType::ValueExpr as usize => crate::ColumnType::ValueExpr,
            x if x == crate::ColumnType::Decrease as usize => crate::ColumnType::Decrease,
            x if x == crate::ColumnType::Increase as usize => crate::ColumnType::Increase,
            x if x == crate::ColumnType::Periods as usize => crate::ColumnType::Periods,
            x if x == crate::ColumnType::PeriodsExpr as usize => crate::ColumnType::PeriodsExpr,
            x if x == crate::ColumnType::SkipPeriods as usize => crate::ColumnType::SkipPeriods,
            x if x == crate::ColumnType::Intervals as usize => crate::ColumnType::Intervals,
            x if x == crate::ColumnType::Frequency as usize => crate::ColumnType::Frequency,
            x if x == crate::ColumnType::EndDate as usize => crate::ColumnType::EndDate,
            x if x == crate::ColumnType::DescriptorList as usize => {
                crate::ColumnType::DescriptorList
            }
            x if x == crate::ColumnType::ParameterList as usize => crate::ColumnType::ParameterList,
            x if x == crate::ColumnType::Interest as usize => crate::ColumnType::Interest,
            x if x == crate::ColumnType::SlInterest as usize => crate::ColumnType::SlInterest,
            x if x == crate::ColumnType::IntOnInterest as usize => crate::ColumnType::IntOnInterest,
            x if x == crate::ColumnType::ValueToInterest as usize => {
                crate::ColumnType::ValueToInterest
            }
            x if x == crate::ColumnType::ValueToPrincipal as usize => {
                crate::ColumnType::ValueToPrincipal
            }
            x if x == crate::ColumnType::AccruedBalance as usize => {
                crate::ColumnType::AccruedBalance
            }
            x if x == crate::ColumnType::Balance as usize => crate::ColumnType::Balance,
            x if x == crate::ColumnType::EventName as usize => crate::ColumnType::EventName,
            x if x == crate::ColumnType::NextName as usize => crate::ColumnType::NextName,
            _ => crate::ColumnType::None,
        }
    }

    /// Get the column name resource key.
    ///
    /// # Arguments
    ///
    /// * `column_value` - Column name value.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn get_col_name_resource_key(column_value: crate::ColumnType) -> String {
        match column_value {
            crate::ColumnType::EventType => String::from(crate::COL_LABEL_EVENT_TYPE),
            crate::ColumnType::Date => String::from(crate::COL_LABEL_DATE),
            crate::ColumnType::DateExpr => String::from(crate::COL_LABEL_DATE_EXPR),
            crate::ColumnType::Sort => String::from(crate::COL_LABEL_SORT),
            crate::ColumnType::Value => String::from(crate::COL_LABEL_VALUE),
            crate::ColumnType::ValueExpr => String::from(crate::COL_LABEL_VALUE_EXPR),
            crate::ColumnType::Decrease => String::from(crate::COL_LABEL_DECREASE),
            crate::ColumnType::Increase => String::from(crate::COL_LABEL_INCREASE),
            crate::ColumnType::Periods => String::from(crate::COL_LABEL_PERIODS),
            crate::ColumnType::PeriodsExpr => String::from(crate::COL_LABEL_PERIODS_EXPR),
            crate::ColumnType::SkipPeriods => String::from(crate::COL_LABEL_SKIP_PERIODS),
            crate::ColumnType::Intervals => String::from(crate::COL_LABEL_INTERVALS),
            crate::ColumnType::Frequency => String::from(crate::COL_LABEL_FREQUENCY),
            crate::ColumnType::EndDate => String::from(crate::COL_LABEL_END_DATE),
            crate::ColumnType::DescriptorList => String::from(crate::COL_LABEL_DESCRIPTOR_LIST),
            crate::ColumnType::ParameterList => String::from(crate::COL_LABEL_PARAMETER_LIST),
            crate::ColumnType::Interest => String::from(crate::COL_LABEL_INTEREST),
            crate::ColumnType::SlInterest => String::from(crate::COL_LABEL_SL_INTEREST),
            crate::ColumnType::IntOnInterest => String::from(crate::COL_LABEL_INT_ON_INTEREST),
            crate::ColumnType::ValueToInterest => String::from(crate::COL_LABEL_VALUE_TO_INTEREST),
            crate::ColumnType::ValueToPrincipal => {
                String::from(crate::COL_LABEL_VALUE_TO_PRINCIPAL)
            }
            crate::ColumnType::AccruedBalance => String::from(crate::COL_LABEL_ACCRUED_BALANCE),
            crate::ColumnType::Balance => String::from(crate::COL_LABEL_BALANCE),
            crate::ColumnType::EventName => String::from(crate::COL_LABEL_EVENT_NAME),
            crate::ColumnType::NextName => String::from(crate::COL_LABEL_NEXT_NAME),
            _ => String::from(crate::COL_LABEL_SEQUENCE),
        }
    }

    /// Returns the constant value for a day count basis mnemonic.
    ///
    /// # Arguments
    ///
    /// * `text_param` - The day count basis mnemonic.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn get_day_count_basis(text_param: &str) -> crate::DayCountType {
        let text = String::from(text_param).to_lowercase();

        match text.as_str() {
            "rule-of-78" => crate::DayCountType::RuleOf78,
            "actual" => crate::DayCountType::Actual,
            "actual-actual-isma" => crate::DayCountType::ActualActualISMA,
            "actual-actual-afb" => crate::DayCountType::ActualActualAFB,
            "actual-365L" => crate::DayCountType::Actual365L,
            "30" => crate::DayCountType::Dc30,
            "30E" => crate::DayCountType::Dc30E,
            "30EP" => crate::DayCountType::Dc30EP,
            _ => crate::DayCountType::Periodic,
        }
    }

    /// Returns the day count basis mnemonic for a constant value.
    ///
    /// # Arguments
    ///
    /// * `day_count_basis` - The constant value.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn get_day_count_basis_mnemonic(day_count_basis: crate::DayCountType) -> String {
        match day_count_basis {
            crate::DayCountType::RuleOf78 => String::from("rule-of-78"),
            crate::DayCountType::Actual => String::from("actual"),
            crate::DayCountType::ActualActualISMA => String::from("actual-actual-isma"),
            crate::DayCountType::ActualActualAFB => String::from("actual-actual-afb"),
            crate::DayCountType::Actual365L => String::from("actual-365L"),
            crate::DayCountType::Dc30 => String::from("30"),
            crate::DayCountType::Dc30E => String::from("30E"),
            crate::DayCountType::Dc30EP => String::from("30EP"),
            _ => String::from("periodic"),
        }
    }

    /// Returns the short day count basis mnemonic for a constant value.
    ///
    /// # Arguments
    ///
    /// * `day_count_basis` - The constant value.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn get_day_count_basis_mnemonic_short(day_count_basis: crate::DayCountType) -> String {
        match day_count_basis {
            crate::DayCountType::RuleOf78 => String::from("78"),
            crate::DayCountType::Actual => String::from("Act"),
            crate::DayCountType::ActualActualISMA => String::from("Act/ISMA"),
            crate::DayCountType::ActualActualAFB => String::from("Act/AFB"),
            crate::DayCountType::Actual365L => String::from("Act/365L"),
            crate::DayCountType::Dc30 => String::from("30"),
            crate::DayCountType::Dc30E => String::from("30E"),
            crate::DayCountType::Dc30EP => String::from("30E+"),
            _ => String::from("Per"),
        }
    }

    /// Searches the various descriptor lists, from lowest
    /// to highest, and returns the constant value for a
    /// group, name, type, and code.
    ///
    /// # Arguments
    ///
    /// * `list_descriptor_user_opt` - The user descriptor (or None).
    /// * `list_descriptor_cashflow_opt` - The cashflow descriptor list (or None).
    /// * `list_descriptor_event_opt` - The event descriptor list (or None).
    /// * `group` - The group name of the descriptor.
    /// * `name` - The name of the descriptor.
    /// * `desc_type` - The type of the descriptor.
    /// * `code` - The code of the descriptor.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn get_descriptor_value(
        list_descriptor_user_opt: Option<&ListDescriptor>,
        list_descriptor_cashflow_opt: Option<&ListDescriptor>,
        list_descriptor_event_opt: Option<&ListDescriptor>,
        group: &str,
        name: &str,
        desc_type: &str,
        code: &str,
    ) -> String {
        let mut value = String::from("");
        let mut orig_list_index: usize;

        match list_descriptor_event_opt.as_ref() {
            None => {}
            Some(o) => {
                orig_list_index = o.index();
                if o.get_element_by_name(group, name, desc_type, code, true) {
                    value = String::from(o.value().as_str());
                    o.get_element(orig_list_index);
                }
            }
        }

        if value.is_empty() {
            match list_descriptor_cashflow_opt.as_ref() {
                None => {}
                Some(o) => {
                    orig_list_index = o.index();
                    if o.get_element_by_name(group, name, desc_type, code, true) {
                        value = String::from(o.value().as_str());
                        o.get_element(orig_list_index);
                    }
                }
            }
        }

        if value.is_empty() {
            match list_descriptor_user_opt.as_ref() {
                None => {}
                Some(o) => {
                    orig_list_index = o.index();
                    if o.get_element_by_name(group, name, desc_type, code, true) {
                        value = String::from(o.value().as_str());
                        o.get_element(orig_list_index);
                    }
                }
            }
        }

        value
    }

    /// Returns the constant value for an event type mnemonic.
    ///
    /// # Arguments
    ///
    /// * `text_param` - The event type mnemonic.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn get_event_type(text_param: &str) -> crate::ExtensionType {
        let text = text_param.to_lowercase();

        match text.as_str() {
            "current-value" => crate::ExtensionType::CurrentValue,
            "interest-change" => crate::ExtensionType::InterestChange,
            "statistic-value" => crate::ExtensionType::StatisticValue,
            _ => crate::ExtensionType::PrincipalChange,
        }
    }

    /// Returns the event type mnemonic for a constant value.
    ///
    /// # Arguments
    ///
    /// * `event_type` - The constant value.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn get_event_type_mnemonic(event_type: crate::ExtensionType) -> String {
        match event_type {
            crate::ExtensionType::CurrentValue => String::from("current-value"),
            crate::ExtensionType::InterestChange => String::from("interest-change"),
            crate::ExtensionType::StatisticValue => String::from("statistic-value"),
            _ => String::from("principal-change"),
        }
    }

    /// Returns the short event type mnemonic for a constant value.
    ///
    /// # Arguments
    ///
    /// * `event_type` - The constant value.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn get_event_type_mnemonic_short(event_type: crate::ExtensionType) -> String {
        match event_type {
            crate::ExtensionType::CurrentValue => String::from("Cur"),
            crate::ExtensionType::StatisticValue => String::from("Stat"),
            crate::ExtensionType::InterestChange => String::from("Int"),
            _ => String::from("Prin"),
        }
    }

    /// Returns the constant value for a frequency mnemonic.
    ///
    /// # Arguments
    ///
    /// * `text_param` - The frequency mnemonic.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn get_frequency(text_param: &str) -> crate::FrequencyType {
        let text = text_param.to_lowercase();
        let frequency: crate::FrequencyType;

        match text.as_str() {
            "1-year" => {
                frequency = crate::FrequencyType::OneYear;
            }
            "6-months" => {
                frequency = crate::FrequencyType::SixMonths;
            }
            "4-months" => {
                frequency = crate::FrequencyType::FourMonths;
            }
            "3-months" => {
                frequency = crate::FrequencyType::ThreeMonths;
            }
            "2-months" => {
                frequency = crate::FrequencyType::TwoMonths;
            }
            "1-month" => {
                frequency = crate::FrequencyType::OneMonth;
            }
            "half-month" => {
                frequency = crate::FrequencyType::HalfMonth;
            }
            "4-weeks" => {
                frequency = crate::FrequencyType::FourWeeks;
            }
            "2-weeks" => {
                frequency = crate::FrequencyType::TwoWeeks;
            }
            "1-week" => {
                frequency = crate::FrequencyType::OneWeek;
            }
            "1-day" => {
                frequency = crate::FrequencyType::OneDay;
            }
            "continuous" => {
                frequency = crate::FrequencyType::Continuous;
            }
            _ => {
                frequency = crate::FrequencyType::None;
            }
        }

        frequency
    }

    /// Returns the frequency mnemonic for a constant value.
    ///
    /// # Arguments
    ///
    /// * `frequency` - The constant value.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn get_frequency_mnemonic(frequency: crate::FrequencyType) -> String {
        match frequency {
            crate::FrequencyType::OneYear => String::from("1-year"),
            crate::FrequencyType::SixMonths => String::from("6-months"),
            crate::FrequencyType::FourMonths => String::from("4-months"),
            crate::FrequencyType::ThreeMonths => String::from("3-months"),
            crate::FrequencyType::TwoMonths => String::from("2-months"),
            crate::FrequencyType::OneMonth => String::from("1-month"),
            crate::FrequencyType::HalfMonth => String::from("half-month"),
            crate::FrequencyType::FourWeeks => String::from("4-weeks"),
            crate::FrequencyType::TwoWeeks => String::from("2-weeks"),
            crate::FrequencyType::OneWeek => String::from("1-week"),
            crate::FrequencyType::OneDay => String::from("1-day"),
            crate::FrequencyType::Continuous => String::from("continuous"),
            _ => String::from("none"),
        }
    }
    /// Returns the enumerated value for a format number.
    ///
    /// # Arguments
    ///
    /// * `val` - The format number.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn get_format(val: usize) -> crate::FormatType {
        match val {
            x if x == crate::FormatType::Date as usize => crate::FormatType::Date,
            x if x == crate::FormatType::Integer as usize => crate::FormatType::Integer,
            x if x == crate::FormatType::Decimal as usize => crate::FormatType::Decimal,
            x if x == crate::FormatType::Currency as usize => crate::FormatType::Currency,
            _ => crate::FormatType::String,
        }
    }

    /// Returns the enumerated value for a function number.
    ///
    /// # Arguments
    ///
    /// * `val` - The function number.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn get_function(val: usize) -> crate::FunctionType {
        match val {
            x if x == crate::FunctionType::Abs as usize => crate::FunctionType::Abs,
            x if x == crate::FunctionType::Am as usize => crate::FunctionType::Am,
            x if x == crate::FunctionType::Cashflow as usize => crate::FunctionType::Cashflow,
            x if x == crate::FunctionType::DateDiff as usize => crate::FunctionType::DateDiff,
            x if x == crate::FunctionType::DateFiscal as usize => crate::FunctionType::DateFiscal,
            x if x == crate::FunctionType::DateNew as usize => crate::FunctionType::DateNew,
            x if x == crate::FunctionType::DateNow as usize => crate::FunctionType::DateNow,
            x if x == crate::FunctionType::Default as usize => crate::FunctionType::Default,
            x if x == crate::FunctionType::Descriptor as usize => crate::FunctionType::Descriptor,
            x if x == crate::FunctionType::Decimal as usize => crate::FunctionType::Decimal,
            x if x == crate::FunctionType::Format as usize => crate::FunctionType::Format,
            x if x == crate::FunctionType::FormatCurrency as usize => {
                crate::FunctionType::FormatCurrency
            }
            x if x == crate::FunctionType::FormatDate as usize => crate::FunctionType::FormatDate,
            x if x == crate::FunctionType::FormatNumber as usize => {
                crate::FunctionType::FormatNumber
            }
            x if x == crate::FunctionType::If as usize => crate::FunctionType::If,
            x if x == crate::FunctionType::Integer as usize => crate::FunctionType::Integer,
            x if x == crate::FunctionType::Len as usize => crate::FunctionType::Len,
            x if x == crate::FunctionType::Lowercase as usize => crate::FunctionType::Lowercase,
            x if x == crate::FunctionType::Max as usize => crate::FunctionType::Max,
            x if x == crate::FunctionType::Mid as usize => crate::FunctionType::Mid,
            x if x == crate::FunctionType::Min as usize => crate::FunctionType::Min,
            x if x == crate::FunctionType::Parse as usize => crate::FunctionType::Parse,
            x if x == crate::FunctionType::Pr as usize => crate::FunctionType::Pr,
            x if x == crate::FunctionType::Replace as usize => crate::FunctionType::Replace,
            x if x == crate::FunctionType::Round as usize => crate::FunctionType::Round,
            x if x == crate::FunctionType::RoundFraction as usize => {
                crate::FunctionType::RoundFraction
            }
            x if x == crate::FunctionType::Set as usize => crate::FunctionType::Set,
            x if x == crate::FunctionType::Trim as usize => crate::FunctionType::Trim,
            x if x == crate::FunctionType::Type as usize => crate::FunctionType::Type,
            x if x == crate::FunctionType::Uppercase as usize => crate::FunctionType::Uppercase,
            _ => crate::FunctionType::None,
        }
    }

    /// Returns the constant value for an interest method mnemonic.
    ///
    /// # Arguments
    ///
    /// * `text_param` - The interest method mnemonic.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn get_interest_method(text_param: &str) -> crate::MethodType {
        let text = text_param.to_lowercase();

        match text.as_str() {
            "simple-interest" => crate::MethodType::SimpleInterest,
            _ => crate::MethodType::Actuarial,
        }
    }

    /// Returns the interest method mnemonic for a constant value.
    ///
    /// # Arguments
    ///
    /// * `interest_method` - The constant value.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn get_interest_method_mnemonic(interest_method: crate::MethodType) -> String {
        match interest_method {
            crate::MethodType::SimpleInterest => String::from("simple-interest"),
            _ => String::from("actuarial"),
        }
    }

    /// Returns the short interest method mnemonic for a constant value.
    ///
    /// # Arguments
    ///
    /// * `interest_method` - The constant value.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn get_interest_method_mnemonic_short(interest_method: crate::MethodType) -> String {
        match interest_method {
            crate::MethodType::SimpleInterest => String::from("Sim"),
            _ => String::from("Act"),
        }
    }

    /// Returns the merge type for a mnemonic.
    ///
    /// # Arguments
    ///
    /// * `merge_type` - The merge type mnemonic.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn get_merge_type(merge_type: &str) -> crate::MergeType {
        let text = merge_type.to_lowercase();

        match text.as_str() {
            "int-1" => crate::MergeType::Int1,
            "int-2" => crate::MergeType::Int2,
            "int-all" => crate::MergeType::IntAll,
            _ => crate::MergeType::IntNone,
        }
    }

    /// Returns the enumerated value for an operator number.
    ///
    /// # Arguments
    ///
    /// * `val` - The operator number.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn get_operator(val: usize) -> crate::OperatorType {
        match val {
            x if x == crate::OperatorType::And as usize => crate::OperatorType::And,
            x if x == crate::OperatorType::Or as usize => crate::OperatorType::Or,
            x if x == crate::OperatorType::Greater as usize => crate::OperatorType::Greater,
            x if x == crate::OperatorType::Less as usize => crate::OperatorType::Less,
            x if x == crate::OperatorType::GreaterEqual as usize => {
                crate::OperatorType::GreaterEqual
            }
            x if x == crate::OperatorType::LessEqual as usize => crate::OperatorType::LessEqual,
            x if x == crate::OperatorType::Equal as usize => crate::OperatorType::Equal,
            x if x == crate::OperatorType::NotEqual as usize => crate::OperatorType::NotEqual,
            x if x == crate::OperatorType::Plus as usize => crate::OperatorType::Plus,
            x if x == crate::OperatorType::Minus as usize => crate::OperatorType::Minus,
            x if x == crate::OperatorType::Times as usize => crate::OperatorType::Times,
            x if x == crate::OperatorType::Divide as usize => crate::OperatorType::Divide,
            x if x == crate::OperatorType::Modulus as usize => crate::OperatorType::Modulus,
            x if x == crate::OperatorType::Exponent as usize => crate::OperatorType::Exponent,
            x if x == crate::OperatorType::UnaryMinus as usize => crate::OperatorType::UnaryMinus,
            x if x == crate::OperatorType::UnaryNot as usize => crate::OperatorType::UnaryNot,
            _ => crate::OperatorType::None,
        }
    }

    /// Returns the constant value for a principal type mnemonic.
    ///
    /// # Arguments
    ///
    /// * `text_param` - The principal type mnemonic.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn get_principal_type(text_param: &str) -> crate::PrincipalType {
        let text = text_param.to_lowercase();

        match text.as_str() {
            "positive" => crate::PrincipalType::Positive,
            "negative" => crate::PrincipalType::Negative,
            "decrease" => crate::PrincipalType::Decrease,
            _ => crate::PrincipalType::Increase,
        }
    }

    /// Returns the principal type mnemonic for a constant value.
    ///
    /// # Arguments
    ///
    /// * `principal_type` - The constant value.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn get_principal_type_mnemonic(principal_type: crate::PrincipalType) -> String {
        match principal_type {
            crate::PrincipalType::Positive => String::from("positive"),
            crate::PrincipalType::Negative => String::from("negative"),
            crate::PrincipalType::Decrease => String::from("decrease"),
            _ => String::from("increase"),
        }
    }

    /// Returns the short principal type mnemonic for a constant value.
    ///
    /// # Arguments
    ///
    /// * `principal_type` - The constant value.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn get_principal_type_mnemonic_short(principal_type: crate::PrincipalType) -> String {
        match principal_type {
            crate::PrincipalType::Positive => String::from("Pos"),
            crate::PrincipalType::Negative => String::from("Neg"),
            crate::PrincipalType::Decrease => String::from("Decr"),
            _ => String::from("Incr"),
        }
    }

    /// Serialize a round balance value.
    ///
    /// # Arguments
    ///
    /// * `round_bal` - Round balance value.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn get_round_balance(round_bal: crate::RoundType) -> String {
        match round_bal {
            crate::RoundType::Bankers => String::from("bankers"),
            crate::RoundType::BiasUp => String::from("bias_up"),
            crate::RoundType::BiasDown => String::from("bias_down"),
            crate::RoundType::Up => String::from("up"),
            crate::RoundType::Truncate => String::from("truncate"),
            _ => String::from("none"),
        }
    }

    /// Return the number of intervals in a year for a frequency.
    ///
    /// # Arguments
    ///
    /// * `frequency` - Frequency value.
    /// * `days_in_year` - Number of days in the year.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn intervals_in_year(frequency: crate::FrequencyType, days_in_year: usize) -> usize {
        match frequency {
            crate::FrequencyType::OneYear => 1,
            crate::FrequencyType::SixMonths => 2,
            crate::FrequencyType::FourMonths => 3,
            crate::FrequencyType::ThreeMonths => 4,
            crate::FrequencyType::TwoMonths => 6,
            crate::FrequencyType::HalfMonth => 24,
            crate::FrequencyType::FourWeeks => 13,
            crate::FrequencyType::TwoWeeks => 26,
            crate::FrequencyType::OneWeek => 52,
            crate::FrequencyType::OneDay | crate::FrequencyType::Continuous => days_in_year,
            _ => 12, // Monthly
        }
    }

    /// Indicates if the year is a leap year.
    ///
    /// # Arguments
    ///
    /// * `year` - The year in YYYY format.
    ///
    /// # Return
    ///
    /// * If a leap year 1, otherwise 0.

    pub fn leap_year(year: usize) -> usize {
        if year % 4 > 0 {
            return 0;
        }
        if year % 100 > 0 {
            return 1;
        }
        if year % 400 > 0 {
            return 0;
        }

        1
    }

    /// Parse a string and return a Decimal value.
    ///
    /// # Arguments
    ///
    /// * `text` - &str represention of the Decimal value.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn parse_decimal(text: &str) -> Decimal {
        match text.parse::<Decimal>() {
            Err(_e) => {
                dec!(0.0)
            }
            Ok(o) => o,
        }
    }

    /// Parse a string and return an integer value.
    ///
    /// # Arguments
    ///
    /// * `text` - &str represention of the integer value.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn parse_integeri(text: &str) -> i32 {
        match text.parse::<i32>() {
            Err(_e) => 0,
            Ok(o) => o,
        }
    }

    /// Parse a string and return an integer value.
    ///
    /// # Arguments
    ///
    /// * `text` - &str represention of the integer value.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn parse_integer(text: &str) -> usize {
        match text.parse::<usize>() {
            Err(_e) => 0,
            Ok(o) => o,
        }
    }

    /// Parse and return token1 from the string.
    ///
    /// # Arguments
    ///
    /// * `text` - The string (token1, token2).
    ///
    /// # Return
    ///
    /// * See description.

    pub fn parse_token1(text: &str) -> &str {
        let tokens: Vec<_> = text.split('~').collect();
        if tokens.len() < 2 {
            return text;
        }
        tokens[0].trim()
    }

    /// Parse and return token2 from the string.
    ///
    /// # Arguments
    ///
    /// * `text` - The string (token1, token2).
    ///
    /// # Return
    ///
    /// * See description.

    pub fn parse_token2(text: &str) -> &str {
        let tokens: Vec<_> = text.split('~').collect();
        if tokens.len() < 2 {
            return text;
        }
        tokens[1].trim()
    }

    /// Multiplies a value by e and return the result.
    ///
    /// # Arguments
    ///
    /// * `value` - The decimal value.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn decimal_exp(value: Decimal) -> Decimal {
        dec!("2.7182818284590452353602874713527") * value
    }

    /// Returns the result of a value to the exponent power.
    ///
    /// # Arguments
    ///
    /// * `value` - The decimal value.
    /// * `exponent` - The exponent value.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn decimal_pow(value: Decimal, mut exponent: usize) -> Decimal {
        let mut result = value;

        while exponent > 1 {
            result *= value;
            exponent -= 1;
        }

        result
    }

    /// Converts a Daily Rate (DR) into a Nominal Annual Rate (NAR).
    ///
    /// # Arguments
    ///
    /// * `dr` - The daily interest rate expressed as a decimal value.
    /// * `days_in_year` - Number of days in the year.
    ///
    /// # Return
    ///
    /// * The nominal annual rate as a decimal value.

    pub fn rate_dr_to_nar(dr: Decimal, days_in_year: usize) -> Decimal {
        dr * dec!(CoreUtility::intervals_in_year(
            crate::FrequencyType::OneDay,
            days_in_year
        ))
    }

    /// Convert an Effective Annual Rate (EAR) into a Nominal Annual Rate (NAR).
    ///
    /// # Arguments
    ///
    /// * `ear` - The effective annual interest rate expressed as a decimal value.
    /// * `compound_frequency` - Actual compounding frequency.
    /// * `days_in_year` - Number of days in the year.
    ///
    /// # Return
    ///
    /// * The nominal annual rate as a decimal value.

    pub fn rate_ear_to_nar(
        ear: Decimal,
        compound_frequency: crate::FrequencyType,
        days_in_year: usize,
    ) -> Decimal {
        let compound_intervals = CoreUtility::intervals_in_year(compound_frequency, days_in_year);
        let exponent = (1.0 / (compound_intervals as f64)) as usize;
        (CoreUtility::decimal_pow(ear + dec!(1.0), exponent) - dec!(1.0)) * dec!(compound_intervals)
    }

    /// Convert an effective rate into a nominal rate.
    ///
    /// # Arguments
    ///
    /// * `eff` - The effective rate expressed as a decimal value.
    /// * `compound_frequency` - Actual compounding frequency.
    /// * `effective_frequency` - Effective compounding frequency.
    /// * `days_in_year` - Number of days in the year.
    ///
    /// # Return
    ///
    /// * The nominal rate as a decimal value.

    pub fn rate_eff_to_nom(
        eff: Decimal,
        compound_frequency: crate::FrequencyType,
        effective_frequency: crate::FrequencyType,
        days_in_year: usize,
    ) -> Decimal {
        if effective_frequency == compound_frequency {
            return eff;
        }
        let compound_intervals = CoreUtility::intervals_in_year(compound_frequency, days_in_year);
        let effective_intervals = CoreUtility::intervals_in_year(effective_frequency, days_in_year);
        let exponent = ((effective_intervals as f64) / (compound_intervals as f64)) as usize;

        dec!(compound_intervals)
            * (CoreUtility::decimal_pow(eff / dec!(effective_intervals) + dec!(1.0), exponent)
                - dec!(1.0))
    }

    /// Converts a Nominal Annual Rate (NAR) into a Daily Rate (DR).
    ///
    /// # Arguments
    ///
    /// * `nar` - The nominal annual interest rate expressed as a decimal value.
    /// * `days_in_year` - Number of days in the year.
    ///
    /// # Return
    ///
    /// * The daily rate as a decimal value.

    pub fn rate_nar_to_dr(nar: Decimal, days_in_year: usize) -> Decimal {
        nar / dec!(CoreUtility::intervals_in_year(
            crate::FrequencyType::OneDay,
            days_in_year
        ))
    }

    /// Convert a Nominal Annual Rate (NAR) into an Effective Annual Rate (EAR).
    ///
    /// # Arguments
    ///
    /// * `nar` - The nominal annual interest rate expressed as a decimal value.
    /// * `compound_frequency` - Actual compounding frequency.
    /// * `days_in_year` - Number of days in the year.
    ///
    /// # Return
    ///
    /// * The effective annual rate as a decimal value.

    pub fn rate_nar_to_ear(
        nar: Decimal,
        compound_frequency: crate::FrequencyType,
        days_in_year: usize,
    ) -> Decimal {
        let compound_intervals = CoreUtility::intervals_in_year(compound_frequency, days_in_year);

        CoreUtility::decimal_pow(
            nar / dec!(compound_intervals) + dec!(1.0),
            compound_intervals,
        ) - dec!(1.0)
    }

    /// Convert a Nominal Annual Rate (NAR) into a Periodic Rate (PR).
    ///
    /// # Arguments
    ///
    /// * `nar` - The nominal annual interest rate expressed as a decimal value.
    /// * `frequency` - Frequency value.
    /// * `days_in_year` - Number of days in the year.
    ///
    /// # Return
    ///
    /// * The periodic rate as a decimal value.

    pub fn rate_nar_to_pr(
        nar: Decimal,
        frequency: crate::FrequencyType,
        days_in_year: usize,
    ) -> Decimal {
        let intervals = CoreUtility::intervals_in_year(frequency, days_in_year);

        nar / dec!(intervals)
    }

    /// Convert a nominal rate into an effective rate.
    ///
    /// # Arguments
    ///
    /// * `nom` - The nominal rate expressed as a decimal value.
    /// * `compound_frequency` - Actual compounding frequency.
    /// * `effective_frequency` - Effective compounding frequency.
    /// * `days_in_year` - Number of days in the year.
    ///
    /// # Return
    ///
    /// * The effective rate as a decimal value.

    pub fn rate_nom_to_eff(
        nom: Decimal,
        compound_frequency: crate::FrequencyType,
        effective_frequency: crate::FrequencyType,
        days_in_year: usize,
    ) -> Decimal {
        if effective_frequency == compound_frequency {
            return nom;
        }
        let compound_intervals = CoreUtility::intervals_in_year(compound_frequency, days_in_year);
        let effective_intervals = CoreUtility::intervals_in_year(effective_frequency, days_in_year);
        let exponent = ((compound_intervals as f64) / (effective_intervals as f64)) as usize;

        dec!(effective_intervals)
            * (CoreUtility::decimal_pow(nom / dec!(compound_intervals) + dec!(1.0), exponent)
                - dec!(1.0))
    }

    /// Convert a Periodic Rate (PR) into a Nominal Annual Rate (NAR).
    ///
    /// # Arguments
    ///
    /// * `pr` - The periodic interest rate expressed as a decimal value.
    /// * `frequency` - Frequency value.
    /// * `days_in_year` - Number of days in the year.
    ///
    /// # Return
    ///
    /// * The nominal annual rate as a decimal value.

    pub fn rate_pr_to_nar(
        pr: Decimal,
        frequency: crate::FrequencyType,
        days_in_year: usize,
    ) -> Decimal {
        let intervals = CoreUtility::intervals_in_year(frequency, days_in_year);
        pr * dec!(intervals)
    }

    /// Round the value to the specified number of decimal digits.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to round.
    /// * `decimal_digits` - The number of decimal digits.
    /// * `round_ctrl` - Round control.
    ///
    /// # Return
    ///
    /// * The rounded value.

    pub fn round(value: Decimal, decimal_digits: usize, round_ctrl: crate::RoundType) -> Decimal {
        if round_ctrl == crate::RoundType::None {
            return value;
        }

        let low_round = dec!(crate::LOW_ROUND);
        let mid_round = dec!(crate::MID_ROUND);
        let high_round = dec!(crate::HIGH_ROUND);
        let whole: Decimal = value.floor();
        let fract: Decimal = value - whole;

        let dv: Decimal = if decimal_digits > 0 {
            CoreUtility::decimal_pow(dec!(10.0), decimal_digits)
        } else {
            dec!(1.0)
        };
        let afract: Decimal = fract * dv;
        let wfract: Decimal = afract.floor();
        let pfract: Decimal = afract - wfract;
        match round_ctrl {
            crate::RoundType::Bankers => {
                if pfract > low_round && pfract < high_round {
                    let ncmp = if decimal_digits > 0 { wfract } else { whole };
                    if ncmp % dec!(2.0) == dec!(0.0) {
                        return whole + wfract;
                    }
                    return whole + (afract.ceil() / dv);
                }
                whole + ((afract + mid_round).floor() / dv)
            }
            crate::RoundType::BiasUp => whole + ((afract + mid_round).floor() / dv),
            crate::RoundType::BiasDown => {
                if pfract > low_round && pfract < high_round {
                    return whole + (wfract / dv);
                }
                whole + ((afract + mid_round).floor() / dv)
            }
            crate::RoundType::Up => whole + (afract.ceil() / dv),
            crate::RoundType::Truncate => whole + (wfract / dv),
            _ => value,
        }
    }

    /// Round the value to the nearest fraction.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to round.
    /// * `fraction` - The rounding fraction.
    /// * `round_ctrl` - Round control.
    ///
    /// # Return
    ///
    /// * The rounded value.

    pub fn round_fraction(
        value: Decimal,
        fraction: Decimal,
        round_ctrl: crate::RoundType,
    ) -> Decimal {
        if round_ctrl == crate::RoundType::None {
            return value;
        }

        let low_round = dec!(crate::LOW_ROUND);
        let mid_round = dec!(crate::MID_ROUND);
        let high_round = dec!(crate::HIGH_ROUND);
        let afract: Decimal = value * fraction;
        let wfract: Decimal = afract.floor();
        let pfract: Decimal = afract - wfract;

        match round_ctrl {
            crate::RoundType::Bankers => {
                if pfract > low_round && pfract < high_round {
                    if wfract % dec!(2.0) == dec!(0.0) {
                        return wfract;
                    }
                    return afract.ceil() * fraction;
                }
                (afract + mid_round).floor() * fraction
            }
            crate::RoundType::BiasUp => (afract + mid_round).floor() * fraction,
            crate::RoundType::BiasDown => {
                if pfract > low_round && pfract < high_round {
                    return wfract * fraction;
                }
                (afract + mid_round).floor() * fraction
            }
            crate::RoundType::Up => afract.ceil() * fraction,
            crate::RoundType::Truncate => wfract * fraction,
            _ => value,
        }
    }

    /// Convert a serial number to a date.
    ///
    /// # Arguments
    ///
    /// * `serial` - Number of days since January 1, SERIAL_BASE_YEAR.
    ///
    /// # Return
    ///
    /// * Date in YYYYMMDD format.

    pub fn serial_to_date(serial: usize) -> usize {
        let mut year = (((serial - 1) as f64) / 365.25) as usize;
        let mut day = serial - (((year as f64) * 365.25).ceil() as usize);

        year += crate::SERIAL_BASE_YEAR;
        let leap = CoreUtility::leap_year(year);

        let val1: usize;
        if day > 59 + leap {
            val1 = 93 + day - leap;
        } else {
            val1 = 91 + day;
        }
        let val2 = val1 * 100 / 3055;
        day = val1 - (val2 * 3055 / 100);
        let month = val2 - 2;
        (year * 10000) + (month * 100) + day
    }

    /// Return the number of 'true' bits in the skip mask.
    ///
    /// # Arguments
    ///
    /// * `skip_mask_len` - The length of the skip mask boolean array.
    /// * `skip_mask` - The skip mask boolean array.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn skip_mask_true_bits(skip_mask_len: usize, skip_mask: u128) -> i32 {
        if skip_mask_len == usize::MAX {
            return 0;
        }

        let mut count: i32 = 0;
        for index in 0..skip_mask_len {
            if (skip_mask & (1 << index)) != 0 {
                count += 1;
            }
        }
        count
    }

    /// Return a string representing the skip mask boolean array.
    /// A value of '0' indicates that the period is not skipped
    /// whereas a value of '1' indicates that the period is
    /// skipped. The skip mask is repeated to cover all periods.
    ///
    /// # Arguments
    ///
    /// * `skip_mask_len` - The length of the skip mask boolean array.
    /// * `skip_mask` - The skip mask boolean array.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn skip_mask_to_string(skip_mask_len: usize, skip_mask: u128) -> String {
        if skip_mask_len == usize::MAX {
            return String::from("");
        }
        let mut skip_mask_str = String::from("");
        for index in 0..skip_mask_len {
            skip_mask_str.push(if (skip_mask & (1 << index)) != 0 {
                '1'
            } else {
                '0'
            });
        }
        skip_mask_str
    }

    /// Return a skip mask length and skip mask boolean array from
    /// a string. A value of '0' indicates that the period is not
    /// skipped whereas a value of '1' indicates that the period is
    /// skipped. The skip mask is repeated to cover all periods.
    ///
    /// # Arguments
    ///
    /// * `skip_mask_str` - A string representing the skip mask.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn string_to_skip_mask(skip_mask_str: &str) -> (usize, u128) {
        let skip_mask_str_len = skip_mask_str.len();
        let mut skip_mask: u128 = 0;
        if skip_mask_str_len == usize::MAX {
            return (0, skip_mask);
        }

        let skip_mask_char: Vec<_> = skip_mask_str.chars().collect();

        for (index, elem) in skip_mask_char.iter().enumerate() {
            if *elem == '1' {
                skip_mask |= 1 << index;
            }
        }
        (skip_mask_str_len, skip_mask)
    }

    /// Round the value to the specified number of decimal digits.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to round.
    /// * `decimal_digits` - The number of decimal digits to round.
    ///
    /// # Return
    ///
    /// * The rounded value.

    pub fn util_round(value: Decimal, decimal_digits: usize) -> Decimal {
        CoreUtility::round(value, decimal_digits, crate::RoundType::Bankers)
    }

    /// Round the value to the nearest fraction.
    ///
    /// # Arguments
    ///
    /// * `dblValue` - The value to round.
    /// * `fraction` - The rounding fraction.
    /// * `round_ctrl` - Round control.
    ///
    /// # Return
    ///
    /// * The rounded value.

    pub fn util_round_fraction(
        value: Decimal,
        fraction: Decimal,
        round_ctrl: crate::RoundType,
    ) -> Decimal {
        CoreUtility::round_fraction(value, fraction, round_ctrl)
    }
}
