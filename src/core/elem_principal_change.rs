//! The principal change definition of an event.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub struct ElemPrincipalChange {
    /// Principal change type.
    pc_type: crate::PrincipalType,
    /// Adjust successive dates to end of month.
    eom: bool,
    /// Apply change to principal balance first for simple interest.
    principal_first: bool,
    /// Mark as balance result statistics.
    balance_statistics: bool,
    /// Mark as auxiliary principal change event.
    auxiliary: bool,
    /// Mark as auxiliary passive principal change event.
    aux_passive: bool,
}

/// The principal change implementation.

impl ElemPrincipalChange {
    /// Create a new principal change element.
    ///
    /// # Arguments
    ///
    /// * `type_param` - Principal change type.
    /// * `eom_param` - End-of-month.
    /// * `principal_first_param` - Principal first.
    /// * `balance_statistics_param` - Balance statistics.
    /// * `auxiliary_param` - Auxiliary parameter.
    /// * `aux_passive_param` - Auxiliary passive.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn new(
        type_param: crate::PrincipalType,
        eom_param: bool,
        principal_first_param: bool,
        balance_statistics_param: bool,
        auxiliary_param: bool,
        aux_passive_param: bool,
    ) -> ElemPrincipalChange {
        ElemPrincipalChange {
            pc_type: type_param,
            eom: eom_param,
            principal_first: principal_first_param,
            balance_statistics: balance_statistics_param,
            auxiliary: auxiliary_param,
            aux_passive: aux_passive_param,
        }
    }

    /// Copy this principal change element as a new principal change element.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn copy(&self) -> ElemPrincipalChange {
        ElemPrincipalChange::new(
            self.pc_type,
            self.eom,
            self.principal_first,
            self.balance_statistics,
            self.auxiliary,
            self.aux_passive,
        )
    }

    /// Tests if this principal change object and another are equal.
    ///
    /// # Arguments
    ///
    /// * `objElemPrincipalChange` - Object to compare.
    /// # Return
    ///
    /// * True if equals, otherwise false.

    pub fn equal(&self, elem_principal_change: &ElemPrincipalChange) -> bool {
        self.pc_type == elem_principal_change.pc_type
            && self.eom == elem_principal_change.eom
            && self.principal_first == elem_principal_change.principal_first
            && self.balance_statistics == elem_principal_change.balance_statistics
            && self.auxiliary == elem_principal_change.auxiliary
            && self.aux_passive == elem_principal_change.aux_passive
    }

    /// Get the principal change type.
    ///
    /// # Return
    ///
    /// * See description.
    ///     

    pub fn pc_type(&self) -> crate::PrincipalType {
        self.pc_type
    }

    /// Get the value of adjust successive dates to end of month.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn eom(&self) -> bool {
        self.eom
    }

    /// Get the value of apply change to principal balance first for simple interest.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn principal_first(&self) -> bool {
        self.principal_first
    }

    /// Get the value of include with balance result statistics.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn balance_statistics(&self) -> bool {
        self.balance_statistics
    }

    /// Get the auxiliary principal change event.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn auxiliary(&self) -> bool {
        self.auxiliary
    }

    /// Get the auxiliary passive principal change event.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn aux_passive(&self) -> bool {
        self.aux_passive
    }

    /// Set the principal change type.
    ///
    /// # Arguments
    ///
    /// * `type_param` - See description.
    ///     

    pub fn set_type(&mut self, type_param: crate::PrincipalType) {
        self.pc_type = type_param;
    }

    /// Set the value of adjust successive dates to end of month.
    ///
    /// # Arguments
    ///
    /// * `eom_param` - See description.

    pub fn set_eom(&mut self, eom_param: bool) {
        self.eom = eom_param;
    }

    /// Set the value of apply change to principal balance first for simple interest.
    ///
    /// # Arguments
    ///
    /// * `principal_first_param` - See description.

    pub fn set_principal_first(&mut self, principal_first_param: bool) {
        self.principal_first = principal_first_param;
    }

    /// Set the value of include with balance result statistics.
    ///
    /// # Arguments
    ///
    /// * `balance_statistics_param` - See description.

    pub fn set_balance_statistics(&mut self, balance_statistics_param: bool) {
        self.balance_statistics = balance_statistics_param;
    }

    /// Set the auxiliary principal change event.
    ///
    /// # Arguments
    ///
    /// * `auxiliary_param` - See description.

    pub fn set_auxiliary(&mut self, auxiliary_param: bool) {
        self.auxiliary = auxiliary_param;
    }

    /// Set the auxiliary passive principal change event.
    ///
    /// # Arguments
    ///
    /// * `aux_passive_param` - See description.

    pub fn set_aux_passive(&mut self, aux_passive_param: bool) {
        self.aux_passive = aux_passive_param;
    }
}
