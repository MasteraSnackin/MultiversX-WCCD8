#![no_std] // This attribute ensures that the contract does not rely on the Rust standard library, which is necessary for smart contracts deployed on blockchain platforms like MultiversX, where standard library support is unavailable.

use multiversx_sc::imports::*; // Import core functionality and types from the MultiversX smart contract library.
use multiversx_sc::derive_imports::*; // Import macros for deriving necessary traits, facilitating easier contract development.

/// The `TokenClaimContract` trait encapsulates the functionality of the smart contract.
/// This trait will be implemented by the smart contract logic, defining the behaviors and operations available.
#[multiversx_sc::contract]
pub trait TokenClaimContract {
    /// The `init` function serves as the constructor for the contract.
    /// It is automatically invoked once during the contract's deployment, allowing for the initialization of state or parameters.
    #[init]
    fn init(&self) {
        // Initialization logic goes here if needed.
        // For this particular contract, no specific initialization actions are required.
    }

    /// The `claim_tokens` function is an endpoint that users can call to claim tokens.
    /// It is marked with the `#[endpoint]` attribute, making it callable by external entities interacting with the contract.
    #[endpoint(claimTokens)]
    fn claim_tokens(&self) {
        // Obtain the address of the caller, i.e., the account that is invoking this function.
        let caller = self.blockchain().get_caller();

        // Specify the amount of tokens to be transferred to the caller.
        // This is set to a fixed value of 100 tokens for this example, but it could be parameterized or dynamically calculated.
        let token_amount = BigUint::from(100u64);

        // Execute the token transfer. The `direct` method is used to send the specified amount of tokens directly to the caller's address.
        // The `&[]` parameter can be used to pass any additional data or payload along with the transfer, currently set as an empty slice.
        self.send().direct(&caller, &token_amount, &[]);
    }
}