//! The deserialize json element of the AmFn engine.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use json::JsonValue;
use regex::Regex;
use rust_decimal::prelude::*;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use super::{
    CalcManager, ElemLocaleFormat, ElemPreferences, ListLocale, ListCashflow, 
    ListExchangeRate, ListTemplateEvent, ListTemplateGroup,
};
use crate::core::{
    CoreUtility, ElemCurrentValue, ElemExtension, ElemInterestChange,
    ElemPrincipalChange, ElemStatisticValue, ListDescriptor, ListEvent, ListParameter,
};

pub struct CalcJsonDeserialize {
    /// Calculator manager element.
    calc_manager: Rc<RefCell<CalcManager>>,
}

/// The deserialize json implementation of the AmFn engine.

impl CalcJsonDeserialize {
    /// Create and return a new deserialization element.
    ///
    /// # Arguments
    ///
    /// * `calc_manager_param` - Calculation manager.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn new(calc_manager_param: &Rc<RefCell<CalcManager>>) -> CalcJsonDeserialize {
        CalcJsonDeserialize {
            calc_manager: Rc::clone(calc_manager_param),
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

    /// Deserialize and ingest serialized Json.
    ///
    /// # Arguments
    ///
    /// * `input_param` - Input containing serialized Json.
    ///
    /// # Return
    ///
    /// * ERROR_NONE if successful, otherwise error code.

    pub fn deserialize(&self, input_param: String) -> Result<(), crate::ErrorType> {
        let data: JsonValue;
        match json::parse(input_param.as_str()) {
            Err(e) => {
                println!("Json error: {:?}", e);
                return Err(crate::ErrorType::Json);
            }
            Ok(o) => {
                data = o;
            }
        }

        self.calc_mgr().set_updating_json(true);

        if !data["cashflows"].is_null() {
            let result = self.deserialize_cashflows(&data["cashflows"]);
            match result {
                Err(e) => {
                    self.calc_mgr().set_updating_json(false);
                    return Err(e);
                }
                Ok(o) => {
                    self.calc_manager.borrow_mut().list_cashflow_mut().append_cashflows(o);
                }
            }
        }

        if !data["exchange-rates"].is_null() {
            let result = self.deserialize_exchange_rates(&data["exchange-rates"]);
            match result {
                Err(e) => {
                    self.calc_mgr().set_updating_json(false);
                    return Err(e);
                }
                Ok(o) => {
                    self.calc_manager.borrow_mut().set_list_exchange_rate(o);
                }
            }
        }

        if !data["locales"].is_null() {
            let result = self.deserialize_locales(&data["locales"]);
            match result {
                Err(e) => {
                    self.calc_mgr().set_updating_json(false);
                    return Err(e);
                }
                Ok(o) => {
                    self.calc_manager.borrow_mut().list_locale_mut().append_locales(o);
                }
            }
        }

        if !data["preferences"].is_null() {
            let result = self.deserialize_preferences(&data["preferences"]);
            match result {
                Err(e) => {
                    self.calc_mgr().set_updating_json(false);
                    return Err(e);
                }
                Ok(o) => {
                    self.calc_manager.borrow_mut().set_preferences(o);
                }
            }
        }

        if !data["template-groups"].is_null() {
            let result = self.deserialize_template_groups(&data["template-groups"]);
            match result {
                Err(e) => {
                    self.calc_mgr().set_updating_json(false);
                    return Err(e);
                }
                Ok(o) => {
                    self.calc_manager.borrow_mut().list_template_group_mut().append_template_groups(o);
                }
            }
        }

        self.calc_mgr().set_updating_json(false);

        Ok(())
    }

    /// Deserialize and ingest Json cashflows.
    ///
    /// # Arguments
    ///
    /// * `cfs` - Json value for cashflows.
    /// * `cashflows` - List of cashflows.
    ///
    /// # Return
    ///
    /// * ERROR_NONE if successful, otherwise error code.

    fn deserialize_cashflows(&self, cfs: &JsonValue) -> Result<ListCashflow, crate::ErrorType> {
        let mut cashflows = ListCashflow::new();
        let mut index: usize = 0;

        cashflows.set_calc_mgr(&self.calc_manager);

        loop {
            let cf = &cfs[index];
            if cf.is_null() {
                break;
            }

            let name: &str;
            match cf["name"].as_str() {
                None => {
                    return Err(crate::ErrorType::Json);
                }
                Some(o) => {
                    name = o;
                }
            }

            let preferences = self.calc_mgr().preferences().copy(true);
            match cashflows.add_cashflow_prep(name, None, Option::from(preferences), "") {
                Err(_e) => {
                    panic!("Add cashflow failed");
                }
                Ok(o) => { 
                    cashflows.add_cashflow(name, o);
                }
            }

            if !cf["preferences"].is_null() {
                match cashflows.preferences_mut() {
                    None => {}
                    Some(o) => {
                        let result =
                            self.deserialize_preferences_with_prefs(&cf["preferences"], o);
                        match result {
                            Err(e) => {
                                return Err(e);
                            }
                            Ok(_o) => {}
                        }
                    }
                }

                cashflows.update_preferences();
            }

            if !cf["event-list"].is_null() {
                match cashflows.list_event_mut() {
                    None => {}
                    Some(o) => {
                        let result = self.deserialize_event_list(&cf["event-list"], o);
                        match result {
                            Err(e) => {
                                return Err(e);
                            }
                            Ok(_o) => {}
                        }
                    }
                }
            }
            index += 1;
        }

        Ok(cashflows)
    }

    /// Deserialize and ingest Json current value.
    ///
    /// # Arguments
    ///
    /// * `cv` - Json value for current value.
    /// * `current_value` - Element current value.
    ///
    /// # Return
    ///
    /// * ERROR_NONE if successful, otherwise error code.

    fn deserialize_current_value(
        &self,
        cv: &JsonValue,
        current_value: &mut ElemCurrentValue,
    ) -> Result<(), crate::ErrorType> {
        match cv["eom"].as_bool() {
            None => {}
            Some(o) => {
                current_value.set_eom(o);
            }
        }

        match cv["passive"].as_bool() {
            None => {}
            Some(o) => {
                current_value.set_passive(o);
            }
        }

        match cv["present"].as_bool() {
            None => {}
            Some(o) => {
                current_value.set_present(o);
            }
        }

        Ok(())
    }

    /// Deserialize and ingest Json descriptors.
    ///
    /// # Arguments
    ///
    /// * `descs` - Json value for descriptors.
    /// * `descriptors` - List of descriptors.
    ///
    /// # Return
    ///
    /// * ERROR_NONE if successful, otherwise error code.

    fn deserialize_descriptor_list(
        &self,
        descs: &JsonValue,
        descriptors: &mut ListDescriptor,
    ) -> Result<(), crate::ErrorType> {
        let mut index: usize = 0;

        descriptors.set_sort_on_add(false);

        loop {
            let desc = &descs[index];
            if desc.is_null() {
                break;
            }

            let mut propagate: bool = false;
            match desc["propagate"].as_bool() {
                None => {}
                Some(o) => {
                    propagate = o;
                }
            }

            let descriptor_code: &str;
            match desc["descriptor-code"].as_str() {
                None => {
                    return Err(crate::ErrorType::Json);
                }
                Some(o) => {
                    descriptor_code = o;
                }
            }

            let descriptor_type: &str;
            match desc["descriptor-type"].as_str() {
                None => {
                    return Err(crate::ErrorType::Json);
                }
                Some(o) => {
                    descriptor_type = o;
                }
            }

            let group: &str;
            match desc["group"].as_str() {
                None => {
                    return Err(crate::ErrorType::Json);
                }
                Some(o) => {
                    group = o;
                }
            }

            let name: &str;
            match desc["name"].as_str() {
                None => {
                    return Err(crate::ErrorType::Json);
                }
                Some(o) => {
                    name = o;
                }
            }

            let value: &str;
            match desc["value"].as_str() {
                None => {
                    return Err(crate::ErrorType::Json);
                }
                Some(o) => {
                    value = o;
                }
            }

            let mut expr = "";
            if !desc["expression"].is_null() {
                match desc["expression"].as_str() {
                    None => {
                        return Err(crate::ErrorType::Json);
                    }
                    Some(o) => {
                        expr = o;
                    }
                }
            }

            descriptors.add_descriptor(
                group,
                name,
                descriptor_type,
                descriptor_code,
                String::from(value),
                expr,
                propagate,
                true,
            );

            match desc["event-index"].as_usize() {
                None => {}
                Some(n) => {
                    descriptors.set_list_event_index(n);
                }
            }

            index += 1;
        }
        descriptors.set_sort_on_add(true); // Sorts list

        Ok(())
    }

    /// Deserialize and ingest Json events.
    ///
    /// # Arguments
    ///
    /// * `evs` - Json value for events.
    /// * `events` - List of events.
    ///
    /// # Return
    ///
    /// * ERROR_NONE if successful, otherwise error code.

    fn deserialize_event_list(
        &self,
        evs: &JsonValue,
        events: &mut ListEvent,
    ) -> Result<(), crate::ErrorType> {
        let mut index: usize = 0;

        events.set_sort_on_add(false);

        loop {
            let ev = &evs[index];
            if ev.is_null() {
                break;
            }

            let event_date: usize;
            match ev["event-date"]["date"].as_str() {
                None => {
                    return Err(crate::ErrorType::Json);
                }
                Some(o) => {
                    event_date = self.get_date(o);
                }
            }

            let mut event_date_expr = "";
            if !ev["event-date"]["expression"].is_null() {
                match ev["event-date"]["expression"].as_str() {
                    None => {
                        return Err(crate::ErrorType::Json);
                    }
                    Some(o) => {
                        event_date_expr = o;
                    }
                }
            }

            let event_value: Decimal;
            match ev["event-value"]["value"].as_str() {
                None => {
                    return Err(crate::ErrorType::Json);
                }
                Some(o) => match o.parse::<Decimal>() {
                    Err(_e) => {
                        return Err(crate::ErrorType::Json);
                    }
                    Ok(o2) => {
                        event_value = o2;
                    }
                },
            }

            let mut event_value_expr = "";
            if !ev["event-value"]["expression"].is_null() {
                match ev["event-value"]["expression"].as_str() {
                    None => {
                        return Err(crate::ErrorType::Json);
                    }
                    Some(o) => {
                        event_value_expr = o;
                    }
                }
            }

            let mut event_value_expr_balance = false;
            if !ev["event-value"]["expr-balance"].is_null() {
                match ev["event-value"]["expr-balance"].as_bool() {
                    None => {
                        return Err(crate::ErrorType::Json);
                    }
                    Some(o) => {
                        event_value_expr_balance = o;
                    }
                }
            }

            let periods: usize;
            match ev["event-periods"]["periods"].as_usize() {
                None => {
                    return Err(crate::ErrorType::Json);
                }
                Some(o) => {
                    periods = o;
                }
            }

            let mut periods_expr = "";
            if !ev["event-periods"]["expression"].is_null() {
                match ev["event-periods"]["expression"].as_str() {
                    None => {
                        return Err(crate::ErrorType::Json);
                    }
                    Some(o) => {
                        periods_expr = o;
                    }
                }
            }

            let skip_len: usize;
            let skip_mask: u128;
            match ev["skip-mask"].as_str() {
                None => {
                    skip_len = 0;
                    skip_mask = 0;
                }
                Some(o) => {
                    let (tskip_len, tskip_mask) = self.get_skip_mask(o);
                    skip_len = tskip_len;
                    skip_mask = tskip_mask;
                }
            }

            let sort_order: usize;
            match ev["sort-order"].as_usize() {
                None => {
                    return Err(crate::ErrorType::Json);
                }
                Some(o) => {
                    sort_order = o;
                }
            }

            let intervals: usize;
            match ev["intervals"].as_usize() {
                None => {
                    return Err(crate::ErrorType::Json);
                }
                Some(o) => {
                    intervals = o;
                }
            }

            let frequency: crate::FrequencyType;
            match ev["frequency"].as_str() {
                None => {
                    return Err(crate::ErrorType::Json);
                }
                Some(o) => {
                    frequency = CoreUtility::get_frequency(o);
                }
            }

            if ev["extension"].is_null() {
                return Err(crate::ErrorType::Json);
            }

            let extension: ElemExtension;
            let result = self.deserialize_extension(&ev["extension"]);
            match result {
                Err(e) => {
                    return Err(e);
                }
                Ok(o) => { 
                    extension = o; 
                }
            }

            let mut params = ListParameter::new();
            if !ev["parameter-list"].is_null() {
                let result = self.deserialize_parameter_list(&ev["parameter-list"], &mut params);
                match result {
                    Err(e) => {
                        return Err(e);
                    }
                    Ok(_o) => {}
                }
            }

            let mut descs = ListDescriptor::new();
            if ev["descriptor-list"].is_null() {
                return Err(crate::ErrorType::Json);
            }
            let result = self.deserialize_descriptor_list(&ev["descriptor-list"], &mut descs);
            match result {
                Err(e) => {
                    return Err(e);
                }
                Ok(_o) => {}
            }

            let event_name: &str;
            match ev["event-name"].as_str() {
                None => {
                    event_name = "";
                }
                Some(o) => {
                    event_name = o;
                }
            }

            let next_name: &str;
            match ev["event-next-name"].as_str() {
                None => {
                    next_name = "";
                }
                Some(o) => {
                    next_name = o;
                }
            }

            events.add_event(
                event_date,
                event_date_expr,
                sort_order,
                event_value,
                event_value_expr,
                event_value_expr_balance,
                periods,
                periods_expr,
                skip_len,
                skip_mask,
                intervals,
                frequency,
                extension,
                Option::from(params),
                Option::from(descs),
                event_name,
                next_name,
            );

            index += 1;
        }

        events.set_sort_on_add(true); // Sorts list
        Ok(())
    }

    /// Deserialize and ingest Json exchange rates.
    ///
    /// # Arguments
    ///
    /// * `exch_rates` - Json value for exchange rates.
    /// * `exchange_rates` - List of exchange rates.
    ///
    /// # Return
    ///
    /// * ERROR_NONE if successful, otherwise error code.

    fn deserialize_exchange_rates(
        &self,
        exch_rates: &JsonValue,
    ) -> Result<ListExchangeRate, crate::ErrorType> {
        let mut exchange_rates = ListExchangeRate::new();
        let mut index: usize = 0;
        exchange_rates.set_sort_on_add(false);

        loop {
            let exch_rate = &exch_rates[index];
            if exch_rate.is_null() {
                break;
            }

            let from_str: &str;
            match exch_rate["from"].as_str() {
                None => {
                    return Err(crate::ErrorType::Json);
                }
                Some(o) => {
                    from_str = o;
                }
            }

            let to_str: &str;
            match exch_rate["to"].as_str() {
                None => {
                    return Err(crate::ErrorType::Json);
                }
                Some(o) => {
                    to_str = o;
                }
            }

            let value: Decimal;
            match exch_rate["value"].as_str() {
                None => {
                    return Err(crate::ErrorType::Json);
                }
                Some(o) => match o.parse::<Decimal>() {
                    Err(_e) => {
                        return Err(crate::ErrorType::Json);
                    }
                    Ok(o2) => {
                        value = o2;
                    }
                },
            }

            exchange_rates.add_exchange_rate(from_str, to_str, value);

            index += 1;
        }
        exchange_rates.set_sort_on_add(true); // Sorts list

        Ok(exchange_rates)
    }

    /// Deserialize and ingest Json extension.
    ///
    /// # Arguments
    ///
    /// * `ext` - Json value for extension.
    ///
    /// # Return
    ///
    /// * ERROR_NONE if successful, otherwise error code.

    pub fn deserialize_extension(
        &self,
        ext: &JsonValue,
    ) -> Result<ElemExtension, crate::ErrorType> {

        let event_type: crate::ExtensionType;
        if !ext["current-value"].is_null() {
            event_type = crate::ExtensionType::CurrentValue;
        } else if !ext["interest-change"].is_null() {
            event_type = crate::ExtensionType::InterestChange;
        } else if !ext["statistic-value"].is_null() {
            event_type = crate::ExtensionType::StatisticValue;
        } else {
            event_type = crate::ExtensionType::PrincipalChange;
        }

        let extension: ElemExtension;
        match event_type {
            crate::ExtensionType::CurrentValue => {
                let mut cv = ElemCurrentValue::new(false, false, false);

                let result = self.deserialize_current_value(&ext["current-value"], &mut cv);
                match result {
                    Err(e) => {
                        return Err(e);
                    }
                    Ok(_o) => {}
                }

                extension = ElemExtension::new_current_value(cv);
            }
            crate::ExtensionType::InterestChange => {
                let mut ic = ElemInterestChange::new(
                    crate::MethodType::Actuarial,
                    crate::DayCountType::Periodic,
                    crate::DEFAULT_DAYS_IN_YEAR,
                    crate::FrequencyType::None,
                    crate::FrequencyType::None,
                    crate::RoundType::None,
                    dec!(0.0),
                );

                let result = self.deserialize_interest_change(&ext["interest-change"], &mut ic);
                match result {
                    Err(e) => {
                        return Err(e);
                    }
                    Ok(_o) => {}
                }

                extension = ElemExtension::new_interest_change(ic);
            }
            crate::ExtensionType::StatisticValue => {
                let mut sv = ElemStatisticValue::new("", false, false);

                let result = self.deserialize_statistic_value(&ext["statistic-value"], &mut sv);
                match result {
                    Err(e) => {
                        return Err(e);
                    }
                    Ok(_o) => {}
                }

                extension = ElemExtension::new_statistic_value(sv);
            }
            _ => {
                let mut pc = ElemPrincipalChange::new(
                    crate::PrincipalType::Increase,
                    false,
                    false,
                    false,
                    false,
                    false,
                );

                let result =
                    self.deserialize_principal_change(&ext["principal-change"], &mut pc);
                match result {
                    Err(e) => {
                        return Err(e);
                    }
                    Ok(_o) => {}
                }

                extension = ElemExtension::new_principal_change(pc);
            }
        }

        Ok(extension)
    }

    /// Deserialize from a string and ingest Json extension.
    ///
    /// # Arguments
    ///
    /// * `ext_param` - String value for extension.
    ///
    /// # Return
    ///
    /// * ERROR_NONE if successful, otherwise error code.

    pub fn deserialize_extension_from_str(
        &self,
        ext_param: &str,
    ) -> Result<ElemExtension, crate::ErrorType> {
        let ext = if ext_param.starts_with('{') { 
            String::from(ext_param) } else { format!("{{{}}}", ext_param) };

        let data: JsonValue;
        match json::parse(ext.as_str()) {
            Err(e) => {
                println!("Json error: {:?}", e);
                return Err(crate::ErrorType::Json);
            }
            Ok(o) => {
                data = o;
            }
        }

        self.deserialize_extension(&data)
    }

    /// Deserialize and ingest Json interest change.
    ///
    /// # Arguments
    ///
    /// * `ic` - Json value for interest change.
    /// * `interest_change` - Element interest change.
    ///
    /// # Return
    ///
    /// * ERROR_NONE if successful, otherwise error code.

    fn deserialize_interest_change(
        &self,
        ic: &JsonValue,
        interest_change: &mut ElemInterestChange,
    ) -> Result<(), crate::ErrorType> {
        let calc_mgr = self.calc_mgr();
        let decimal_digits = calc_mgr.decimal_digits(false);

        match ic["round-balance"].as_str() {
            None => {}
            Some(o) => {
                let round_balance: crate::RoundType;

                match o {
                    "bankers" => {
                        round_balance = crate::RoundType::Bankers;
                    }
                    "bias-up" => {
                        round_balance = crate::RoundType::BiasUp;
                    }
                    "bias-down" => {
                        round_balance = crate::RoundType::BiasDown;
                    }
                    "up" => {
                        round_balance = crate::RoundType::Up;
                    }
                    "truncate" => {
                        round_balance = crate::RoundType::Truncate;
                    }
                    "yes" => {
                        round_balance = crate::RoundType::Bankers;
                    }
                    "no" => {
                        round_balance = crate::RoundType::None;
                    }
                    _ => {
                        round_balance = crate::RoundType::None;
                    }
                }

                interest_change.set_round_balance(round_balance);
            }
        }

        match ic["day-count-basis"].as_str() {
            None => {}
            Some(o) => {
                let day_count_basis: crate::DayCountType;

                match o {
                    "rule-of-78" => {
                        day_count_basis = crate::DayCountType::RuleOf78;
                    }
                    "actual" => {
                        day_count_basis = crate::DayCountType::Actual;
                    }
                    "actual-actual-isma" => {
                        day_count_basis = crate::DayCountType::ActualActualISMA;
                    }
                    "actual-actual-afb" => {
                        day_count_basis = crate::DayCountType::ActualActualAFB;
                    }
                    "actual-365L" => {
                        day_count_basis = crate::DayCountType::Actual365L;
                    }
                    "30" => {
                        day_count_basis = crate::DayCountType::Dc30;
                    }
                    "30E" => {
                        day_count_basis = crate::DayCountType::Dc30E;
                    }
                    "30EP" => {
                        day_count_basis = crate::DayCountType::Dc30EP;
                    }
                    _ => {
                        day_count_basis = crate::DayCountType::Periodic;
                    }
                }

                interest_change.set_day_count_basis(day_count_basis);
            }
        }
        match ic["days-in-year"].as_usize() {
            None => {}
            Some(o) => {
                interest_change.set_days_in_year(o);
            }
        }

        match ic["effective-frequency"].as_str() {
            None => {}
            Some(o) => {
                interest_change.set_effective_frequency(CoreUtility::get_frequency(o));
            }
        }

        match ic["interest-frequency"].as_str() {
            None => {}
            Some(o) => {
                interest_change.set_interest_frequency(CoreUtility::get_frequency(o));
            }
        }
        match ic["interest-method"].as_str() {
            None => {}
            Some(o) => {
                let interest_method: crate::MethodType;

                match o {
                    "simple-interest" => {
                        interest_method = crate::MethodType::SimpleInterest;
                    }
                    _ => {
                        interest_method = crate::MethodType::Actuarial;
                    }
                }

                interest_change.set_method(interest_method);
            }
        }

        match ic["round-decimal-digits"].as_str() {
            None => {
                interest_change.set_round_decimal_digits(dec!(decimal_digits));
            }
            Some(o) => {
                interest_change.set_round_decimal_digits(CoreUtility::parse_decimal(o));

                if interest_change.round_balance() == crate::RoundType::None {
                    interest_change.set_round_balance(crate::RoundType::Bankers);
                }
            }
        }

        Ok(())
    }

    /// Deserialize and ingest Json locales.
    ///
    /// # Arguments
    ///
    /// * `locales` - Json value for locales.
    ///
    /// # Return
    ///
    /// * List of locales if successful, otherwise error code.

    fn deserialize_locales(&self, locales: &JsonValue) -> Result<ListLocale, crate::ErrorType> {
        let mut list_locale = ListLocale::new();
        let mut index: usize = 0;

        loop {
            let locale = &locales[index];
            if locale.is_null() {
                break;
            }

            let locale_str: &str;
            match locale["locale-str"].as_str() {
                None => {
                    return Err(crate::ErrorType::Json);
                }
                Some(o) => {
                    locale_str = o;
                }
            }

            let currency_code: &str;
            match locale["currency-code"].as_str() {
                None => {
                    return Err(crate::ErrorType::Json);
                }
                Some(o) => {
                    currency_code = o;
                }
            }

            let mut decimal_digits = crate::DEFAULT_DECIMAL_DIGITS;
            match locale["decimal-digits"].as_usize() {
                None => {}
                Some(o) => {
                    decimal_digits = o;
                }
            }

            let format_in: ElemLocaleFormat;
            if locale["format-in"].is_null() {
                return Err(crate::ErrorType::Json);
            }
            match self.deserialize_locale_format(&locale["format-in"]) {
                Err(e) => {
                    return Err(e);
                }
                Ok(o) => {
                    format_in = o;
                }
            }

            let format_out: ElemLocaleFormat;
            if locale["format-out"].is_null() {
                return Err(crate::ErrorType::Json);
            }
            match self.deserialize_locale_format(&locale["format-out"]) {
                Err(e) => {
                    return Err(e);
                }
                Ok(o) => {
                    format_out = o;
                }
            }

            let mut resources: HashMap<String, String> = HashMap::new();
            let mut index2: usize = 0;

            loop {
                let resource = &locale["resources"][index2];
                if resource.is_null() {
                    break;
                }
                let key: &str;
                match resource["key"].as_str() {
                    None => {
                        return Err(crate::ErrorType::Json);
                    }
                    Some(o) => {
                        key = o;
                    }
                }
                let text: &str;
                match resource["text"].as_str() {
                    None => {
                        return Err(crate::ErrorType::Json);
                    }
                    Some(o) => {
                        text = o;
                    }
                }
                resources.insert(String::from(key), String::from(text));
                index2 += 1;
            }

            list_locale.add_locale(
                locale_str,
                currency_code,
                decimal_digits,
                format_in,
                format_out,
                resources,
            );

            index += 1;
        }

        Ok(list_locale)
    }

    /// Deserialize and ingest Json locale format.
    ///
    /// # Arguments
    ///
    /// * `locale_format` - Json value for locale format.
    ///
    /// # Return
    ///
    /// * Locale format element if successful, otherwise error code.

    fn deserialize_locale_format(
        &self,
        locale_format: &JsonValue,
    ) -> Result<ElemLocaleFormat, crate::ErrorType> {
        let date_regex: &str;
        match locale_format["date-regex"].as_str() {
            None => {
                return Err(crate::ErrorType::Json);
            }
            Some(o) => {
                date_regex = o;
            }
        }

        let date_replace: &str;
        match locale_format["date-replace"].as_str() {
            None => {
                return Err(crate::ErrorType::Json);
            }
            Some(o) => {
                date_replace = o;
            }
        }
        let integer_regex: &str;
        match locale_format["integer-regex"].as_str() {
            None => {
                return Err(crate::ErrorType::Json);
            }
            Some(o) => {
                integer_regex = o;
            }
        }

        let integer_replace: &str;
        match locale_format["integer-replace"].as_str() {
            None => {
                return Err(crate::ErrorType::Json);
            }
            Some(o) => {
                integer_replace = o;
            }
        }
        let decimal_regex: &str;
        match locale_format["decimal-regex"].as_str() {
            None => {
                return Err(crate::ErrorType::Json);
            }
            Some(o) => {
                decimal_regex = o;
            }
        }

        let decimal_replace: &str;
        match locale_format["decimal-replace"].as_str() {
            None => {
                return Err(crate::ErrorType::Json);
            }
            Some(o) => {
                decimal_replace = o;
            }
        }
        let currency_regex: &str;
        match locale_format["currency-regex"].as_str() {
            None => {
                return Err(crate::ErrorType::Json);
            }
            Some(o) => {
                currency_regex = o;
            }
        }

        let currency_replace: &str;
        match locale_format["currency-replace"].as_str() {
            None => {
                return Err(crate::ErrorType::Json);
            }
            Some(o) => {
                currency_replace = o;
            }
        }

        Ok(ElemLocaleFormat::new(
            date_regex,
            date_replace,
            integer_regex,
            integer_replace,
            decimal_regex,
            decimal_replace,
            currency_regex,
            currency_replace,
        ))
    }

    /// Deserialize and ingest Json parameters.
    ///
    /// # Arguments
    ///
    /// * `params` - Json value for parameters.
    /// * `parameters` - List of parameters.
    ///
    /// # Return
    ///
    /// * ERROR_NONE if successful, otherwise error code.

    fn deserialize_parameter_list(
        &self,
        params: &JsonValue,
        parameters: &mut ListParameter,
    ) -> Result<(), crate::ErrorType> {
        let mut index: usize = 0;

        loop {
            let param = &params[index];
            if param.is_null() {
                break;
            }

            let name: &str;
            match param["name"].as_str() {
                None => {
                    return Err(crate::ErrorType::Json);
                }
                Some(o) => {
                    name = o;
                }
            }

            let param_type: &str;
            match param["parameter-type"].as_str() {
                None => {
                    return Err(crate::ErrorType::Json);
                }
                Some(o) => {
                    param_type = o;
                }
            }

            let value: &str;
            match param["value"].as_str() {
                None => {
                    return Err(crate::ErrorType::Json);
                }
                Some(o) => {
                    value = o;
                }
            }

            parameters.add_parameter(name, true);

            match param_type {
                "integer" => {
                    parameters.set_integeri(CoreUtility::parse_integeri(value));
                }
                "decimal" => {
                    parameters.set_decimal(CoreUtility::parse_decimal(value));
                }
                _ => {
                    parameters.set_string(value);
                }
            }

            index += 1;
        }

        Ok(())
    }

    /// Deserialize and ingest Json preferences.
    ///
    /// # Arguments
    ///
    /// * `prefs` - Json value for preferences value.
    /// * `elem_level_param` - Element level
    ///
    /// # Return
    ///
    /// * ERROR_NONE if successful, otherwise error code.

    fn deserialize_preferences(
        &self,
        prefs: &JsonValue,
    ) -> Result<ElemPreferences, crate::ErrorType> {
        let mut preferences = ElemPreferences::new(
            &self.calc_manager,
            "",
            "",
            "",
            "",
            0,
            crate::DEFAULT_DECIMAL_DIGITS,
            -1,
            -1,
            -1,
            None,
            None,
            false,
            true,
        );

        match self.deserialize_preferences_with_prefs(prefs, &mut preferences) {
            Err(e) => Err(e),
            Ok(_o) => Ok(preferences),
        }
    }

    /// Deserialize and ingest Json preferences.
    ///
    /// # Arguments
    ///
    /// * `prefs` - Json value for preferences value.
    /// * `preferences` - Element preferences value.
    ///
    /// # Return
    ///
    /// * ERROR_NONE if successful, otherwise error code.

    fn deserialize_preferences_with_prefs(
        &self,
        prefs: &JsonValue,
        preferences: &mut ElemPreferences
    ) -> Result<(), crate::ErrorType> {
        match prefs["combine-principal"].as_i32() {
            None => {}
            Some(o) => {
                preferences.set_combine_principal(o);
            }
        }

        match prefs["compress-descriptor"].as_i32() {
            None => {}
            Some(o) => {
                preferences.set_compress_descriptor(o);
            }
        }

        match prefs["decimal-digits"].as_usize() {
            None => {}
            Some(o) => {
                preferences.set_decimal_digits(o);
            }
        }

        match prefs["default-encoding"].as_str() {
            None => {}
            Some(o) => {
                preferences.set_default_encoding(o);
            }
        }

        if !prefs["descriptor-list"].is_null() {
            let result = self.deserialize_descriptor_list(
                &prefs["descriptor-list"],
                preferences.list_descriptor_mut(),
            );
            match result {
                Err(e) => {
                    return Err(e);
                }
                Ok(_o) => {}
            }
        }

        match prefs["fiscal-year-start"].as_usize() {
            None => {}
            Some(o) => {
                preferences.set_fiscal_year_start(o);
            }
        }

        match prefs["group"].as_str() {
            None => {}
            Some(o) => {
                preferences.set_group(o);
            }
        }

        match prefs["locale"].as_str() {
            None => {}
            Some(o) => {
                preferences.set_locale_str(o);
            }
        }

        match prefs["omit-statistic-events"].as_i32() {
            None => {}
            Some(o) => {
                preferences.set_statistic_events(o);
            }
        }

        if !prefs["parameter-list"].is_null() {
            let result = self.deserialize_parameter_list(
                &prefs["parameter-list"],
                preferences.list_parameter_mut(),
            );
            match result {
                Err(e) => {
                    return Err(e);
                }
                Ok(_o) => {}
            }
        }

        Ok(())
    }

    /// Deserialize and ingest Json principal change.
    ///
    /// # Arguments
    ///
    /// * `pc` - Json value for principal change.
    /// * `prin_change` - Element principal change.
    ///
    /// # Return
    ///
    /// * ERROR_NONE if successful, otherwise error code.

    fn deserialize_principal_change(
        &self,
        pc: &JsonValue,
        prin_change: &mut ElemPrincipalChange,
    ) -> Result<(), crate::ErrorType> {
        match pc["auxiliary"].as_bool() {
            None => {}
            Some(o) => {
                prin_change.set_auxiliary(o);
            }
        }

        match pc["eom"].as_bool() {
            None => {}
            Some(o) => {
                prin_change.set_eom(o);
            }
        }

        match pc["passive"].as_bool() {
            None => {}
            Some(o) => {
                prin_change.set_aux_passive(o);
            }
        }

        match pc["principal-first"].as_bool() {
            None => {}
            Some(o) => {
                prin_change.set_principal_first(o);
            }
        }

        match pc["statistics"].as_bool() {
            None => {}
            Some(o) => {
                prin_change.set_balance_statistics(o);
            }
        }

        match pc["principal-type"].as_str() {
            None => {}
            Some(o) => {
                prin_change.set_type(CoreUtility::get_principal_type(o));
            }
        }
        Ok(())
    }

    /// Deserialize and ingest Json statistic value.
    ///
    /// # Arguments
    ///
    /// * `sv` - Json value for statistic value.
    /// * `stat_value` - Element statistic value.
    ///
    /// # Return
    ///
    /// * ERROR_NONE if successful, otherwise error code.

    fn deserialize_statistic_value(
        &self,
        sv: &JsonValue,
        stat_value: &mut ElemStatisticValue,
    ) -> Result<(), crate::ErrorType> {
        match sv["eom"].as_bool() {
            None => {}
            Some(o) => {
                stat_value.set_eom(o);
            }
        }

        match sv["final"].as_bool() {
            None => {}
            Some(o) => {
                stat_value.set_final(o);
            }
        }

        match sv["name"].as_str() {
            None => {}
            Some(o) => {
                stat_value.set_name(o);
            }
        }

        Ok(())
    }

    /// Deserialize and ingest Json template events.
    ///
    /// # Arguments
    ///
    /// * `templ_events` - Json value for template events.
    /// * `template_events` - List of template events.
    ///
    /// # Return
    ///
    /// * ERROR_NONE if successful, otherwise error code.

    fn deserialize_template_events(
        &self,
        templ_events: &JsonValue,
        template_events: &mut ListTemplateEvent,
    ) -> Result<(), crate::ErrorType> {
        let mut index: usize = 0;

        template_events.set_sort_on_add(false);

        loop {
            let templ_event = &templ_events[index];
            if templ_event.is_null() {
                break;
            }

            let name: &str;
            match templ_event["name"].as_str() {
                None => {
                    return Err(crate::ErrorType::Json);
                }
                Some(o) => {
                    name = o;
                }
            }

            let initial: bool;
            match templ_event["initial"].as_bool() {
                None => {
                    initial = false;
                }
                Some(o) => {
                    initial = o;
                }
            }

            if templ_event["event-list"].is_null() {
                return Err(crate::ErrorType::Json);
            }

            let mut list_event = ListEvent::new(false);

            let result = self.deserialize_event_list(&templ_event["event-list"], &mut list_event);
            match result {
                Err(_e) => {
                    panic!("Deserialize event list failed");
                }
                Ok(_o) => {}
            }

            template_events.add_template_event(name, initial, Option::from(list_event));

            index += 1;
        }
        template_events.set_sort_on_add(true); // Sorts list

        Ok(())
    }

    /// Deserialize and ingest Json template groups.
    ///
    /// # Arguments
    ///
    /// * `templ_groups` - Json value for template groups.
    /// * `template_groups` - List of template groups.
    ///
    /// # Return
    ///
    /// * ERROR_NONE if successful, otherwise error code.

    fn deserialize_template_groups(
        &self,
        templ_groups: &JsonValue,
    ) -> Result<ListTemplateGroup, crate::ErrorType> {
        let mut template_groups = ListTemplateGroup::new();
        let mut index: usize = 0;

        template_groups.set_calc_mgr(&self.calc_manager);
        template_groups.set_sort_on_add(false);

        loop {
            let templ_group = &templ_groups[index];
            if templ_group.is_null() {
                break;
            }

            let group: &str;
            match templ_group["group"].as_str() {
                None => {
                    return Err(crate::ErrorType::Json);
                }
                Some(o) => {
                    group = o;
                }
            }

            match template_groups.add_template_group(group) {
                Err(_e) => { panic!("Add template group failed"); }
                Ok(_o) => { }
            }

            if !templ_group["preferences"].is_null() {
                let result = self.deserialize_preferences_with_prefs(
                    &templ_group["preferences"],
                    template_groups.preferences_mut()
                );
                match result {
                    Err(_e) => {
                        panic!("Deserialize preferences failed");
                    }
                    Ok(_o) => {}
                }
            }

            if templ_group["template-events"].is_null() {
                return Err(crate::ErrorType::Json);
            }

            let result = self.deserialize_template_events(
                &templ_group["template-events"],
                template_groups.list_template_event_mut(),
            );
            match result {
                Err(_e) => {
                    panic!("Deserialize template events failed");
                }
                Ok(_o) => {}
            }

            index += 1;
        }
        template_groups.set_sort_on_add(true); // Sorts list

        Ok(template_groups)
    }

    /// Parse and return a numeric date.
    ///
    /// # Arguments
    ///
    /// * `event_date` - String date.
    ///
    /// # Return
    ///
    /// * See description.

    fn get_date(&self, event_date: &str) -> usize {
        match Regex::new(r"(\d{4})-(\d{2})-(\d{2})") {
            Err(_e) => {
                panic!("Invalid regular expression");
            }
            Ok(o) => match o.captures(event_date) {
                None => 0,
                Some(dt) => {
                    let year = CoreUtility::parse_integer(&dt[1]);
                    let month = CoreUtility::parse_integer(&dt[2]);
                    let day = CoreUtility::parse_integer(&dt[3]);

                    year * 10000 + month * 100 + day
                }
            },
        }
    }

    /// Parse and return a skip mask length and skip mask.
    ///
    /// # Arguments
    ///
    /// * `skip_mask_param` - String skip mask.
    ///
    /// # Return
    ///
    /// * See description.

    fn get_skip_mask(&self, skip_mask_param: &str) -> (usize, u128) {
        let skip_mask_len = skip_mask_param.len();
        let mut skip_mask: u128 = 0;

        for (index, ch) in skip_mask_param.chars().enumerate() {
            if ch == '1' {
                skip_mask |= 1 << index;
            }
        }

        (skip_mask_len, skip_mask)
    }
}
