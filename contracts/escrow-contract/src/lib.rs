// Do not link against libstd (i.e. anything defined in `std::`)
#![no_std]
#![feature(alloc_error_handler)]

use miden::{Felt, component, component_storage};

/// Storage layout for the account component (empty).
#[component_storage]
struct EscrowContractStorage;

/// API of the account component.
#[component]
trait EscrowContract {
    /// Adds two field elements.
    fn add(&self, a: Felt, b: Felt) -> Felt;
}

#[component]
impl EscrowContract for EscrowContractStorage {
    fn add(&self, a: Felt, b: Felt) -> Felt {
        a + b
    }
}
