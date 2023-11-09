#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait SellNftsContract {
    #[init]
    fn init(&self) {}
}
