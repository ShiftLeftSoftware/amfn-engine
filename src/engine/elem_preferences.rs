//! Preferences definition element.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cell::{Cell, Ref, RefCell};
use std::rc::Rc;

use super::CalcManager;
use crate::core::{CoreManager, CoreUtility, ListDescriptor, ListParameter};
use crate::{ElemLevelType, ElemUpdateType, ListTrait};

pub struct ElemPreferences {
    /// Calculator manager element.
    calc_manager: Rc<RefCell<CalcManager>>,

    /// ISO language code (ISO 639)_ISO country code (ISO 3166).
    locale_str: String,
    /// Cross rate international currency code (e.g., USD, GBP, JPY, AUD, EUR, other currency code).
    cross_rate_code: String,
    /// Default encoding (us-ascii, iso-8859-1, utf-8, utf-16be, utf-16le, utf-16, other encoding).
    default_encoding: String,
    /// Template group name.
    group: String,
    /// Start of fiscal year in MMDD format.
    fiscal_year_start: Cell<usize>,
    /// Number of significant decimal digits.
    decimal_digits: Cell<usize>,

    /// Parameter list.
    list_parameter: ListParameter,
    /// Descriptor list.
    list_descriptor: ListDescriptor,
    /// Combine principal change events that are identical except their
    /// values and ListDescriptor objects (1=set, 0=reset, -1=not set).
    combine_principal: i32,
    /// After balancing and during compression, merge ListDescriptor
    /// objects (1=set, 0=reset, -1=not set).
    compress_descriptor: i32,
    /// Eliminate statistic events from the amortization list
    /// (1=set, 0=reset, -1=not set).
    statistic_events: i32,

    /// Element level.
    elem_level: ElemLevelType,
}

/// Preferences definition implementation.

impl ElemPreferences {
    /// Create and return a new preferences element.
    ///
    /// # Arguments
    ///
    /// * `calc_manager_param` - Calculator manager element.
    /// * `locale_str_param` - Locale string.
    /// * `cross_rate_code_param` - Cross rate code.
    /// * `default_encoding_param` - Default encoding.
    /// * `group_param` - Group name.
    /// * `fiscal_year_start_param` - Fiscal year start.
    /// * `decimal_digits_param` - Decimal digits.
    /// * `combine_principal_param` - Combine principal.
    /// * `compress_descriptor_param` - Compress descriptors.
    /// * `statistic_events_param` - Statistic events.
    /// * `list_parameter_param` - List parameter.
    /// * `list_descriptor_param` - List descriptor.
    /// * `copy_propagate_param` - Copy propogate.
    /// * `elem_level_param` - Element level
    /// * `updating_json` - Updating from Json.
    ///
    /// # Return
    ///
    /// * See description.
    #[allow(clippy::too_many_arguments)]

    pub fn new(
        calc_manager_param: &Rc<RefCell<CalcManager>>,
        locale_str_param: &str,
        cross_rate_code_param: &str,
        default_encoding_param: &str,
        group_param: &str,
        fiscal_year_start_param: usize,
        decimal_digits_param: usize,
        combine_principal_param: i32,
        compress_descriptor_param: i32,
        statistic_events_param: i32,
        list_parameter_param: Option<&ListParameter>,
        list_descriptor_param: Option<&ListDescriptor>,
        copy_propagate_param: bool,
        elem_level_param: ElemLevelType,
        updating_json: bool,
    ) -> ElemPreferences {
        ElemPreferences::new_with_calc_manager(
            calc_manager_param,
            calc_manager_param.borrow().core_manager(),
            locale_str_param,
            cross_rate_code_param,
            default_encoding_param,
            group_param,
            fiscal_year_start_param,
            decimal_digits_param,
            combine_principal_param,
            compress_descriptor_param,
            statistic_events_param,
            list_parameter_param,
            list_descriptor_param,
            copy_propagate_param,
            elem_level_param,
            updating_json,
        )
    }

