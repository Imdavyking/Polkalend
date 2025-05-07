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
        pub fn get_collaterial_ratio(&self) -> U256 {
            self.min_collateral_ratio
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: H160, value: U256) -> Result<()> {
            let from = self.env().caller();
            // self.transfer_from_to(&from, &to, value)
        }

        // allow lenders to deposit tokens into the liquidity pool
        // #[ink(message, payable)]
        // pub fn create_loan(&mut self, token: H160, amount: U256, duration: U256) -> bool {
        //     true
        // let caller = self.env().caller();

        // if amount.is_zero() {
        //     return Err(Error::ZeroAmount);
        // }
        // if duration.is_zero() {
        //     return Err(Error::ZeroDuration);
        // }

        // let current_liquidity = self.liquidity_pool.get((caller, token)).unwrap_or_default();
        // let updated = current_liquidity.checked_add(amount).unwrap();

        // self.liquidity_pool.insert((caller, token), &updated);

        // if token == H160::zero() {
        //     assert!(
        //         self.env().transferred_value() == amount,
        //         "payment was not equal to the amount"
        //     );
        // } else {
        //     assert!(
        //         self.env().transferred_value() == U256::zero(),
        //         "you should not send native tokens"
        //     );
        //     build_call::<Environment>()
        //         .call(token)
        //         .exec_input(
        //             ExecutionInput::new(Selector::new(ink::selector_bytes!("transfer_from")))
        //                 .push_arg(caller)
        //                 .push_arg(address())
        //                 .push_arg(amount),
        //         )
        //         .returns::<()>()
        //         .invoke();
        // }

        // self.env().emit_event(LoanCreated {
        //     lender: caller,
        //     token,
        //     amount,
        // });

        // // Ok(())
        // }

        // #[ink(message)]
        // pub fn accept_loan(
        //     &mut self,
        //     lender: H160,
        //     token: H160,
        //     amount: U256,
        //     collateral_token: H160,
        //     collateral_amount: U256,
        // ) -> Result<()> {
        //     let borrower = self.env().caller();

        //     if amount.is_zero() || collateral_amount.is_zero() {
        //         return Err(Error::ZeroAmount);
        //     }

        //     let liquidity = self.liquidity_pool.get((lender, token)).unwrap_or_default();
        //     if liquidity < amount {
        //         return Err(Error::InsufficientLiquidity);
        //     }

        //     // Check minimum collateral ratio
        //     let required_collateral = amount
        //         .checked_mul(U256::from(self.min_collateral_ratio))
        //         .unwrap()
        //         .checked_div(U256::from(10_000))
        //         .unwrap();
        //     if collateral_amount < required_collateral {
        //         return Err(Error::ZeroDuration); // Using ZeroDuration here as a placeholder
        //     }

        //     // Lock collateral
        //     self.collateral
        //         .insert((borrower, collateral_token), &collateral_amount);

        //     // Reduce liquidity
        //     self.liquidity_pool
        //         .insert((lender, token), &(liquidity.checked_sub(amount).unwrap()));

        //     // Track loan
        //     self.debt.insert((borrower, token), &amount);
        //     let mut loans = self.active_loans.get(borrower).unwrap_or_default();
        //     loans.push((lender, token, amount));
        //     self.active_loans.insert(borrower, &loans);

        //     // Send funds to borrower
        //     if token == H160::zero() {
        //         self.env()
        //             .transfer(borrower, amount)
        //             .map_err(|_| Error::InsufficientLiquidity)?; // Fallback error
        //     } else {
        //         build_call::<Environment>()
        //             .call(token)
        //             .exec_input(
        //                 ExecutionInput::new(Selector::new(ink::selector_bytes!("transfer")))
        //                     .push_arg(borrower)
        //                     .push_arg(amount),
        //             )
        //             .returns::<()>()
        //             .invoke();
        //     }

        //     Ok(())
        // }

        // #[ink(message)]
        // pub fn pay_loan(&mut self, token: H160, lender: H160, amount: U256) -> Result<()> {
        //     let borrower = self.env().caller();
        //     let debt = self.debt.get((borrower, token)).unwrap_or_default();

        //     if debt < amount {
        //         return Err(Error::InsufficientLiquidity);
        //     }

        //     // Transfer repayment to lender
        //     if token == H160::zero() {
        //         assert!(
        //             self.env().transferred_value() == amount,
        //             "payment does not match amount"
        //         );
        //         self.env()
        //             .transfer(lender, amount)
        //             .map_err(|_| Error::InsufficientLiquidity)?;
        //     } else {
        //         build_call::<Environment>()
        //             .call(token)
        //             .exec_input(
        //                 ExecutionInput::new(Selector::new(ink::selector_bytes!("transfer_from")))
        //                     .push_arg(borrower)
        //                     .push_arg(lender)
        //                     .push_arg(amount),
        //             )
        //             .returns::<()>()
        //             .invoke();
        //     }

        //     // Update debt
        //     let updated_debt = debt.checked_sub(amount).unwrap();
        //     self.debt.insert((borrower, token), &updated_debt);

        //     // If loan is fully repaid, release collateral
        //     if updated_debt.is_zero() {
        //         let collateral_token = token; // This assumes same token used. Adapt logic for different tokens.
        //         let collateral_amount = self
        //             .collateral
        //             .get((borrower, collateral_token))
        //             .unwrap_or_default();

        //         if collateral_token == H160::zero() {
        //             self.env()
        //                 .transfer(borrower, collateral_amount)
        //                 .map_err(|_| Error::InsufficientLiquidity)?;
        //         } else {
        //             build_call::<Environment>()
        //                 .call(collateral_token)
        //                 .exec_input(
        //                     ExecutionInput::new(Selector::new(ink::selector_bytes!("transfer")))
        //                         .push_arg(borrower)
        //                         .push_arg(collateral_amount),
        //                 )
        //                 .returns::<()>()
        //                 .invoke();
        //         }

        //         self.collateral
        //             .insert((borrower, collateral_token), &U256::zero());
        //     }

        //     Ok(())
        // }

        // #[ink(message)]
        // pub fn set_min_collateral_ratio(&mut self, new_ratio: U256) -> Result<()> {
        //     if self.env().caller() != self.owner {
        //         return Err(Error::Unauthorized);
        //     }
        //     self.min_collateral_ratio = new_ratio;
        //     Ok(())
        // }

        // #[ink(message)]
        // pub fn get_user_loans(&self, borrower: H160) -> Vec<(H160, H160, U256)> {
        //     self.active_loans.get(borrower).unwrap_or_default()
        // }

        // #[ink(message)]
        // pub fn get_liquidity(&self, lender: H160, token: H160) -> U256 {
        //     self.liquidity_pool.get((lender, token)).unwrap_or_default()
        // }

        // #[ink(message)]
        // pub fn get_debt(&self, borrower: H160, token: H160) -> U256 {
        //     self.debt.get((borrower, token)).unwrap_or_default()
        // }

        // #[ink(message)]
        // pub fn get_collateral(&self, borrower: H160, token: H160) -> U256 {
        //     self.collateral.get((borrower, token)).unwrap_or_default()
        // }

        // #[ink(message)]
        // pub fn get_active_loans(&self, borrower: H160) -> Vec<(H160, H160, U256)> {
        //     self.active_loans.get(borrower).unwrap_or_default()
        // }

        // #[ink(message)]
        // pub fn update_min_collateral_ratio(&mut self, new_ratio: U256) -> Result<()> {
        //     if self.env().caller() != self.owner {
        //         return Err(Error::NotOwner);
        //     }
        //     if new_ratio < U256::from(10000) {
        //         return Err(Error::InvalidRatio);
        //     }
        //     self.min_collateral_ratio = new_ratio;
        //     Ok(())
        // }

        // #[ink(message)]
        // pub fn set_owner(&mut self, new_owner: H160) -> Result<()> {
        //     if self.env().caller() != self.owner {
        //         return Err(Error::NotOwner);
        //     }
        //     if new_owner == H160::zero() {
        //         return Err(Error::InvalidOwner);
        //     }
        //     self.owner = new_owner;
        //     Ok(())
        // }
        // #[ink(message)]
        // pub fn calculate_required_collateral(&self, amount: U256) -> U256 {
        //     amount
        //         .checked_mul(U256::from(self.min_collateral_ratio))
        //         .unwrap()
        //         .checked_div(U256::from(10_000))
        //         .unwrap()
        // }

        // #[ink(message)]
        // pub fn is_undercollateralized(&self, borrower: H160, token: H160) -> bool {
        //     let debt = self.get_debt(borrower, token);
        //     let collateral = self.get_collateral(borrower, token);
        //     let required = self.calculate_required_collateral(debt);
        //     collateral < required
        // }

        // #[ink(message)]
        // pub fn liquidate(&mut self, borrower: H160, token: H160) -> Result<()> {
        //     let debt = self.get_debt(borrower, token);
        //     if debt.is_zero() {
        //         return Err(Error::LoanNotFound);
        //     }

        //     if !self.is_undercollateralized(borrower, token) {
        //         return Err(Error::LoanNotEligibleForLiquidation);
        //     }

        //     let collateral_token = token; // simplify for demo
        //     let collateral_amount = self.get_collateral(borrower, collateral_token);

        //     self.debt.insert((borrower, token), &U256::zero());
        //     self.collateral
        //         .insert((borrower, collateral_token), &U256::zero());

        //     let caller = self.env().caller();
        //     if collateral_token == H160::zero() {
        //         self.env()
        //             .transfer(caller, collateral_amount)
        //             .map_err(|_| Error::TransferFailed)?;
        //     } else {
        //         build_call::<Environment>()
        //             .call(collateral_token)
        //             .exec_input(
        //                 ExecutionInput::new(Selector::new(ink::selector_bytes!("transfer")))
        //                     .push_arg(caller)
        //                     .push_arg(collateral_amount),
        //             )
        //             .returns::<()>()
        //             .invoke();
        //     }

        //     Ok(())
        // }
    }
}
