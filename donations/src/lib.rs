#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub mod views;
pub mod storage;

#[elrond_wasm::contract]
pub trait Donations: 
    views::ViewsModule +
    storage::StorageModule
{
    #[init]
    fn init(&self) {
        let egld_token = EgldOrEsdtTokenIdentifier::egld();
        if !self.allowed_token_ids().contains(&egld_token) {
            self.enable_token(egld_token);
        }
    }

    /**
     * It allows the smart contract owner to enable a token
     * 
     * Validation
     * [x] It should check that the caller is the SC owner
     * [x] It should check that the token is not already enabled
     * 
     * Actions
     * [x] It should add the token to allowed_token_ids storage
     */
    #[only_owner]
    #[endpoint(enableToken)]
    fn enable_token(&self, token_identifier: EgldOrEsdtTokenIdentifier<Self::Api>) {
        require!(!self.allowed_token_ids().contains(&token_identifier), "This token is already enabled");

        self.allowed_token_ids().insert(token_identifier);
    }

    /**
     * It allows the smart contract owner to disable a token
     * 
     * Validation
     * [x] It should check that the caller is the SC owner
     * [x] It should check that the token is enabled
     * 
     * Actions
     * [x] It should remove the token from allowed_token_ids storage
     */
    #[only_owner]
    #[endpoint(disableToken)]
    fn disable_token(&self, token_identifier: &EgldOrEsdtTokenIdentifier<Self::Api>) {
        require!(self.allowed_token_ids().contains(token_identifier), "This token is not enabled");

        self.allowed_token_ids().remove(token_identifier);
    }

    /**
     * It allows the smart contract owner to whitelist an address for withdrawing the tokens
     * 
     * Validation
     * [x] It should check that the caller is the SC owner
     * [x] It should check that the address is not already on the whitelist
     * 
     * Actions
     * [x] It should add the address to allowed_withdraw_addresses storage
     */
    #[only_owner]
    #[endpoint(whitelistWithdrawAddress)]
    fn whitelist_withdraw_address(&self, address: ManagedAddress<Self::Api>) {
        require!(!self.allowed_withdraw_addresses().contains(&address), "This address is already whitelisted");

        self.allowed_withdraw_addresses().insert(address);
    }

    /**
     * It allows the smart contract owner to remove a withdraw address from the whitelist
     * 
     * Validation
     * [x] It should check that the caller is the SC owner
     * [x] It should check that the address is on the whitelist
     * 
     * Actions
     * [x] It should remove the address from allowed_withdraw_addresses storage
     */
    #[only_owner]
    #[endpoint(removeWithdrawAddress)]
    fn remove_withdraw_address(&self, address: &ManagedAddress<Self::Api>) {
        require!(self.allowed_withdraw_addresses().contains(address), "This address is not whitelisted");

        self.allowed_withdraw_addresses().remove(address);
    }

    /**
     * It allows anyone to donate with EGLD
     */
    #[payable("EGLD")]
    #[endpoint(donateEgld)]
    fn donate_egld(&self) {
        self.register_donation();
    }

    /**
     * It allows anyone to donate with ESDT
     */
    #[payable("*")]
    #[endpoint(donateEsdt)]
    fn donate_esdt(&self) {
        self.register_donation();
    }

    /**
     * It allows the whitelisted addresses to claim the tokens donated
     * 
     * Validation
     * [x] It should check that the address is whitelisted to claim the tokens
     * 
     * Actions
     * [x] It should send the entire SC balance for donated tokens to the caller
     */
    #[endpoint(claimTokens)]
    fn claim_tokens(&self) {
        let caller = self.blockchain().get_caller();

        require!(
            self.allowed_withdraw_addresses().contains(&caller),
            "Only whitelisted addresses can withdraw the tokens"
        );

        let donation_tokens = self.donation_token_ids();

        let mut has_egld_transfer = false;
        let mut esdt_transfers: ManagedVec<Self::Api, EsdtTokenPayment<Self::Api>> = ManagedVec::new();

        require!(donation_tokens.len() > 0, "Nothing to claim");

        for token_identifier in donation_tokens.iter() {
            let balance = self.blockchain().get_sc_balance(&token_identifier, 0u64);

            if balance > BigUint::zero() {
                if token_identifier.is_egld() {
                    self.send().direct_egld(&caller, &balance);
                    has_egld_transfer = true;
                } else {
                    esdt_transfers.push(EsdtTokenPayment::new(token_identifier.unwrap_esdt(), 0, balance));
                }
            }
        }

        require!(esdt_transfers.len() > 0 || has_egld_transfer, "Nothing to claim");

        self.send().direct_multi(&caller,&esdt_transfers);
    }

    /**
     * It checks and registers the donation
     * 
     * Validation
     * [x] It should check that the payment token is enabled
     * 
     * Actions
     * [x] It should add the user address to the donors storage
     * [x] It should add the donation token to the donation_token_ids storage
     * [x] It should save the donation amount to the user_donations storage
     * [x] It should save the donation amount to the total_donations storage
     */
    #[inline]
    fn register_donation(&self) {
        let payment = self.call_value().egld_or_single_esdt();

        require!(self.allowed_token_ids().contains(&payment.token_identifier), "This token is not enabled");

        let caller = self.blockchain().get_caller();

        if !self.donors().contains(&caller) {
            self.donors().insert(caller.clone());
        }

        if !self.donation_token_ids().contains(&payment.token_identifier) {
            self.donation_token_ids().insert(payment.token_identifier.clone());
        }

        let mut existing_user_donation = BigUint::zero();
        let mut existing_token_donation = BigUint::zero();

        if !self.user_donations(&caller, &payment.token_identifier).is_empty() {
            existing_user_donation = self.user_donations(&caller, &payment.token_identifier).get();
        }

        if !self.total_donations(&payment.token_identifier).is_empty() {
            existing_token_donation = self.total_donations(&payment.token_identifier).get();
        }

        self.total_donations(&payment.token_identifier).set(existing_token_donation + &payment.amount);
        self.user_donations(&caller, &payment.token_identifier).set(existing_user_donation + &payment.amount);
    }
}