    /// Create and return a new preferences element.
    ///
    /// # Arguments
    ///
    /// * `calc_manager_param` - Calculator manager element.
    /// * `core_manager_param` - CoreManager element.
    /// * `locale_str_param` - Locale string.
    /// * `cross_rate_code_param` - Cross rate code.
    /// * `default_encoding_param` - Default encoding.
    /// * `group_param` - Group name.
    /// * `fiscal_year_start_param` - Fiscal year start.
    /// * `decimal_digits_param` - Decimal digits.
    /// * `combine_principal_param` - Combine principal.
    /// * `compress_descriptor_param` - Compress descriptors.
    /// * `statistic_events_param` - Statistic events.
    /// * `list_parameter_param` - List parameter.
    /// * `list_descriptor_param` - List descriptor.
    /// * `copy_propagate_param` - Copy propogate.
    /// * `elem_level_param` - Element level
    /// * `updating_json` - Updating from Json.
    ///
    /// # Return
    ///
    /// * See description.
    #[allow(clippy::too_many_arguments)]

    pub fn new_with_calc_manager(
        calc_manager_param: &Rc<RefCell<CalcManager>>,
        core_manager_param: &Rc<RefCell<CoreManager>>,
        locale_str_param: &str,
        cross_rate_code_param: &str,
        default_encoding_param: &str,
        group_param: &str,
        fiscal_year_start_param: usize,
        decimal_digits_param: usize,
        combine_principal_param: i32,
        compress_descriptor_param: i32,
        statistic_events_param: i32,
        list_parameter_param: Option<&ListParameter>,
        list_descriptor_param: Option<&ListDescriptor>,
        copy_propagate_param: bool,
        elem_level_param: ElemLevelType,
        updating_json: bool,
    ) -> ElemPreferences {
        let new_list_parameter: ListParameter;
        let new_list_descriptor: ListDescriptor;
        match list_parameter_param.as_ref() {
            None => {
                new_list_parameter = ListParameter::new(core_manager_param, elem_level_param);
            }
            Some(o) => {
                new_list_parameter = o.copy(elem_level_param, updating_json);
            }
        }

        match list_descriptor_param.as_ref() {
            None => {
                new_list_descriptor = ListDescriptor::new(core_manager_param, elem_level_param);
            }
            Some(o) => {
                new_list_descriptor = o.copy(copy_propagate_param, elem_level_param, updating_json);
            }
        }

        ElemPreferences {
            calc_manager: Rc::clone(calc_manager_param),
            locale_str: String::from(locale_str_param),
            cross_rate_code: String::from(cross_rate_code_param),
            default_encoding: String::from(default_encoding_param),
            group: String::from(group_param),
            fiscal_year_start: Cell::new(fiscal_year_start_param),
            decimal_digits: Cell::new(decimal_digits_param),
            combine_principal: combine_principal_param,
            compress_descriptor: compress_descriptor_param,
            statistic_events: statistic_events_param,
            list_parameter: new_list_parameter,
            list_descriptor: new_list_descriptor,
            elem_level: elem_level_param,
        }
    }

    /// Clear all values and lists.

    pub fn clear(&mut self) {
        self.locale_str = String::from("");
        self.cross_rate_code = String::from("");
        self.default_encoding = String::from("");
        self.group = String::from("");
        self.fiscal_year_start.set(0);
        self.decimal_digits.set(0);
        self.combine_principal = -1;
        self.compress_descriptor = -1;
        self.statistic_events = -1;

        self.list_parameter.clear();
        self.list_descriptor.clear();
    }

    /// Returns the calculation manager.
    ///
    /// # Return
    ///
    /// * See description.

    fn calc_mgr(&self) -> Ref<CalcManager> {
        self.calc_manager.borrow()
    }

    /// Copy this preferences element and return a new preferences element.
    ///
    /// # Arguments
    ///
    /// * `updating_json` - Updating from Json.
    /// * `elem_level_param` - Element level
    ///
    /// # Return
    ///
    /// * See description.

