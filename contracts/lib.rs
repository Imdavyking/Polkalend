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
        pub fn new(min_collateral_ratio: u128) -> Self {
            Self {
                collateral: Mapping::default(),
                debt: Mapping::default(),
                liquidity_pool: Mapping::default(),
                active_loans: Mapping::default(),
                min_collateral_ratio,
                owner: caller(),
            }
        }

        // allow lenders to deposit tokens into the liquidity pool
        #[ink(message, payable)]
        pub fn create_loan(&mut self, token: H160, amount: U256, duration: U256) -> Result<()> {
            let caller = self.env().caller();

            // Check if amount or duration is zero
            if amount.is_zero() || duration.is_zero() {
                return Err(Error::ZeroAmount);
            }

            let current_liquidity = self.liquidity_pool.get((caller, token)).unwrap_or_default();
            let total_success_liquidity = current_liquidity.checked_add(amount).unwrap();

            self.liquidity_pool
                .insert((caller, token), &(total_success_liquidity));
            if token == H160::zero() {
                assert!(
                    self.env().transferred_value() == amount,
                    "payment was not equal to the amount"
                );
                // Native token logic (you may track value sent via `self.env().transferred_value()`)
                // For now we just record it — you'd want to validate this against `amount`.
            } else {
                assert!(
                    self.env().transferred_value() == U256::zero(),
                    "you should not send native tokens"
                );
                // Call ERC-20 token's transfer_from on the token contract
                build_call::<Environment>()
                    .call(token)
                    .exec_input(
                        ExecutionInput::new(Selector::new(ink::selector_bytes!("transfer_from"))) // transferFrom(address from, address to, uint256 amount)
                            .push_arg(caller)
                            .push_arg(address())
                            .push_arg(amount),
                    )
                    .returns::<()>()
                    .invoke();
            }

            Ok(())
        }
    }
}
