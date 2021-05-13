//! The AmFn utility methods.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rust_decimal::prelude::*;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

use super::{CalcExpression, CalcManager};
use crate::core::{
    CoreUtility, ElemBalanceResult, ElemColumn, ElemExtension, ElemSymbol, ListAmortization,
    ListColumn, ListDescriptor, ListEvent, ListParameter, ListStatisticHelper, ListSummary,
};
use crate::ListTrait;

pub struct CalcUtility {}

/// The AmFn utility methods implementation.

impl CalcUtility {
    /// Convert a value from the cashflow code to the event code.
    /// Cross rates are used if the exchange rate is unavailable and
    /// the cross rate international currency code is not empty.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to convert.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn convert_currency_event(
        calc_manager_param: &Ref<CalcManager>,
        cashflow_currency_code: &str,
        event_currency_code: &str,
        value: Decimal,
    ) -> Decimal {
        if event_currency_code.is_empty() || event_currency_code == cashflow_currency_code {
            return value;
        }

        calc_manager_param.list_exchange_rate().convert_currency(
            value,
            cashflow_currency_code,
            event_currency_code,
            calc_manager_param.cross_rate_code(true),
        )
    }

    /// Create the event type parameter list.
    ///
    /// # Arguments
    ///
    /// * `calc_manager_param` - Calculation manager.
    /// * `elem_type` - Type of the event.
    /// * `elem_extension` - A ElemPrincipalChange, ElemCurrentValue,
    ///     ElemInterestChange, or ElemStatisticValue element.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn create_event_type_list_parameter(
        calc_manager_param: &Rc<RefCell<CalcManager>>,
        elem_type: crate::ExtensionType,
        elem_extension: &ElemExtension,
    ) -> ListParameter {
        let calc_manager = Rc::clone(calc_manager_param);
        let mut list_parameter = ListParameter::new();
        let updating_json = calc_manager.borrow().updating_json();
        match elem_type {
            crate::ExtensionType::CurrentValue => {
                list_parameter.add_parameter("intEOM", "", "", updating_json);
                list_parameter.set_integer(if elem_extension.cv_eom() { 1 } else { 0 });
                list_parameter.add_parameter("intPassive", "", "", updating_json);
                list_parameter.set_integer(if elem_extension.cv_passive() { 1 } else { 0 });
                list_parameter.add_parameter("intPresent", "", "", updating_json);
                list_parameter.set_integer(if elem_extension.cv_present() { 1 } else { 0 });
            }
            crate::ExtensionType::InterestChange => {
                list_parameter.add_parameter("strMethod", "", "", updating_json);
                list_parameter.set_string(
                    CoreUtility::get_interest_method_mnemonic_short(elem_extension.ic_method())
                        .as_str(),
                );
                list_parameter.add_parameter("strDayCount", "", "", updating_json);
                list_parameter.set_string(
                    CoreUtility::get_day_count_basis_mnemonic_short(
                        elem_extension.ic_day_count_basis(),
                    )
                    .as_str(),
                );
                list_parameter.add_parameter("intDaysInYear", "", "", updating_json);
                list_parameter.set_integer(elem_extension.ic_days_in_year());
                list_parameter.add_parameter("strEffFreq", "", "", updating_json);
                list_parameter.set_string(
                    CoreUtility::get_frequency_mnemonic(elem_extension.ic_effective_frequency())
                        .as_str(),
                );
                list_parameter.add_parameter("strExpFreq", "", "", updating_json);
                list_parameter.set_string(
                    CoreUtility::get_frequency_mnemonic(elem_extension.ic_interest_frequency())
                        .as_str(),
                );
                list_parameter.add_parameter("strRound", "", "", updating_json);
                list_parameter.set_string(
                    CoreUtility::get_round_balance(elem_extension.ic_round_balance()).as_str(),
                );
                list_parameter.add_parameter("decRoundDD", "", "", updating_json);
                list_parameter.set_decimal(elem_extension.ic_round_decimal_digits());
            }
            crate::ExtensionType::StatisticValue => {
                list_parameter.add_parameter("strName", "", "", updating_json);
                list_parameter.set_string(elem_extension.sv_name());
                list_parameter.add_parameter("intEOM", "", "", updating_json);
                list_parameter.set_integer(if elem_extension.sv_eom() { 1 } else { 0 });
                list_parameter.add_parameter("intFinal", "", "", updating_json);
                list_parameter.set_integer(if elem_extension.sv_is_final() { 1 } else { 0 });
            }
            _ => {
                list_parameter.add_parameter("strPrinType", "", "", updating_json);
                list_parameter.set_string(
                    CoreUtility::get_principal_type_mnemonic_short(elem_extension.pc_type())
                        .as_str(),
                );
                list_parameter.add_parameter("intEOM", "", "", updating_json);
                list_parameter.set_integer(if elem_extension.pc_eom() { 1 } else { 0 });
                list_parameter.add_parameter("intPrinFirst", "", "", updating_json);
                list_parameter.set_integer(if elem_extension.pc_principal_first() {
                    1
                } else {
                    0
                });
                list_parameter.add_parameter("intBalStats", "", "", updating_json);
                list_parameter.set_integer(if elem_extension.pc_balance_statistics() {
                    1
                } else {
                    0
                });
                list_parameter.add_parameter("intAuxiliary", "", "", updating_json);
                list_parameter.set_integer(if elem_extension.pc_auxiliary() { 1 } else { 0 });
                list_parameter.add_parameter("intPassive", "", "", updating_json);
                list_parameter.set_integer(if elem_extension.pc_aux_passive() {
                    1
                } else {
                    0
                });
            }
        }
        list_parameter
    }

    /// Evaluate all of the descriptors in the list.
    /// For each descriptor that specifies an expression,
    /// execute the expression using the list of parameters.
    ///
    /// # Arguments
    ///
    /// * `calc_manager_param` - Calculation manager.
    /// * `expression` - CalcExpression element.
    /// * `list_parameter` - List of parameters used with evaluation.
    /// * `list_descriptor` - List of descriptors to evaluate.

    pub fn evaluate_descriptors(
        calc_manager_param: &Rc<RefCell<CalcManager>>,
        expression: &RefCell<CalcExpression>,
        list_parameter: &ListParameter,
        list_descriptor: &ListDescriptor,
    ) {
        let calc_manager = Rc::clone(calc_manager_param);
        let calc_mgr = calc_manager.borrow();

        let orig_index = list_descriptor.index();
        let mut index: usize = 0;

        loop {
            if !list_descriptor.get_element(index) {
                break;
            }

            if list_descriptor.desc_type() == crate::TYPE_LOCALE {
                calc_mgr
                    .list_locale()
                    .select_event_locale(list_descriptor.code());
            }

            if list_descriptor.value_expr().is_empty() {
                index += 1;
                continue;
            }

            let mut list_descriptor_cashflow: Option<&ListDescriptor> = None;
            match calc_mgr.list_cashflow().preferences().as_ref() {
                None => {}
                Some(o) => {
                    list_descriptor_cashflow = Option::from(o.list_descriptor());
                }
            }
            expression.borrow_mut().init_expression(
                list_descriptor_cashflow,
                None,
                Option::from(list_parameter),
                list_descriptor.value_expr().as_str(),
            );

            let elem_result_symbol: ElemSymbol;
            let result = expression.borrow().evaluate(
                calc_mgr.list_cashflow().list_amortization(),
                calc_mgr.list_cashflow().elem_balance_result(),
            );
            match result {
                Err(e) => {
                    let error_string = calc_mgr.get_error_string(e);
                    list_descriptor
                        .set_value(format!("{}{}", crate::ERROR_PREFIX, error_string).as_str());
                    calc_mgr.list_locale().select_event_locale("");
                    index += 1;
                    continue;
                }
                Ok(o) => {
                    elem_result_symbol = o;
                }
            }

            let value: String;
            match elem_result_symbol.sym_type() {
                crate::TokenType::Integer => {
                    value = format!("{}", elem_result_symbol.sym_integer());
                }
                crate::TokenType::Decimal => {
                    value = format!("{}", elem_result_symbol.sym_decimal());
                }
                crate::TokenType::String => {
                    value = String::from(elem_result_symbol.sym_string());
                }
                _ => {
                    value = String::from("");
                }
            }
            list_descriptor.set_value(value.as_str());
            calc_mgr.list_locale().select_event_locale("");

            index += 1;
        }

        list_descriptor.set_index(orig_index);
    }

    /// Evaluate the expression.
    ///
    /// # Arguments
    ///
    /// * `calc_manager_param` - Calculation manager.
    /// * `list_parameter` - List of parameters used with evaluation.
    /// * `expression_str` - The expression to evaluate.
    /// * `cashflow` - Search the cashflow preferences.
    ///
    /// # Return
    ///
    /// * Resulting symbol if successful, otherwise an error
    ///     message in the symbol.

    pub fn evaluate_expression(
        calc_manager_param: &Rc<RefCell<CalcManager>>,
        list_parameter: Option<&ListParameter>,
        expression_str: &str,
        cashflow: bool,
    ) -> ElemSymbol {
        let calc_manager = Rc::clone(calc_manager_param);
        let calc_mgr = calc_manager.borrow();
        let mut list_descriptor_cashflow: Option<&ListDescriptor> = None;

        match calc_mgr.list_cashflow().preferences().as_ref() {
            None => {}
            Some(o) => {
                list_descriptor_cashflow = Option::from(o.list_descriptor());
            }
        }

        let mut expression = CalcExpression::new(
            &calc_manager,
            calc_mgr.fiscal_year_start(cashflow),
            calc_mgr.decimal_digits(cashflow),
        );

        expression.init_expression(
            list_descriptor_cashflow,
            None,
            list_parameter,
            expression_str,
        );

        let mut elem_result_symbol: ElemSymbol;
        let result = expression.evaluate(
            calc_mgr.list_cashflow().list_amortization(),
            calc_mgr.list_cashflow().elem_balance_result(),
        );
        match result {
            Err(e) => {
                elem_result_symbol = ElemSymbol::new();
                let error_string = calc_mgr.get_error_string(e);
                elem_result_symbol
                    .set_string(format!("{}{}", crate::ERROR_PREFIX, error_string).as_str());
            }
            Ok(o) => {
                elem_result_symbol = o;
            }
        }

        elem_result_symbol
    }

    /// Get the appropriate event list value as a string.
    ///
    /// # Arguments
    ///
    /// * `calc_manager_param` - Calculation manager.
    /// * `elem_column` - Column element.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn get_event_value(
        calc_manager_param: &Rc<RefCell<CalcManager>>,
        elem_column: &ElemColumn,
    ) -> String {
        let calc_manager = Rc::clone(calc_manager_param);
        let calc_mgr = calc_manager.borrow();
        let decimal_digits = calc_mgr.decimal_digits(true);
        let list_locale = calc_mgr.list_locale();
        let list_cashflow = calc_mgr.list_cashflow();
        let list_event_opt = list_cashflow.list_event();
        let mut result = String::from("");

        let list_event: &ListEvent;
        match list_event_opt.as_ref() {
            None => {
                return result;
            }
            Some(o) => {
                list_event = o;
            }
        }

        let orig_list_index = list_event.index();
        list_locale.select_event_locale("");
        if elem_column.col_type() == crate::TYPE_LOCALE && !elem_column.code().is_empty() {
            list_locale.select_event_locale(elem_column.code());
        }
        match CoreUtility::get_col_name(elem_column.col_name_index()) {
            crate::ColumnType::None => {
                let list_descriptor_opt = list_event.list_descriptor();
                match list_descriptor_opt.as_ref() {
                    None => {
                        return result;
                    }
                    Some(o) => {
                        if o.get_element_by_name(
                            elem_column.group(),
                            elem_column.name(),
                            elem_column.col_type(),
                            elem_column.code(),
                            true,
                        ) {
                            match elem_column.format() {
                                crate::FormatType::Date => match o.value().parse::<usize>() {
                                    Err(_e) => {}
                                    Ok(o2) => {
                                        result = list_locale.format_date_out(o2);
                                    }
                                },
                                crate::FormatType::Integer => match o.value().parse::<i32>() {
                                    Err(_e) => {}
                                    Ok(o2) => {
                                        result = list_locale.format_integeri_out(o2);
                                    }
                                },
                                crate::FormatType::Decimal => match o.value().parse::<Decimal>() {
                                    Err(_e) => {}
                                    Ok(o2) => {
                                        result = list_locale.format_decimal_out(o2);
                                    }
                                },
                                crate::FormatType::Currency => match o.value().parse::<Decimal>() {
                                    Err(_e) => {}
                                    Ok(o2) => {
                                        result =
                                            list_locale.format_currency_out(o2, decimal_digits);
                                    }
                                },
                                _ => {
                                    result = String::from(o.value().as_str());
                                }
                            }
                        }
                    }
                }
            }
            crate::ColumnType::EventType => {
                result = String::from(list_event.event_type());
            }
            crate::ColumnType::Date => {
                result = list_locale.format_date_out(list_event.event_date());
            }
            crate::ColumnType::DateExpr => {
                if !list_event.date_expr().is_empty() {
                    result = String::from(list_event.date_expr());
                }
            }
            crate::ColumnType::Sort => {
                result = list_event.sort_order().to_string();
            }
            crate::ColumnType::Value => match list_event.elem_type() {
                crate::ExtensionType::PrincipalChange => {
                    result = list_locale.format_currency_out(list_event.value(), decimal_digits);
                }
                crate::ExtensionType::InterestChange => {
                    result = list_locale.format_decimal_out(list_event.value());
                }
                _ => {}
            },
            crate::ColumnType::ValueExpr => {
                if !list_event.value_expr().is_empty() {
                    result = String::from(list_event.value_expr());
                }
            }
            crate::ColumnType::Periods => {
                if list_event.elem_type() != crate::ExtensionType::StatisticValue {
                    result = format!("{}", list_event.periods());
                }
            }
            crate::ColumnType::PeriodsExpr => {
                if !list_event.periods_expr().is_empty() {
                    result = String::from(list_event.periods_expr());
                }
            }
            crate::ColumnType::SkipPeriods => {
                if list_event.skip_mask_len() > 0 {
                    result = CoreUtility::skip_mask_to_string(
                        list_event.skip_mask_len(),
                        list_event.skip_mask(),
                    );
                }
            }
            crate::ColumnType::Intervals => {
                result = format!("{}", list_event.intervals());
            }
            crate::ColumnType::Frequency => {
                calc_manager
                    .borrow()
                    .core_manager()
                    .map_frequency()
                    .get_element_by_value(list_event.frequency() as usize);
                result =
                    String::from(list_locale.get_resource(
                        calc_mgr.core_manager().map_frequency().key()));
            }
            crate::ColumnType::EndDate => {
                if list_event.periods() > 1 {
                    let val = CalcManager::util_date_new(
                        list_event.event_date(),
                        list_event.periods() - 1,
                        list_event.frequency(),
                        list_event.intervals(),
                        list_event.elem_extension().extension_eom(),
                    );
                    result = list_locale.format_date_out(val);
                }
            }
            crate::ColumnType::ParameterList => match list_event.list_parameter().as_ref() {
                None => {
                    result = String::from("");
                }
                Some(o) => {
                    if o.count() > 0 {
                        result = format!("{}", o.count());
                    }
                }
            },
            crate::ColumnType::DescriptorList => match list_event.list_descriptor().as_ref() {
                None => {
                    result = String::from("");
                }
                Some(o) => {
                    if o.count() > 0 {
                        result = format!("{}", o.count());
                    }
                }
            },
            crate::ColumnType::EventName => {
                if !list_event.event_name().is_empty() {
                    result = String::from(list_event.event_name());
                }
            }
            crate::ColumnType::NextName => {
                if !list_event.next_name().is_empty() {
                    result = String::from(list_event.next_name());
                }
            }
            _ => {}
        }
        list_event.get_element(orig_list_index);
        list_locale.select_event_locale("");
        result
    }

    /// Get the appropriate amortization list value as a string.
    ///
    /// # Arguments
    ///
    /// * `calc_manager_param` - Calculation manager.
    /// * `elem_column` - Column element.
    /// * `list_am_opt` - Amortization list.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn get_am_value(
        calc_manager_param: &Rc<RefCell<CalcManager>>,
        elem_column: &ElemColumn,
        list_am: &ListAmortization,
    ) -> String {
        let calc_manager = Rc::clone(calc_manager_param);
        let calc_mgr = calc_manager.borrow();
        let decimal_digits = calc_mgr.decimal_digits(true);
        let list_locale = calc_mgr.list_locale();
        let cashflow_currency_code = String::from(list_locale.cashflow_currency_code());
        let event_currency_code = String::from(list_locale.event_currency_code());
        let list_cashflow = calc_mgr.list_cashflow();
        let elem_balance_result_opt = list_cashflow.elem_balance_result();

        let elem_balance_result: &ElemBalanceResult;
        match elem_balance_result_opt.as_ref() {
            None => {
                return String::from("");
            }
            Some(o) => {
                elem_balance_result = o;
            }
        }

        let orig_list_index = list_am.index();
        if orig_list_index == usize::MAX {
            return String::from("");
        }

        list_locale.select_event_locale("");
        if elem_column.col_type() == crate::TYPE_LOCALE && !elem_column.code().is_empty() {
            list_locale.select_event_locale(elem_column.code());
        }

        let mut result = String::from("");
        match CoreUtility::get_col_name(elem_column.col_name_index()) {
            crate::ColumnType::None => {
                let list_descriptor_opt = list_am.list_descriptor();
                match list_descriptor_opt.as_ref() {
                    None => {}
                    Some(o) => {
                        if o.get_element_by_name(
                            elem_column.group(),
                            elem_column.name(),
                            elem_column.col_type(),
                            elem_column.code(),
                            true,
                        ) {
                            match elem_column.format() {
                                crate::FormatType::Date => match o.value().parse::<usize>() {
                                    Err(_e) => {}
                                    Ok(o2) => {
                                        result = list_locale.format_date_out(o2);
                                    }
                                },
                                crate::FormatType::Integer => match o.value().parse::<i32>() {
                                    Err(_e) => {}
                                    Ok(o2) => {
                                        result = list_locale.format_integeri_out(o2);
                                    }
                                },
                                crate::FormatType::Decimal => match o.value().parse::<Decimal>() {
                                    Err(_e) => {}
                                    Ok(o2) => {
                                        result = list_locale.format_decimal_out(o2);
                                    }
                                },
                                crate::FormatType::Currency => match o.value().parse::<Decimal>() {
                                    Err(_e) => {}
                                    Ok(o2) => {
                                        result = list_locale.format_currency_out(
                                            CalcUtility::convert_currency_event(
                                                &calc_mgr,
                                                cashflow_currency_code.as_str(),
                                                event_currency_code.as_str(),
                                                o2,
                                            ),
                                            decimal_digits,
                                        );
                                    }
                                },
                                _ => {
                                    result = String::from(o.value().as_str());
                                }
                            }
                        }
                    }
                }
            }
            crate::ColumnType::Sequence => {
                let val = list_am.stat_sequence();
                if val > 0 {
                    result = format!("{}", val);
                }
            }
            crate::ColumnType::EventType => {
                result = String::from(list_am.event_type());
            }
            crate::ColumnType::Date => {
                result = list_locale.format_date_out(list_am.event_date());
            }
            crate::ColumnType::Sort => {
                result = format!("{}", list_am.sort_order());
            }
            crate::ColumnType::Value => match list_am.elem_type() {
                crate::ExtensionType::PrincipalChange => {
                    result = list_locale.format_currency_out(
                        CalcUtility::convert_currency_event(
                            &calc_mgr,
                            cashflow_currency_code.as_str(),
                            event_currency_code.as_str(),
                            list_am.value(),
                        ),
                        decimal_digits,
                    );
                }
                crate::ExtensionType::InterestChange => {
                    result = list_locale.format_decimal_out(list_am.value());
                }
                _ => {}
            },
            crate::ColumnType::Periods => {
                result = format!("{}", list_am.periods());
            }
            crate::ColumnType::Decrease => {
                if list_am.principal_decrease() > dec!(0.0) {
                    result = list_locale.format_currency_out(
                        CalcUtility::convert_currency_event(
                            &calc_mgr,
                            cashflow_currency_code.as_str(),
                            event_currency_code.as_str(),
                            list_am.principal_decrease(),
                        ),
                        decimal_digits,
                    );
                }
            }
            crate::ColumnType::Increase => {
                if list_am.principal_increase() > dec!(0.0) {
                    result = list_locale.format_currency_out(
                        CalcUtility::convert_currency_event(
                            &calc_mgr,
                            cashflow_currency_code.as_str(),
                            event_currency_code.as_str(),
                            list_am.principal_increase(),
                        ),
                        decimal_digits,
                    );
                }
            }
            crate::ColumnType::Intervals => {
                result = format!("{}", list_am.intervals());
            }
            crate::ColumnType::Frequency => {
                calc_manager
                    .borrow()
                    .core_manager()
                    .map_frequency()
                    .get_element_by_value(list_am.frequency() as usize);
                result =
                    String::from(list_locale.get_resource(
                        calc_mgr.core_manager().map_frequency().key()));
            }
            crate::ColumnType::ParameterList => match list_am.list_parameter().as_ref() {
                None => {
                    result = String::from("");
                }
                Some(o) => {
                    if o.count() > 0 {
                        result = format!("{}", o.count());
                    }
                }
            },
            crate::ColumnType::DescriptorList => match list_am.list_descriptor().as_ref() {
                None => {
                    result = String::from("");
                }
                Some(o) => {
                    if o.count() > 0 {
                        result = format!("{}", o.count());
                    }
                }
            },
            crate::ColumnType::Interest => {
                if !(list_am.interest() == dec!(0.0)
                    && list_am.elem_type() == crate::ExtensionType::StatisticValue)
                {
                    result = list_locale.format_currency_out(
                        CalcUtility::convert_currency_event(
                            &calc_mgr,
                            cashflow_currency_code.as_str(),
                            event_currency_code.as_str(),
                            list_am.interest(),
                        ),
                        decimal_digits,
                    );
                }
            }
            crate::ColumnType::SlInterest => {
                if !(list_am.sl_interest() == dec!(0.0)
                    && list_am.elem_type() == crate::ExtensionType::StatisticValue)
                {
                    result = list_locale.format_currency_out(
                        CalcUtility::convert_currency_event(
                            &calc_mgr,
                            cashflow_currency_code.as_str(),
                            event_currency_code.as_str(),
                            list_am.sl_interest(),
                        ),
                        decimal_digits,
                    );
                }
            }
            crate::ColumnType::IntOnInterest => {
                if !(list_am.interest() - list_am.sl_interest() == dec!(0.0)
                    && list_am.elem_type() == crate::ExtensionType::StatisticValue)
                {
                    result = list_locale.format_currency_out(
                        CalcUtility::convert_currency_event(
                            &calc_mgr,
                            cashflow_currency_code.as_str(),
                            event_currency_code.as_str(),
                            list_am.interest() - list_am.sl_interest(),
                        ),
                        decimal_digits,
                    );
                }
            }
            crate::ColumnType::ValueToInterest => {
                if !(list_am.value_to_interest() == dec!(0.0)
                    && list_am.elem_type() == crate::ExtensionType::StatisticValue)
                {
                    result = list_locale.format_currency_out(
                        CalcUtility::convert_currency_event(
                            &calc_mgr,
                            cashflow_currency_code.as_str(),
                            event_currency_code.as_str(),
                            list_am.value_to_interest(),
                        ),
                        decimal_digits,
                    );
                }
            }
            crate::ColumnType::ValueToPrincipal => {
                if !(list_am.value_to_principal() == dec!(0.0)
                    && list_am.elem_type() == crate::ExtensionType::StatisticValue)
                {
                    result = list_locale.format_currency_out(
                        CalcUtility::convert_currency_event(
                            &calc_mgr,
                            cashflow_currency_code.as_str(),
                            event_currency_code.as_str(),
                            list_am.value_to_principal(),
                        ),
                        decimal_digits,
                    );
                }
            }
            crate::ColumnType::AccruedBalance => {
                if !(list_am.acc_balance() == dec!(0.0)
                    && list_am.elem_type() == crate::ExtensionType::StatisticValue)
                {
                    result = list_locale.format_currency_out(
                        CalcUtility::convert_currency_event(
                            &calc_mgr,
                            cashflow_currency_code.as_str(),
                            event_currency_code.as_str(),
                            list_am.acc_balance(),
                        ),
                        decimal_digits,
                    );
                }
            }
            crate::ColumnType::Balance => {
                if !(list_am.balance() == dec!(0.0)
                    && list_am.elem_type() == crate::ExtensionType::StatisticValue)
                {
                    let balance = CoreUtility::round(
                        list_am.balance(),
                        elem_column.decimal_digits(),
                        crate::RoundType::Bankers,
                    );
                    if elem_balance_result.polarity() < 0 {
                        if balance > dec!(0.0) {
                            result = format!(
                                "+{}",
                                list_locale.format_currency_out(
                                    CalcUtility::convert_currency_event(
                                        &calc_mgr,
                                        cashflow_currency_code.as_str(),
                                        event_currency_code.as_str(),
                                        balance
                                    ),
                                    decimal_digits
                                )
                            );
                        } else {
                            result = list_locale.format_currency_out(
                                CalcUtility::convert_currency_event(
                                    &calc_mgr,
                                    cashflow_currency_code.as_str(),
                                    event_currency_code.as_str(),
                                    balance.abs(),
                                ),
                                decimal_digits,
                            );
                        }
                    } else {
                        result = list_locale.format_currency_out(
                            CalcUtility::convert_currency_event(
                                &calc_mgr,
                                cashflow_currency_code.as_str(),
                                event_currency_code.as_str(),
                                balance,
                            ),
                            decimal_digits,
                        );
                    }
                }
            }
            _ => {}
        }
        list_locale.select_event_locale("");
        list_am.get_element(orig_list_index);
        result
    }

    /// Normalize the expression.
    ///
    /// # Arguments
    ///
    /// * `calc_manager_param` - Calculation manager.
    /// * `expression` - The expression to normalize.
    /// * `new_line` - If true, format with a newline character between expressions.
    ///
    /// # Return
    ///
    /// * Normalized expression.

    pub fn normalize_expression(
        calc_manager_param: &Rc<RefCell<CalcManager>>,
        expression: &str,
        new_line: bool,
    ) -> String {
        let calc_manager = Rc::clone(calc_manager_param);

        let mut calc_expression = CalcExpression::new(
            &calc_manager,
            calc_manager.borrow().fiscal_year_start(false),
            calc_manager.borrow().decimal_digits(false),
        );

        calc_expression.init_expression(None, None, None, expression);
        calc_expression.normalize_expression(new_line)
    }

    /// Create and return a column list object.
    ///
    /// # Arguments
    ///
    /// * `calc_manager_param` - Calculation manager.
    /// * `elem_type` - The type of table.
    /// * `cashflow` - Search the cashflow preferences.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn parse_columns(
        calc_manager_param: &Rc<RefCell<CalcManager>>,
        elem_type: crate::TableType,
        cashflow: bool,
    ) -> ListColumn {
        let calc_manager = Rc::clone(calc_manager_param);
        let calc_mgr = calc_manager.borrow();
        let list_locale = calc_mgr.list_locale();

        let mut group = String::from(if !cashflow {
            crate::GROUP_TEMPLATE
        } else if elem_type == crate::TableType::Amortization {
            crate::GROUP_AM
        } else {
            crate::GROUP_EVENT
        });

        let locale_str: &str;
        if cashflow {
            locale_str = list_locale.cashflow_locale().locale_str();
        } else {
            locale_str = list_locale.user_locale().locale_str();
        }

        let mut columns = calc_manager.borrow().descriptor_value(
            group.as_str(),
            crate::NAME_COLUMNS,
            crate::TYPE_LOCALE,
            locale_str,
            true,
            false,
        );

        if columns.is_empty() {
            columns = calc_manager.borrow().descriptor_value(
                group.as_str(),
                crate::NAME_COLUMNS,
                crate::TYPE_CUSTOM,
                "",
                true,
                false,
            );
            if columns.is_empty() {
                columns = String::from(if !cashflow {
                    crate::DEFAULT_TEMPLATE_COLUMNS
                } else if elem_type == crate::TableType::Amortization {
                    crate::DEFAULT_AM_COLUMNS
                } else {
                    crate::DEFAULT_EVENT_COLUMNS
                });
            }
        }

        let mut list_column = ListColumn::new();
        for column in columns.split('|') {
            let text = column.trim();
            let mut col_name = CoreUtility::parse_token1(text);
            let mut width = CoreUtility::parse_integer(CoreUtility::parse_token2(text));

            if width < crate::MINIMUM_COLUMN_WIDTH {
                width = crate::MINIMUM_COLUMN_WIDTH;
            }

            group = String::from("");

            let mut col_header: String;
            let mut col_description = String::from("");
            let mut name = String::from("");
            let mut elem_type = String::from("");
            let mut code = String::from("");
            let mut format = crate::FormatType::String;
            let mut col_name_index: usize = 0;
            let mut decimal_digits = calc_manager.borrow().decimal_digits(cashflow);
            let mut column_editable: bool = false;
            let mut desc_text: String;

            if calc_manager
                .borrow()
                .core_manager()
                .map_col_names()
                .get_element_by_key(col_name)
            {
                col_name_index = calc_manager.borrow().core_manager().map_col_names().value();

                col_header = calc_manager.borrow().descriptor_value(
                    crate::GROUP_COLHEADER,
                    col_name,
                    crate::TYPE_LOCALE,
                    locale_str,
                    true,
                    false,
                );

                if col_header.is_empty() {
                    col_header = calc_manager.borrow().descriptor_value(
                        crate::GROUP_COLHEADER,
                        col_name,
                        crate::TYPE_CUSTOM,
                        "",
                        true,
                        false,
                    );
                    if col_header.is_empty() {
                        col_header = String::from(
                            list_locale.get_resource(
                                CalcManager::col_name_resource_key(CoreUtility::get_col_name(
                                    col_name_index,
                                ))
                                .as_str(),
                            ),
                        );
                    }
                }

                column_editable = (calc_manager.borrow().core_manager().map_col_names().value_ext()
                    & crate::MAPCOLNAMES_EDITABLE)
                    != 0;
            } else {
                desc_text = calc_manager.borrow().descriptor_value(
                    crate::GROUP_COLVALUE,
                    col_name,
                    crate::TYPE_LOCALE,
                    locale_str,
                    true,
                    false,
                );
                if desc_text.is_empty() {
                    desc_text = calc_manager.borrow().descriptor_value(
                        crate::GROUP_COLVALUE,
                        col_name,
                        crate::TYPE_CUSTOM,
                        "",
                        true,
                        false,
                    );
                    if desc_text.is_empty() {
                        continue;
                    }
                }

                let tokens: Vec<_> = desc_text.split('~').collect();
                if tokens.len() != 3 && tokens.len() != 6 {
                    continue;
                }

                if tokens.len() == 3 {
                    col_name = tokens[0].trim();
                    elem_type = crate::TYPE_LOCALE.to_string();
                    code = String::from(tokens[1].trim());
                    decimal_digits = CoreUtility::parse_integer(tokens[2].trim());

                    if !calc_manager
                        .borrow()
                        .core_manager()
                        .map_col_names()
                        .get_element_by_key(col_name)
                    {
                        continue;
                    }

                    col_name_index = calc_manager.borrow().core_manager().map_col_names().value();

                    col_header = calc_manager.borrow().descriptor_value(
                        crate::GROUP_COLHEADER,
                        col_name,
                        elem_type.as_str(),
                        code.as_str(),
                        true,
                        false,
                    );
                    if col_header.is_empty() {
                        col_header = calc_manager.borrow().descriptor_value(
                            crate::GROUP_COLHEADER,
                            col_name,
                            crate::TYPE_CUSTOM,
                            "",
                            true,
                            false,
                        );
                        if col_header.is_empty() {
                            calc_mgr.list_locale().select_event_locale(code.as_str());
                            col_header = String::from(
                                list_locale.get_resource(
                                    CalcManager::col_name_resource_key(CoreUtility::get_col_name(
                                        col_name_index,
                                    ))
                                    .as_str(),
                                ),
                            );
                            calc_mgr.list_locale().select_event_locale("");
                        }
                    }
                } else {
                    group = String::from(tokens[0].trim());
                    name = String::from(tokens[1].trim());
                    elem_type = String::from(tokens[2].trim());
                    code = String::from(tokens[3].trim());
                    format = CoreUtility::get_format(CoreUtility::parse_integer(tokens[4].trim()));
                    decimal_digits = CoreUtility::parse_integer(tokens[5].trim());

                    col_header = calc_manager.borrow().descriptor_value(
                        crate::GROUP_COLHEADER,
                        col_name,
                        elem_type.as_str(),
                        code.as_str(),
                        true,
                        false,
                    );
                    if !col_header.is_empty() {
                        col_header = calc_manager.borrow().descriptor_value(
                            crate::GROUP_COLHEADER,
                            col_name,
                            crate::TYPE_CUSTOM,
                            "",
                            true,
                            false,
                        );
                    }
                }
            }

            let col_header_str = col_header;

            let tokens: Vec<_> = col_header_str.split('~').collect();
            if tokens.is_empty() {
                continue;
            }

            col_header = String::from(tokens[0].trim());
            if tokens.len() > 1 {
                col_description = String::from(tokens[1].trim());
            }
            if col_header.is_empty() {
                col_header = String::from(col_name);
            }
            let col = CoreUtility::get_col_name(col_name_index);
            match col {
                crate::ColumnType::Sequence
                | crate::ColumnType::Sort
                | crate::ColumnType::Value
                | crate::ColumnType::Decrease
                | crate::ColumnType::Increase
                | crate::ColumnType::Periods
                | crate::ColumnType::SkipPeriods
                | crate::ColumnType::Intervals
                | crate::ColumnType::ParameterList
                | crate::ColumnType::DescriptorList
                | crate::ColumnType::Interest
                | crate::ColumnType::SlInterest
                | crate::ColumnType::IntOnInterest
                | crate::ColumnType::ValueToInterest
                | crate::ColumnType::ValueToPrincipal
                | crate::ColumnType::AccruedBalance
                | crate::ColumnType::Balance => {
                    format = crate::FormatType::Currency;
                }
                crate::ColumnType::EndDate | crate::ColumnType::Date => {
                    format = crate::FormatType::Date;
                }
                _ => {}
            }

            list_column.add_column(
                col_name,
                col_name_index,
                col_header.as_str(),
                col_description.as_str(),
                group.as_str(),
                name.as_str(),
                elem_type.as_str(),
                code.as_str(),
                format,
                decimal_digits,
                width,
                column_editable,
            );
        }
        list_column
    }

    /// Create and return a summary list object.
    ///
    /// # Arguments
    ///
    /// * `calc_manager_param` - Calculation manager.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn parse_summary(calc_manager_param: &Rc<RefCell<CalcManager>>) -> ListSummary {
        let calc_manager = Rc::clone(calc_manager_param);
        let calc_mgr = calc_manager.borrow();
        let list_locale = calc_mgr.list_locale();
        let list_cashflow = calc_mgr.list_cashflow();
        let preferences = list_cashflow.preferences();

        let mut list_summary = ListSummary::new();
        let locale_str = list_locale.cashflow_locale().locale_str();

        let list_parameter: &ListParameter;
        let list_descriptor: &ListDescriptor;
        match preferences.as_ref() {
            None => {
                return list_summary;
            }
            Some(o) => {
                list_parameter = o.list_parameter();
                list_descriptor = o.list_descriptor();
            }
        }

        let mut calc_expression = CalcExpression::new(
            &calc_manager,
            calc_mgr.fiscal_year_start(true),
            calc_mgr.decimal_digits(true),
        );

        let mut last_yield: Decimal;
        match list_cashflow.elem_balance_result() {
            None => {
                last_yield = dec!(0.0);
            }
            Some(o) => {
                last_yield = o.result_yield();
            }
        }

        if last_yield == dec!(0.0) {
            let list_event: &ListEvent;
            match list_cashflow.list_event() {
                None => {
                    return list_summary;
                }
                Some(o) => {
                    list_event = o;
                }
            }

            let mut list_am: ListAmortization;
            match list_cashflow.list_amortization() {
                None => {
                    return list_summary;
                }
                Some(o) => {
                    list_am = o.copy(true);
                }
            }

            let mut list_stat_helper: ListStatisticHelper;
            match list_cashflow.list_statistic_helper() {
                None => {
                    return list_summary;
                }
                Some(o) => {
                    list_stat_helper = o.copy();
                }
            }

            match list_cashflow.calculate().calculate_yield(
                list_event,
                &mut list_am,
                &mut list_stat_helper,
                dec!(0.0),
            ) {
                Err(_e) => {
                    last_yield = dec!(0.0);
                }
                Ok(o) => {
                    last_yield = o.result_yield();
                }
            }

            if last_yield > dec!(0.0) {
                match list_cashflow.elem_balance_result() {
                    None => {}
                    Some(o) => {
                        o.set_result_yield(last_yield);
                    }
                }
            }
        }

        calc_expression.set_symbol_decimal("decYield", last_yield);

        let mut summary = calc_mgr.descriptor_value(
            crate::GROUP_GENERAL,
            crate::NAME_SUMMARY,
            crate::TYPE_LOCALE,
            locale_str,
            true,
            false,
        );

        if summary.is_empty() {
            summary = calc_mgr.descriptor_value(
                crate::GROUP_GENERAL,
                crate::NAME_SUMMARY,
                crate::TYPE_CUSTOM,
                "",
                true,
                false,
            );
            if summary.is_empty() {
                return list_summary;
            }
        }

        for summary in summary.split('|') {
            let name = summary.trim();

            let mut text = calc_mgr.descriptor_value(
                crate::GROUP_SUMMARY,
                name,
                crate::TYPE_LOCALE,
                locale_str,
                true,
                false,
            );

            if text.is_empty() {
                text = calc_mgr.descriptor_value(
                    crate::GROUP_SUMMARY,
                    name,
                    crate::TYPE_CUSTOM,
                    "",
                    true,
                    false,
                );
                if text.is_empty() {
                    continue;
                }
            }

            let tokens: Vec<_> = text.split('~').collect();
            let label_expr: String;
            let result_expr: String;

            if tokens.len() < 2 {
                label_expr = String::from(name);
                result_expr = String::from(text.as_str());
            } else {
                label_expr = String::from(tokens[0].trim());
                result_expr = String::from(tokens[1].trim());
            }
            calc_expression.init_expression(
                Option::from(list_descriptor),
                None,
                Option::from(list_parameter),
                label_expr.as_str(),
            );

            let mut label_str = String::from("");

            let mut elem_result_symbol: ElemSymbol;
            let result = calc_expression.evaluate(
                list_cashflow.list_amortization(),
                list_cashflow.elem_balance_result(),
            );

            match result {
                Err(e) => {
                    let error_string = calc_mgr.get_error_string(e);
                    label_str = format!("{}{}", crate::ERROR_PREFIX, error_string);
                }
                Ok(o) => {
                    elem_result_symbol = o;

                    match elem_result_symbol.sym_type() {
                        crate::TokenType::Integer => {
                            label_str = elem_result_symbol.sym_integer().to_string();
                        }
                        crate::TokenType::Decimal => {
                            label_str =
                                list_locale.format_decimal_out(elem_result_symbol.sym_decimal());
                        }
                        crate::TokenType::String => {
                            label_str = String::from(elem_result_symbol.sym_string());
                        }
                        _ => {}
                    }
                }
            }
            if label_str.is_empty() {
                continue; // Omit summary item
            }

            calc_expression.init_expression(
                Option::from(list_descriptor),
                None,
                Option::from(list_parameter),
                result_expr.as_str(),
            );

            let mut result_str = String::from("");
            let result = calc_expression.evaluate(
                list_cashflow.list_amortization(),
                list_cashflow.elem_balance_result(),
            );
            match result {
                Err(e) => {
                    let error_string = calc_mgr.get_error_string(e);
                    result_str = format!("{}{}", crate::ERROR_PREFIX, error_string);
                }
                Ok(o) => {
                    elem_result_symbol = o;

                    match elem_result_symbol.sym_type() {
                        crate::TokenType::Integer => {
                            result_str = elem_result_symbol.sym_integer().to_string();
                        }
                        crate::TokenType::Decimal => {
                            result_str =
                                list_locale.format_decimal_out(elem_result_symbol.sym_decimal());
                        }
                        crate::TokenType::String => {
                            result_str = String::from(elem_result_symbol.sym_string());
                        }
                        _ => {}
                    }
                }
            }
            list_summary.add_summary(
                name,
                label_str.as_str(),
                label_expr.as_str(),
                result_str.as_str(),
                result_expr.as_str(),
            );
        }

        list_summary
    }

    /// Set the appropriate event list value and
    /// return it as a string.
    ///
    /// # Arguments
    ///
    /// * `calc_manager_param` - Calculation manager.
    /// * `col_name_index` - Column name index.
    /// * `col_type` - Column type.
    /// * `col_code` - Column code.
    /// * `index` - Event row index.
    /// * `value_param` - Value to set as a string.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn set_event_value(
        calc_manager_param: &Rc<RefCell<CalcManager>>,
        col_name_index: usize,
        col_type: &str,
        col_code: &str,
        index: usize,
        value_param: &str,
    ) -> String {
        let calc_manager = Rc::clone(calc_manager_param);
        let mut result = String::from("");

        {
            let calc_mgr = calc_manager.borrow();
            let list_locale = calc_mgr.list_locale();
            let list_cashflow = calc_mgr.list_cashflow();
            let list_event_opt = list_cashflow.list_event();

            let list_event: &ListEvent;
            match list_event_opt.as_ref() {
                None => {
                    return result;
                }
                Some(o) => {
                    list_event = o;
                }
            }

            if !list_event.get_element(index) {
                return result;
            }

            list_locale.select_event_locale("");
            if col_type == crate::TYPE_LOCALE && !col_code.is_empty() {
                list_locale.select_event_locale(col_code);
            }

            match CoreUtility::get_col_name(col_name_index) {
                crate::ColumnType::Date => {
                    result = list_locale.format_date_in(value_param);
                }
                crate::ColumnType::DateExpr => {
                    result = String::from(value_param);
                }
                crate::ColumnType::Sort => {
                    result = list_locale.format_integer_in(value_param);
                }
                crate::ColumnType::Value => match list_event.elem_type() {
                    crate::ExtensionType::PrincipalChange => {
                        result = list_locale.format_currency_in(value_param);
                    }
                    crate::ExtensionType::InterestChange => {
                        result = list_locale.format_decimal_in(value_param);
                    }
                    _ => {}
                },
                crate::ColumnType::ValueExpr => {
                    result = String::from(value_param);
                }
                crate::ColumnType::Periods => {
                    result = list_locale.format_integer_in(value_param);
                }
                crate::ColumnType::PeriodsExpr => {
                    result = String::from(value_param);
                }
                crate::ColumnType::SkipPeriods => {
                    result = String::from(value_param);
                }
                crate::ColumnType::Intervals => {
                    result = list_locale.format_integer_in(value_param);
                }
                crate::ColumnType::Frequency => {
                    result = String::from(value_param);
                }
                _ => {}
            }
        }

        {
            let mut calc_mgr = calc_manager.borrow_mut();
            let list_cashflow = calc_mgr.list_cashflow_mut();
            let mut list_event_opt = list_cashflow.list_event_mut();

            let list_event: &mut ListEvent;
            match list_event_opt.as_mut() {
                None => {
                    return result;
                }
                Some(o) => {
                    list_event = o;
                }
            }

            list_event.set_sort_on_add(false);

            match CoreUtility::get_col_name(col_name_index) {
                crate::ColumnType::Date => {
                    list_event.set_event_date(CoreUtility::parse_date(result.as_str()));
                }
                crate::ColumnType::DateExpr => {
                    list_event.set_date_expr(result.as_str());
                }
                crate::ColumnType::Sort => {
                    list_event.set_sort_order(CoreUtility::parse_integer(result.as_str()));
                }
                crate::ColumnType::Value => match list_event.elem_type() {
                    crate::ExtensionType::PrincipalChange => {
                        list_event.set_value(CoreUtility::parse_decimal(result.as_str()));
                    }
                    crate::ExtensionType::InterestChange => {
                        list_event.set_value(CoreUtility::parse_decimal(result.as_str()));
                    }
                    _ => {}
                },
                crate::ColumnType::ValueExpr => {
                    list_event.set_value_expr(result.as_str());
                }
                crate::ColumnType::Periods => {
                    list_event.set_periods(CoreUtility::parse_integer(result.as_str()));
                }
                crate::ColumnType::PeriodsExpr => {
                    list_event.set_periods_expr(result.as_str());
                }
                crate::ColumnType::SkipPeriods => {
                    let (skip_mask_len, skip_mask) =
                        CoreUtility::string_to_skip_mask(result.as_str());
                    list_event.set_skip_mask(skip_mask_len, skip_mask);
                }
                crate::ColumnType::Intervals => {
                    list_event.set_intervals(CoreUtility::parse_integer(result.as_str()));
                }
                crate::ColumnType::Frequency => {
                    list_event.set_frequency(CoreUtility::get_frequency(result.as_str()));
                }
                _ => {}
            }
        }

        let column = ElemColumn::new(
            "",
            col_name_index,
            "",
            "",
            "",
            "",
            "",
            "",
            crate::FormatType::String,
            0,
            0,
            false,
        );

        result = CalcUtility::get_event_value(calc_manager_param, &column);

        {
            let mut calc_mgr = calc_manager.borrow_mut();
            let list_cashflow = calc_mgr.list_cashflow_mut();
            let mut list_event_opt = list_cashflow.list_event_mut();

            let list_event: &mut ListEvent;
            match list_event_opt.as_mut() {
                None => {
                    return result;
                }
                Some(o) => {
                    list_event = o;
                }
            }

            list_event.set_sort_on_add(true);
        }

        {
            let calc_mgr = calc_manager.borrow();
            let list_locale = calc_mgr.list_locale();
            list_locale.select_event_locale("");
        }

        result
    }

    /// Set the appropriate event list extension values.
    ///
    /// # Arguments
    ///
    /// * `calc_manager_param` - Calculation manager.
    /// * `index` - Event row index.
    /// * `ext_param` - Extension values to set.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_extension_values(
        calc_manager_param: &Rc<RefCell<CalcManager>>,
        index: usize,
        ext_param: &ElemExtension,
    ) -> bool {
        let calc_manager = Rc::clone(calc_manager_param);
        let mut calc_mgr = calc_manager.borrow_mut();
        let list_cashflow = calc_mgr.list_cashflow_mut();
        let mut list_event_opt = list_cashflow.list_event_mut();

        let list_event: &mut ListEvent;
        match list_event_opt.as_mut() {
            None => {
                return false;
            }
            Some(o) => {
                list_event = o;
            }
        }

        let orig_list_index = list_event.index();
        if !list_event.get_element(index) {
            return false;
        }

        if list_event.elem_type() != ext_param.extension_type() {
            return false;
        }

        let elem_type = list_event.elem_type();
        let ext = list_event.elem_extension_mut();

        match elem_type {
            crate::ExtensionType::CurrentValue => {
                ext.set_cv_eom(ext_param.cv_eom());
                ext.set_cv_passive(ext_param.cv_passive());
                ext.set_cv_present(ext_param.cv_present());
            }
            crate::ExtensionType::InterestChange => {
                ext.set_ic_method(ext_param.ic_method());
                ext.set_ic_day_count_basis(ext_param.ic_day_count_basis());
                ext.set_ic_days_in_year(ext_param.ic_days_in_year());
                ext.set_ic_effective_frequency(ext_param.ic_effective_frequency());
                ext.set_ic_interest_frequency(ext_param.ic_interest_frequency());
                ext.set_ic_round_balance(ext_param.ic_round_balance());
                ext.set_ic_round_decimal_digits(ext_param.ic_round_decimal_digits());
            }
            crate::ExtensionType::StatisticValue => {
                ext.set_sv_name(ext_param.sv_name());
                ext.set_sv_eom(ext_param.sv_eom());
                ext.set_sv_final(ext_param.sv_is_final());
            }
            _ => {
                ext.set_pc_type(ext_param.pc_type());
                ext.set_pc_eom(ext_param.pc_eom());
                ext.set_pc_principal_first(ext_param.pc_principal_first());
                ext.set_pc_balance_statistics(ext_param.pc_balance_statistics());
                ext.set_pc_auxiliary(ext_param.pc_auxiliary());
                ext.set_pc_aux_passive(ext_param.pc_aux_passive());
            }
        }

        list_event.get_element(orig_list_index);

        true
    }

    /// Set the appropriate event list parameter values.
    ///
    /// # Arguments
    ///
    /// * `calc_manager` - Calculation manager.
    /// * `index_param` - Event row index.
    /// * `parameters` - Parameters to set.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_parameter_values(
        calc_manager: &Rc<RefCell<CalcManager>>,
        index_param: usize,
        parameters: Vec<String>,
    ) -> bool {
        let mut calc_mgr = calc_manager.borrow_mut();
        let list_cashflow = calc_mgr.list_cashflow_mut();

        let list_event: &mut ListEvent;
        match list_cashflow.list_event_mut() {
            None => {
                return false;
            }
            Some(o) => {
                list_event = o;
            }
        }

        let orig_index = list_event.index();
        if !list_event.get_element(index_param) {
            return false;
        }

        let list_parameter: &mut ListParameter;
        match list_event.list_parameter_mut() {
            None => {
                return false;
            }
            Some(o) => {
                list_parameter = o;
            }
        }

        let orig_param_index = list_parameter.index();
        let mut index: usize = 0;
        loop {
            if !list_parameter.get_element(index) {
                break;
            }

            match list_parameter.param_type() {
                crate::TokenType::Integer => {
                    list_parameter
                        .set_integeri(CoreUtility::parse_integeri(parameters[index].as_str()));
                }
                crate::TokenType::Decimal => {
                    list_parameter
                        .set_decimal(CoreUtility::parse_decimal(parameters[index].as_str()));
                }
                _ => {
                    list_parameter.set_string(parameters[index].as_str());
                }
            }
            index += 1;
        }

        list_parameter.get_element(orig_param_index);
        list_event.get_element(orig_index);

        true
    }
}
