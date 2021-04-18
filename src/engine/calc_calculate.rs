//! The primary calculation methods.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rust_decimal::prelude::*;
use std::cell::{Cell, Ref, RefCell, RefMut};
use std::rc::Rc;

use super::{CalcExpression, CalcManager, CalcUtility};
use crate::core::{
    CoreUtility, ElemBalanceResult, ElemExtension, ElemSymbol, ListAmortization, ListDescriptor,
    ListEvent, ListParameter, ListStatisticHelper,
};
use crate::ListTrait;

pub struct CalcCalculate {
    /// Calculator manager element.
    calc_manager: Rc<RefCell<CalcManager>>,

    /// Start of fiscal year in MMDD format.
    fiscal_year_start: Cell<usize>,
    /// Number of significant decimal digits.
    decimal_digits: Cell<usize>,
    /// Cashflow descriptor list.
    list_descriptor_cashflow: Option<ListDescriptor>,

    /// Expression object for this calculate object.
    calc_expression: RefCell<CalcExpression>,
    /// Last compounded amount calculated by the interest method.
    interest: Cell<Decimal>,
    /// Last straight-line amount calculated by the interest method.
    sl_interest: Cell<Decimal>,
    /// Last date that interest was factored through.
    last_interest_date: Cell<usize>,
    /// One or more statistic events were seen in the amortization method.
    statistic_event_seen: Cell<bool>,
}

/// The primary calculation implementation methods.

