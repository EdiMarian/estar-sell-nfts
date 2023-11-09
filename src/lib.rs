#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait SellNftsContract {
    #[init]
    fn init(&self, collection_identifier: TokenIdentifier) {
        self.collection().set(collection_identifier);
    }

    #[only_owner]
    #[payable("*")]
    #[endpoint(setNfts)]
    fn set_nfts(&self, #[payment_multi] nfts: ManagedRef<ManagedVec<EsdtTokenPayment<Self::Api>>>) {
        let collection_identifier = self.collection().get();

        for nft in nfts.iter() {
            let (identifier, nonce, _amount) = nft.into_tuple();
            require!(identifier == collection_identifier, "Invalid NFT identifier!");

            self.nonces().insert(nonce);
        }
    }

    #[only_owner]
    #[payable("*")]
    #[endpoint(removeNfts)]
    fn remove_nfts(&self, nonces: MultiValueEncoded<u64>) {
        for nonce in nonces.into_iter() {
            require!(self.nonces().contains(&nonce), "This NFT doesn't exist in this SC.");
            self.nonces().remove(&nonce);
        }
    }

    #[only_owner]
    #[endpoint(setFirstTokenPayment)]
    fn set_first_token_payment(&self, identifier: TokenIdentifier, amount: BigUint) {
        require!(identifier.is_valid_esdt_identifier(), "Invalid token identifier!");
        require!(amount > BigUint::zero(), "Invalid amount!");

        let token_payment = EsdtTokenPayment::new(identifier, 0, amount);

        self.first_token_payment().set(token_payment);
    }

    #[only_owner]
    #[endpoint(setSecondTokenPayment)]
    fn set_second_token_payment(&self, identifier: TokenIdentifier, amount: BigUint) {
        require!(identifier.is_valid_esdt_identifier(), "Invalid token identifier!");
        require!(amount > BigUint::zero(), "Invalid amount!");

        let token_payment = EsdtTokenPayment::new(identifier, 0, amount);

        self.second_token_payment().set(token_payment);
    }

    #[view(getCollection)]
    #[storage_mapper("collection")]
    fn collection(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getNonces)]
    #[storage_mapper("nonces")]
    fn nonces(&self) -> SetMapper<u64>;

    #[view(getFirstTokenPayment)]
    #[storage_mapper("first_token_payment")]
    fn first_token_payment(&self) -> SingleValueMapper<EsdtTokenPayment>;

    #[view(getSecondTokenPayment)]
    #[storage_mapper("second_token_payment")]
    fn second_token_payment(&self) -> SingleValueMapper<EsdtTokenPayment>;
}
