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

            self.nonces().push(&nonce);
        }
    }

    #[only_owner]
    #[payable("*")]
    #[endpoint(removeNfts)]
    fn remove_nfts(&self, nonces_index: MultiValueEncoded<usize>) {
        for nonce_index in nonces_index.into_iter() {
            self.nonces().swap_remove(nonce_index);
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
        let third_token_payment = self.third_token_payment().get();
        require!(
            identifier == first_token_payment.token_identifier
            ||
            (identifier == second_token_payment.token_identifier && second_token_payment.token_nonce == nonce)
            ||
            identifier == third_token_payment.token_identifier
            , "Invalid token payment!");
            
            let nonces_left = self.nonces().len();
            require!(nonces_left >= amount_of_tokens as usize, "Not enough NFTs to mint.");
            let mut user_mints = self.user_mints(&caller).get();
            let user_premium_mints = self.user_premium_mints(&caller).get();

        let collection_identifier = self.collection().get();
            
        if identifier == first_token_payment.token_identifier {
            require!(amount == first_token_payment.amount * amount_of_tokens, "Payment amount invalid!");
        } else if identifier == second_token_payment.token_identifier {
            require!(amount == second_token_payment.amount * amount_of_tokens, "Payment amount invalid!");
        } else {
            require!(amount == third_token_payment.amount * amount_of_tokens, "Payment amount invalid!");
            require!(amount_of_tokens <= user_premium_mints, "You don't have enough premium mints!");

            // premium mint logic
            for _ in 0..amount_of_tokens {
                self.mint_single_nft(&caller, &collection_identifier);
            }

            self.user_premium_mints(&caller).update(|premium_mints| *premium_mints -= amount_of_tokens);
        }

        if identifier == first_token_payment.token_identifier && identifier == second_token_payment.token_identifier {
            user_mints += amount_of_tokens;
            let premium_mints = user_mints / MULTIPLIER;

            if premium_mints > 0u64 {
                self.user_premium_mints(&caller).update(|mints| *mints += premium_mints);

                let remaining_user_mints = user_mints - (premium_mints * MULTIPLIER);
                self.user_mints(&caller).set(remaining_user_mints);
            } else {
                self.user_mints(&caller).set(user_mints);
            }

            // mint logic
            for _ in 0..amount_of_tokens {
                self.mint_single_nft(&caller, &collection_identifier);
            }
        }
    }

    fn mint_single_nft(&self, address: &ManagedAddress, collection_identifier: &TokenIdentifier) {
        let nfts_left = self.nonces();
        let random_nonce_index = self.generate_random_number(nfts_left.len()); 
        let nonce = nfts_left.get(random_nonce_index);

        self.send().direct_esdt(address, collection_identifier, nonce, &BigUint::from(1u64));
        self.nonces().swap_remove(random_nonce_index);
    }

    fn generate_random_number(&self, max: usize) -> usize {
        let mut rand_source = RandomnessSource::new();

        rand_source.next_usize_in_range(1, max + 1)
    }

    #[view(getCollection)]
    #[storage_mapper("collection")]
    fn collection(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getNonces)]
    #[storage_mapper("nonces")]
    fn nonces(&self) -> VecMapper<u64>;

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