    pub fn copy(&self, elem_level_param: ElemLevelType, updating_json: bool) -> ElemPreferences {
        self.copy_with_calc_manager(&self.calc_manager, elem_level_param, updating_json)
    }

    /// Copy this preferences element and return a new preferences element.
    ///
    /// # Arguments
    ///
    /// * `calc_manager` - Calculator manager element.
    /// * `elem_level_param` - Element level
    /// * `updating_json` - Updating from Json.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn copy_with_calc_manager(
        &self,
        calc_manager: &Rc<RefCell<CalcManager>>,
        elem_level_param: ElemLevelType,
        updating_json: bool,
    ) -> ElemPreferences {
        ElemPreferences::new(
            calc_manager,
            self.locale_str.as_str(),
            self.cross_rate_code.as_str(),
            self.default_encoding.as_str(),
            self.group.as_str(),
            self.fiscal_year_start.get(),
            self.decimal_digits.get(),
            self.combine_principal,
            self.compress_descriptor,
            self.statistic_events,
            Option::from(&self.list_parameter.copy(elem_level_param, updating_json)),
            Option::from(
                &self
                    .list_descriptor
                    .copy(false, elem_level_param, updating_json),
            ),
            false,
            elem_level_param,
            updating_json,
        )
    }

    /// Get the locale.
    ///
    /// # Return
    ///
    /// * ISO language code (ISO 639)_ISO country code (ISO 3166).

    pub fn locale_str(&self) -> &str {
        &self.locale_str
    }

    /// Get the cross rate international currency code (e.g., USD, GBP, JPY, AUD, EUR, other currency code).
    ///
    /// # Return
    ///
    /// * See description.

    pub fn cross_rate_code(&self) -> &str {
        &self.cross_rate_code
    }

    /// Get the default encoding (us-ascii, iso-8859-1, utf-8, utf-16be, utf-16le, utf-16, other encoding).
    ///
    /// # Return
    ///
    /// * See description.

    pub fn default_encoding(&self) -> &str {
        &self.default_encoding
    }

    /// Get the template group name.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn group(&self) -> &str {
        &self.group
    }

    /// Get the start of fiscal year in MMDD format.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn fiscal_year_start(&self) -> usize {
        self.fiscal_year_start.get()
    }

    /// Get the number of significant decimal digits.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn decimal_digits(&self) -> usize {
        self.decimal_digits.get()
    }

    /// Get the combine principal change events that are identical except
    /// their values and ListDescriptor objects (1=set,0=reset,-1=not set).
    ///
    /// # Return
    ///
    /// * See description.

    pub fn combine_principal(&self) -> i32 {
        self.combine_principal
    }

    /// Get the after balancing and during compression, merge
    /// ListDescriptor objects (1=set,0=reset,-1=not set).
    ///
    /// # Return
    ///
    /// * See description.

    pub fn compress_descriptor(&self) -> i32 {
        self.compress_descriptor
    }

    /// Get the eliminate statistic events from the compressed
    /// event list (1=set,0=reset,-1=not set).
    ///
    /// # Return
    ///
    /// * See description.

    pub fn statistic_events(&self) -> i32 {
        self.statistic_events
    }

    /// Get the element level.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn elem_level(&self) -> ElemLevelType {
        self.elem_level
    }

    /// Get the parameter list.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_parameter(&self) -> &ListParameter {
        &self.list_parameter
    }

    /// Get the mut parameter list.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_parameter_mut(&mut self) -> &mut ListParameter {
        &mut self.list_parameter
    }

    /// Get the descriptor list.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_descriptor(&self) -> &ListDescriptor {
        &self.list_descriptor
    }

    /// Get the mut descriptor list.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_descriptor_mut(&mut self) -> &mut ListDescriptor {
        &mut self.list_descriptor
    }

    /// Set the locale name.
    ///
    /// # Arguments
    ///
    /// * `locale_str_param` - See description.

    pub fn set_locale_str(&mut self, locale_str_param: &str) {
        self.locale_str = String::from(locale_str_param);

        self.set_updated();
    }

    /// Set the cross rate international currency code (e.g., USD, GBP, JPY, AUD, EUR, other currency code).
    ///
    /// # Arguments
    ///
    /// * `cross_rate_code_param` - See description.

    pub fn set_cross_rate_code(&mut self, cross_rate_code_param: &str) {
        self.cross_rate_code = String::from(cross_rate_code_param);

        self.set_updated();
    }

    /// Set the default encoding (us-ascii, iso-8859-1, utf-8, utf-16be, utf-16le, utf-16, other encoding).
    ///
    /// # Arguments
    ///
    /// * `default_encoding_param` - See description.

    pub fn set_default_encoding(&mut self, default_encoding_param: &str) {
        self.default_encoding = String::from(default_encoding_param);

        self.set_updated();
    }

    /// Set the template group name.
    ///
    /// # Arguments
    ///
    /// * `group_param` - See description.

    pub fn set_group(&mut self, group_param: &str) {
        self.set_group_result(group_param);

        self.set_updated();
    }

    /// Set the template group name result.
    ///
    /// # Arguments
    ///
    /// * `group_param` - See description.

    pub fn set_group_result(&mut self, group_param: &str) {
        self.group = String::from(group_param);
    }

    /// Set the start of fiscal year in MMDD format.
    ///
    /// # Arguments
    ///
    /// * `fiscal_year_start_param` - See description.

    pub fn set_fiscal_year_start(&self, fiscal_year_start_param: usize) {
        self.fiscal_year_start.set(fiscal_year_start_param);

        if self.elem_level != ElemLevelType::Engine {
            self.calc_mgr().list_cashflow().update_preferences();
        }

        self.set_updated();
    }

    /// Set the number of significant decimal digits.
    ///
    /// # Arguments
    ///
    /// * `decimal_digits_param` - See description.

    pub fn set_decimal_digits(&self, decimal_digits_param: usize) {
        self.decimal_digits.set(decimal_digits_param);

        if self.elem_level != ElemLevelType::Engine {
            self.calc_mgr().list_cashflow().update_preferences();
        }

        self.set_updated();
    }

    /// Set the combine principal change events that are identical except
    /// their values and ListDescriptor objects (1=set,0=reset,-1=not set).
    ///
    /// # Arguments
    ///
    /// * `combine_principal_param` - See description.

    pub fn set_combine_principal(&mut self, combine_principal_param: i32) {
        self.combine_principal = combine_principal_param;

        self.set_updated();
    }

    /// Set the After balancing and during compression, merge
    /// ListDescriptor objects (1=set,0=reset,-1=not set).
    ///
    /// # Arguments
    ///
    /// * `compress_descriptor_param` - See description.

    pub fn set_compress_descriptor(&mut self, compress_descriptor_param: i32) {
        self.compress_descriptor = compress_descriptor_param;

        self.set_updated();
    }

    /// Set the Eliminate statistic events from the compressed
    /// event list (1=set,0=reset,-1=not set).
    ///
    /// # Arguments
    ///
    /// * `statistic_events_param` - See description.

    pub fn set_statistic_events(&mut self, statistic_events_param: i32) {
        self.statistic_events = statistic_events_param;

        self.set_updated();
    }

    /// Set the element level.
    ///
    /// # Arguments
    ///
    /// * `elem_level_param` - See description.

    pub fn set_elem_level(&mut self, elem_level_param: ElemLevelType) {
        self.elem_level = elem_level_param;
    }

    /// Call the updated signal.

    fn set_updated(&self) {
        self.calc_mgr().mgr().notify(CoreUtility::format_update(
            ElemUpdateType::Preferences,
            self.elem_level,
        ));
    }
}
