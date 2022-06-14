//! The serilaize json element of the AmFn engine.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cell::{Cell, Ref, RefCell};
use std::rc::Rc;

use rust_decimal::prelude::*;

use super::{
    CalcManager, ElemPreferences, ListCashflow, ListExchangeRate, ListTemplateEvent,
    ListTemplateGroup,
};
use crate::core::{
    CoreUtility, ElemBalanceResult, ElemCurrentValue, ElemExtension, ElemInterestChange,
    ElemPrincipalChange, ElemStatisticValue, ExtensionValue, ListAmortization, ListDescriptor,
    ListEvent, ListParameter,
};
use crate::ListTrait;

pub struct CalcJsonSerialize {
    /// Calculator manager element.
    calc_manager: Rc<RefCell<CalcManager>>,

    /// Active Json Element depth
    depth: Cell<usize>,
}

/// The serilaize json implementation of the AmFn engine.

impl CalcJsonSerialize {
    /// Create and return a new serialization element.
    ///
    /// # Arguments
    ///
    /// * `calc_manager_param` - Calculation manager.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn new(calc_manager_param: &Rc<RefCell<CalcManager>>) -> CalcJsonSerialize {
        CalcJsonSerialize {
            calc_manager: Rc::clone(calc_manager_param),
            depth: Cell::new(0),
        }
    }

    /// Returns the calculation manager.
    ///
    /// # Return
    ///
    /// * See description.

    fn calc_mgr(&self) -> Ref<CalcManager> {
        self.calc_manager.borrow()
    }

    /// Serialize elements into Json.
    ///
    /// # Arguments
    ///
    /// * `options` - Determines the elements that are serialized.
    ///
    /// # Return
    ///
    /// * ERROR_NONE if successful, otherwise error code.

    pub fn serialize(&self, options: usize) -> String {
        let mut buf = String::from("");

        self.depth.set(0);

        buf.push('{');
        buf.push_str(crate::LINE_ENDING);
        self.increment_depth();

        let mut add_comma = options & crate::JSON_SERIALIZE_EXCHANGE_RATES != 0
            || options & crate::JSON_SERIALIZE_PREFERENCES != 0
            || options & crate::JSON_SERIALIZE_TEMPLATES != 0;

        let is_am_list = options & crate::JSON_SERIALIZE_AMORTIZATION_LIST != 0
            || options & crate::JSON_SERIALIZE_AMORTIZATION_LIST_ROLLUPS != 0
            || options & crate::JSON_SERIALIZE_AMORTIZATION_LIST_DETAILS != 0;

        if (options & crate::JSON_SERIALIZE_CASHFLOW_PREFERENCES != 0)
            || (options & crate::JSON_SERIALIZE_EVENT_LIST != 0)
            || is_am_list
        {
            self.serialize_cashflows(
                self.calc_mgr().list_cashflow(),
                &mut buf,
                options,
                add_comma,
            );
        }

        add_comma = options & crate::JSON_SERIALIZE_PREFERENCES != 0
            || options & crate::JSON_SERIALIZE_TEMPLATES != 0;

        if options & crate::JSON_SERIALIZE_EXCHANGE_RATES != 0 {
            self.serialize_exchange_rates(
                self.calc_mgr().list_exchange_rate(),
                &mut buf,
                add_comma,
            );
        }

        add_comma = options & crate::JSON_SERIALIZE_TEMPLATES != 0;

        if options & crate::JSON_SERIALIZE_PREFERENCES != 0 {
            self.serialize_preferences(self.calc_mgr().preferences(), &mut buf, add_comma);
        }

        if options & crate::JSON_SERIALIZE_TEMPLATES != 0 {
            self.serialize_template_groups(self.calc_mgr().list_template_group(), &mut buf, false);
        }

        self.decrement_depth();
        buf.push('}');
        buf.push_str(crate::LINE_ENDING);

        buf
    }

    /// Serialize list of amortization elements from the
    /// currently selected cashflow.
    ///
    /// # Arguments
    ///
    /// * `include_rollups` - Include rollup elements.
    /// * `include_details` - Include detail elements (if include_rollups is true).
    /// * `buf` - Buffer to append serialization.
    /// * `add_comma` - Append comma on last line of output.

    fn serialize_am_list(
        &self,
        include_rollups: bool,
        include_details: bool,
        buf: &mut String,
        add_comma: bool,
    ) {
        let calc_mgr = self.calc_mgr();
        let decimal_digits = calc_mgr.decimal_digits(true);
        let list_locale = calc_mgr.list_locale();

        let compress_descriptor = calc_mgr.compress_descriptor(true);
        let omit_statistic_events = calc_mgr.statistic_events(true);

        let list_am: ListAmortization = match calc_mgr.list_cashflow().create_cashflow_output(
            include_rollups,
            include_details,
            compress_descriptor,
            omit_statistic_events,
            true,
        ) {
            Err(_e) => {
                panic!("Cannot create amortization list for output")
            }
            Ok(o) => o,
        };

        buf.push_str(self.indent().as_str());
        buf.push_str("\"am-list\": [");
        buf.push_str(crate::LINE_ENDING);
        self.increment_depth();

        let mut index: usize = 0;
        if list_am.get_element(index) {
            let mut deserialize_list = true;

            while deserialize_list {
                let ext = list_am.elem_extension().extension_value();

                buf.push_str(self.indent().as_str());
                buf.push('{');
                buf.push_str(crate::LINE_ENDING);
                self.increment_depth();
                if !list_am.event_type().is_empty() {
                    buf.push_str(self.indent().as_str());
                    buf.push_str("\"event-type\": \"");
                    buf.push_str(list_am.event_type());
                    buf.push_str("\",");
                    buf.push_str(crate::LINE_ENDING);
                }
                buf.push_str(self.indent().as_str());
                buf.push_str("\"date\": \"");
                buf.push_str(list_locale.format_date_out(list_am.event_date()).as_str());
                buf.push_str("\",");
                buf.push_str(crate::LINE_ENDING);
                buf.push_str(self.indent().as_str());
                buf.push_str("\"frequency\": \"");
                buf.push_str(CoreUtility::get_frequency_mnemonic(list_am.frequency()).as_str());
                buf.push_str("\",");
                buf.push_str(crate::LINE_ENDING);
                buf.push_str(self.indent().as_str());
                buf.push_str("\"intervals\": ");
                buf.push_str(list_locale.format_integer_out(list_am.intervals()).as_str());
                buf.push(',');
                buf.push_str(crate::LINE_ENDING);
                buf.push_str(self.indent().as_str());
                buf.push_str("\"periods\": ");
                buf.push_str(list_locale.format_integer_out(list_am.periods()).as_str());
                buf.push(',');
                buf.push_str(crate::LINE_ENDING);
                buf.push_str(self.indent().as_str());
                buf.push_str("\"value\": \"");

                match ext {
                    ExtensionValue::InterestChange(_o) => {
                        buf.push_str(list_locale.format_decimal_out(list_am.value()).as_str());
                    }
                    _ => {
                        buf.push_str(
                            list_locale
                                .format_currency_out(list_am.value(), decimal_digits)
                                .as_str(),
                        );
                    }
                }

                buf.push_str("\",");
                buf.push_str(crate::LINE_ENDING);
                buf.push_str(self.indent().as_str());
                buf.push_str("\"value-to-interest\": \"");
                buf.push_str(
                    list_locale
                        .format_currency_out(list_am.value_to_interest(), decimal_digits)
                        .as_str(),
                );
                buf.push_str("\",");
                buf.push_str(crate::LINE_ENDING);
                buf.push_str(self.indent().as_str());
                buf.push_str("\"value-to-principal\": \"");
                buf.push_str(
                    list_locale
                        .format_currency_out(list_am.value_to_principal(), decimal_digits)
                        .as_str(),
                );
                buf.push_str("\",");
                buf.push_str(crate::LINE_ENDING);
                buf.push_str(self.indent().as_str());
                buf.push_str("\"principal-decrease\": \"");
                buf.push_str(
                    list_locale
                        .format_currency_out(list_am.principal_decrease(), decimal_digits)
                        .as_str(),
                );
                buf.push_str("\",");
                buf.push_str(crate::LINE_ENDING);
                buf.push_str(self.indent().as_str());
                buf.push_str("\"principal-increase\": \"");
                buf.push_str(
                    list_locale
                        .format_currency_out(list_am.principal_increase(), decimal_digits)
                        .as_str(),
                );
                buf.push_str("\",");
                buf.push_str(crate::LINE_ENDING);
                buf.push_str(self.indent().as_str());
                buf.push_str("\"interest\": \"");
                buf.push_str(
                    list_locale
                        .format_currency_out(list_am.interest(), decimal_digits)
                        .as_str(),
                );
                buf.push_str("\",");
                buf.push_str(crate::LINE_ENDING);
                buf.push_str(self.indent().as_str());
                buf.push_str("\"sl-interest\": \"");
                buf.push_str(
                    list_locale
                        .format_currency_out(list_am.sl_interest(), decimal_digits)
                        .as_str(),
                );
                buf.push_str("\",");
                buf.push_str(crate::LINE_ENDING);
                buf.push_str(self.indent().as_str());
                buf.push_str("\"accrued-balance\": \"");
                buf.push_str(
                    list_locale
                        .format_currency_out(list_am.acc_balance(), decimal_digits)
                        .as_str(),
                );
                buf.push_str("\",");
                buf.push_str(crate::LINE_ENDING);
                buf.push_str(self.indent().as_str());
                buf.push_str("\"balance\": \"");
                buf.push_str(
                    list_locale
                        .format_currency_out(list_am.balance(), decimal_digits)
                        .as_str(),
                );
                buf.push_str("\",");
                buf.push_str(crate::LINE_ENDING);
                buf.push_str(self.indent().as_str());
                buf.push_str("\"sort-order\": ");
                buf.push_str(
                    list_locale
                        .format_integer_out(list_am.sort_order())
                        .as_str(),
                );
                buf.push(',');
                buf.push_str(crate::LINE_ENDING);

                buf.push_str(
                    self.serialize_extension(
                        list_am.elem_extension(),
                        list_am.value(),
                        list_am.frequency(),
                        true,
                        false,
                    )
                    .as_str(),
                );

                match list_am.list_descriptor() {
                    None => {}
                    Some(o) => {
                        self.serialize_descriptor_list(o, buf, false);
                    }
                }
                index += 1;
                deserialize_list = list_am.get_element(index);

                self.decrement_depth();
                buf.push_str(self.indent().as_str());
                buf.push('}');
                if deserialize_list {
                    buf.push(',');
                }
                buf.push_str(crate::LINE_ENDING);
            }
        }

        self.decrement_depth();
        buf.push_str(self.indent().as_str());
        buf.push(']');
        if add_comma {
            buf.push(',');
        }
        buf.push_str(crate::LINE_ENDING);
    }

    /// Serialize balance result element.
    ///
    /// # Arguments
    ///
    /// * `balance_result` - Balance result to serialize.
    /// * `buf` - Buffer to append serialization.
    /// * `add_comma` - Append comma on last line of output.

    fn serialize_balance_result(
        &self,
        balance_result: &ElemBalanceResult,
        buf: &mut String,
        add_comma: bool,
    ) {
        let calc_mgr = self.calc_mgr();
        let decimal_digits = calc_mgr.decimal_digits(true);
        let list_locale = calc_mgr.list_locale();

        buf.push_str(self.indent().as_str());
        buf.push_str("\"balance-result\": {");
        buf.push_str(crate::LINE_ENDING);
        self.increment_depth();
        buf.push_str(self.indent().as_str());
        buf.push_str("\"accrued-balance-seen\": ");
        buf.push_str(self.get_bool_str(balance_result.acc_balance_seen()));
        buf.push(',');
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"polarity\": \"");
        buf.push_str(self.get_polarity(balance_result.polarity()));
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"rule-of-78-seen\": ");
        buf.push_str(self.get_bool_str(balance_result.rule_of_78_seen()));
        buf.push(',');
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"auxiliary-active-decrease\": \"");
        buf.push_str(
            list_locale
                .format_currency_out(balance_result.aux_active_decrease(), decimal_digits)
                .as_str(),
        );
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"auxiliary-active-increase\": \"");
        buf.push_str(
            list_locale
                .format_currency_out(balance_result.aux_active_increase(), decimal_digits)
                .as_str(),
        );
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"auxiliary-passive-decrease\": \"");
        buf.push_str(
            list_locale
                .format_currency_out(balance_result.aux_passive_decrease(), decimal_digits)
                .as_str(),
        );
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"auxiliary-passive-increase\": \"");
        buf.push_str(
            list_locale
                .format_currency_out(balance_result.aux_passive_increase(), decimal_digits)
                .as_str(),
        );
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"final-accrued-balance\": \"");
        buf.push_str(
            list_locale
                .format_currency_out(balance_result.acc_balance(), decimal_digits)
                .as_str(),
        );
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"final-balance\": \"");
        buf.push_str(
            list_locale
                .format_currency_out(balance_result.balance(), decimal_digits)
                .as_str(),
        );
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"final-balance-date\": \"");
        buf.push_str(
            list_locale
                .format_date_out(balance_result.balance_date())
                .as_str(),
        );
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"interest-present\": \"");
        buf.push_str(
            list_locale
                .format_currency_out(balance_result.interest_present(), decimal_digits)
                .as_str(),
        );
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"interest-total\": \"");
        buf.push_str(
            list_locale
                .format_currency_out(balance_result.interest_total(), decimal_digits)
                .as_str(),
        );
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"sl-interest-present\": \"");
        buf.push_str(
            list_locale
                .format_currency_out(balance_result.sl_interest_present(), decimal_digits)
                .as_str(),
        );
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"sl-interest-total\": \"");
        buf.push_str(
            list_locale
                .format_currency_out(balance_result.sl_interest_total(), decimal_digits)
                .as_str(),
        );
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"principal-changes-present\": ");
        buf.push_str(
            list_locale
                .format_integer_out(balance_result.prin_present())
                .as_str(),
        );
        buf.push(',');
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"principal-changes-total\": ");
        buf.push_str(
            list_locale
                .format_integer_out(balance_result.prin_total())
                .as_str(),
        );
        buf.push(',');
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"principal-total-decrease\": \"");
        buf.push_str(
            list_locale
                .format_currency_out(balance_result.prin_decrease(), decimal_digits)
                .as_str(),
        );
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"principal-total-increase\": \"");
        buf.push_str(
            list_locale
                .format_currency_out(balance_result.prin_increase(), decimal_digits)
                .as_str(),
        );
        buf.push('"');
        buf.push_str(crate::LINE_ENDING);

        self.decrement_depth();
        buf.push_str(self.indent().as_str());
        buf.push('}');
        if add_comma {
            buf.push(',');
        }
        buf.push_str(crate::LINE_ENDING);
    }

    /// Serialize list of cashflows.
    ///
    /// # Arguments
    ///
    /// * `cashflows` - List of cashflows to serialize.
    /// * `buf` - Buffer to append serialization.
    /// * `options` - Serialization options.
    /// * `add_comma` - Append comma on last line of output.

    fn serialize_cashflows(
        &self,
        cashflows: &ListCashflow,
        buf: &mut String,
        options: usize,
        add_comma: bool,
    ) {
        buf.push_str(self.indent().as_str());
        buf.push_str("\"cashflows\": [");
        buf.push_str(crate::LINE_ENDING);
        self.increment_depth();

        let orig_index = cashflows.index();
        let mut index = 0;
        if options & crate::JSON_SERIALIZE_CASHFLOW_SELECTED != 0 {
            index = orig_index;
        }

        if cashflows.get_element(index) {
            let mut deserialize_list = true;

            while deserialize_list {
                buf.push_str(self.indent().as_str());
                buf.push('{');
                buf.push_str(crate::LINE_ENDING);
                self.increment_depth();

                buf.push_str(self.indent().as_str());
                buf.push_str("\"name\": \"");
                buf.push_str(cashflows.name());
                buf.push_str("\",");
                buf.push_str(crate::LINE_ENDING);

                if options & crate::JSON_SERIALIZE_CASHFLOW_PREFERENCES != 0 {
                    match cashflows.preferences() {
                        None => {}
                        Some(o) => {
                            self.serialize_preferences(o, buf, true);
                        }
                    }
                }

                let is_am_list = options & crate::JSON_SERIALIZE_AMORTIZATION_LIST != 0
                    || options & crate::JSON_SERIALIZE_AMORTIZATION_LIST_ROLLUPS != 0
                    || options & crate::JSON_SERIALIZE_AMORTIZATION_LIST_DETAILS != 0;

                let is_am_rollups = options & crate::JSON_SERIALIZE_AMORTIZATION_LIST_ROLLUPS != 0
                    || options & crate::JSON_SERIALIZE_AMORTIZATION_LIST_DETAILS != 0;

                let is_am_details = options & crate::JSON_SERIALIZE_AMORTIZATION_LIST_DETAILS != 0;

                if options & crate::JSON_SERIALIZE_EVENT_LIST != 0 {
                    match cashflows.list_event() {
                        None => {}
                        Some(o) => {
                            self.serialize_event_list(o, buf, is_am_list);
                        }
                    }
                }

                if is_am_list {
                    self.serialize_am_list(is_am_rollups, is_am_details, buf, true);
                    match cashflows.elem_balance_result() {
                        None => {}
                        Some(o) => {
                            self.serialize_balance_result(o, buf, false);
                        }
                    }
                }

                if options & crate::JSON_SERIALIZE_CASHFLOW_SELECTED != 0 {
                    deserialize_list = false;
                } else {
                    index += 1;
                    deserialize_list = cashflows.get_element(index);
                }

                self.decrement_depth();
                buf.push_str(self.indent().as_str());
                buf.push('}');
                if deserialize_list {
                    buf.push(',');
                }
                buf.push_str(crate::LINE_ENDING);
            }
        }

        self.decrement_depth();
        buf.push_str(self.indent().as_str());
        buf.push(']');
        if add_comma {
            buf.push(',');
        }
        buf.push_str(crate::LINE_ENDING);

        cashflows.get_element(orig_index);
    }

    /// Serialize current value element.
    ///
    /// # Arguments
    ///
    /// * `current_value` - Current value element.
    /// * `buf` - Buffer to append serialization.

    fn serialize_current_value(&self, current_value: &ElemCurrentValue, buf: &mut String) {
        buf.push_str(self.indent().as_str());
        buf.push_str("\"current-value\": {");
        buf.push_str(crate::LINE_ENDING);
        self.increment_depth();

        buf.push_str(self.indent().as_str());
        buf.push_str("\"passive\": ");
        buf.push_str(self.get_bool_str(current_value.passive()));
        buf.push(',');
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"present\": ");
        buf.push_str(self.get_bool_str(current_value.present()));
        buf.push(',');
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"eom\": ");
        buf.push_str(self.get_bool_str(current_value.eom()));
        buf.push_str(crate::LINE_ENDING);

        self.decrement_depth();
        buf.push_str(self.indent().as_str());
        buf.push('}');
        buf.push_str(crate::LINE_ENDING);
    }

    /// Serialize list of descriptors.
    ///
    /// # Arguments
    ///
    /// * `list_descriptor` - List of descriptors to serialize.
    /// * `buf` - Buffer to append serialization.
    /// * `add_comma` - Append comma on last line of output.

    fn serialize_descriptor_list(
        &self,
        list_descriptor: &ListDescriptor,
        buf: &mut String,
        add_comma: bool,
    ) {
        buf.push_str(self.indent().as_str());
        buf.push_str("\"descriptor-list\": [");
        buf.push_str(crate::LINE_ENDING);
        self.increment_depth();

        let mut index: usize = 0;
        if list_descriptor.get_element(index) {
            let mut deserialize_list = true;

            while deserialize_list {
                buf.push_str(self.indent().as_str());
                buf.push('{');
                buf.push_str(crate::LINE_ENDING);
                self.increment_depth();
                buf.push_str(self.indent().as_str());
                buf.push_str("\"group\": \"");
                buf.push_str(list_descriptor.group());
                buf.push_str("\",");
                buf.push_str(crate::LINE_ENDING);
                buf.push_str(self.indent().as_str());
                buf.push_str("\"name\": \"");
                buf.push_str(list_descriptor.name());
                buf.push_str("\",");
                buf.push_str(crate::LINE_ENDING);
                buf.push_str(self.indent().as_str());
                buf.push_str("\"descriptor-type\": \"");
                buf.push_str(list_descriptor.desc_type());
                buf.push_str("\",");
                buf.push_str(crate::LINE_ENDING);
                buf.push_str(self.indent().as_str());
                buf.push_str("\"descriptor-code\": \"");
                buf.push_str(list_descriptor.code());
                buf.push_str("\",");
                buf.push_str(crate::LINE_ENDING);
                buf.push_str(self.indent().as_str());
                buf.push_str("\"value\": \"");
                buf.push_str(
                    self.escape_string(list_descriptor.value().as_str())
                        .as_str(),
                );
                buf.push_str("\",");
                buf.push_str(crate::LINE_ENDING);
                if !list_descriptor.value_expr().is_empty() {
                    buf.push_str(self.indent().as_str());
                    buf.push_str("\"expression\": \"");
                    buf.push_str(
                        self.escape_string(list_descriptor.value_expr().as_str())
                            .as_str(),
                    );
                    buf.push_str("\",");
                    buf.push_str(crate::LINE_ENDING);
                }
                buf.push_str(self.indent().as_str());
                buf.push_str("\"event-index\": ");
                buf.push_str(list_descriptor.index().to_string().as_str());
                buf.push(',');
                buf.push_str(crate::LINE_ENDING);
                buf.push_str(self.indent().as_str());
                buf.push_str("\"propagate\": ");
                buf.push_str(self.get_bool_str(list_descriptor.propagate()));
                buf.push_str(crate::LINE_ENDING);
                index += 1;
                deserialize_list = list_descriptor.get_element(index);

                self.decrement_depth();
                buf.push_str(self.indent().as_str());
                buf.push('}');
                if deserialize_list {
                    buf.push(',');
                }
                buf.push_str(crate::LINE_ENDING);
            }
        }

        self.decrement_depth();
        buf.push_str(self.indent().as_str());
        buf.push(']');
        if add_comma {
            buf.push(',');
        }
        buf.push_str(crate::LINE_ENDING);
    }

    /// Serialize list of event elements.
    ///
    /// # Arguments
    ///
    /// * `list_event` - List of event elements to serialize.
    /// * `buf` - Buffer to append serialization.
    /// * `add_comma` - Append comma on last line of output.

    fn serialize_event_list(&self, list_event: &ListEvent, buf: &mut String, add_comma: bool) {
        buf.push_str(self.indent().as_str());
        buf.push_str("\"event-list\": [");
        buf.push_str(crate::LINE_ENDING);
        self.increment_depth();

        let mut index: usize = 0;
        if list_event.get_element(index) {
            let mut deserialize_list = true;

            while deserialize_list {
                buf.push_str(self.indent().as_str());
                buf.push('{');
                buf.push_str(crate::LINE_ENDING);
                self.increment_depth();
                let event_date = list_event.event_date();
                let mut end_date = event_date;
                let periods = list_event.periods();
                if periods > 1 {
                    end_date = CoreUtility::date_new(
                        event_date,
                        event_date,
                        list_event.frequency(),
                        list_event.intervals() * (periods - 1),
                        list_event.eom(),
                    );
                }
                if !list_event.event_name().is_empty() {
                    buf.push_str(self.indent().as_str());
                    buf.push_str("\"event-name\": \"");
                    buf.push_str(list_event.event_name());
                    buf.push_str("\",");
                    buf.push_str(crate::LINE_ENDING);
                }
                if !list_event.event_type().is_empty() {
                    buf.push_str(self.indent().as_str());
                    buf.push_str("\"event-type\": \"");
                    buf.push_str(list_event.event_type());
                    buf.push_str("\",");
                    buf.push_str(crate::LINE_ENDING);
                }
                buf.push_str(self.indent().as_str());
                buf.push_str("\"event-date\": {");
                buf.push_str(crate::LINE_ENDING);
                self.increment_depth();
                buf.push_str(self.indent().as_str());
                buf.push_str("\"date\": \"");
                buf.push_str(self.get_date_str(event_date).as_str());
                if list_event.date_expr().is_empty() {
                    buf.push('"');
                } else {
                    buf.push_str("\",");
                }
                buf.push_str(crate::LINE_ENDING);
                if !list_event.date_expr().is_empty() {
                    buf.push_str(self.indent().as_str());
                    buf.push_str("\"expression\": \"");
                    buf.push_str(self.escape_string(list_event.date_expr()).as_str());
                    buf.push('"');
                    buf.push_str(crate::LINE_ENDING);
                }

                self.decrement_depth();
                buf.push_str(self.indent().as_str());
                buf.push_str("},");
                buf.push_str(crate::LINE_ENDING);
                buf.push_str(self.indent().as_str());
                buf.push_str("\"end-date\": \"");
                buf.push_str(self.get_date_str(end_date).as_str());
                buf.push_str("\",");
                buf.push_str(crate::LINE_ENDING);
                buf.push_str(self.indent().as_str());
                buf.push_str("\"frequency\": \"");
                buf.push_str(CoreUtility::get_frequency_mnemonic(list_event.frequency()).as_str());
                buf.push_str("\",");
                buf.push_str(crate::LINE_ENDING);
                buf.push_str(self.indent().as_str());
                buf.push_str("\"intervals\": ");
                buf.push_str(list_event.intervals().to_string().as_str());
                buf.push(',');
                buf.push_str(crate::LINE_ENDING);
                buf.push_str(self.indent().as_str());
                buf.push_str("\"event-periods\": {");
                buf.push_str(crate::LINE_ENDING);
                self.increment_depth();
                buf.push_str(self.indent().as_str());
                buf.push_str("\"periods\": ");
                buf.push_str(list_event.periods().to_string().as_str());
                if !list_event.periods_expr().is_empty() {
                    buf.push(',');
                }
                buf.push_str(crate::LINE_ENDING);
                if !list_event.periods_expr().is_empty() {
                    buf.push_str(self.indent().as_str());
                    buf.push_str("\"expression\": \"");
                    buf.push_str(self.escape_string(list_event.periods_expr()).as_str());
                    buf.push('"');
                    buf.push_str(crate::LINE_ENDING);
                }

                self.decrement_depth();
                buf.push_str(self.indent().as_str());
                buf.push_str("},");
                buf.push_str(crate::LINE_ENDING);
                buf.push_str(self.indent().as_str());
                buf.push_str("\"event-value\": {");
                buf.push_str(crate::LINE_ENDING);
                self.increment_depth();
                buf.push_str(self.indent().as_str());
                buf.push_str("\"value\": \"");
                buf.push_str(list_event.value().to_string().as_str());
                if list_event.value_expr().is_empty() {
                    buf.push('"');
                } else {
                    buf.push_str("\",");
                }
                buf.push_str(crate::LINE_ENDING);
                if !list_event.value_expr().is_empty() {
                    buf.push_str(self.indent().as_str());
                    buf.push_str("\"expression\": \"");
                    buf.push_str(self.escape_string(list_event.value_expr()).as_str());
                    buf.push_str("\",");
                    buf.push_str(crate::LINE_ENDING);

                    buf.push_str(self.indent().as_str());
                    buf.push_str("\"expr-balance\": ");
                    buf.push_str(self.get_bool_str(list_event.value_expr_balance()));
                    buf.push_str(crate::LINE_ENDING);
                }

                self.decrement_depth();
                buf.push_str(self.indent().as_str());
                buf.push_str("},");
                buf.push_str(crate::LINE_ENDING);
                buf.push_str(self.indent().as_str());
                buf.push_str("\"skip-mask\": \"");
                buf.push_str(
                    CoreUtility::skip_mask_to_string(
                        list_event.skip_mask_len(),
                        list_event.skip_mask(),
                    )
                    .as_str(),
                );
                buf.push_str("\",");
                buf.push_str(crate::LINE_ENDING);
                buf.push_str(self.indent().as_str());
                buf.push_str("\"sort-order\": ");
                buf.push_str(list_event.sort_order().to_string().as_str());
                buf.push(',');
                buf.push_str(crate::LINE_ENDING);

                buf.push_str(
                    self.serialize_extension(
                        list_event.elem_extension(),
                        dec!(0.0),
                        list_event.frequency(),
                        true,
                        false,
                    )
                    .as_str(),
                );

                match list_event.list_parameter() {
                    None => {}
                    Some(o) => {
                        self.serialize_parameter_list(o, buf, true);
                    }
                }

                match list_event.list_descriptor() {
                    None => {}
                    Some(o) => {
                        self.serialize_descriptor_list(o, buf, true);
                    }
                }
                buf.push_str(self.indent().as_str());
                buf.push_str("\"event-next-name\": \"");
                buf.push_str(list_event.next_name());
                buf.push('"');
                buf.push_str(crate::LINE_ENDING);
                index += 1;
                deserialize_list = list_event.get_element(index);

                self.decrement_depth();
                buf.push_str(self.indent().as_str());
                buf.push('}');
                if deserialize_list {
                    buf.push(',');
                }
                buf.push_str(crate::LINE_ENDING);
            }
        }

        self.decrement_depth();
        buf.push_str(self.indent().as_str());
        buf.push(']');
        if add_comma {
            buf.push(',');
        }
        buf.push_str(crate::LINE_ENDING);
    }

    /// Serialize list of exchange rates.
    ///
    /// # Arguments
    ///
    /// * `exchange_rates` - List of exchange rates to serialize.
    /// * `buf` - Buffer to append serialization.
    /// * `add_comma` - Append comma on last line of output.

    fn serialize_exchange_rates(
        &self,
        exchange_rates: &ListExchangeRate,
        buf: &mut String,
        add_comma: bool,
    ) {
        buf.push_str(self.indent().as_str());
        buf.push_str("\"exchange-rates\": [");
        buf.push_str(crate::LINE_ENDING);
        self.increment_depth();

        let mut index: usize = 0;
        if exchange_rates.get_element(index) {
            let mut deserialize_list = true;

            while deserialize_list {
                buf.push_str(self.indent().as_str());
                buf.push('{');
                buf.push_str(crate::LINE_ENDING);
                self.increment_depth();
                buf.push_str(self.indent().as_str());
                buf.push_str("\"from\": \"");
                buf.push_str(exchange_rates.from_code());
                buf.push_str("\",");
                buf.push_str(crate::LINE_ENDING);

                buf.push_str(self.indent().as_str());
                buf.push_str("\"to\": \"");
                buf.push_str(exchange_rates.to_code());
                buf.push_str("\",");
                buf.push_str(crate::LINE_ENDING);

                buf.push_str(self.indent().as_str());
                buf.push_str("\"value\": \"");
                buf.push_str(exchange_rates.exchange_rate().to_string().as_str());
                buf.push('"');
                buf.push_str(crate::LINE_ENDING);
                index += 1;
                deserialize_list = exchange_rates.get_element(index);

                self.decrement_depth();
                buf.push_str(self.indent().as_str());
                buf.push('}');
                if deserialize_list {
                    buf.push(',');
                }
                buf.push_str(crate::LINE_ENDING);
            }
        }

        self.decrement_depth();
        buf.push_str(self.indent().as_str());
        buf.push(']');
        if add_comma {
            buf.push(',');
        }
        buf.push_str(crate::LINE_ENDING);
    }

    /// Serialize extension.
    ///
    /// # Arguments
    ///
    /// * `ext` - Extension to serialize.
    /// * `buf` - Buffer to append serialization.
    /// * `add_comma` - Append comma on last line of output.
    /// * `all_data` - Serialize all data properties.

    pub fn serialize_extension(
        &self,
        ext: &ElemExtension,
        value: Decimal,
        frequency: crate::FrequencyType,
        add_comma: bool,
        all_data: bool,
    ) -> String {
        let mut buf = String::from("");
        buf.push_str(self.indent().as_str());
        buf.push_str("\"extension\": {");
        buf.push_str(crate::LINE_ENDING);
        self.increment_depth();

        match ext.extension_value() {
            ExtensionValue::CurrentValue(o) => {
                self.serialize_current_value(o, &mut buf);
            }
            ExtensionValue::InterestChange(o) => {
                self.serialize_interest_change(o, &mut buf, value, frequency, all_data);
            }
            ExtensionValue::PrincipalChange(o) => {
                self.serialize_principal_change(o, &mut buf);
            }
            ExtensionValue::StatisticValue(o) => {
                self.serialize_statistic_value(o, &mut buf);
            }
        }

        self.decrement_depth();
        buf.push_str(self.indent().as_str());
        buf.push('}');
        if add_comma {
            buf.push(',');
        }
        buf.push_str(crate::LINE_ENDING);

        buf
    }

    /// Serialize interest change element.
    ///
    /// # Arguments
    ///
    /// * `interest_change` - Interest change element.
    /// * `buf` - Buffer to append serialization.
    /// * `nar` - Nominal annual rate.
    /// * `frequency` - Frequency value.
    /// * `all_data` - Serialize all data properties.

    fn serialize_interest_change(
        &self,
        interest_change: &ElemInterestChange,
        buf: &mut String,
        nar: Decimal,
        frequency: crate::FrequencyType,
        all_data: bool,
    ) {
        let calc_mgr = self.calc_mgr();
        let decimal_digits = calc_mgr.decimal_digits(false);
        let list_locale = calc_mgr.list_locale();

        buf.push_str(self.indent().as_str());
        buf.push_str("\"interest-change\": {");
        buf.push_str(crate::LINE_ENDING);
        self.increment_depth();

        buf.push_str(self.indent().as_str());
        buf.push_str("\"interest-method\": \"");
        buf.push_str(CoreUtility::get_interest_method_mnemonic(interest_change.method()).as_str());
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"day-count-basis\": \"");
        buf.push_str(
            CoreUtility::get_day_count_basis_mnemonic(interest_change.day_count_basis()).as_str(),
        );
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);

        if all_data || interest_change.effective_frequency() != crate::FrequencyType::None {
            buf.push_str(self.indent().as_str());
            buf.push_str("\"effective-frequency\": \"");
            buf.push_str(
                CoreUtility::get_frequency_mnemonic(interest_change.effective_frequency()).as_str(),
            );
            buf.push_str("\",");
            buf.push_str(crate::LINE_ENDING);
        }

        if all_data || interest_change.interest_frequency() != crate::FrequencyType::None {
            buf.push_str(self.indent().as_str());
            buf.push_str("\"interest-frequency\": \"");
            buf.push_str(
                CoreUtility::get_frequency_mnemonic(interest_change.interest_frequency()).as_str(),
            );
            buf.push_str("\",");
            buf.push_str(crate::LINE_ENDING);
        }

        if (interest_change.day_count_basis() == crate::DayCountType::Periodic
            || interest_change.day_count_basis() == crate::DayCountType::RuleOf78)
            && interest_change.days_in_year() > 0
            && nar > dec!(0.0)
        {
            buf.push_str(self.indent().as_str());
            buf.push_str("\"interest-statistics\": {");
            buf.push_str(crate::LINE_ENDING);
            self.increment_depth();
            buf.push_str(self.indent().as_str());
            buf.push_str("\"nar\": \"");
            buf.push_str(list_locale.format_decimal_out(nar).as_str());
            buf.push_str("\",");
            buf.push_str(crate::LINE_ENDING);
            buf.push_str(self.indent().as_str());
            buf.push_str("\"ear\": \"");
            buf.push_str(
                list_locale
                    .format_decimal_out(
                        CoreUtility::rate_nar_to_ear(
                            nar / dec!(100.0),
                            frequency,
                            interest_change.days_in_year(),
                        ) * dec!(100.0),
                    )
                    .as_str(),
            );
            buf.push_str("\",");
            buf.push_str(crate::LINE_ENDING);
            buf.push_str(self.indent().as_str());
            buf.push_str("\"pr\": \"");
            buf.push_str(
                list_locale
                    .format_decimal_out(
                        CoreUtility::rate_nar_to_pr(
                            nar / dec!(100.0),
                            frequency,
                            interest_change.days_in_year(),
                        ) * dec!(100.0),
                    )
                    .as_str(),
            );
            buf.push_str("\",");
            buf.push_str(crate::LINE_ENDING);
            buf.push_str(self.indent().as_str());
            buf.push_str("\"dr\": \"");
            buf.push_str(
                list_locale
                    .format_decimal_out(
                        CoreUtility::rate_nar_to_dr(
                            nar / dec!(100.0),
                            interest_change.days_in_year(),
                        ) * dec!(100.0),
                    )
                    .as_str(),
            );
            buf.push('"');
            buf.push_str(crate::LINE_ENDING);

            self.decrement_depth();
            buf.push_str(self.indent().as_str());
            buf.push_str("},");
            buf.push_str(crate::LINE_ENDING);
        }

        if all_data || interest_change.round_balance() != crate::RoundType::None {
            buf.push_str(self.indent().as_str());
            buf.push_str("\"round-balance\": \"");
            buf.push_str(CoreUtility::get_round_balance(interest_change.round_balance()).as_str());
            buf.push_str("\",");
            buf.push_str(crate::LINE_ENDING);
        }

        let dd: usize = interest_change
            .round_decimal_digits()
            .to_usize()
            .unwrap_or(0);

        if all_data || dd != decimal_digits {
            buf.push_str(self.indent().as_str());
            buf.push_str("\"round-decimal-digits\": \"");
            buf.push_str(interest_change.round_decimal_digits().to_string().as_str());
            buf.push_str("\",");
            buf.push_str(crate::LINE_ENDING);
        }
        buf.push_str(self.indent().as_str());
        buf.push_str("\"days-in-year\": ");
        buf.push_str(interest_change.days_in_year().to_string().as_str());
        buf.push_str(crate::LINE_ENDING);

        self.decrement_depth();
        buf.push_str(self.indent().as_str());
        buf.push('}');
        buf.push_str(crate::LINE_ENDING);
    }

    /// Serialize list of parameters.
    ///
    /// # Arguments
    ///
    /// * `list_parameter` - List of parameters to serialize.
    /// * `buf` - Buffer to append serialization.
    /// * `add_comma` - Append comma on last line of output.

    fn serialize_parameter_list(
        &self,
        list_parameter: &ListParameter,
        buf: &mut String,
        add_comma: bool,
    ) {
        buf.push_str(self.indent().as_str());
        buf.push_str("\"parameter-list\": [");
        buf.push_str(crate::LINE_ENDING);
        self.increment_depth();

        let mut index: usize = 0;
        if list_parameter.get_element(index) {
            let mut deserialize_list = true;

            while deserialize_list {
                buf.push_str(self.indent().as_str());
                buf.push('{');
                buf.push_str(crate::LINE_ENDING);
                self.increment_depth();
                buf.push_str(self.indent().as_str());
                buf.push_str("\"name\": \"");
                buf.push_str(list_parameter.name());
                buf.push_str("\",");
                buf.push_str(crate::LINE_ENDING);
                buf.push_str(self.indent().as_str());
                buf.push_str("\"parameter-type\": \"");
                buf.push_str(CoreUtility::get_param_type(list_parameter.param_type()).as_str());
                buf.push_str("\",");
                buf.push_str(crate::LINE_ENDING);
                buf.push_str(self.indent().as_str());
                buf.push_str("\"value\": \"");

                match list_parameter.param_type() {
                    crate::TokenType::Integer => {
                        buf.push_str(list_parameter.param_integer().to_string().as_str());
                    }
                    crate::TokenType::Decimal => {
                        buf.push_str(list_parameter.param_decimal().to_string().as_str());
                    }
                    _ => {
                        buf.push_str(self.escape_string(list_parameter.param_string()).as_str());
                    }
                }

                buf.push('"');
                buf.push_str(crate::LINE_ENDING);
                index += 1;
                deserialize_list = list_parameter.get_element(index);

                self.decrement_depth();
                buf.push_str(self.indent().as_str());
                buf.push('}');
                if deserialize_list {
                    buf.push(',');
                }
                buf.push_str(crate::LINE_ENDING);
            }
        }

        self.decrement_depth();
        buf.push_str(self.indent().as_str());
        buf.push(']');
        if add_comma {
            buf.push(',');
        }
        buf.push_str(crate::LINE_ENDING);
    }

    /// Serialize preferences element.
    ///
    /// # Arguments
    ///
    /// * `preferences` - Preferences element.
    /// * `buf` - Buffer to append serialization.
    /// * `add_comma` - Append comma on last line of output.

    fn serialize_preferences(
        &self,
        preferences: &ElemPreferences,
        buf: &mut String,
        add_comma: bool,
    ) {
        buf.push_str(self.indent().as_str());
        buf.push_str("\"preferences\": {");
        buf.push_str(crate::LINE_ENDING);
        self.increment_depth();
        buf.push_str(self.indent().as_str());
        buf.push_str("\"group\": \"");
        buf.push_str(preferences.group());
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"locale\": \"");
        buf.push_str(preferences.locale_str());
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);

        if !preferences.cross_rate_code().is_empty() {
            buf.push_str(self.indent().as_str());
            buf.push_str("\"cross-rate-code\": \"");
            buf.push_str(preferences.cross_rate_code());
            buf.push_str("\",");
            buf.push_str(crate::LINE_ENDING);
        }

        if !preferences.default_encoding().is_empty() {
            buf.push_str(self.indent().as_str());
            buf.push_str("\"default-encoding\": \"");
            buf.push_str(preferences.default_encoding());
            buf.push_str("\",");
            buf.push_str(crate::LINE_ENDING);
        }

        buf.push_str(self.indent().as_str());
        buf.push_str("\"decimal-digits\": ");
        buf.push_str(preferences.decimal_digits().to_string().as_str());
        buf.push(',');
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"fiscal-year-start\": ");
        buf.push_str(preferences.fiscal_year_start().to_string().as_str());
        buf.push(',');
        buf.push_str(crate::LINE_ENDING);

        if preferences.target() != dec!(0.0) {
            buf.push_str(self.indent().as_str());
            buf.push_str("\"target\": \"");
            buf.push_str(preferences.target().to_string().as_str());
            buf.push_str("\",");
            buf.push_str(crate::LINE_ENDING);
        }

        buf.push_str(self.indent().as_str());
        buf.push_str("\"combine-principal\": ");
        buf.push_str(preferences.combine_principal().to_string().as_str());
        buf.push(',');
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"compress-descriptor\": ");
        buf.push_str(preferences.compress_descriptor().to_string().as_str());
        buf.push(',');
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"omit-statistic-events\": ");
        buf.push_str(preferences.statistic_events().to_string().as_str());
        buf.push(',');
        buf.push_str(crate::LINE_ENDING);

        self.serialize_parameter_list(preferences.list_parameter(), buf, true);

        self.serialize_descriptor_list(preferences.list_descriptor(), buf, false);

        self.decrement_depth();
        buf.push_str(self.indent().as_str());
        buf.push('}');
        if add_comma {
            buf.push(',');
        }
        buf.push_str(crate::LINE_ENDING);
    }

    /// Serialize principal change element.
    ///
    /// # Arguments
    ///
    /// * `prin_change` - Principal change element.
    /// * `buf` - Buffer to append serialization.

    fn serialize_principal_change(&self, prin_change: &ElemPrincipalChange, buf: &mut String) {
        buf.push_str(self.indent().as_str());
        buf.push_str("\"principal-change\": {");
        buf.push_str(crate::LINE_ENDING);
        self.increment_depth();

        buf.push_str(self.indent().as_str());
        buf.push_str("\"principal-type\": \"");
        buf.push_str(CoreUtility::get_principal_type_mnemonic(prin_change.pc_type()).as_str());
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"auxiliary\": ");
        buf.push_str(self.get_bool_str(prin_change.auxiliary()));
        buf.push(',');
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"passive\": ");
        buf.push_str(self.get_bool_str(prin_change.aux_passive()));
        buf.push(',');
        buf.push_str(crate::LINE_ENDING);
        buf.push_str(self.indent().as_str());
        buf.push_str("\"principal-first\": ");
        buf.push_str(self.get_bool_str(prin_change.principal_first()));
        buf.push(',');
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"statistics\": ");
        buf.push_str(self.get_bool_str(prin_change.balance_statistics()));
        buf.push(',');
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"eom\": ");
        buf.push_str(self.get_bool_str(prin_change.eom()));
        buf.push_str(crate::LINE_ENDING);

        self.decrement_depth();
        buf.push_str(self.indent().as_str());
        buf.push('}');
        buf.push_str(crate::LINE_ENDING);
    }

    /// Serialize statistic value element.
    ///
    /// # Arguments
    ///
    /// * `statistic_value` - Statistic value element.
    /// * `buf` - Buffer to append serialization.

    fn serialize_statistic_value(&self, statistic_value: &ElemStatisticValue, buf: &mut String) {
        buf.push_str(self.indent().as_str());
        buf.push_str("\"statistic-value\": {");
        buf.push_str(crate::LINE_ENDING);
        self.increment_depth();

        buf.push_str(self.indent().as_str());
        buf.push_str("\"name\": \"");
        buf.push_str(statistic_value.name());
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"final\": ");
        buf.push_str(self.get_bool_str(statistic_value.is_final()));
        buf.push(',');
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"eom\": ");
        buf.push_str(self.get_bool_str(statistic_value.eom()));
        buf.push_str(crate::LINE_ENDING);

        self.decrement_depth();
        buf.push_str(self.indent().as_str());
        buf.push('}');
        buf.push_str(crate::LINE_ENDING);
    }

    /// Serialize list of template events.
    ///
    /// # Arguments
    ///
    /// * `template_events` - List of template events to serialize.
    /// * `buf` - Buffer to append serialization.
    /// * `add_comma` - Append comma on last line of output.

    fn serialize_template_events(
        &self,
        template_events: &ListTemplateEvent,
        buf: &mut String,
        add_comma: bool,
    ) {
        buf.push_str(self.indent().as_str());
        buf.push_str("\"template-events\": [");
        buf.push_str(crate::LINE_ENDING);
        self.increment_depth();

        let mut index: usize = 0;
        if template_events.get_element(index) {
            let mut deserialize_list = true;

            while deserialize_list {
                buf.push_str(self.indent().as_str());
                buf.push('{');
                buf.push_str(crate::LINE_ENDING);
                self.increment_depth();

                self.serialize_event_list(template_events.list_event(), buf, true);
                buf.push_str(self.indent().as_str());
                buf.push_str("\"name\": \"");
                buf.push_str(template_events.name());
                buf.push('"');
                buf.push_str(crate::LINE_ENDING);
                index += 1;
                deserialize_list = template_events.get_element(index);

                self.decrement_depth();
                buf.push_str(self.indent().as_str());
                buf.push('}');
                if deserialize_list {
                    buf.push(',');
                }
                buf.push_str(crate::LINE_ENDING);
            }
        }
        self.decrement_depth();
        buf.push_str(self.indent().as_str());
        buf.push(']');
        if add_comma {
            buf.push(',');
        }
        buf.push_str(crate::LINE_ENDING);
    }

    /// Serialize list of template groups.
    ///
    /// # Arguments
    ///
    /// * `template_groups` - List of template groups to serialize.
    /// * `buf` - Buffer to append serialization.
    /// * `add_comma` - Append comma on last line of output.

    fn serialize_template_groups(
        &self,
        template_groups: &ListTemplateGroup,
        buf: &mut String,
        add_comma: bool,
    ) {
        buf.push_str(self.indent().as_str());
        buf.push_str("\"template-groups\": [");
        buf.push_str(crate::LINE_ENDING);
        self.increment_depth();

        let mut index: usize = 0;
        if template_groups.get_element(index) {
            let mut deserialize_list = true;

            while deserialize_list {
                buf.push_str(self.indent().as_str());
                buf.push('{');
                buf.push_str(crate::LINE_ENDING);
                self.increment_depth();
                buf.push_str(self.indent().as_str());
                buf.push_str("\"group\": \"");
                buf.push_str(template_groups.group());
                buf.push_str("\",");
                buf.push_str(crate::LINE_ENDING);

                self.serialize_preferences(template_groups.preferences(), buf, true);

                self.serialize_template_events(template_groups.list_template_event(), buf, false);
                index += 1;
                deserialize_list = template_groups.get_element(index);

                self.decrement_depth();
                buf.push_str(self.indent().as_str());
                buf.push('}');
                if deserialize_list {
                    buf.push(',');
                }
                buf.push_str(crate::LINE_ENDING);
            }
        }
        self.decrement_depth();
        buf.push_str(self.indent().as_str());
        buf.push(']');
        if add_comma {
            buf.push(',');
        }
        buf.push_str(crate::LINE_ENDING);
    }

    /// Escape the input string and return json output.
    ///
    /// # Arguments
    ///
    /// * `input_str` - Input string.
    ///
    /// # Return
    ///
    /// * See description.

    fn escape_string(&self, input_str: &str) -> String {
        input_str.replace('"', "\\\"")
    }

    /// Decrement the indentation depth.

    fn decrement_depth(&self) {
        if self.depth.get() == 0 {
            return;
        }

        self.depth.set(self.depth.get() - 1);
    }

    /// Serialize a boolean value.
    ///
    /// # Arguments
    ///
    /// * `opt` - Boolean option.
    ///
    /// # Return
    ///
    /// * See description.

    fn get_bool_str(&self, opt: bool) -> &str {
        if opt {
            "true"
        } else {
            "false"
        }
    }

    /// Serialize a date value.
    ///
    /// # Arguments
    ///
    /// * `event_date` - Date value.
    ///
    /// # Return
    ///
    /// * See description.

    fn get_date_str(&self, event_date: usize) -> String {
        return format!(
            "{:04}-{:02}-{:02}",
            event_date / 10000,
            event_date / 100 % 100,
            event_date % 100
        );
    }

    /// Serialize a polarity value.
    ///
    /// # Arguments
    ///
    /// * `polarity` - Polarity value.
    ///
    /// # Return
    ///
    /// * See description.

    fn get_polarity(&self, polarity: i32) -> &str {
        if polarity < 0 {
            return "negative";
        }

        "positive"
    }

    /// Increment the indentation depth.

    fn increment_depth(&self) {
        self.depth.set(self.depth.get() + 1);
    }

    /// Return a string containing spaces relative to the indentation depth.
    ///
    /// # Return
    ///
    /// * See description.

    fn indent(&self) -> String {
        " ".repeat(self.depth.get() * crate::TAB_SPACES)
    }
}
