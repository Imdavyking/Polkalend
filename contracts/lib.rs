#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[allow(clippy::cast_possible_truncation)]
#[ink::contract]
mod polkalend {
    use ink::env::call::{build_call, ExecutionInput, Selector};
    use ink::env::{address, caller};
    use ink::prelude::vec::Vec;
    use ink::{storage::Mapping, H160, U256};

    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        ZeroAmount,
        ZeroDuration,
        InsufficientLiquidity,
        CollateralTooLow,
        TransferFailed,
        Unauthorized,
    }

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

    pub type Result<T> = core::result::Result<T, Error>;

    #[ink(storage)]
    pub struct LendingPlatform {
        /// Mapping: (user, token_address) => collateral U256
        collateral: Mapping<(H160, H160), U256>,
        /// Mapping: (borrower, token_address) => debt U256
        debt: Mapping<(H160, H160), U256>,
        /// Mapping: (lender, token_address) => available lendable liquidity
        liquidity_pool: Mapping<(H160, H160), U256>,
        /// Active loans: borrower => list of (lender, token, amount)
        active_loans: Mapping<H160, Vec<(H160, H160, U256)>>,
        /// Minimum collateral ratio (e.g., 150% = 15000 basis points)
        min_collateral_ratio: u128,
        /// Owner of the contract (admin)
        owner: H160,
    }

    impl LendingPlatform {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                collateral: Mapping::default(),
                debt: Mapping::default(),
                liquidity_pool: Mapping::default(),
                active_loans: Mapping::default(),
                min_collateral_ratio: 15000,
                owner: caller(),
            }
        }

        // allow lenders to deposit tokens into the liquidity pool
        #[ink(message, payable)]
        pub fn create_loan(&mut self, token: H160, amount: U256, duration: U256) -> Result<()> {
            let caller = self.env().caller();

            if amount.is_zero() {
                return Err(Error::ZeroAmount);
            }
            if duration.is_zero() {
                return Err(Error::ZeroDuration);
            }

            let current_liquidity = self.liquidity_pool.get((caller, token)).unwrap_or_default();
            let updated = current_liquidity.checked_add(amount).unwrap();

            self.liquidity_pool.insert((caller, token), &updated);

            if token == H160::zero() {
                assert!(
                    self.env().transferred_value() == amount,
                    "payment was not equal to the amount"
                );
            } else {
                assert!(
                    self.env().transferred_value() == U256::zero(),
                    "you should not send native tokens"
                );
                build_call::<Environment>()
                    .call(token)
                    .exec_input(
                        ExecutionInput::new(Selector::new(ink::selector_bytes!("transfer_from")))
                            .push_arg(caller)
                            .push_arg(address())
                            .push_arg(amount),
                    )
                    .returns::<()>()
                    .invoke();
            }

            self.env().emit_event(LoanCreated {
                lender: caller,
                token,
                amount,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn accept_loan(
            &mut self,
            lender: H160,
            token: H160,
            amount: U256,
            collateral_token: H160,
            collateral_amount: U256,
        ) -> Result<()> {
            let borrower = self.env().caller();

            if amount.is_zero() || collateral_amount.is_zero() {
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
            if collateral_amount < required_collateral {
                return Err(Error::ZeroDuration); // Using ZeroDuration here as a placeholder
            }

            // Lock collateral
            self.collateral
                .insert((borrower, collateral_token), &collateral_amount);

            // Reduce liquidity
            self.liquidity_pool
                .insert((lender, token), &(liquidity.checked_mul(amount).unwrap()));

            // Track loan
            self.debt.insert((borrower, token), &amount);
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

        #[ink(message)]
        pub fn pay_loan(&mut self, token: H160, lender: H160, amount: U256) -> Result<()> {
            let borrower = self.env().caller();
            let debt = self.debt.get((borrower, token)).unwrap_or_default();

            if debt < amount {
                return Err(Error::InsufficientLiquidity);
            }

            // Transfer repayment to lender
            if token == H160::zero() {
                assert!(
                    self.env().transferred_value() == amount,
                    "payment does not match amount"
                );
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
            let updated_debt = debt.checked_mul(amount).unwrap();
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
    }
}
