// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           21
// Async Callback (empty):               1
// Total number of exported functions:  23

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    sell_nfts
    (
        init => init
        addSwapOperation => add_swap_operation
        clearSwapOperations => clear_swap_operations
        withdrawToken => withdraw_token
        addToWhitelist => add_to_whitelist
        removeFromWhitelist => remove_from_whitelist
        setNfts => set_nfts
        removeNfts => remove_nfts
        setFirstTokenPayment => set_first_token_payment
        setSecondTokenPayment => set_second_token_payment
        setThirdTokenPayment => set_third_token_payment
        mint => mint
        getCollection => collection
        getDexRouterAddress => dex_router_address
        getSwapOperations => swap_operations
        getNonces => nonces
        getFirstTokenPayment => first_token_payment
        getSecondTokenPayment => second_token_payment
        getThirdTokenPayment => third_token_payment
        getUserWhitelist => user_whitelist
        getUserMints => user_mints
        getUserPremiumMints => user_premium_mints
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
