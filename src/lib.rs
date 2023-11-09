#![no_std]

const MULTIPLIER: u64 = 4;

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
    fn set_second_token_payment(&self, identifier: TokenIdentifier, nonce: u64, amount: BigUint) {
        require!(identifier.is_valid_esdt_identifier(), "Invalid token identifier!");
        require!(amount > BigUint::zero(), "Invalid amount!");

        let token_payment = EsdtTokenPayment::new(identifier, nonce, amount);

        self.second_token_payment().set(token_payment);
    }

    #[only_owner]
    #[endpoint(setThirdTokenPayment)]
    fn set_third_token_payment(&self, identifier: TokenIdentifier, amount: BigUint) {
        require!(identifier.is_valid_esdt_identifier(), "Invalid token identifier!");
        require!(amount > BigUint::zero(), "Invalid amount!");

        let token_payment = EsdtTokenPayment::new(identifier, 0, amount);

        self.third_token_payment().set(token_payment);
    }

    #[payable("*")]
    #[endpoint(mint)]
    fn mint(&self, amount_of_tokens: u64) {
        let (identifier, nonce, amount) = self.call_value().single_esdt().into_tuple();
        let caller = self.blockchain().get_caller();
        let first_token_payment = self.first_token_payment().get();
        let second_token_payment = self.second_token_payment().get();
        require!(identifier == first_token_payment.token_identifier || identifier == second_token_payment.token_identifier, "Invalid token payment!");
    
        // let nonces_left = self.nonces().len();
        // require!(nonces_left >= amount_of_tokens as usize, "Not enough NFTs to mint.");

        // if identifier == second_token_payment.token_identifier {
        //     require!(self.user_mints(&caller).get() >= 3u64, "You aren't eligible to mint with this token!");
        //     require!(amount == second_token_payment.amount * amount_of_tokens, "Payment amount invalid!");
        // } else {
        //     require!(amount == first_token_payment.amount * amount_of_tokens, "Payment amount invalid!");
        //     self.user_mints(&caller).update(|nb_of_mints| *nb_of_mints += 1);
        // }
    }

    fn mint_single_nft(&self, address: &ManagedAddress) {
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

    #[view(getThirdTokenPayment)]
    #[storage_mapper("third_token_payment")]
    fn third_token_payment(&self) -> SingleValueMapper<EsdtTokenPayment>;

    #[view(getUserMints)]
    #[storage_mapper("user_mints")]
    fn user_mints(&self, address: &ManagedAddress) -> SingleValueMapper<u64>;

    #[view(getUserPremiumMints)]
    #[storage_mapper("user_premium_mints")]
    fn user_premium_mints(&self, address: &ManagedAddress) -> SingleValueMapper<u64>;
}
