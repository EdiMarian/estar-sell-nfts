#![no_std]

const MULTIPLIER: u64 = 4;
const DECIMALS: u64 = 1000000000000000000;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::contract]
pub trait SellNftsContract {
    #[init]
    fn init(&self, collection_identifier: TokenIdentifier, dex_address: ManagedAddress) {
        self.collection().set(collection_identifier);
        self.dex_router_address().set(dex_address);
    }

    #[only_owner]
    #[endpoint(addSwapOperation)]
    fn add_swap_operation(&self, address: ManagedAddress, token_id: TokenIdentifier) {
        self.swap_operations()
            .push(&SwapOperation { address, token_id });
    }

    #[only_owner]
    #[endpoint(clearSwapOperations)]
    fn clear_swap_operations(&self) {
        self.swap_operations()
            .clear();
    }

    #[only_owner]
    #[endpoint(withdrawToken)]
    fn withdraw_token(&self, identifier: TokenIdentifier, nonce: u64) {
        let caller = self.blockchain().get_caller();
       let sc_balance = self.blockchain().get_esdt_balance(&self.blockchain().get_sc_address(), &identifier, nonce);

       self.send().direct_esdt(&caller, &identifier, nonce, &sc_balance);
    }

    #[only_owner]
    #[endpoint(addToWhitelist)]
    fn add_to_whitelist(&self, addresses: MultiValueEncoded<ManagedAddress>) {
        for address in addresses.into_iter() {
            self.user_whitelist(&address).set(true);
        }
    }

    #[only_owner]
    #[endpoint(removeFromWhitelist)]
    fn remove_from_whitelist(&self, addresses: MultiValueEncoded<ManagedAddress>) {
        for address in addresses.into_iter() {
            self.user_whitelist(&address).set(false);
        }
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

        if identifier == first_token_payment.token_identifier || identifier == second_token_payment.token_identifier {
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
        
        if !self.user_whitelist(&caller).get() && identifier == first_token_payment.token_identifier {
            let mut swap_operations = MultiValueEncoded::new();
            for operation in self.swap_operations().iter() {
                swap_operations.push(operation.to_swap_type());
            }
            let router_address = self.dex_router_address().get();
            let amount_to_swap = (BigUint::from(900000000000000000u64) * amount.clone()) / self.token_decimals(1);

            let min_amount = BigUint::from(1_000u32);
            let _: IgnoreValue = self.contract_proxy(router_address).multi_pair_swap_fixed_input(&min_amount, swap_operations).with_esdt_transfer(EsdtTokenPayment::new(
                identifier.clone(),
                nonce,
                amount_to_swap,
            )).execute_on_dest_context();
        }
    }

    fn token_decimals(&self, amount: u64) -> BigUint {
        BigUint::from(amount) * BigUint::from(DECIMALS)
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

    #[proxy]
    fn contract_proxy(&self, sc_address: ManagedAddress) -> swap_router_proxy::Proxy<Self::Api>;

    #[view(getCollection)]
    #[storage_mapper("collection")]
    fn collection(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getDexRouterAddress)]
    #[storage_mapper("dex_router_address")]
    fn dex_router_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getSwapOperations)]
    #[storage_mapper("swap_operations")]
    fn swap_operations(&self) -> VecMapper<SwapOperation<Self::Api>>;

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

    #[view(getUserWhitelist)]
    #[storage_mapper("user_whitelist")]
    fn user_whitelist(&self, address: &ManagedAddress) -> SingleValueMapper<bool>;

    #[view(getUserMints)]
    #[storage_mapper("user_mints")]
    fn user_mints(&self, address: &ManagedAddress) -> SingleValueMapper<u64>;

    #[view(getUserPremiumMints)]
    #[storage_mapper("user_premium_mints")]
    fn user_premium_mints(&self, address: &ManagedAddress) -> SingleValueMapper<u64>;
}

#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, TypeAbi, Clone, ManagedVecItem,
)]
pub struct SwapOperation<M: ManagedTypeApi> {
    pub address: ManagedAddress<M>,
    pub token_id: TokenIdentifier<M>,
}

impl<M: ManagedTypeApi> SwapOperation<M> {
    pub fn to_swap_type(&self) -> SwapOperationType<M> {
        MultiValue2::from((self.address.clone(), self.token_id.clone()))
    }
}

#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, TypeAbi, Clone, ManagedVecItem,
)]
pub struct SwapResultType<M: ManagedTypeApi> {
    pub total_fee: BigUint<M>,
    pub special_fee: BigUint<M>,
    pub payment_in: EsdtTokenPayment<M>,
    pub payment_out: EsdtTokenPayment<M>,
    pub refund_amount_in: BigUint<M>,
}

pub type SwapOperationType<M> = MultiValue2<ManagedAddress<M>, TokenIdentifier<M>>;

mod swap_router_proxy {
    multiversx_sc::imports!();
    use crate::{SwapOperationType, SwapResultType};

    #[multiversx_sc::proxy]
    pub trait SwapRouterContract {
        #[payable("*")]
        #[endpoint(multiPairSwapFixedInput)]
        fn multi_pair_swap_fixed_input(
            &self,
            amount_out_min: BigUint,
            swap_operations: MultiValueEncoded<SwapOperationType<Self::Api>>,
        ) -> ManagedVec<SwapResultType<Self::Api>>;

        #[view(getEquivalent)]
        fn get_equivalent(
            &self,
            amount_in: BigUint,
            swap_operations: MultiValueEncoded<SwapOperationType<Self::Api>>,
        ) -> BigUint;
    }
}
