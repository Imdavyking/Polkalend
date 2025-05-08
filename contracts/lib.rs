#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[allow(clippy::cast_possible_truncation)]
#[ink::contract]
mod polkalend {
    use ink::env::call::{build_call, ExecutionInput, Selector};
    use ink::env::{address, caller};
    use ink::prelude::vec::Vec;
    use ink::{storage::Mapping, H160, U256};

    #[ink(event)]
    pub struct LoanCreated {
        #[ink(topic)]
        lender: H160,
        #[ink(topic)]
        token: H160,
        amount: U256,
    }

    #[ink(event)]
    pub struct LoanAccepted {
        #[ink(topic)]
        borrower: H160,
        #[ink(topic)]
        lender: H160,
        #[ink(topic)]
        token: H160,
        amount: U256,
        collateral_token: H160,
        collateral_amount: U256,
    }

    #[ink(event)]
    pub struct LoanRepaid {
        #[ink(topic)]
        borrower: H160,
        #[ink(topic)]
        lender: H160,
        #[ink(topic)]
        token: H160,
        amount: U256,
    }

    #[ink(event)]
    pub struct CollateralReleased {
        #[ink(topic)]
        borrower: H160,
        #[ink(topic)]
        collateral_token: H160,
        amount: U256,
    }

    #[ink(storage)]
    pub struct LendingPlatform {
        // Mapping: (user, token_address) => collateral U256
        collateral: Mapping<(H160, H160), U256>,
        /// Mapping: (borrower, token_address) => debt U256
        debt: Mapping<(H160, H160), U256>,
        /// Mapping: (lender, token_address) => available lendable liquidity
        liquidity_pool: Mapping<(H160, H160), U256>,
        /// Active loans: borrower => list of (lender, token, amount)
        active_loans: Mapping<H160, Vec<(H160, H160, U256)>>,
        // Minimum collateral ratio (e.g., 150% = 15000 basis points)
        min_collateral_ratio: U256,
        // Owner of the contract (admin)
        owner: H160,
    }

    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        ZeroAmount,
        ZeroDuration,
        InsufficientLiquidity,
        CollateralTooLow,
        TransferFailed,
        Unauthorized,
        NotOwner,
        InvalidOwner,
        InvalidRatio,
        LoanNotFound,
        LoanNotEligibleForLiquidation,
        InsufficientTransferredValue,
        YouShouldNotSendDot,
        PaymentDoesNotMatchAmount,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl LendingPlatform {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                collateral: Default::default(),
                debt: Default::default(),
                liquidity_pool: Default::default(),
                active_loans: Default::default(),
                min_collateral_ratio: U256::from(15000),
                owner: caller(),
            }
        }

        #[ink(message)]
        pub fn get_user_h160(&self) -> H160 {
            self.env().caller()
        }

        #[ink(message)]
        pub fn get_owner(&self) -> H160 {
            self.owner
        }

        #[ink(message)]
        pub fn get_min_collateral_ratio(&self) -> U256 {
            self.min_collateral_ratio
        }

        // allow lenders to deposit tokens into the liquidity pool
        #[ink(message, payable)]
        pub fn create_loan(&mut self, token: H160, amount: U256, duration: U256) -> Result<()> {
            let user = self.env().caller();

            if amount.is_zero() {
                return Err(Error::ZeroAmount);
            }
            if duration.is_zero() {
                return Err(Error::ZeroDuration);
            }

            let current_liquidity = self.liquidity_pool.get((user, token)).unwrap_or_default();
            let updated = current_liquidity.checked_add(amount).unwrap();

            self.liquidity_pool.insert((user, token), &updated);

            if token == H160::zero() {
                if self.env().transferred_value() != amount {
                    return Err(Error::InsufficientTransferredValue);
                }
            } else {
                if self.env().transferred_value() != U256::zero() {
                    return Err(Error::YouShouldNotSendDot);
                }
                build_call::<Environment>()
                    .call(token)
                    .exec_input(
                        ExecutionInput::new(Selector::new(ink::selector_bytes!("transfer_from")))
                            .push_arg(user)
                            .push_arg(address())
                            .push_arg(amount),
                    )
                    .returns::<()>()
                    .invoke();
            }

            // self.env().emit_event(LoanCreated {
            //     lender: user,
            //     token,
            //     amount,
            // });

            Ok(())
        }

        #[ink(message, payable)]
        pub fn lock_collateral(
            &mut self,
            collateral_token: H160,
            collateral_amount: U256,
        ) -> Result<()> {
            let borrower = self.env().caller();

            if collateral_amount.is_zero() {
                return Err(Error::ZeroAmount);
            }

            let current_collateral = self
                .collateral
                .get((borrower, collateral_token))
                .unwrap_or_default();
            let updated = current_collateral.checked_add(collateral_amount).unwrap();

            self.collateral
                .insert((borrower, collateral_token), &updated);

            if collateral_token == H160::zero() {
                if self.env().transferred_value() != collateral_amount {
                    return Err(Error::InsufficientTransferredValue);
                }
            } else {
                if self.env().transferred_value() != U256::zero() {
                    return Err(Error::YouShouldNotSendDot);
                }
                build_call::<Environment>()
                    .call(collateral_token)
                    .exec_input(
                        ExecutionInput::new(Selector::new(ink::selector_bytes!("transfer_from")))
                            .push_arg(borrower)
                            .push_arg(address())
                            .push_arg(collateral_amount),
                    )
                    .returns::<()>()
                    .invoke();
            }
            Ok(())
        }

        #[ink(message)]
        pub fn accept_loan(&mut self, lender: H160, token: H160, amount: U256) -> Result<()> {
            let borrower = self.env().caller();

            if amount.is_zero() {
                return Err(Error::ZeroAmount);
            }

            let liquidity = self.liquidity_pool.get((lender, token)).unwrap_or_default();
            if liquidity < amount {
                return Err(Error::InsufficientLiquidity);
            }

            // Check minimum collateral ratio
            let required_collateral = amount
                .checked_mul(U256::from(self.min_collateral_ratio))
                .unwrap()
                .checked_div(U256::from(10_000))
                .unwrap();

            //TODO: track the collateral for the borrower
            //TODO: Check if borrower has enough collateral

            let current_collateral = self.collateral.get((borrower, token)).unwrap_or_default();

            if current_collateral < required_collateral {
                return Err(Error::CollateralTooLow);
            }

            // Reduce liquidity
            self.liquidity_pool
                .insert((lender, token), &(liquidity.checked_sub(amount).unwrap()));

            // Track loan
            let previous_debt = self.debt.get((borrower, token)).unwrap_or_default();
            let new_debt = previous_debt.checked_add(amount).unwrap();
            self.debt.insert((borrower, token), &new_debt);
            let mut loans = self.active_loans.get(borrower).unwrap_or_default();
            loans.push((lender, token, amount));
            self.active_loans.insert(borrower, &loans);

            // Send funds to borrower
            if token == H160::zero() {
                self.env()
                    .transfer(borrower, amount)
                    .map_err(|_| Error::InsufficientLiquidity)?; // Fallback error
            } else {
                build_call::<Environment>()
                    .call(token)
                    .exec_input(
                        ExecutionInput::new(Selector::new(ink::selector_bytes!("transfer")))
                            .push_arg(borrower)
                            .push_arg(amount),
                    )
                    .returns::<()>()
                    .invoke();
            }

            Ok(())
        }

        #[ink(message, payable)]
        pub fn pay_loan(&mut self, token: H160, lender: H160, amount: U256) -> Result<()> {
            let borrower = self.env().caller();
            let debt = self.debt.get((borrower, token)).unwrap_or_default();

            if debt < amount {
                return Err(Error::InsufficientLiquidity);
            }

            // Transfer repayment to lender
            if token == H160::zero() {
                if self.env().transferred_value() != amount {
                    return Err(Error::PaymentDoesNotMatchAmount);
                }
                self.env()
                    .transfer(lender, amount)
                    .map_err(|_| Error::InsufficientLiquidity)?;
            } else {
                build_call::<Environment>()
                    .call(token)
                    .exec_input(
                        ExecutionInput::new(Selector::new(ink::selector_bytes!("transfer_from")))
                            .push_arg(borrower)
                            .push_arg(lender)
                            .push_arg(amount),
                    )
                    .returns::<()>()
                    .invoke();
            }

            // Update debt
            let updated_debt = debt.checked_sub(amount).unwrap();
            self.debt.insert((borrower, token), &updated_debt);

            // If loan is fully repaid, release collateral
            if updated_debt.is_zero() {
                let collateral_token = token; // This assumes same token used. Adapt logic for different tokens.
                let collateral_amount = self
                    .collateral
                    .get((borrower, collateral_token))
                    .unwrap_or_default();

                if collateral_token == H160::zero() {
                    self.env()
                        .transfer(borrower, collateral_amount)
                        .map_err(|_| Error::InsufficientLiquidity)?;
                } else {
                    build_call::<Environment>()
                        .call(collateral_token)
                        .exec_input(
                            ExecutionInput::new(Selector::new(ink::selector_bytes!("transfer")))
                                .push_arg(borrower)
                                .push_arg(collateral_amount),
                        )
                        .returns::<()>()
                        .invoke();
                }

                self.collateral
                    .insert((borrower, collateral_token), &U256::zero());
            }

            Ok(())
        }

        #[ink(message)]
        pub fn set_min_collateral_ratio(&mut self, new_ratio: U256) -> Result<()> {
            if self.env().caller() != self.owner {
                return Err(Error::Unauthorized);
            }
            self.min_collateral_ratio = new_ratio;
            Ok(())
        }

        #[ink(message)]
        pub fn get_user_loans(&self, borrower: H160) -> Vec<(H160, H160, U256)> {
            self.active_loans.get(borrower).unwrap_or_default()
        }

        #[ink(message)]
        pub fn get_liquidity(&self, lender: H160, token: H160) -> U256 {
            self.liquidity_pool.get((lender, token)).unwrap_or_default()
        }

        #[ink(message)]
        pub fn get_debt(&self, borrower: H160, token: H160) -> U256 {
            self.debt.get((borrower, token)).unwrap_or_default()
        }

        #[ink(message)]
        pub fn get_collateral(&self, borrower: H160, token: H160) -> U256 {
            self.collateral.get((borrower, token)).unwrap_or_default()
        }

        #[ink(message)]
        pub fn get_active_loans(&self, borrower: H160) -> Vec<(H160, H160, U256)> {
            self.active_loans.get(borrower).unwrap_or_default()
        }

        #[ink(message)]
        pub fn update_min_collateral_ratio(&mut self, new_ratio: U256) -> Result<()> {
            if self.env().caller() != self.owner {
                return Err(Error::NotOwner);
            }
            if new_ratio < U256::from(10000) {
                return Err(Error::InvalidRatio);
            }
            self.min_collateral_ratio = new_ratio;
            Ok(())
        }

        #[ink(message)]
        pub fn set_owner(&mut self, new_owner: H160) -> Result<()> {
            if self.env().caller() != self.owner {
                return Err(Error::NotOwner);
            }
            if new_owner == H160::zero() {
                return Err(Error::InvalidOwner);
            }
            self.owner = new_owner;
            Ok(())
        }
        #[ink(message)]
        pub fn calculate_required_collateral(&self, amount: U256) -> U256 {
            amount
                .checked_mul(U256::from(self.min_collateral_ratio))
                .unwrap()
                .checked_div(U256::from(10_000))
                .unwrap()
        }

        #[ink(message)]
        pub fn is_undercollateralized(&self, borrower: H160, token: H160) -> bool {
            let debt = self.get_debt(borrower, token);
            let collateral = self.get_collateral(borrower, token);
            let required = self.calculate_required_collateral(debt);
            collateral < required
        }

        #[ink(message)]
        pub fn liquidate(&mut self, borrower: H160, token: H160) -> Result<()> {
            let debt = self.get_debt(borrower, token);
            if debt.is_zero() {
                return Err(Error::LoanNotFound);
            }

            if !self.is_undercollateralized(borrower, token) {
                return Err(Error::LoanNotEligibleForLiquidation);
            }

            let collateral_token = token; // simplify for demo
            let collateral_amount = self.get_collateral(borrower, collateral_token);

            self.debt.insert((borrower, token), &U256::zero());
            self.collateral
                .insert((borrower, collateral_token), &U256::zero());

            let caller = self.env().caller();
            if collateral_token == H160::zero() {
                self.env()
                    .transfer(caller, collateral_amount)
                    .map_err(|_| Error::TransferFailed)?;
            } else {
                build_call::<Environment>()
                    .call(collateral_token)
                    .exec_input(
                        ExecutionInput::new(Selector::new(ink::selector_bytes!("transfer")))
                            .push_arg(caller)
                            .push_arg(collateral_amount),
                    )
                    .returns::<()>()
                    .invoke();
            }

            Ok(())
        }
    }
}
