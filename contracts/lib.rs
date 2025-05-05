#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod polkalend {
    use ink::env::caller;
    use ink::prelude::vec::Vec;
    use ink::{storage::Mapping, H160, U256};

    #[derive(Debug, PartialEq, Eq, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
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

        #[ink(message)]
        pub fn create_loan(&mut self, token: H160, amount: U256, duration: U256) -> Result<()> {
            self.liquidity_pool.insert((caller(), token), &amount);
        }
    }
}