impl CalcCalculate {
    /// Create and return a new calculate element.
    ///
    /// # Arguments
    ///
    /// * `list_descriptor_cashflow_param` - Cashflow descriptor list.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn new(
        calc_manager_param: &Rc<RefCell<CalcManager>>,
        list_descriptor_cashflow_param: Option<&ListDescriptor>,
    ) -> CalcCalculate {
        let fys = calc_manager_param.borrow().fiscal_year_start(false) % 10000;
        let dd = calc_manager_param.borrow().decimal_digits(false);

        let list_descriptor: Option<ListDescriptor>;
        match list_descriptor_cashflow_param.as_ref() {
            None => {
                list_descriptor = None;
            }
            Some(o) => {
                list_descriptor = Option::from(o.copy(false, false));
            }
        }

        return CalcCalculate {
            calc_manager: Rc::clone(calc_manager_param),
            fiscal_year_start: Cell::new(fys),
            decimal_digits: Cell::new(dd),
            list_descriptor_cashflow: list_descriptor,
            calc_expression: RefCell::new(CalcExpression::new(&calc_manager_param, fys, dd)),
            interest: Cell::new(dec!(0.0)),
            sl_interest: Cell::new(dec!(0.0)),
            last_interest_date: Cell::new(0),
            statistic_event_seen: Cell::new(false),
        };
    }

    /// Get the calculation manager.
    ///
    /// # Return
    ///
    /// * See description.

    fn calc_mgr(&self) -> Ref<CalcManager> {
        self.calc_manager.borrow()
    }

    /// Get the expression.
    ///
    /// # Return
    ///
    /// * See description.

    fn expr(&self) -> Ref<CalcExpression> {
        self.calc_expression.borrow()
    }

    /// Get the mutable expression.
    ///
    /// # Return
    ///
    /// * See description.

    fn expr_mut(&self) -> RefMut<CalcExpression> {
        self.calc_expression.borrow_mut()
    }

    /// Performs primary calculations on an amortization cashflow.
    /// Passes through an entire amortization cashflow calculating
    /// the interest, accrued interest balance and balance for
    /// each amortization element within the cashflow. This method
    /// is called by all higher-level calculation methods in this
    /// element.
    ///
    /// # Arguments
    ///
    /// * `list_am` - The amortization list to balance.
    /// * `list_statistic_helper` - The list of active statistic elements.
    /// * `elem_balance_prev` - The prior results from this method (only used
    ///     with rule-of-78).
    /// * `include_aux_passive` - Include auxiliary passive events in the cashflow.
    /// * `rule_of_78_balance` - Indicates a second call to this
    ///     method to prorate the previously calculated interest
    ///     according to the Rule of 78. The first call must specify
    ///     that cv_rule_of_78_seen is true in elem_balance_result.
    /// * `optimize` - If true the value expression for each event
    ///     is not evaluated (if it exists).
    ///
    /// # Return
    ///
    /// * The results from this method or an error code.

    pub fn balance_cashflow(
        &self,
        list_am: &mut ListAmortization,
        list_statistic_helper: &mut ListStatisticHelper,
        elem_balance_prev: &ElemBalanceResult,
        include_aux_passive: bool,
        rule_of_78_balance: bool,
        optimize: bool,
    ) -> Result<ElemBalanceResult, crate::ErrorType> {
        let mut balance = dec!(0.0);
        let mut sl_balance = dec!(0.0);
        let mut acc_balance = dec!(0.0);
        let mut total_interest = dec!(0.0);
        let mut int_rate = dec!(0.0);
        let mut round_decimal_digits = dec!(-1.0);

        let orig_list_index = list_am.index();

        let mut frequency = crate::FrequencyType::OneMonth;
        let mut int_method = crate::MethodType::Actuarial;
        let mut int_day_count_basis = crate::DayCountType::Periodic;
        let mut int_days_in_year = crate::DEFAULT_DAYS_IN_YEAR;
        let mut int_effective_frequency = crate::FrequencyType::None;
        let mut int_round_balance = crate::RoundType::None;
        let mut total_prin_stats: usize = 0;
        let mut sum_of_the_digits: usize = 0;
        let mut bal_date: usize = 0;

        let mut cv_present_seen: bool = false;
        let mut int_rule_of_78_seen: bool = false;

        if rule_of_78_balance {
            total_prin_stats = elem_balance_prev.prin_total();
            total_interest = elem_balance_prev.interest_total();
            for am_index in 1..(elem_balance_prev.prin_total() + 1) {
                sum_of_the_digits += am_index;
            }
        }
        self.expr_mut().clear();
        list_statistic_helper.clear();
        
        let mut elem_balance_result = ElemBalanceResult::new();
        self.last_interest_date.set(0);

        for am_index in 0..list_am.count() {
            if !list_am.get_element(am_index) {
                break;
            }
            let elem_type = list_am.elem_type();
            let orig_date = list_am.orig_date();
            let event_date = list_am.event_date();
            let orig_value = list_am.orig_value();
            let mut value = list_am.value();
            let value_expr = list_am.value_expr();
            let event_sequence = list_am.event_sequence();
            let list_parameter = list_am.list_parameter();
            self.interest.set(dec!(0.0));
            self.sl_interest.set(dec!(0.0));
            if self.last_interest_date.get() == 0 {
                self.last_interest_date.set(event_date);
            }

            let orig_balance = balance;
            let orig_acc_balance = acc_balance;
            let orig_interest_date = self.last_interest_date.get();

            let mut stat_name = String::from("");
            let mut prin_type = crate::PrincipalType::Increase;
            let mut prin_eom: bool = false;
            let mut prin_first: bool = false;
            let mut prin_balance_statistics: bool = false;
            let mut prin_auxilary: bool = false;
            let mut prin_aux_passive: bool = false;
            let mut cv_passive: bool = false;
            let mut cv_present: bool = false;
            let mut stat_final: bool = false;

            let ext = list_am.elem_extension();
            match elem_type {
                crate::ExtensionType::CurrentValue => {
                    bal_date = event_date;
                    prin_eom = ext.cv_eom();
                    cv_passive = ext.cv_passive();
                    cv_present = ext.cv_present();
                }
                crate::ExtensionType::InterestChange => {
                    // Set options after interest is calculated
                    bal_date = event_date;
                }
                crate::ExtensionType::StatisticValue => {
                    stat_name = String::from(ext.sv_name());
                    stat_final = ext.sv_is_final();
                }
                _ => {
                    bal_date = event_date;
                    prin_type = ext.pc_type();
                    prin_eom = ext.pc_eom();
                    prin_first = ext.pc_principal_first();
                    prin_balance_statistics = ext.pc_balance_statistics();
                    prin_auxilary = ext.pc_auxiliary();
                    prin_aux_passive = ext.pc_aux_passive();
                }
            }

            if event_date > self.last_interest_date.get() {
                if !(rule_of_78_balance
                    && int_day_count_basis == crate::DayCountType::RuleOf78
                    && elem_type == crate::ExtensionType::PrincipalChange
                    && prin_balance_statistics)
                {
                    self.interest(
                        int_method,
                        int_day_count_basis,
                        int_days_in_year,
                        orig_date,
                        event_date,
                        frequency,
                        int_effective_frequency,
                        int_rate,
                        balance,
                        sl_balance,
                        prin_eom,
                    );
                } else {
                    // Rule of 78 interest allocation
                    let alloc =
                        (total_prin_stats - elem_balance_result.prin_total()) / sum_of_the_digits;
                    self.interest.set(total_interest * dec!(alloc));
                    self.sl_interest.set(dec!(0.0)); // The straight-line balance is not known when using the Rule of 78
                    self.last_interest_date.set(event_date);
                }
            }
            if int_round_balance != crate::RoundType::None {
                if round_decimal_digits < dec!(0.0) {
                    self.interest.set(CoreUtility::round(
                        self.interest.get(),
                        self.decimal_digits.get(),
                        int_round_balance,
                    ));
                    self.sl_interest.set(CoreUtility::round(
                        self.sl_interest.get(),
                        self.decimal_digits.get(),
                        int_round_balance,
                    ));
                } else if round_decimal_digits > dec!(0.0) && round_decimal_digits < dec!(1.0) {
                    self.interest.set(CoreUtility::round_fraction(
                        self.interest.get(),
                        round_decimal_digits,
                        int_round_balance,
                    ));
                    self.sl_interest.set(CoreUtility::round_fraction(
                        self.sl_interest.get(),
                        round_decimal_digits,
                        int_round_balance,
                    ));
                } else {
                    match round_decimal_digits.to_usize() {
                        None => {}
                        Some(o) => {
                            self.interest.set(CoreUtility::round(
                                self.interest.get(),
                                o,
                                int_round_balance,
                            ));
                            self.sl_interest.set(CoreUtility::round(
                                self.sl_interest.get(),
                                o,
                                int_round_balance,
                            ));
                        }
                    }
                }
            }

            if !optimize && !value_expr.is_empty() {
                // Perform late evaluation
                self.expr_mut().init_expression(
                    self.list_descriptor_cashflow.as_ref(),
                    None,
                    list_parameter,
                    value_expr,
                );
                self.expr_mut().set_symbol_decimal("decValue", orig_value);
                self.expr_mut()
                    .set_symbol_decimal("decInterest", self.interest.get());
                self.expr_mut()
                    .set_symbol_decimal("decSLInterest", self.sl_interest.get());
                self.expr_mut()
                    .set_symbol_decimal("decAccBalance", acc_balance);
                // Include the interest accrued for this event
                self.expr_mut()
                    .set_symbol_decimal("decBalance", balance + self.interest.get());
                self.expr_mut()
                    .set_symbol_integer("intSequence", event_sequence);

                let result_symbol: ElemSymbol;
                let result = self
                    .expr()
                    .evaluate(Option::from(&*list_am), Option::from(elem_balance_prev));
                match result {
                    Err(e) => {
                        list_am.get_element(orig_list_index);
                        return Err(e);
                    }
                    Ok(o) => {
                        result_symbol = o;
                    }
                }

                match result_symbol.sym_type() {
                    crate::TokenType::Integer => {
                        value = dec!(result_symbol.sym_integer());
                    }
                    crate::TokenType::Decimal => {
                        value = result_symbol.sym_decimal();
                    }
                    crate::TokenType::String => {
                        value = CoreUtility::parse_decimal(result_symbol.sym_string());
                    }
                    _ => {}
                }

                list_am.set_value(value);
            }

            if int_method == crate::MethodType::SimpleInterest {
                acc_balance += self.interest.get();
            } else if balance < dec!(0.0) {
                // METHOD_ACTUARIAL
                balance -= self.interest.get();
            } else {
                balance += self.interest.get();
            }

            match elem_type {
                crate::ExtensionType::CurrentValue => {
                    list_am.set_interest(self.interest.get()); // Report for passive or active
                    list_am.set_sl_interest(self.sl_interest.get());
                    list_am.set_acc_balance(acc_balance);
                    list_am.set_balance(balance);
                    if !cv_present && cv_passive {
                        // Passive event
                        self.last_interest_date.set(orig_interest_date);
                        balance = orig_balance;
                        acc_balance = orig_acc_balance;
                    } else {
                        elem_balance_result.incr_interest_total(self.interest.get());
                        elem_balance_result.incr_sl_interest_total(self.sl_interest.get());
                        if cv_present_seen {
                            elem_balance_result.incr_interest_present(self.interest.get());
                            elem_balance_result.incr_sl_interest_present(self.sl_interest.get());
                        }
                        for statistic_index in 0..list_statistic_helper.count() {
                            if !list_statistic_helper.get_element(statistic_index) {
                                break;
                            }
                            list_statistic_helper.incr_interest(self.interest.get());
                            list_statistic_helper.incr_sl_interest(self.sl_interest.get());
                        }
                        if !cv_present_seen && cv_present {
                            elem_balance_result.set_cur_first_pv_index(am_index);
                            cv_present_seen = true;
                        }
                    }
                }
                crate::ExtensionType::InterestChange => {
                    int_rate = list_am.value();
                    frequency = list_am.frequency();
                    int_method = list_am.elem_extension().ic_method();
                    int_day_count_basis = list_am.elem_extension().ic_day_count_basis();
                    int_days_in_year = list_am.elem_extension().ic_days_in_year();
                    int_effective_frequency = list_am.elem_extension().ic_effective_frequency();
                    int_round_balance = list_am.elem_extension().ic_round_balance();
                    round_decimal_digits = list_am.elem_extension().ic_round_decimal_digits();
                    if int_method == crate::MethodType::Actuarial {
                        // Fold-in any residual accrued interest balance
                        balance += if balance < dec!(0.0) {
                            -acc_balance
                        } else {
                            acc_balance
                        };
                        acc_balance = dec!(0.0);
                    }
                    sl_balance = balance; // Set the straight-line balance after an interest change
                    if !int_rule_of_78_seen {
                        int_rule_of_78_seen = int_day_count_basis == crate::DayCountType::RuleOf78;
                    }
                    list_am.set_interest(self.interest.get());
                    list_am.set_sl_interest(self.sl_interest.get());
                    list_am.set_acc_balance(acc_balance);
                    list_am.set_balance(balance);
                    elem_balance_result.incr_interest_total(self.interest.get());
                    elem_balance_result.incr_sl_interest_total(self.sl_interest.get());
                    if cv_present_seen {
                        elem_balance_result.incr_interest_present(self.interest.get());
                        elem_balance_result.incr_sl_interest_present(self.sl_interest.get());
                    }
                    for statistic_index in 0..list_statistic_helper.count() {
                        if !list_statistic_helper.get_element(statistic_index) {
                            break;
                        }
                        list_statistic_helper.incr_interest(self.interest.get());
                        list_statistic_helper.incr_sl_interest(self.sl_interest.get());
                    }
                    if elem_balance_result.int_first_index() == usize::MAX {
                        elem_balance_result.set_int_first_index(am_index);
                    }
                    elem_balance_result.set_int_last_index(am_index);
                }
                crate::ExtensionType::StatisticValue => {
                    let bresult = list_statistic_helper.get_element_by_name(stat_name.as_str());
                    if bresult {
                        list_am.set_principal_decrease(list_statistic_helper.principal_decrease());
                        list_am.set_principal_increase(list_statistic_helper.principal_increase());
                        list_am.set_interest(list_statistic_helper.interest());
                        list_am.set_sl_interest(list_statistic_helper.sl_interest());
                        list_am.set_value_to_interest(list_statistic_helper.value_to_interest());
                        list_am.set_value_to_principal(list_statistic_helper.value_to_principal());
                        list_statistic_helper.reset();
                    }
                    let mut stat_final_no_element: bool = false;
                    if stat_final {
                        if !bresult {
                            stat_final_no_element = true;
                        } else {
                            list_statistic_helper.remove();
                        }
                    } else if bresult {
                        list_statistic_helper.set_last_date(event_date);
                        list_statistic_helper.set_elem_am_index(am_index);
                    } else {
                        list_statistic_helper.add_statistic_helper(
                            stat_name.as_str(),
                            event_date,
                            am_index,
                        );
                    }
                    if !stat_final_no_element {
                        self.last_interest_date.set(orig_interest_date); // Passive event
                        balance = orig_balance;
                        acc_balance = orig_acc_balance;
                    }
                }
                _ => {
                    let mut prin_value = value;
                    if prin_type == crate::PrincipalType::Positive
                        || prin_type == crate::PrincipalType::Negative
                    {
                        if prin_type == crate::PrincipalType::Positive {
                            balance = value;
                        } else {
                            balance = -value;
                        }
                        self.interest.set(dec!(0.0));
                        self.sl_interest.set(dec!(0.0));
                        acc_balance = dec!(0.0);
                        if !(prin_auxilary && prin_aux_passive && !include_aux_passive) {
                            for statistic_index in 0..list_statistic_helper.count() {
                                if !list_statistic_helper.get_element(statistic_index) {
                                    break;
                                }
                                list_statistic_helper.reset();
                            }
                        }
                    } else if int_method == crate::MethodType::SimpleInterest
                        && prin_type == crate::PrincipalType::Increase
                        && balance < dec!(0.0)
                    {
                        if prin_first {
                            balance += prin_value;
                            if balance > dec!(0.0) && acc_balance > dec!(0.0) {
                                // Balance went positive
                                let mut dresult = balance - acc_balance;
                                if dresult < dec!(0.0) {
                                    dresult = dec!(0.0); // Balance remaining after accrued is subtracted
                                }
                                dresult = balance - dresult; // Amount of accrued interest satisfied by the positive balance
                                prin_value -= dresult;
                                if prin_value < dec!(0.0) {
                                    prin_value = dec!(0.0);
                                }
                                acc_balance -= dresult;
                                balance -= dresult;
                            }
                        } else {
                            prin_value -= acc_balance;
                            if prin_value < dec!(0.0) {
                                prin_value = dec!(0.0);
                            }
                            acc_balance -= value;
                            if acc_balance < dec!(0.0) {
                                acc_balance = dec!(0.0);
                            }
                            balance += prin_value;
                        }
                    } else if prin_type == crate::PrincipalType::Increase {
                        let bresult = balance < dec!(0.0);
                        balance += prin_value;
                        if int_method == crate::MethodType::Actuarial && bresult {
                            prin_value -= self.interest.get();
                            if prin_value < dec!(0.0) {
                                prin_value = dec!(0.0);
                            }
                        }
                    } else {
                        balance -= prin_value;
                    }
                    list_am.set_interest(self.interest.get()); // Report for normal or auxiliary
                    list_am.set_sl_interest(self.sl_interest.get());
                    list_am.set_acc_balance(acc_balance);
                    list_am.set_balance(balance);
                    let mut is_passive: bool = false;
                    if prin_auxilary {
                        if prin_aux_passive {
                            if prin_type == crate::PrincipalType::Negative
                                || prin_type == crate::PrincipalType::Decrease
                            {
                                elem_balance_result.incr_aux_passive_decrease(value);
                            } else {
                                elem_balance_result.incr_aux_passive_increase(value);
                            }
                            if !include_aux_passive {
                                // Passive event
                                self.last_interest_date.set(orig_interest_date);
                                balance = orig_balance;
                                acc_balance = orig_acc_balance;
                                is_passive = true;
                            }
                        } else if prin_type == crate::PrincipalType::Negative
                            || prin_type == crate::PrincipalType::Decrease
                        {
                            elem_balance_result.incr_aux_active_decrease(value);
                        } else {
                            elem_balance_result.incr_aux_active_increase(value);
                        }
                    } else if prin_type == crate::PrincipalType::Negative
                        || prin_type == crate::PrincipalType::Decrease
                    {
                        elem_balance_result.incr_prin_decrease(value);
                    } else {
                        elem_balance_result.incr_prin_increase(value);
                    }
                    if !is_passive {
                        list_am.set_value_to_interest(value - prin_value);
                        list_am.set_value_to_principal(
                            if prin_type == crate::PrincipalType::Negative
                                || prin_type == crate::PrincipalType::Decrease
                            {
                                dec!(0.0)
                            } else {
                                prin_value
                            },
                        );
                        for statistic_index in 0..list_statistic_helper.count() {
                            if !list_statistic_helper.get_element(statistic_index) {
                                break;
                            }
                            if prin_type == crate::PrincipalType::Negative
                                || prin_type == crate::PrincipalType::Decrease
                            {
                                list_statistic_helper.incr_principal_decrease(value);
                            } else {
                                list_statistic_helper.incr_principal_increase(value);
                            }
                            list_statistic_helper.incr_interest(self.interest.get());
                            list_statistic_helper.incr_sl_interest(self.sl_interest.get());
                            list_statistic_helper.incr_value_to_interest(value - prin_value);
                            list_statistic_helper.incr_value_to_principal(
                                if prin_type == crate::PrincipalType::Negative
                                    || prin_type == crate::PrincipalType::Decrease
                                {
                                    dec!(0.0)
                                } else {
                                    prin_value
                                },
                            );
                        }
                        elem_balance_result.incr_interest_total(self.interest.get());
                        elem_balance_result.incr_sl_interest_total(self.sl_interest.get());
                        if cv_present_seen {
                            elem_balance_result.incr_interest_present(self.interest.get());
                            elem_balance_result.incr_sl_interest_present(self.sl_interest.get());
                        }
                        if prin_balance_statistics {
                            elem_balance_result.incr_prin_total(1);
                            if cv_present_seen {
                                elem_balance_result.incr_prin_present(1);
                            }
                            if elem_balance_result.prin_first_stat_index() == usize::MAX {
                                elem_balance_result.set_prin_first_stat_index(am_index);
                            }
                            if cv_present_seen
                                && elem_balance_result.prin_first_stat_pv_index() == usize::MAX
                            {
                                elem_balance_result.set_prin_first_stat_pv_index(am_index);
                            }
                            elem_balance_result.set_prin_last_stat_index(am_index);
                            list_am.set_stat_sequence(elem_balance_result.prin_total());
                        }
                        if acc_balance > dec!(0.0) {
                            elem_balance_result.set_acc_balance_seen(true);
                        }
                        if elem_balance_result.prin_first_index() == usize::MAX {
                            elem_balance_result.set_prin_first_index(am_index);
                            if prin_type == crate::PrincipalType::Negative
                                || prin_type == crate::PrincipalType::Decrease
                            {
                                elem_balance_result.set_polarity(-1); // Negative CF
                            }
                        }
                        if cv_present_seen
                            && elem_balance_result.prin_first_pv_index() == usize::MAX
                        {
                            elem_balance_result.set_prin_first_pv_index(am_index);
                        }
                        elem_balance_result.set_prin_last_index(am_index);
                    }
                }
            }
        }

        elem_balance_result.set_acc_balance(acc_balance);
        elem_balance_result.set_balance(balance);
        elem_balance_result.set_balance_date(bal_date);
        elem_balance_result.set_rule_of_78_seen(int_rule_of_78_seen);
        if !cv_present_seen {
            elem_balance_result.incr_interest_present(elem_balance_result.interest_total());
            elem_balance_result.incr_sl_interest_present(elem_balance_result.sl_interest_total());
            elem_balance_result.incr_prin_present(elem_balance_result.prin_total());
        }
        if elem_balance_result.prin_first_stat_pv_index() == usize::MAX {
            elem_balance_result
                .set_prin_first_stat_pv_index(elem_balance_result.prin_first_stat_index());
        }
        if elem_balance_result.prin_first_pv_index() == usize::MAX {
            elem_balance_result.set_prin_first_pv_index(elem_balance_result.prin_first_index());
        }

        let mut result = 0;
        for am_index in 0..list_am.count() {
            if !list_am.get_element(am_index) {
                break;
            }
            if list_am.elem_type() != crate::ExtensionType::StatisticValue {
                break;
            }
            result += 1;
        }
        if result > 0 {
            elem_balance_result
                .set_prin_first_index(elem_balance_result.prin_first_index() - result);
            elem_balance_result
                .set_prin_first_stat_index(elem_balance_result.prin_first_stat_index() - result);
            elem_balance_result
                .set_prin_first_pv_index(elem_balance_result.prin_first_pv_index() - result);
            elem_balance_result.set_prin_first_stat_pv_index(
                elem_balance_result.prin_first_stat_pv_index() - result,
            );
            elem_balance_result.set_prin_last_index(elem_balance_result.prin_last_index() - result);
            elem_balance_result
                .set_prin_last_stat_index(elem_balance_result.prin_last_stat_index() - result);
            elem_balance_result
                .set_cur_first_pv_index(elem_balance_result.cur_first_pv_index() - result);
            elem_balance_result.set_int_first_index(elem_balance_result.int_first_index() - result);
            elem_balance_result.set_int_last_index(elem_balance_result.int_last_index() - result);

            while result > 0 {
                if !list_am.get_element(0) {
                    break;
                }
                list_am.remove();
                result -= 1;
            }
        }
        list_am.get_element(orig_list_index);

        Ok(elem_balance_result)
    }

    /// Calculates the value for the current interest change event.
    /// Calculates an interest value that will satisfy the
    /// condition that the remaining balance of the cashflow
    /// is the smallest amount greater than or equal to the given
    /// parameter value.
    ///
    /// # Arguments
    ///
    /// * `list_event` - The event list to calculate.
    /// * `list_am` - The amortization list to balance.
    /// * `list_statistic_helper` - The list of active statistic elements.
    /// * `new_value` - The desired remaining balance or dec_zero.
    ///
    /// # Return
    ///
    /// * The results from this method or an error code.

    pub fn calculate_interest(
        &self,
        list_event: &ListEvent,
        list_am: &mut ListAmortization,
        list_statistic_helper: &mut ListStatisticHelper,
        new_value: Decimal,
    ) -> Result<ElemBalanceResult, crate::ErrorType> {
        let event_index = list_event.index();
        if event_index == usize::MAX {
            return Err(crate::ErrorType::Index);
        }

        if list_event.elem_type() != crate::ExtensionType::InterestChange {
            return Err(crate::ErrorType::Index);
        }

        let value_expr_am = !list_event.value_expr().is_empty() && !list_event.value_expr_balance();
        let mut result: Result<(), crate::ErrorType>;
        if !value_expr_am {
            result = self.expand_with_list(list_event, list_am, true);
            match result {
                Err(e) => {
                    return Err(e);
                }
                Ok(_o) => {}
            }
        }
        let mut elem_balance_result = ElemBalanceResult::new();

        let dec_zero = dec!(0.0);
        let dec_two = dec!(2.0);
        let dec_ten = dec!(10.0);
        let max_calc_principal = dec!(crate::MAX_CALC_PRINCIPAL);
        let max_calc_interest = dec!(crate::MAX_CALC_INTEREST);

        let smallest_fraction =
            dec!(1.0) / CoreUtility::decimal_pow(dec_ten, crate::MAXIMUM_DISPLAY_DECIMAL_DIGITS);
        let decrement_fraction = smallest_fraction / dec_ten;
        let mut last_interest = dec!(-1.0);
        let mut calc_interest = dec_ten;
        let mut iterations: usize = 1;
        let mut adjust_down: bool;
        while iterations <= crate::MAXIMUM_ITERATIONS_CALCULATE_INTEREST {
            if value_expr_am {
                list_event.set_value_result(calc_interest);
                result = self.expand_with_list(list_event, list_am, true);
                match result {
                    Err(e) => {
                        return Err(e);
                    }
                    Ok(_o) => {}
                }
            } else {
                list_am.set_all_index_values(event_index, calc_interest);
            }

            let result_balance = self.balance_cashflow(
                list_am,
                list_statistic_helper,
                &elem_balance_result,
                false,
                false,
                false,
            );
            match result_balance {
                Err(e) => {
                    return Err(e);
                }
                Ok(o) => {
                    elem_balance_result = o;
                }
            }
            let balance = CoreUtility::round(
                elem_balance_result.balance()
                    + if elem_balance_result.balance() < dec_zero {
                        -elem_balance_result.acc_balance()
                    } else {
                        elem_balance_result.acc_balance()
                    },
                self.decimal_digits.get(),
                crate::RoundType::Bankers,
            );
            if balance.abs() >= max_calc_principal {
                calc_interest = dec!(2005.0);
                break;
            }

            if elem_balance_result.polarity() < 0 {
                adjust_down = balance < new_value;
            } else {
                adjust_down = balance > new_value;
            }

            if balance == new_value
                || calc_interest > max_calc_interest
                || ((calc_interest - last_interest).abs() <= smallest_fraction
                    && if elem_balance_result.polarity() > 0 {
                        balance >= new_value
                    } else {
                        balance <= new_value
                    })
            {
                break;
            }

            let mut orig_interest = calc_interest;
            if adjust_down {
                if last_interest < dec_zero || (iterations == 1 && last_interest > calc_interest) {
                    if calc_interest > last_interest {
                        calc_interest -= (calc_interest - last_interest) * dec_two;
                    } else {
                        calc_interest -= (last_interest - calc_interest) * dec_two;
                    }
                    if calc_interest < dec_zero {
                        calc_interest = dec_zero;
                    }
                } else {
                    if (calc_interest - last_interest).abs() <= decrement_fraction {
                        calc_interest -= decrement_fraction;
                        orig_interest = calc_interest; // Possibly terminate loop
                        if calc_interest < dec_zero {
                            orig_interest = dec_zero;
                            calc_interest = dec_zero;
                        }
                    } else if calc_interest > last_interest {
                        calc_interest -= (calc_interest - last_interest) / dec_two;
                    } else {
                        calc_interest -= (last_interest - calc_interest) / dec_two;
                    }
                    iterations += 1;
                }
            } else {
                // Adjust up
                if last_interest < dec_zero || (iterations == 1 && last_interest < calc_interest) {
                    if calc_interest > last_interest {
                        calc_interest += (calc_interest - last_interest) * dec_two;
                    } else {
                        calc_interest += (last_interest - calc_interest) * dec_two;
                    }
                } else {
                    if (calc_interest - last_interest).abs() <= decrement_fraction {
                        calc_interest += decrement_fraction;
                        orig_interest = calc_interest; // Possibly terminate loop
                    } else if calc_interest > last_interest {
                        calc_interest += (calc_interest - last_interest) / dec_two;
                    } else {
                        calc_interest += (last_interest - calc_interest) / dec_two;
                    }
                    iterations += 1;
                }
            }

            last_interest = orig_interest;
        }

        if calc_interest > max_calc_interest {
            calc_interest = dec_zero;
        }

        if calc_interest > dec_zero {
            // Make sure that rounding does not affect the result in the wrong direction
            calc_interest = CoreUtility::round(
                calc_interest,
                crate::MAXIMUM_DISPLAY_DECIMAL_DIGITS,
                crate::RoundType::Bankers,
            );
            if value_expr_am {
                list_event.set_value_result(calc_interest);
                result = self.expand_with_list(list_event, list_am, true);
                match result {
                    Err(e) => {
                        return Err(e);
                    }
                    Ok(_o) => {}
                }
            } else {
                list_am.set_all_index_values(event_index, calc_interest);
            }

            let result_balance = self.balance_cashflow(
                list_am,
                list_statistic_helper,
                &elem_balance_result,
                false,
                false,
                false,
            );
            match result_balance {
                Err(e) => {
                    return Err(e);
                }
                Ok(o) => {
                    elem_balance_result = o;
                }
            }
            let balance = CoreUtility::round(
                elem_balance_result.balance()
                    + (if elem_balance_result.balance() < dec_zero {
                        -elem_balance_result.acc_balance()
                    } else {
                        elem_balance_result.acc_balance()
                    }),
                self.decimal_digits.get(),
                crate::RoundType::Bankers,
            );

            let bresult = if elem_balance_result.polarity() > 0 {
                balance < new_value
            } else {
                balance > new_value
            };
            if bresult {
                if elem_balance_result.polarity() < 0 {
                    adjust_down = balance < new_value;
                } else {
                    adjust_down = balance > new_value;
                }
                if adjust_down {
                    calc_interest -= smallest_fraction;
                } else {
                    calc_interest += smallest_fraction;
                }
            }
        }

        if calc_interest == dec_zero {
            return Err(crate::ErrorType::CalcInterest);
        }

        list_event.set_value(calc_interest);
        
        let result_balance = self.balance_cashflow(
            list_am,
            list_statistic_helper,
            &elem_balance_result,
            false,
            false,
            false,
        );
        match result_balance {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_balance_result = o;
            }
        }

        elem_balance_result.set_result_decimal(calc_interest);
        
        Ok(elem_balance_result)
    }

    /// Calculates the periods for the current principal change event.
    /// Calculates the number of periods that will satisfy the
    /// condition that the remaining balance of the cashflow
    /// is the smallest amount greater than or equal to the given
    /// parameter value.
    ///
    /// # Arguments
    ///
    /// * `list_event` - The event list to calculate.
    /// * `list_am` - The amortization list to balance.
    /// * `list_statistic_helper` - The list of active statistic elements.
    /// * `new_value` - The desired remaining balance or dec_zero.
    ///
    /// # Return
    ///
    /// * The results from this method or an error code.

    pub fn calculate_periods(
        &self,
        list_event: &ListEvent,
        list_am: &mut ListAmortization,
        list_statistic_helper: &mut ListStatisticHelper,
        new_value: Decimal,
    ) -> Result<ElemBalanceResult, crate::ErrorType> {
        if list_event.elem_type() == crate::ExtensionType::StatisticValue {
            return Err(crate::ErrorType::Index);
        }

        let mut elem_balance_result = ElemBalanceResult::new();

        if list_event.elem_type() == crate::ExtensionType::InterestChange
            && list_event.value_expr().is_empty()
        {
            list_event.set_periods(1);
            return Ok(elem_balance_result);
        }
        let mut adjust_negative = false;
        if list_event.elem_type() == crate::ExtensionType::PrincipalChange {
            let pc_type = list_event.elem_extension().pc_type();
            adjust_negative = pc_type == crate::PrincipalType::Negative
                || pc_type == crate::PrincipalType::Decrease;
        }

        let intervals_in_year =
            CoreUtility::intervals_in_year(list_event.frequency(), crate::DEFAULT_DAYS_IN_YEAR);
        let intervals = list_event.intervals();
        let mut periods = intervals_in_year * intervals * 10;
        let mut iterations: usize = 1;
        let mut last_periods: usize = 0;
        let mut new_last_periods = true;
        let mut last_adjust_down = false;

        let dec_zero = dec!(0.0);

        while iterations <= crate::MAXIMUM_ITERATIONS_CALCULATE_PERIODS {
            list_event.set_periods_result(periods);

            let result = self.expand_with_list(list_event, list_am, true);
            match result {
                Err(e) => {
                    return Err(e);
                }
                Ok(_o) => {}
            }

            let result_balance = self.balance_cashflow(
                list_am,
                list_statistic_helper,
                &elem_balance_result,
                false,
                false,
                false,
            );
            match result_balance {
                Err(e) => {
                    return Err(e);
                }
                Ok(o) => {
                    elem_balance_result = o;
                }
            }
            let balance = CoreUtility::round(
                elem_balance_result.balance()
                    + (if elem_balance_result.balance() < dec_zero {
                        -elem_balance_result.acc_balance()
                    } else {
                        elem_balance_result.acc_balance()
                    }),
                    self.decimal_digits.get(),
                crate::RoundType::Bankers,
            );
            let mut adjust_down;
            if adjust_negative {
                adjust_down = balance < new_value;
            } else {
                adjust_down = balance > new_value;
            }

            if balance == new_value
                || periods > intervals_in_year * intervals * 100
                || (((periods as i32) - (last_periods as i32)).abs() <= 1
                    && adjust_down != last_adjust_down
                    && if elem_balance_result.polarity() > 0 {
                        balance >= new_value
                    } else {
                        balance <= new_value
                    })
            {
                break;
            }

            let orig_periods = periods;

            if adjust_down {
                if new_last_periods || (iterations == 1 && last_periods > periods) {
                    if periods > last_periods {
                        periods -= (periods - last_periods) * 2;
                    } else {
                        periods -= (last_periods - periods) * 2;
                    }
                    if periods == 0 {
                        periods = 1;
                    }
                    new_last_periods = false
                } else {
                    if ((periods as i32) - (last_periods as i32)).abs() <= 1 {
                        periods -= 1;
                        if periods < 1 {
                            periods = 1;
                            adjust_down = false;
                        }
                    } else if periods > last_periods {
                        periods -= (periods - last_periods) / 2;
                    } else {
                        periods -= (last_periods - periods) / 2;
                    }
                    iterations += 1;
                }
            } else {
                // Adjust up
                if new_last_periods || (iterations == 1 && last_periods < periods) {
                    if periods > last_periods {
                        periods += (periods - last_periods) * 2;
                    } else {
                        periods += (last_periods - periods) * 2;
                    }
                    new_last_periods = false
                } else {
                    if ((periods as i32) - (last_periods as i32)).abs() <= 1 {
                        periods += 1;
                    } else if periods > last_periods {
                        periods += (periods - last_periods) / 2;
                    } else {
                        periods += (last_periods - periods) / 2;
                    }
                    iterations += 1;
                }
            }

            last_periods = orig_periods;
            last_adjust_down = adjust_down;
        }

        if periods > intervals_in_year * intervals * 100 {
            periods = 0;
        }

        if periods == 0 {
            return Err(crate::ErrorType::CalcPeriods);
        }

        list_event.set_periods(periods);

        let result_balance = self.balance_cashflow(
            list_am,
            list_statistic_helper,
            &elem_balance_result,
            false,
            false,
            false,
        );
        match result_balance {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_balance_result = o;
            }
        }

        elem_balance_result.set_result_integer(periods as i32);

        Ok(elem_balance_result)
    }

    /// Calculates the value for the current principal change event.
    /// Calculates a principal amount that will satisfy the
    /// condition that the remaining balance of the cashflow
    /// is the smallest amount greater than or equal to the given
    /// parameter value.
    ///
    /// # Arguments
    ///
    /// * `list_event` - The event list to calculate.
    /// * `list_am` - The amortization list to balance.
    /// * `list_statistic_helper` - The list of active statistic elements.
    /// * `new_value` - The desired remaining balance or dec_zero.
    ///
    /// # Return
    ///
    /// * The results from this method or an error code.

    pub fn calculate_principal(
        &self,
        list_event: &ListEvent,
        list_am: &mut ListAmortization,
        list_statistic_helper: &mut ListStatisticHelper,
        new_value: Decimal,
    ) -> Result<ElemBalanceResult, crate::ErrorType> {
        let mut adjust_negative = false;
        if list_event.elem_type() == crate::ExtensionType::PrincipalChange {
            let pc_type = list_event.elem_extension().pc_type();
            adjust_negative = pc_type == crate::PrincipalType::Negative
                || pc_type == crate::PrincipalType::Decrease;
        }

        let value_expr_am = !list_event.value_expr().is_empty() && !list_event.value_expr_balance();
        if !value_expr_am {
            let result = self.expand_with_list(list_event, list_am, true);
            match result {
                Err(e) => {
                    return Err(e);
                }
                Ok(_o) => {}
            }
        }

        let mut elem_balance_result = ElemBalanceResult::new();

        let dec_zero = dec!(0.0);
        let dec_two = dec!(2.0);
        let dec_ten = dec!(10.0);
        let max_calc_principal = dec!(crate::MAX_CALC_PRINCIPAL);

        let decrement_fraction =
            dec!(1.0) / CoreUtility::decimal_pow(dec_ten, self.decimal_digits.get() + 1);
        let mut last_principal = dec!(-1.0);
        let mut principal = dec!(20000.0); // Starting principal
        let mut orig_principal: Decimal;
        let mut iterations: usize = 1;
        let event_index = list_event.index();
        if event_index == usize::MAX
            || list_event.elem_type() != crate::ExtensionType::PrincipalChange
        {
            return Err(crate::ErrorType::Index);
        }

        let simple_calc = event_index + 1 == list_event.count()
            && list_event.periods() <= 1
            && list_event.value_expr().is_empty();
        if simple_calc {
            principal = dec_zero;
        }

        while iterations <= crate::MAXIMUM_ITERATIONS_CALCULATE_PRINCIPAL {
            if value_expr_am {
                list_event.set_value_result(principal);
                let result = self.expand_with_list(list_event, list_am, true);
                match result {
                    Err(e) => {
                        return Err(e);
                    }
                    Ok(_o) => {}
                }
            } else {
                list_am.set_all_index_values(event_index, principal);
            }

            let result_balance = self.balance_cashflow(
                list_am,
                list_statistic_helper,
                &elem_balance_result,
                false,
                false,
                false,
            );
            match result_balance {
                Err(e) => {
                    return Err(e);
                }
                Ok(o) => {
                    elem_balance_result = o;
                }
            }
            let balance = CoreUtility::round(
                elem_balance_result.balance()
                    + (if elem_balance_result.balance() < dec_zero {
                        -elem_balance_result.acc_balance()
                    } else {
                        elem_balance_result.acc_balance()
                    }),
                self.decimal_digits.get(),
                crate::RoundType::Bankers,
            );
            if simple_calc {
                principal = new_value - balance;
                break;
            }

            let adjust_down: bool;
            if adjust_negative {
                adjust_down = balance < new_value;
            } else {
                adjust_down = balance > new_value;
            }

            if balance == new_value
                || principal > max_calc_principal
                || (CoreUtility::round(principal, self.decimal_digits.get(), crate::RoundType::Bankers)
                    == CoreUtility::round(
                        last_principal,
                        self.decimal_digits.get(),
                        crate::RoundType::Bankers,
                    )
                    && (if elem_balance_result.polarity() > 0 {
                        balance >= new_value
                    } else {
                        balance <= new_value
                    }))
            {
                break;
            }

            orig_principal = principal;

            if adjust_down {
                if last_principal < dec_zero || (iterations == 1 && last_principal > principal) {
                    if principal > last_principal {
                        principal -= (principal - last_principal) * dec_two;
                    } else {
                        principal -= (last_principal - principal) * dec_two;
                    }
                    if principal < dec_zero {
                        principal = dec_zero;
                    }
                } else {
                    if (principal - last_principal).abs() <= decrement_fraction {
                        principal -= decrement_fraction;
                        orig_principal = principal; // Possibly terminate loop
                        if principal < dec_zero {
                            orig_principal = dec_zero;
                            principal = dec_zero;
                        }
                    } else if principal > last_principal {
                        principal -= (principal - last_principal) / dec_two;
                    } else {
                        principal -= (last_principal - principal) / dec_two;
                    }
                    iterations += 1;
                }
            } else {
                // Adjust up
                if last_principal < dec_zero || (iterations == 1 && last_principal < principal) {
                    if principal > last_principal {
                        principal += (principal - last_principal) * dec_two;
                    } else {
                        principal += (last_principal - principal) * dec_two;
                    }
                } else {
                    if (principal - last_principal).abs() <= decrement_fraction {
                        principal += decrement_fraction;
                        orig_principal = principal; // Possibly terminate loop
                    } else if principal > last_principal {
                        principal += (principal - last_principal) / dec_two;
                    } else {
                        principal += (last_principal - principal) / dec_two;
                    }
                    iterations += 1;
                }
            }

            last_principal = orig_principal;
        }

        if principal > max_calc_principal {
            principal = dec_zero;
        }

        if principal > dec_zero {
            // Make sure that rounding does not affect the result in the wrong direction
            principal =
                CoreUtility::round(principal, self.decimal_digits.get(), crate::RoundType::Bankers);
            if value_expr_am {
                list_event.set_value_result(principal);
                let result = self.expand_with_list(list_event, list_am, true);
                match result {
                    Err(e) => {
                        return Err(e);
                    }
                    Ok(_o) => {}
                }
            } else {
                list_am.set_all_index_values(event_index, principal);
            }

            let result_balance = self.balance_cashflow(
                list_am,
                list_statistic_helper,
                &elem_balance_result,
                false,
                false,
                false,
            );
            match result_balance {
                Err(e) => {
                    return Err(e);
                }
                Ok(o) => {
                    elem_balance_result = o;
                }
            }
            let balance = CoreUtility::round(
                elem_balance_result.balance()
                    + (if elem_balance_result.balance() < dec_zero {
                        -elem_balance_result.acc_balance()
                    } else {
                        elem_balance_result.acc_balance()
                    }),
                    self.decimal_digits.get(),
                crate::RoundType::Bankers,
            );

            let adjust: bool = if elem_balance_result.polarity() > 0 {
                balance < new_value
            } else {
                balance > new_value
            };
            if adjust {
                let adjust_down: bool;
                if adjust_negative {
                    adjust_down = balance < new_value;
                } else {
                    adjust_down = balance > new_value;
                }
                if adjust_down {
                    principal -= decrement_fraction * dec_ten;
                } else {
                    principal += decrement_fraction * dec_ten;
                }
            }
        }

        list_event.set_value(principal);

        if principal == dec_zero {
            return Err(crate::ErrorType::CalcPrincipal);
        }

        let result_balance = self.balance_cashflow(
            list_am,
            list_statistic_helper,
            &elem_balance_result,
            false,
            false,
            false,
        );
        match result_balance {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_balance_result = o;
            }
        }

        elem_balance_result.set_result_decimal(principal);

        Ok(elem_balance_result)
    }

    /// Calculates the value for an overall yield (i.e., APR).
    /// Calculates an overall yield value that will satisfy the
    /// condition that the remaining balance of the cashflow
    /// is the smallest amount greater than or equal to the given
    /// parameter value.
    ///
    /// # Arguments
    ///
    /// * `list_event` - The event list to calculate.
    /// * `list_am` - The amortization list to balance.
    /// * `list_statistic_helper` - The list of active statistic elements.
    /// * `new_value` - The desired remaining balance or dec_zero.
    ///
    /// # Return
    ///
    /// * The results from this method or an error code.

    pub fn calculate_yield(
        &self,
        list_event: &ListEvent,
        list_am: &mut ListAmortization,
        list_statistic_helper: &mut ListStatisticHelper,
        new_value: Decimal,
    ) -> Result<ElemBalanceResult, crate::ErrorType> {
        let dec_zero = dec!(0.0);
        let dec_two = dec!(2.0);
        let dec_ten = dec!(10.0);
        let max_calc_principal = dec!(crate::MAX_CALC_PRINCIPAL);
        let max_calc_interest = dec!(crate::MAX_CALC_INTEREST);

        let smallest_fraction =
            dec!(1.0) / CoreUtility::decimal_pow(dec_ten, crate::MAXIMUM_DISPLAY_DECIMAL_DIGITS);
        let decrement_fraction = smallest_fraction / dec_ten;
        let mut last_interest = dec!(-1.0);
        let mut calc_interest = dec_ten;
        let mut iterations: usize = 1;

        let result = self.expand_with_list(list_event, list_am, true);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(_o) => {}
        }

        let mut elem_balance_result = ElemBalanceResult::new();

        let result_balance = self.balance_cashflow(
            list_am,
            list_statistic_helper,
            &elem_balance_result,
            true,
            false,
            false,
        );
        match result_balance {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_balance_result = o;
            }
        }

        while iterations <= crate::MAXIMUM_ITERATIONS_CALCULATE_YIELD {
            list_am.set_all_interest_values(calc_interest);

            let result_balance = self.balance_cashflow(
                list_am,
                list_statistic_helper,
                &elem_balance_result,
                true,
                false,
                true,
            );
            match result_balance {
                Err(e) => {
                    return Err(e);
                }
                Ok(o) => {
                    elem_balance_result = o;
                }
            }
            let balance = CoreUtility::round(
                elem_balance_result.balance()
                    + (if elem_balance_result.balance() < dec_zero {
                        -elem_balance_result.acc_balance()
                    } else {
                        elem_balance_result.acc_balance()
                    }),
                    self.decimal_digits.get(),
                crate::RoundType::Bankers,
            );
            if balance.abs() >= max_calc_principal {
                calc_interest = dec!(2005.0);
                break;
            }

            let adjust_down: bool;
            if elem_balance_result.polarity() < 0 {
                adjust_down = balance < new_value;
            } else {
                adjust_down = balance > new_value;
            }

            if balance == new_value
                || calc_interest > max_calc_interest
                || (calc_interest - last_interest).abs() <= smallest_fraction
                    && if elem_balance_result.polarity() > 0 {
                        balance >= new_value
                    } else {
                        balance <= new_value
                    }
            {
                break;
            }

            let mut orig_interest = calc_interest;
            if adjust_down {
                if last_interest < dec_zero || (iterations == 1 && last_interest > calc_interest) {
                    if calc_interest > last_interest {
                        calc_interest -= (calc_interest - last_interest) * dec_two;
                    } else {
                        calc_interest -= (last_interest - calc_interest) * dec_two;
                    }
                    if calc_interest < dec_zero {
                        calc_interest = dec_zero;
                    }
                } else {
                    if (calc_interest - last_interest).abs() <= decrement_fraction {
                        calc_interest -= decrement_fraction;
                        orig_interest = calc_interest; // Possibly terminate loop
                        if calc_interest < dec_zero {
                            orig_interest = dec_zero;
                            calc_interest = dec_zero;
                        }
                    } else if calc_interest > last_interest {
                        calc_interest -= (calc_interest - last_interest) / dec_two;
                    } else {
                        calc_interest -= (last_interest - calc_interest) / dec_two;
                    }
                    iterations += 1;
                }
            } else {
                // Adjust up
                if last_interest < dec_zero || (iterations == 1 && last_interest < calc_interest) {
                    if calc_interest > last_interest {
                        calc_interest += (calc_interest - last_interest) * dec_two;
                    } else {
                        calc_interest += (last_interest - calc_interest) * dec_two;
                    }
                } else {
                    if (calc_interest - last_interest).abs() <= decrement_fraction {
                        calc_interest += decrement_fraction;
                        orig_interest = calc_interest; // Possibly terminate loop
                    } else if calc_interest > last_interest {
                        calc_interest += (calc_interest - last_interest) / dec_two;
                    } else {
                        calc_interest += (last_interest - calc_interest) / dec_two;
                    }
                    iterations += 1;
                }
            }

            last_interest = orig_interest;
        }

        if calc_interest > max_calc_interest {
            calc_interest = dec_zero;
        }

        if calc_interest > dec_zero {
            // Make sure that rounding does not affect the result in the wrong direction
            calc_interest = CoreUtility::round(
                calc_interest,
                crate::MAXIMUM_DISPLAY_DECIMAL_DIGITS,
                crate::RoundType::Bankers,
            );
            list_am.set_all_interest_values(calc_interest);

            let result_balance = self.balance_cashflow(
                list_am,
                list_statistic_helper,
                &elem_balance_result,
                true,
                false,
                true,
            );
            match result_balance {
                Err(e) => {
                    return Err(e);
                }
                Ok(o) => {
                    elem_balance_result = o;
                }
            }
            let balance = CoreUtility::round(
                elem_balance_result.balance()
                    + (if elem_balance_result.balance() < dec_zero {
                        -elem_balance_result.acc_balance()
                    } else {
                        elem_balance_result.acc_balance()
                    }),
                    self.decimal_digits.get(),
                crate::RoundType::Bankers,
            );

            let adjust = if elem_balance_result.polarity() > 0 {
                balance < new_value
            } else {
                balance > new_value
            };
            if adjust {
                let adjust_down: bool;
                if elem_balance_result.polarity() < 0 {
                    adjust_down = balance < new_value;
                } else {
                    adjust_down = balance > new_value;
                }
                if adjust_down {
                    calc_interest -= smallest_fraction;
                } else {
                    calc_interest += smallest_fraction;
                }
            }
        }

        if calc_interest == dec_zero {
            return Err(crate::ErrorType::CalcInterest);
        }

        elem_balance_result.set_result_yield(calc_interest);

        Ok(elem_balance_result)
    }

    /// Combine the amortization principal events from dec_two amortization element
    /// lists into a new amortization element list. All events except principal
    /// change and interest change events are discarded.
    ///
    /// # Arguments
    ///
    /// * `list_am1` - The first amortization element list to be combined.
    /// * `list_am2` - The second amortization element list to be combined.
    ///
    /// # Return
    ///
    /// * The resulting combined amortization element list or an error code.

    pub fn combine_cashflow(
        &self,
        list_am1: &ListAmortization,
        list_am2: &ListAmortization,
    ) -> Result<ListAmortization, crate::ErrorType> {
        let mut list_am_opt: Option<&ListAmortization>;
        let mut extension1_opt: Option<&ElemExtension> = None;
        let mut extension2_opt: Option<&ElemExtension> = None;

        let orig_list_index1 = list_am1.index();
        let orig_list_index2 = list_am2.index();
        let mut am_index1: usize = 0;
        let mut am_index2: usize = 0;
        let mut action: usize = 3; // Initially fetch both amortization element lists
        let mut elem_type1 = crate::ExtensionType::PrincipalChange;
        let mut elem_type2 = crate::ExtensionType::PrincipalChange;
        let mut event_date1: usize = 0;
        let mut event_date2: usize = 0;
        let mut next_element1: bool = true;
        let mut next_element2: bool = true;

        let updating_json = self.calc_mgr().updating_json();

        let mut new_list_am = ListAmortization::new();

        loop {
            if next_element1 && (action == 1 || action == 3) {
                next_element1 = list_am1.get_element(am_index1);

                if next_element1 {
                    elem_type1 = list_am1.elem_type();
                    event_date1 = list_am1.event_date();
                    extension1_opt = Option::from(list_am1.elem_extension());
                }

                am_index1 += 1;

                if next_element1
                    && elem_type1 != crate::ExtensionType::PrincipalChange
                    && elem_type1 != crate::ExtensionType::InterestChange
                {
                    continue;
                }
            }

            if next_element2 && (action == 2 || action == 3) {
                next_element2 = list_am2.get_element(am_index2);

                if next_element2 {
                    elem_type2 = list_am1.elem_type();
                    event_date2 = list_am2.event_date();
                    extension2_opt = Option::from(list_am2.elem_extension());
                }

                am_index2 += 1;

                if next_element2
                    && elem_type2 != crate::ExtensionType::InterestChange
                    && elem_type2 != crate::ExtensionType::PrincipalChange
                {
                    continue;
                }
            }

            action = 0;
            if next_element1 && next_element2 {
                let extension1: &ElemExtension;
                match extension1_opt.as_ref() {
                    None => {
                        return Err(crate::ErrorType::Index);
                    }
                    Some(o) => {
                        extension1 = o;
                    }
                }

                let extension2: &ElemExtension;
                match extension2_opt.as_ref() {
                    None => {
                        return Err(crate::ErrorType::Index);
                    }
                    Some(o) => {
                        extension2 = o;
                    }
                }
                if event_date1 < event_date2 {
                    action = 1;
                } else if event_date1 > event_date2 {
                    action = 2;
                } else if event_date1 == event_date2 && elem_type1 == elem_type2 {
                    if elem_type1 == crate::ExtensionType::InterestChange {
                        if extension1.ic_method() == extension2.ic_method()
                            && extension1.ic_day_count_basis() == extension2.ic_day_count_basis()
                            && extension1.ic_days_in_year() == extension2.ic_days_in_year()
                            && extension1.ic_effective_frequency()
                                == extension2.ic_effective_frequency()
                            && extension1.ic_interest_frequency()
                                == extension2.ic_interest_frequency()
                            && extension1.ic_round_balance() == extension2.ic_round_balance()
                            && extension1.ic_round_decimal_digits()
                                == extension2.ic_round_decimal_digits()
                        {
                            action = 3; // Combine
                        } else {
                            action = 1; // Default ListAm1
                        }
                    } else {
                        // PrincipalChange
                        if extension1.pc_type() == extension2.pc_type()
                            && extension1.pc_auxiliary() == extension2.pc_auxiliary()
                            && extension1.pc_aux_passive() == extension2.pc_aux_passive()
                            && extension1.pc_principal_first() == extension2.pc_principal_first()
                            && extension1.pc_balance_statistics()
                                == extension2.pc_balance_statistics()
                            && extension1.pc_eom() == extension2.pc_eom()
                        {
                            action = 3; // Combine
                        } else {
                            action = 1; // Default ListAm1
                        }
                    }
                } else {
                    action = 1; // Default ListAm1
                }
            } else if next_element1 {
                action = 1;
            } else if next_element2 {
                action = 2;
            }

            if action == 0 {
                break;
            }
            if action == 1 || action == 3 {
                list_am_opt = Option::from(list_am1);
            } else {
                list_am_opt = Option::from(list_am2);
            }
            let list_am: &ListAmortization;
            match list_am_opt.as_ref() {
                None => {
                    return Err(crate::ErrorType::Index);
                }
                Some(o) => {
                    list_am = o;
                }
            }

            let mut list_descriptor_copy: ListDescriptor;
            match list_am.list_descriptor().as_ref() {
                None => {
                    return Err(crate::ErrorType::Index);
                }
                Some(o) => {
                    list_descriptor_copy = o.copy(false, updating_json);
                }
            }

            let mut value: Decimal = list_am.value();

            if action == 3 {
                // Combine
                if elem_type1 == crate::ExtensionType::PrincipalChange {
                    value += list_am2.value();
                }
                match list_am2.list_descriptor().as_ref() {
                    None => {
                        return Err(crate::ErrorType::Index);
                    }
                    Some(o) => {
                        o.copy_list_descriptor(&mut list_descriptor_copy, false, updating_json);
                    }
                }
            }

            let new_elem_extension = list_am.elem_extension().copy();

            let list_parameter: Option<ListParameter>;
            match list_am.list_parameter().as_ref() {
                None => {
                    return Err(crate::ErrorType::Index);
                }
                Some(o) => {
                    list_parameter = Option::from(o.copy(updating_json));
                }
            }
            new_list_am.add_amortization(
                list_am.event_type(),
                list_am.event_date(),
                list_am.event_date(),
                list_am.sort_order(),
                value,
                "",
                list_am.periods(),
                list_am.intervals(),
                list_am.frequency(),
                dec!(0.0),
                dec!(0.0),
                list_am.list_event_index(),
                list_am.event_sequence(),
                list_am.stat_sequence(),
                new_elem_extension,
                list_parameter,
                Option::from(list_descriptor_copy),
            );
        }
        list_am1.get_element(orig_list_index1);
        list_am2.get_element(orig_list_index2);

        Ok(new_list_am)
    }

    /// Create a cashflow for output that may include rollups
    /// that combine principal change events that are identical
    /// except their dates but maintain a periodic flow according
    /// to the frequencies.
    ///
    /// # Arguments
    ///
    /// * `list_am` - The amortization list to produce.
    /// * `include_rollups` - Include rollup elements.
    /// * `include_details` - Include detail elements.
    /// * `compress_descriptor` - If true merge ListDescriptor
    ///     objects into a single ListDescriptor object where applicable,
    ///     otherwise do not compress amortization elements that have different
    ///     ListDescriptor objects.
    /// * `omit_statistic_events` - If true the statistic events are
    ///     eliminated from the resulting event list.
    /// * `updating_json` - Updating from Json.
    ///
    /// # Return
    ///
    /// * The resulting amortization list or an error code.

    pub fn create_cashflow_output(
        &self,
        list_am: &ListAmortization,
        include_rollups: bool,
        include_details: bool,
        compress_descriptor: bool,
        omit_statistic_events: bool,
        updating_json: bool,
    ) -> Result<ListAmortization, crate::ErrorType> {
        if !include_rollups {
            // Just return a copy of the amortization list.
            return Ok(list_am.copy(updating_json));
        }

        let orig_list_index = list_am.index();

        let mut rollup_list_am = ListAmortization::new();
        let mut am_index: usize = 0;
        while am_index < list_am.count() {
            if !list_am.get_element(am_index) {
                break;
            }

            let am_index_start = am_index;
            am_index += 1;

            let new_type = list_am.elem_type();
            if omit_statistic_events && new_type == crate::ExtensionType::StatisticValue {
                continue;
            }

            let new_event_type = list_am.event_type();
            let mut new_date = list_am.event_date();
            let orig_date = new_date;
            let new_sort = list_am.sort_order();
            let new_value = list_am.value();
            let new_value_expr = list_am.value_expr();
            let mut new_principal_decrease = list_am.principal_decrease();
            let mut new_principal_increase = list_am.principal_increase();
            let mut new_interest = list_am.interest();
            let mut new_sl_interest = list_am.sl_interest();
            let mut new_value_to_interest = list_am.value_to_interest();
            let mut new_value_to_principal = list_am.value_to_principal();
            let new_acc_balance = list_am.acc_balance();
            let new_balance = list_am.balance();
            let new_list_event_index = list_am.list_event_index();
            let mut new_periods = 1;
            let new_intervals = list_am.intervals();
            let new_frequency = list_am.frequency();

            let new_elem_extension = list_am.elem_extension().copy();
            let new_eom = new_elem_extension.extension_eom();
            let new_list_parameter: ListParameter;
            match list_am.list_parameter().as_ref() {
                None => {
                    return Err(crate::ErrorType::Index);
                }
                Some(o) => {
                    new_list_parameter = o.copy(updating_json);
                }
            }
            let mut new_list_descriptor: ListDescriptor;
            match list_am.list_descriptor().as_ref() {
                None => {
                    return Err(crate::ErrorType::Index);
                }
                Some(o) => {
                    new_list_descriptor = o.copy(false, updating_json);
                }
            }

            while am_index < list_am.count() {
                if !list_am.get_element(am_index) {
                    break;
                }
                let elem_type = list_am.elem_type();
                let event_date = list_am.event_date();
                let value = list_am.value();
                let value_expr = list_am.value_expr();
                let intervals = list_am.intervals();
                let frequency = list_am.frequency();
                let elem_extension = list_am.elem_extension();
                let list_descriptor = list_am.list_descriptor();

                if !(omit_statistic_events && elem_type == crate::ExtensionType::StatisticValue) {
                    if new_type != elem_type {
                        break;
                    }

                    match new_type {
                        crate::ExtensionType::PrincipalChange
                        | crate::ExtensionType::CurrentValue => {
                            if !elem_extension.equal(&new_elem_extension) {
                                break;
                            }
                        }
                        _ => {
                            break;
                        }
                    }
                    let mut break_on_descriptor = false;
                    if !compress_descriptor {
                        match list_descriptor.as_ref() {
                            None => {
                                return Err(crate::ErrorType::Index);
                            }
                            Some(o) => {
                                break_on_descriptor = !new_list_descriptor.equal(o);
                            }
                        }
                    }

                    if new_type != elem_type
                        || new_value != value
                        || new_value_expr != value_expr
                        || new_intervals != intervals
                        || new_frequency != frequency
                        || break_on_descriptor
                    {
                        break;
                    }

                    new_date = CoreUtility::date_new(
                        orig_date,
                        new_date,
                        new_frequency,
                        new_intervals,
                        new_eom,
                    );
                    if new_date != event_date {
                        break;
                    }

                    if compress_descriptor {
                        match list_descriptor.as_ref() {
                            None => {
                                return Err(crate::ErrorType::Index);
                            }
                            Some(o) => {
                                o.copy_list_descriptor(
                                    &mut new_list_descriptor,
                                    false,
                                    updating_json,
                                );
                            }
                        }
                    }

                    if new_type == crate::ExtensionType::PrincipalChange {
                        match new_elem_extension.pc_type() {
                            crate::PrincipalType::Negative => {
                                new_principal_decrease = value;
                            }
                            crate::PrincipalType::Positive => {
                                new_principal_increase = value;
                            }
                            crate::PrincipalType::Decrease => {
                                new_principal_decrease += value;
                            }
                            _ => {
                                new_principal_increase += value;
                            }
                        }
                    }
                    new_interest += list_am.interest();
                    new_sl_interest += list_am.sl_interest();
                    new_value_to_interest += list_am.value_to_interest();
                    new_value_to_principal += list_am.value_to_principal();
                    new_periods += 1;
                }
                am_index += 1;
            }

            if new_periods > 1 {
                // Add a rollup element
                rollup_list_am.add_amortization_ex(
                    new_event_type,
                    orig_date,
                    orig_date,
                    new_sort,
                    new_value,
                    new_value_expr,
                    new_periods,
                    new_intervals,
                    new_frequency,
                    new_principal_decrease,
                    new_principal_increase,
                    new_interest,
                    new_sl_interest,
                    new_value_to_interest,
                    new_value_to_principal,
                    new_acc_balance,
                    new_balance,
                    new_list_event_index,
                    am_index_start,
                    0,
                    new_elem_extension,
                    Option::from(new_list_parameter),
                    Option::from(new_list_descriptor),
                );
            }
        }

        let mut new_list_am = ListAmortization::new();
        let mut rollup_periods: usize = 0;
        let mut rollup_index: i32 = 0;
        let mut last_rollup_index: i32 = -1;
        am_index = 0;

        while am_index < list_am.count() {
            if !list_am.get_element(am_index) {
                break;
            }
            am_index += 1;

            let new_event_type = list_am.event_type();
            let new_type = list_am.elem_type();

            if omit_statistic_events && new_type == crate::ExtensionType::StatisticValue {
                continue;
            }

            if rollup_periods == 0 && (rollup_index as usize) < rollup_list_am.count() {
                if rollup_index > last_rollup_index {
                    rollup_list_am.get_element(rollup_index as usize);
                    last_rollup_index = rollup_index;
                }

                if rollup_index == last_rollup_index && am_index > rollup_list_am.event_sequence() {
                    let new_date = rollup_list_am.event_date();
                    let new_sort = rollup_list_am.sort_order();
                    let new_value = rollup_list_am.value();
                    let new_value_expr = rollup_list_am.value_expr();
                    let new_principal_decrease = rollup_list_am.principal_decrease();
                    let new_principal_increase = rollup_list_am.principal_increase();
                    let new_interest = rollup_list_am.interest();
                    let new_sl_interest = rollup_list_am.sl_interest();
                    let new_value_to_interest = rollup_list_am.value_to_interest();
                    let new_value_to_principal = rollup_list_am.value_to_principal();
                    let new_acc_balance = rollup_list_am.acc_balance();
                    let new_balance = rollup_list_am.balance();
                    let new_list_event_index = rollup_list_am.list_event_index();
                    let new_event_sequence = list_am.event_sequence();
                    let new_stat_sequence = list_am.stat_sequence();
                    let new_periods = rollup_list_am.periods();
                    let new_intervals = rollup_list_am.intervals();
                    let new_frequency = rollup_list_am.frequency();
                    let new_elem_extension = rollup_list_am.elem_extension().copy();
                    let new_list_parameter: ListParameter;
                    match rollup_list_am.list_parameter().as_ref() {
                        None => {
                            return Err(crate::ErrorType::Index);
                        }
                        Some(o) => {
                            new_list_parameter = o.copy(updating_json);
                        }
                    }
                    let new_list_descriptor: ListDescriptor;
                    match rollup_list_am.list_descriptor().as_ref() {
                        None => {
                            return Err(crate::ErrorType::Index);
                        }
                        Some(o) => {
                            new_list_descriptor = o.copy(false, updating_json);
                        }
                    }

                    new_list_am.add_amortization_ex(
                        rollup_list_am.event_type(),
                        new_date,
                        new_date,
                        new_sort,
                        new_value,
                        new_value_expr,
                        new_periods,
                        new_intervals,
                        new_frequency,
                        new_principal_decrease,
                        new_principal_increase,
                        new_interest,
                        new_sl_interest,
                        new_value_to_interest,
                        new_value_to_principal,
                        new_acc_balance,
                        new_balance,
                        new_list_event_index,
                        new_event_sequence,
                        new_stat_sequence,
                        new_elem_extension,
                        Option::from(new_list_parameter),
                        Option::from(new_list_descriptor),
                    );

                    rollup_periods = new_periods;
                    rollup_index += 1;
                }
            }

            if include_details || rollup_periods == 0 {
                let new_date = list_am.event_date();
                let new_sort = list_am.sort_order();
                let new_value = list_am.value();
                let new_value_expr = list_am.value_expr();
                let new_principal_decrease = list_am.principal_decrease();
                let new_principal_increase = list_am.principal_increase();
                let new_interest = list_am.interest();
                let new_sl_interest = list_am.sl_interest();
                let new_value_to_interest = list_am.value_to_interest();
                let new_value_to_principal = list_am.value_to_principal();
                let new_acc_balance = list_am.acc_balance();
                let new_balance = list_am.balance();
                let new_list_event_index = list_am.list_event_index();
                let new_event_sequence = list_am.event_sequence();
                let new_stat_sequence = list_am.stat_sequence();
                let new_periods = 1;
                let new_intervals = list_am.intervals();
                let new_frequency = list_am.frequency();

                let new_elem_extension = list_am.elem_extension().copy();
                let new_list_parameter: ListParameter;
                match list_am.list_parameter().as_ref() {
                    None => {
                        return Err(crate::ErrorType::Index);
                    }
                    Some(o) => {
                        new_list_parameter = o.copy(updating_json);
                    }
                }
                let new_list_descriptor: ListDescriptor;
                match list_am.list_descriptor().as_ref() {
                    None => {
                        return Err(crate::ErrorType::Index);
                    }
                    Some(o) => {
                        new_list_descriptor = o.copy(false, updating_json);
                    }
                }

                new_list_am.add_amortization_ex(
                    new_event_type,
                    new_date,
                    new_date,
                    new_sort,
                    new_value,
                    new_value_expr,
                    new_periods,
                    new_intervals,
                    new_frequency,
                    new_principal_decrease,
                    new_principal_increase,
                    new_interest,
                    new_sl_interest,
                    new_value_to_interest,
                    new_value_to_principal,
                    new_acc_balance,
                    new_balance,
                    new_list_event_index,
                    new_event_sequence,
                    new_stat_sequence,
                    new_elem_extension,
                    Option::from(new_list_parameter),
                    Option::from(new_list_descriptor),
                );
            }

            if rollup_periods > 0 {
                rollup_periods -= 1;
            }
        }
        if !list_am.get_element(orig_list_index) {
            list_am.get_element(0);
        }

        Ok(new_list_am)
    }

    /// Create an amortization list that is composed of the number
    /// of periods, intervals, and frequency for each event.
    /// Interest events are additionally expanded based upon
    /// their amortization frequency.
    ///
    /// # Arguments
    ///
    /// * `list_event` - The event list to create.
    /// * `optimize` - If true the ListDescriptor for each event
    ///     is not evaluated and statistic events are not expanded in the cashflow.
    ///
    /// # Return
    ///
    /// * The amortization list or an error code.

    pub fn expand_cashflow(
        &self,
        list_event: &ListEvent,
        optimize: bool,
    ) -> Result<ListAmortization, crate::ErrorType> {
        let mut list_am = ListAmortization::new();

        let result = self.expand_with_list(list_event, &mut list_am, optimize);

        match result {
            Err(e) => Err(e),
            Ok(_o) => Ok(list_am),
        }
    }

    /// Create an amortization list that is composed of the number
    /// of periods, intervals, and frequency for each event.
    /// Interest events are additionally expanded based upon
    /// their amortization frequency.
    ///
    /// # Arguments
    ///
    /// * `list_event` - The event list to create.
    /// * `list_am` - The resulting amortization element list.
    /// * `optimize` - If true the ListDescriptor for each event
    ///     is not evaluated and statistic events are not expanded in the cashflow.
    ///
    /// # Return
    ///
    /// * An success (Ok) or an error code.

    fn expand_with_list(
        &self,
        list_event: &ListEvent,
        list_am: &mut ListAmortization,
        optimize: bool,
    ) -> Result<(), crate::ErrorType> {
        let orig_list_index = list_event.index();
        let updating_json = self.calc_mgr().updating_json();
        self.statistic_event_seen.set(false);

        self.expr_mut().clear();
        list_am.clear();

        let mut event_index = 0;
        while event_index < list_event.count() {
            if !list_event.get_element(event_index) {
                break;
            }

            let mut event_sequence: usize = 0;
            let event_type = list_event.event_type();
            let elem_type = list_event.elem_type();
            let mut event_date = list_event.event_date();
            let orig_date = event_date;
            let sort_order = list_event.sort_order();
            let mut value = list_event.value();
            let orig_value = value;
            let value_expr = list_event.value_expr();
            let value_expr_balance = list_event.value_expr_balance();
            let list_parameter: ListParameter;
            match list_event.list_parameter().as_ref() {
                None => {
                    return Err(crate::ErrorType::Index);
                }
                Some(o) => {
                    list_parameter = o.copy(updating_json);
                }
            }
            let mut periods: usize = 0;
            if list_event.periods_expr().is_empty() {
                periods = list_event.periods();
            } else {
                self.expr_mut().init_expression(
                    self.list_descriptor_cashflow.as_ref(),
                    None,
                    Option::from(&list_parameter),
                    list_event.periods_expr(),
                );

                self.expr_mut()
                    .set_symbol_integer("intPeriods", list_event.periods());
                let result_symbol: ElemSymbol;
                let result = self.expr().evaluate(None, None);
                match result {
                    Err(e) => {
                        list_event.get_element(orig_list_index);
                        return Err(e);
                    }
                    Ok(o) => {
                        result_symbol = o;
                    }
                }

                match result_symbol.sym_type() {
                    crate::TokenType::Integer => {
                        periods = result_symbol.sym_integer();
                    }
                    crate::TokenType::Decimal => match result_symbol.sym_decimal().to_usize() {
                        None => {}
                        Some(o) => {
                            periods = o;
                        }
                    },
                    crate::TokenType::String => {
                        periods = CoreUtility::parse_integer(result_symbol.sym_string());
                    }
                    _ => {}
                }
            }

            if periods == 0 {
                periods = 1;
            }
            let mut orig_periods = periods;

            let mut intervals = list_event.intervals();
            if intervals == 0 {
                intervals = 1;
            }

            let frequency = list_event.frequency();
            let mut am_frequency = frequency;
            let mut principal_decrease: Decimal = dec!(0.0);
            let mut principal_increase: Decimal = dec!(0.0);

            let list_desc = list_event.list_descriptor();
            let list_descriptor: &ListDescriptor;
            match list_desc.as_ref() {
                None => {
                    return Err(crate::ErrorType::Index);
                }
                Some(o) => {
                    list_descriptor = o;
                }
            }

            let orig_descriptor_list_index = list_descriptor.index();

            for index in 0..list_descriptor.count() {
                if !list_descriptor.get_element(index) {
                    break;
                }
                list_descriptor.set_list_event_index(event_index); // Mark descriptor
            }

            list_descriptor.get_element(orig_descriptor_list_index);

            let new_eom = list_event.elem_extension().extension_eom();

            match elem_type {
                crate::ExtensionType::InterestChange => {
                    if list_event.elem_extension().ic_interest_frequency()
                        != crate::FrequencyType::None
                    {
                        am_frequency = list_event.elem_extension().ic_interest_frequency();
                    }
                }
                crate::ExtensionType::StatisticValue => {
                    periods = 1; // Perpetual until next statistic with the same name is seen
                    orig_periods = periods;
                    list_event.set_skip_mask(0, 0);
                    self.statistic_event_seen.set(true);
                }
                _ => {}
            }

            if !optimize {
                let mut new_list_parameter = list_parameter.copy(updating_json);
                new_list_parameter.add_parameter("intDate", updating_json);
                new_list_parameter.set_integer(orig_date);
                new_list_parameter.add_parameter("decValue", updating_json);
                new_list_parameter.set_decimal(orig_value);
                new_list_parameter.add_parameter("intPeriods", updating_json);
                new_list_parameter.set_integer(periods);
                new_list_parameter.add_parameter("intIntervals", updating_json);
                new_list_parameter.set_integer(intervals);
                new_list_parameter.add_parameter("strFrequency", updating_json);
                new_list_parameter
                    .set_string(CoreUtility::get_frequency_mnemonic(frequency).as_str());
                new_list_parameter.add_parameter("intEOM", updating_json);
                new_list_parameter.set_integer(if new_eom { 1 } else { 0 });

                CalcUtility::evaluate_descriptors(
                    // Evaluate all the descriptors in the event
                    &self.calc_manager,
                    &self.calc_expression,
                    &new_list_parameter,
                    list_descriptor,
                );
            }

            let skip_mask_len = list_event.skip_mask_len();
            let skip_mask = list_event.skip_mask();

            while periods > 0 {
                if skip_mask_len == 0
                    || (skip_mask & (1 << ((orig_periods - periods) % skip_mask_len))) == 0
                        && !(optimize && elem_type == crate::ExtensionType::StatisticValue)
                {
                    if !value_expr.is_empty() && !value_expr_balance {
                        self.expr_mut().init_expression(
                            self.list_descriptor_cashflow.as_ref(),
                            None,
                            Option::from(&list_parameter),
                            value_expr,
                        );

                        self.expr_mut().set_symbol_decimal("decValue", orig_value);
                        self.expr_mut()
                            .set_symbol_integer("intSequence", event_sequence);

                        let result_symbol: ElemSymbol;
                        let result = self.expr().evaluate(None, None);
                        match result {
                            Err(e) => {
                                list_event.get_element(orig_list_index);
                                return Err(e);
                            }
                            Ok(o) => {
                                result_symbol = o;
                            }
                        }

                        match result_symbol.sym_type() {
                            crate::TokenType::Integer => {
                                value = dec!(result_symbol.sym_integer());
                            }
                            crate::TokenType::Decimal => {
                                value = result_symbol.sym_decimal();
                            }
                            crate::TokenType::String => {
                                value = CoreUtility::parse_decimal(result_symbol.sym_string());
                            }
                            _ => {}
                        }
                    }

                    if elem_type == crate::ExtensionType::PrincipalChange {
                        match list_event.elem_extension().pc_type() {
                            crate::PrincipalType::Negative | crate::PrincipalType::Decrease => {
                                principal_decrease = value;
                            }
                            _ => {
                                principal_increase = value;
                            }
                        }
                    }

                    let new_elem_extension = list_event.elem_extension().copy();
                    list_am.add_amortization(
                        event_type,
                        orig_date,
                        event_date,
                        sort_order,
                        value,
                        if value_expr_balance { value_expr } else { "" },
                        1,
                        intervals,
                        frequency,
                        principal_decrease,
                        principal_increase,
                        event_index,
                        event_sequence,
                        0,
                        new_elem_extension,
                        Option::from(list_parameter.copy(updating_json)),
                        Option::from(list_descriptor.copy(false, updating_json)),
                    );

                    event_sequence += 1;
                }

                periods -= 1;
                if periods > 0 {
                    event_date = CoreUtility::date_new(
                        orig_date,
                        event_date,
                        am_frequency,
                        intervals,
                        new_eom,
                    );
                }
            }

            event_index += 1;
        }

        list_am.sort();
        list_am.get_element(0);
        list_event.get_element(orig_list_index);

        Ok(())
    }

    /// Calculates the compounded and straight-line interest due for the interest period.
    /// Upon return, sets the module-level variables interest and sl_interest.
    /// Based upon the previous interest date self.last_interest_date and the event_date parameter,
    /// an interest period is established. The interest period is split up into sub-periods
    /// consisting of whole interest frequency periods and a possible stub period that is
    /// smaller than an interest frequency period. If a stub period does exists, it is always
    /// aligned at the beginning of the interest period. The previous interest date
    /// last_interest_date reflects the parameter event_date following a call to this method.
    ///
    /// # Arguments
    ///
    /// * `method` - Interest method used.
    /// * `day_count_basis` - Day count basis.
    /// * `days_in_year` - Number of days in the year.
    /// * `orig_date` - The original date of the current event.
    /// * `event_date` - The new interest date to compare with self.last_interest_date.
    /// * `frequency` - Interest frequency.
    /// * `effective_frequency` - Optional effective frequency.
    /// * `int_rate` - The current nominal annual interest rate expressed as a percentage.
    /// * `balance` - The current balance of the cashflow.
    /// * `sl_balance` - The straight-line balance of the cashflow.
    /// * `eom` - Adjust successive dates to end of month.
    #[allow(clippy::too_many_arguments)]

    pub fn interest(
        &self,
        method: crate::MethodType,
        day_count_basis: crate::DayCountType,
        days_in_year: usize,
        orig_date: usize,
        event_date: usize,
        frequency: crate::FrequencyType,
        effective_frequency: crate::FrequencyType,
        int_rate: Decimal,
        mut balance: Decimal,
        mut sl_balance: Decimal,
        eom: bool,
    ) {
        self.interest.set(dec!(0.0));
        self.sl_interest.set(dec!(0.0));

        let mut nominal_rate = int_rate / dec!(100.0);

        if !(effective_frequency == crate::FrequencyType::None || effective_frequency == frequency)
        {
            nominal_rate = CoreUtility::rate_eff_to_nom(
                nominal_rate,
                frequency,
                effective_frequency,
                days_in_year,
            );
        }
        let periodic_rate = CoreUtility::rate_nar_to_pr(nominal_rate, frequency, days_in_year);

        let mut last_interest_serial = CoreUtility::date_to_serial(self.last_interest_date.get());
        let event_date_serial = CoreUtility::date_to_serial(event_date);
        let mut comp_date = event_date;
        let mut comp_serial = event_date_serial;
        let periods_in_year = CoreUtility::intervals_in_year(frequency, days_in_year);
        let mut first_positive = balance >= dec!(0.0);
        let mut stub_period;
        if balance < dec!(0.0) {
            balance = -balance;
        }

        if sl_balance < dec!(0.0) {
            sl_balance = -sl_balance;
        }

        let mut calc_int = dec!(0.0);
        let mut calc_bal = balance;

        loop {
            let prev_date = CoreUtility::date_newi(orig_date, comp_date, frequency, -1, eom);
            let prev_serial = CoreUtility::date_to_serial(prev_date);

            if first_positive {
                if last_interest_serial > prev_serial {
                    return; // Positive cashflow and a whole compounding period has not past yet
                }

                first_positive = false;
            }

            stub_period = prev_serial < last_interest_serial;

            if prev_serial <= last_interest_serial {
                break;
            }
            comp_date = prev_date;
            comp_serial = prev_serial;
        }
        loop {
            let day_count_factor = CoreUtility::day_count_factor(
                last_interest_serial,
                comp_serial,
                day_count_basis,
                days_in_year,
                periods_in_year,
            );
            let calc_interest: Decimal;
            if frequency == crate::FrequencyType::Continuous {
                calc_interest =
                    calc_bal * CoreUtility::decimal_exp(nominal_rate * day_count_factor) - calc_bal;
            } else if (day_count_basis == crate::DayCountType::Periodic
                || day_count_basis == crate::DayCountType::RuleOf78)
                && !stub_period
            {
                calc_interest = calc_bal * periodic_rate;
            } else {
                calc_interest = calc_bal * nominal_rate * day_count_factor;
            }
            if method == crate::MethodType::SimpleInterest {
                calc_int += calc_interest;
            } else {
                // METHOD_ACTUARIAL
                calc_bal += calc_interest;
            }
            if frequency == crate::FrequencyType::Continuous {
                self.sl_interest.set(
                    self.sl_interest.get()
                        + (sl_balance * CoreUtility::decimal_exp(nominal_rate * day_count_factor)
                            - sl_balance),
                );
            } else if (day_count_basis == crate::DayCountType::Periodic
                || day_count_basis == crate::DayCountType::RuleOf78)
                && !stub_period
            {
                self.sl_interest
                    .set(self.sl_interest.get() + (sl_balance * periodic_rate));
            } else {
                self.sl_interest
                    .set(self.sl_interest.get() + (sl_balance * nominal_rate * day_count_factor));
            }

            self.last_interest_date.set(comp_date);
            last_interest_serial = comp_serial;

            if comp_serial >= event_date_serial {
                break;
            }

            comp_date = CoreUtility::date_new(orig_date, comp_date, frequency, 1, eom);
            comp_serial = CoreUtility::date_to_serial(comp_date);

            if comp_serial > event_date_serial {
                comp_date = event_date;
                comp_serial = event_date_serial;
            }
            stub_period = false; // By definition, not a stub period
        }

        self.last_interest_date.set(event_date);

        self.interest.set(calc_bal - balance + calc_int);
    }

    /// Merges dec_two event lists into a new event list according
    /// to the order defined by event_date and sort_order with a bias
    /// towards the first list given equal sort keys.
    /// All events except principal change and interest change
    /// events are discarded
    ///
    /// # Arguments
    ///
    /// * `list_event1` - The first event list to be merged.
    /// * `list_event2` - The second event list to be merged.
    /// * `interest_event_action` - The action to be performed
    ///     when interest events are encountered.
    ///
    /// # Return
    ///
    /// * The resulting event list or an error code.

    pub fn merge_cashflow(
        &self,
        list_event1: &ListEvent,
        list_event2: &ListEvent,
        interest_event_action: crate::MergeType,
    ) -> Result<ListEvent, crate::ErrorType> {
        let orig_list_index1 = list_event1.index();
        let orig_list_index2 = list_event2.index();
        let mut event_index1: usize = 0;
        let mut event_index2: usize = 0;
        let mut action = crate::MergeType::IntAll; // Initially fetch both event lists
        let mut event_date1: usize = 0;
        let mut event_date2: usize = 0;
        let mut sort1: usize = 0;
        let mut sort2: usize = 0;
        let mut next_element1: bool = true;
        let mut next_element2: bool = true;
        let updating_json = self.calc_mgr().updating_json();

        let mut new_list_event = ListEvent::new(list_event1.cashflow());
        new_list_event.set_sort_on_add(false);

        loop {
            if next_element1
                && (action == crate::MergeType::Int1 || action == crate::MergeType::IntAll)
            {
                next_element1 = list_event1.get_element(event_index1);

                if next_element1 {
                    event_date1 = list_event1.event_date();
                    sort1 = list_event1.sort_order();
                }

                event_index1 += 1;
            }

            if next_element2
                && (action == crate::MergeType::Int2 || action == crate::MergeType::IntAll)
            {
                next_element2 = list_event2.get_element(event_index2);
                if next_element2 {
                    event_date2 = list_event2.event_date();
                    sort2 = list_event2.sort_order();
                }

                event_index2 += 1;
            }

            action = crate::MergeType::IntNone;
            if next_element1 && next_element2 {
                if event_date1 < event_date2 {
                    action = crate::MergeType::Int1;
                } else if event_date1 > event_date2 {
                    action = crate::MergeType::Int2;
                } else if sort1 < sort2 {
                    action = crate::MergeType::Int1;
                } else if sort1 > sort2 {
                    action = crate::MergeType::Int2;
                } else {
                    action = crate::MergeType::Int1; // Default ListEvent1
                }
            } else if next_element1 {
                action = crate::MergeType::Int1;
            } else if next_element2 {
                action = crate::MergeType::Int2;
            }

            if action == crate::MergeType::IntNone {
                break;
            }

            let list_event = if action == crate::MergeType::Int1 {
                list_event1
            } else {
                list_event2
            };
            let index = list_event.index();

            match list_event.elem_type() {
                crate::ExtensionType::PrincipalChange => {
                    let result = list_event.copy_event(&mut new_list_event, index, updating_json);
                    match result {
                        Err(e) => {
                            return Err(e);
                        }
                        Ok(_o) => {}
                    }
                }
                crate::ExtensionType::InterestChange => {
                    if interest_event_action == crate::MergeType::IntAll
                        || interest_event_action == action
                    {
                        let result =
                            list_event.copy_event(&mut new_list_event, index, updating_json);
                        match result {
                            Err(e) => {
                                return Err(e);
                            }
                            Ok(_o) => {}
                        }
                    }
                }
                _ => {}
            }
        }
        new_list_event.set_sort_on_add(true); // Sorts list
        list_event1.get_element(orig_list_index1);
        list_event2.get_element(orig_list_index2);

        Ok(new_list_event)
    }

    /// Normalize the amortization cashflow by combining principal
    /// change events that are identical except their values and
    /// their ListDescriptor objects. The ListDescriptor
    /// objects are merged into a single ListDescriptor object.
    ///
    /// # Arguments
    ///
    /// * `list_am` - The amortization list to normalize.
    /// * `list_statistic_helper` - The list of active statistic elements.
    /// * `combine_principal` - Combine principal change events that are
    ///     identical except their values and their ListDescriptor objects.
    ///     The ListDescriptor objects are merged into a single
    ///     ListDescriptor object.
    ///
    /// # Return
    ///
    /// * The normalized list amortization or an error code.

    pub fn normalize_cashflow(
        &self,
        list_am: &ListAmortization,
        list_statistic_helper: &mut ListStatisticHelper,
        combine_principal: bool,
    ) -> Result<ListAmortization, crate::ErrorType> {
        let mut new_list_am = ListAmortization::new();
        let orig_list_index = list_am.index();
        let mut new_date: usize = 0;
        let updating_json = self.calc_mgr().updating_json();
        let mut am_index: usize = 0;
        while am_index < list_am.count() {
            if !list_am.get_element(am_index) {
                break;
            }
            am_index += 1;

            new_date = list_am.event_date();
            let new_event_type = list_am.event_type();
            let new_type = list_am.elem_type();
            let new_orig_date = list_am.orig_date();
            let new_sort = list_am.sort_order();
            let mut new_value = list_am.value();
            let new_value_expr = list_am.value_expr();
            let new_periods = list_am.periods();
            let new_intervals = list_am.intervals();
            let new_frequency = list_am.frequency();
            let mut principal_decrease = list_am.principal_decrease();
            let mut principal_increase = list_am.principal_increase();

            let new_list_event_index = list_am.list_event_index();
            let new_event_sequence = list_am.event_sequence();
            let new_stat_sequence = list_am.stat_sequence();

            if new_type == crate::ExtensionType::StatisticValue {
                let stat_name = list_am.elem_extension().sv_name();
                let stat_final = list_am.elem_extension().sv_is_final();

                let bresult = list_statistic_helper.get_element_by_name(stat_name);
                if stat_final {
                    if !bresult {
                        break;
                    }
                    list_statistic_helper.remove();
                } else if bresult {
                    list_statistic_helper.set_last_date(new_date);
                    list_statistic_helper.set_elem_am_index(list_am.index());
                } else {
                    list_statistic_helper.add_statistic_helper(
                        stat_name,
                        new_date,
                        list_am.index(),
                    );
                }
            }
            let new_elem_extension = list_am.elem_extension().copy();

            let mut new_list_parameter: ListParameter;
            match list_am.list_parameter().as_ref() {
                None => {
                    return Err(crate::ErrorType::Index);
                }
                Some(o) => {
                    new_list_parameter = o.copy(updating_json);
                }
            }

            let mut new_list_descriptor: ListDescriptor;
            match list_am.list_descriptor().as_ref() {
                None => {
                    return Err(crate::ErrorType::Index);
                }
                Some(o) => {
                    new_list_descriptor = o.copy(false, updating_json);
                }
            }

            for statistic_index in 0..list_statistic_helper.count() {
                if !list_statistic_helper.get_element(statistic_index) {
                    break;
                }

                if !list_am.get_element(list_statistic_helper.elem_am_index()) {
                    continue;
                }

                if list_am.frequency() == crate::FrequencyType::Continuous {
                    continue;
                }

                let event_date = CoreUtility::date_new(
                    list_am.orig_date(),
                    list_statistic_helper.last_date(),
                    list_am.frequency(),
                    list_am.intervals(),
                    list_am.elem_extension().extension_eom(),
                );

                if new_date > event_date {
                    let elem_extension = list_am.elem_extension().copy();

                    let list_parameter: ListParameter;
                    match list_am.list_parameter().as_ref() {
                        None => {
                            return Err(crate::ErrorType::Index);
                        }
                        Some(o) => {
                            list_parameter = o.copy(updating_json);
                        }
                    }

                    let list_descriptor: ListDescriptor;
                    match list_am.list_descriptor().as_ref() {
                        None => {
                            return Err(crate::ErrorType::Index);
                        }
                        Some(o) => {
                            list_descriptor = o.copy(false, updating_json);
                        }
                    }

                    new_list_am.add_amortization(
                        list_am.event_type(),
                        list_am.orig_date(),
                        event_date,
                        list_am.sort_order(),
                        list_am.value(),
                        list_am.value_expr(),
                        list_am.periods(),
                        list_am.intervals(),
                        list_am.frequency(),
                        dec!(0.0),
                        dec!(0.0),
                        list_am.list_event_index(),
                        list_am.event_sequence(),
                        list_am.stat_sequence(),
                        elem_extension,
                        Option::from(list_parameter),
                        Option::from(list_descriptor),
                    );

                    list_statistic_helper.set_last_date(event_date);
                }
            }
            while combine_principal && am_index < list_am.count() {
                if !list_am.get_element(am_index) {
                    break;
                }

                let elem_type = list_am.elem_type();
                let event_date = list_am.event_date();
                let value = list_am.value();
                let value_expr = list_am.value_expr();
                let mut prin_change_is_equal: bool = false;
                if new_type == crate::ExtensionType::PrincipalChange {
                    prin_change_is_equal = new_elem_extension.equal(list_am.elem_extension());
                }

                if new_type != crate::ExtensionType::PrincipalChange
                    || elem_type != crate::ExtensionType::PrincipalChange
                    || new_date != event_date
                    || !new_value_expr.is_empty()
                    || !value_expr.is_empty()
                    || !prin_change_is_equal
                {
                    break;
                }

                let list_parameter = list_am.list_parameter();
                match list_parameter.as_ref() {
                    None => {
                        return Err(crate::ErrorType::Index);
                    }
                    Some(o) => {
                        if !new_list_parameter.equal(o) {
                            o.copy_list_parameter(&mut new_list_parameter, updating_json);
                        }
                    }
                }
                let list_descriptor = list_am.list_descriptor();
                match list_descriptor.as_ref() {
                    None => {
                        return Err(crate::ErrorType::Index);
                    }
                    Some(o) => {
                        if !new_list_descriptor.equal(o) {
                            o.copy_list_descriptor(&mut new_list_descriptor, false, updating_json);
                        }
                    }
                }

                if new_type == crate::ExtensionType::PrincipalChange {
                    let pc_type = new_elem_extension.pc_type();

                    if pc_type == crate::PrincipalType::Positive
                        || pc_type == crate::PrincipalType::Negative
                    {
                        new_value = value;
                    } else {
                        new_value += value;
                    }
                    match pc_type {
                        crate::PrincipalType::Negative => {
                            principal_decrease = value;
                        }
                        crate::PrincipalType::Positive => {
                            principal_increase = value;
                        }
                        crate::PrincipalType::Decrease => {
                            principal_decrease += value;
                        }
                        _ => {
                            principal_increase += value;
                        }
                    }
                }
                am_index += 1;
            }

            new_list_am.add_amortization(
                new_event_type,
                new_orig_date,
                new_date,
                new_sort,
                new_value,
                new_value_expr,
                new_periods,
                new_intervals,
                new_frequency,
                principal_decrease,
                principal_increase,
                new_list_event_index,
                new_event_sequence,
                new_stat_sequence,
                new_elem_extension,
                Option::from(new_list_parameter.copy(updating_json)),
                Option::from(new_list_descriptor.copy(false, updating_json)),
            );
        }

        for statistic_index in 0..list_statistic_helper.count() {
            if !list_statistic_helper.get_element(statistic_index) {
                break;
            }

            if !list_am.get_element(list_statistic_helper.elem_am_index()) {
                continue;
            }

            if list_am.frequency() == crate::FrequencyType::Continuous
                || new_date > list_statistic_helper.last_date()
            {
                let event_date = CoreUtility::date_new(
                    list_am.orig_date(),
                    list_statistic_helper.last_date(),
                    list_am.frequency(),
                    list_am.intervals(),
                    list_am.elem_extension().extension_eom(),
                );
                let elem_extension = list_am.elem_extension().copy();
                let list_parameter: ListParameter;
                match list_am.list_parameter().as_ref() {
                    None => {
                        return Err(crate::ErrorType::Index);
                    }
                    Some(o) => {
                        list_parameter = o.copy(updating_json);
                    }
                }

                let list_descriptor: ListDescriptor;
                match list_am.list_descriptor().as_ref() {
                    None => {
                        return Err(crate::ErrorType::Index);
                    }
                    Some(o) => {
                        list_descriptor = o.copy(false, updating_json);
                    }
                }

                new_list_am.add_amortization(
                    list_am.event_type(),
                    list_am.orig_date(),
                    event_date,
                    list_am.sort_order(),
                    list_am.value(),
                    list_am.value_expr(),
                    list_am.periods(),
                    list_am.intervals(),
                    list_am.frequency(),
                    dec!(0.0),
                    dec!(0.0),
                    list_am.list_event_index(),
                    list_am.event_sequence(),
                    list_am.stat_sequence(),
                    elem_extension,
                    Option::from(list_parameter),
                    Option::from(list_descriptor),
                );
            }
        }

        if !new_list_am.get_element(orig_list_index) {
            // Try to set via the unnormalized index
            new_list_am.get_element(0);
        }

        Ok(new_list_am)
    }

    /// Set the start of fiscal year in MMDD format.
    ///
    /// # Arguments
    ///
    /// * `fiscal_year_start_param` - Start of fiscal year in MMDD format.
    pub fn set_fiscal_year_start(&self, fiscal_year_start_param: usize) {
        self.fiscal_year_start.set(fiscal_year_start_param);
    }

    /// Set the number of significant decimal digits.
    ///
    /// # Arguments
    ///
    /// * `decimal_digits_param` - Number of significant decimal digits.
    pub fn set_decimal_digits(&self, decimal_digits_param: usize) {
        self.decimal_digits.set(decimal_digits_param);
    }

    /// Splits the currently selected principal change event into
    /// multiple principal change events whenever an active event
    /// is seen in the event list that is between the start and end
    /// dates of the principal change event.
    ///
    /// # Arguments
    ///
    /// * `list_event` - The event list to calculate.
    /// * `all_events` - If true split the principal change event when any
    ///     active event is seen (otherwise only split the principal change event
    ///     when an interest change event is seen).
    ///
    /// # Return
    ///
    /// * The amortization list of events or an error code.

    pub fn split_cashflow(
        &self,
        list_event: &mut ListEvent,
        all_events: bool,
    ) -> Result<(), crate::ErrorType> {
        let orig_event_index = list_event.index();
        if orig_event_index == usize::MAX {
            return Err(crate::ErrorType::Index);
        }

        if list_event.elem_type() != crate::ExtensionType::PrincipalChange {
            return Err(crate::ErrorType::Index);
        }
        let updating_json = self.calc_mgr().updating_json();
        let mut new_date = list_event.event_date();
        let mut end_date = new_date;
        let frequency = list_event.frequency();
        let intervals = list_event.intervals();
        let eom = list_event.elem_extension().extension_eom();
        if list_event.periods() > 1 {
            end_date = CalcManager::util_date_new(
                end_date,
                list_event.periods() - 1,
                frequency,
                intervals,
                eom,
            );
        }

        let list_am: ListAmortization;
        let result = self.expand_cashflow(list_event, true);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                list_am = o;
            }
        }

        let mut am_index: usize = 0;
        while am_index < list_am.count() {
            if !list_am.get_element(am_index) {
                break;
            }

            if list_am.list_event_index() == orig_event_index {
                break;
            }

            am_index += 1;
        }

        let mut list_periods = Vec::new();
        let mut periods: usize = 0;
        while am_index < list_am.count() {
            if !list_am.get_element(am_index) {
                break;
            }

            if list_am.event_date() > end_date {
                break;
            }

            if list_am.list_event_index() == orig_event_index {
                periods += 1;
                am_index += 1;
                continue;
            }

            if periods == 0 {
                am_index += 1;
                continue;
            }

            if all_events {
                if (list_am.elem_type() == crate::ExtensionType::PrincipalChange
                    && list_am.elem_extension().pc_aux_passive())
                    || (list_am.elem_type() == crate::ExtensionType::CurrentValue
                        && list_am.elem_extension().cv_passive())
                    || list_am.elem_type() == crate::ExtensionType::StatisticValue
                {
                    am_index += 1;
                    continue;
                }
            } else if list_am.elem_type() != crate::ExtensionType::InterestChange {
                am_index += 1;
                continue;
            }

            list_periods.push(periods);

            periods = 0;

            am_index += 1;
        }
        if periods > 0 {
            list_periods.push(periods);
        }
        if list_periods.is_empty() {
            return Err(crate::ErrorType::Index);
        }
        periods = list_periods[0];
        list_event.set_periods(periods);
        new_date = CalcManager::util_date_new(new_date, periods, frequency, intervals, eom);
        for elem_periods in list_periods {
            list_event.get_element(orig_event_index);

            let list_event_copy = list_event.copy(updating_json);

            let result = list_event_copy.copy_list_event(list_event, updating_json);
            match result {
                Err(e) => {
                    return Err(e);
                }
                Ok(_o) => {}
            }

            periods = elem_periods;
            list_event.set_date_result(new_date);
            list_event.set_periods(periods);
            new_date = CalcManager::util_date_new(new_date, periods, frequency, intervals, eom);
        }
        list_event.set_sort_updated(true);
        list_event.get_element(orig_event_index);
        Ok(())
    }

    /// Transform the amortization list by creating an event list
    /// composed of events that are either before the current value
    /// present event (if after_pv is false) or after the current
    /// value present event (if after_pv is true). The current
    /// value present event itself is transformed into a principal
    /// change event.
    ///
    /// # Arguments
    ///
    /// * `list_am` - The amortization list to transform.
    /// * `after_pv` - If true transform the events after the
    ///     current value present event. Otherwise, transform the events
    ///     before the current value present event.
    /// * `omit_interest_events` - If true the interest events are
    ///     eliminated from the resulting event list.
    /// * `all_events` - If true transforms all events regardless
    ///     of the current value present event.
    /// * `cashflow` - Originated from cashflow.
    ///
    /// # Return
    ///
    /// * The event list or an error code.

    pub fn transform_cashflow(
        &self,
        list_am: &ListAmortization,
        after_pv: bool,
        omit_interest_events: bool,
        all_events: bool,
        cashflow: bool,
    ) -> Result<ListEvent, crate::ErrorType> {
        let orig_list_index = list_am.index();
        let mut principal_index = usize::MAX;
        let mut interest_index = usize::MAX;
        let mut cv_present_seen = all_events;
        let updating_json = self.calc_mgr().updating_json();

        let mut new_list_event = ListEvent::new(cashflow);
        new_list_event.set_sort_on_add(false);
        if !cv_present_seen {
            let mut index: usize = 0;

            while index < list_am.count() {
                if !list_am.get_element(index) {
                    break;
                }

                if list_am.elem_type() == crate::ExtensionType::CurrentValue
                    && list_am.elem_extension().cv_present()
                {
                    break;
                }

                index += 1;
            }

            cv_present_seen = index >= list_am.count(); // Transform all events
        }

        let mut am_index = 0;
        while am_index < list_am.count() {
            if !list_am.get_element(am_index) {
                break;
            }

            let elem_type = list_am.elem_type();
            let event_date = list_am.event_date();
            let mut sort_order = list_am.sort_order();
            let mut value = list_am.value();
            let mut periods = list_am.periods();
            let mut intervals = list_am.intervals();
            let frequency = list_am.frequency();
            let mut list_descriptor = list_am.list_descriptor();
            match elem_type {
                crate::ExtensionType::CurrentValue => {
                    if !(after_pv || all_events) && list_am.elem_extension().cv_present() {
                        value = list_am.balance() - list_am.acc_balance();

                        let mut index = list_am.count() - 1;
                        while index > am_index {
                            if !list_am.get_element(index) {
                                break;
                            }
                            if list_am.elem_type() == crate::ExtensionType::PrincipalChange {
                                break;
                            }

                            index -= 1;
                        }

                        if list_am.elem_type() == crate::ExtensionType::PrincipalChange {
                            sort_order = list_am.sort_order();
                            periods = 1;
                            intervals = list_am.intervals();

                            let frequency = list_am.frequency();
                            list_descriptor = list_am.list_descriptor();
                            let new_elem_extension = list_am.elem_extension().copy();

                            let new_list_parameter = ListParameter::new();
                            let mut new_list_descriptor = ListDescriptor::new();

                            match list_descriptor.as_ref() {
                                None => {
                                    return Err(crate::ErrorType::Index);
                                }
                                Some(o) => {
                                    o.copy_list_descriptor(
                                        &mut new_list_descriptor,
                                        false,
                                        updating_json,
                                    );
                                }
                            }

                            new_list_event.add_event(
                                event_date,
                                "",
                                sort_order,
                                value,
                                "",
                                false,
                                periods,
                                "",
                                0,
                                0,
                                intervals,
                                frequency,
                                new_elem_extension,
                                Option::from(new_list_parameter),
                                Option::from(new_list_descriptor),
                                "",
                                "",
                            );
                        }

                        break; // end loop
                    }

                    if !cv_present_seen
                        && principal_index != usize::MAX
                        && list_am.elem_extension().cv_present()
                    {
                        if !list_am.get_element(principal_index) {
                            break;
                        }

                        sort_order = list_am.sort_order();
                        value = list_am.value();
                        periods = list_am.periods();
                        intervals = list_am.intervals();
                        let frequency = list_am.frequency();
                        list_descriptor = list_am.list_descriptor();
                        let new_elem_extension = list_am.elem_extension().copy();

                        let new_list_parameter = ListParameter::new();
                        let mut new_list_descriptor = ListDescriptor::new();

                        match list_descriptor.as_ref() {
                            None => {
                                return Err(crate::ErrorType::Index);
                            }
                            Some(o) => {
                                o.copy_list_descriptor(
                                    &mut new_list_descriptor,
                                    false,
                                    updating_json,
                                );
                            }
                        }

                        new_list_event.add_event(
                            event_date,
                            "",
                            sort_order,
                            value,
                            "",
                            false,
                            periods,
                            "",
                            0,
                            0,
                            intervals,
                            frequency,
                            new_elem_extension,
                            Option::from(new_list_parameter),
                            Option::from(new_list_descriptor),
                            "",
                            "",
                        );

                        if interest_index != usize::MAX {
                            if !list_am.get_element(interest_index) {
                                break;
                            }

                            sort_order = list_am.sort_order();
                            value = list_am.value();
                            periods = list_am.periods();
                            intervals = list_am.intervals();
                            let frequency = list_am.frequency();
                            let new_elem_extension = list_am.elem_extension().copy();

                            let new_list_parameter = ListParameter::new();
                            let mut new_list_descriptor = ListDescriptor::new();
                            match list_descriptor.as_ref() {
                                None => {
                                    return Err(crate::ErrorType::Index);
                                }
                                Some(o) => {
                                    o.copy_list_descriptor(
                                        &mut new_list_descriptor,
                                        false,
                                        updating_json,
                                    );
                                }
                            }

                            new_list_event.add_event(
                                event_date,
                                "",
                                sort_order,
                                value,
                                "",
                                false,
                                periods,
                                "",
                                0,
                                0,
                                intervals,
                                frequency,
                                new_elem_extension,
                                Option::from(new_list_parameter),
                                Option::from(new_list_descriptor),
                                "",
                                "",
                            );
                        }

                        cv_present_seen = true;
                    } else if !after_pv || cv_present_seen {
                        let new_elem_extension = list_am.elem_extension().copy();

                        let new_list_parameter = ListParameter::new();
                        let mut new_list_descriptor = ListDescriptor::new();

                        match list_descriptor.as_ref() {
                            None => {
                                return Err(crate::ErrorType::Index);
                            }
                            Some(o) => {
                                o.copy_list_descriptor(
                                    &mut new_list_descriptor,
                                    false,
                                    updating_json,
                                );
                            }
                        }

                        new_list_event.add_event(
                            event_date,
                            "",
                            sort_order,
                            value,
                            "",
                            false,
                            periods,
                            "",
                            0,
                            0,
                            intervals,
                            frequency,
                            new_elem_extension,
                            Option::from(new_list_parameter),
                            Option::from(new_list_descriptor),
                            "",
                            "",
                        );
                    }
                }
                crate::ExtensionType::InterestChange => {
                    interest_index = am_index;

                    if !((after_pv && !cv_present_seen) || omit_interest_events) {
                        let new_elem_extension = list_am.elem_extension().copy();

                        let new_list_parameter = ListParameter::new();
                        let mut new_list_descriptor = ListDescriptor::new();

                        match list_descriptor.as_ref() {
                            None => {
                                return Err(crate::ErrorType::Index);
                            }
                            Some(o) => {
                                o.copy_list_descriptor(
                                    &mut new_list_descriptor,
                                    false,
                                    updating_json,
                                );
                            }
                        }

                        new_list_event.add_event(
                            event_date,
                            "",
                            sort_order,
                            value,
                            "",
                            false,
                            periods,
                            "",
                            0,
                            0,
                            intervals,
                            frequency,
                            new_elem_extension,
                            Option::from(new_list_parameter),
                            Option::from(new_list_descriptor),
                            "",
                            "",
                        );
                    }
                }
                crate::ExtensionType::StatisticValue => {
                    let new_elem_extension = list_am.elem_extension().copy();

                    let new_list_parameter = ListParameter::new();
                    let mut new_list_descriptor = ListDescriptor::new();

                    match list_descriptor.as_ref() {
                        None => {
                            return Err(crate::ErrorType::Index);
                        }
                        Some(o) => {
                            o.copy_list_descriptor(&mut new_list_descriptor, false, updating_json);
                        }
                    }

                    new_list_event.add_event(
                        event_date,
                        "",
                        sort_order,
                        value,
                        "",
                        false,
                        periods,
                        "",
                        0,
                        0,
                        intervals,
                        frequency,
                        new_elem_extension,
                        Option::from(new_list_parameter),
                        Option::from(new_list_descriptor),
                        "",
                        "",
                    );
                }
                _ => {
                    if principal_index == usize::MAX
                        && !list_am.elem_extension().pc_balance_statistics()
                    {
                        principal_index = am_index;
                    }

                    if !after_pv || cv_present_seen {
                        let new_elem_extension = list_am.elem_extension().copy();

                        let new_list_parameter = ListParameter::new();
                        let mut new_list_descriptor = ListDescriptor::new();

                        match list_descriptor.as_ref() {
                            None => {
                                return Err(crate::ErrorType::Index);
                            }
                            Some(o) => {
                                o.copy_list_descriptor(
                                    &mut new_list_descriptor,
                                    false,
                                    updating_json,
                                );
                            }
                        }

                        new_list_event.add_event(
                            event_date,
                            "",
                            sort_order,
                            value,
                            "",
                            false,
                            periods,
                            "",
                            0,
                            0,
                            intervals,
                            frequency,
                            new_elem_extension,
                            Option::from(new_list_parameter),
                            Option::from(new_list_descriptor),
                            "",
                            "",
                        );
                    }
                }
            }

            am_index += 1;
        }
        new_list_event.set_sort_on_add(true); // Sorts list
        list_am.get_element(orig_list_index);

        Ok(new_list_event)
    }
}
