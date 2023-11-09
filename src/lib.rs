#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait SellNftsContract {
    #[init]
    fn init(&self, collection_identifier: TokenIdentifier) {
        self.collection().set(collection_identifier);
    }

    #[view(getCollection)]
    #[storage_mapper("collection")]
    fn collection(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getNonces)]
    #[storage_mapper("nonces")]
    fn nonces(&self) -> SetMapper<u64>;
}
