elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait StorageModule {
    /** 
     * Stores the addresses allowed to withdraw the tokens donated
    **/
    #[storage_mapper("allowed_withdraw_addresses")]
    fn allowed_withdraw_addresses(&self) -> SetMapper<ManagedAddress<Self::Api>>;

    /** 
     * Stores the donor addresses
    **/
    #[view(getDonors)]
    #[storage_mapper("donors")]
    fn donors(&self) -> SetMapper<ManagedAddress<Self::Api>>;

    /** 
     * Stores the current allowed token identifiers
    **/
    #[view(getAllowedTokenIds)]
    #[storage_mapper("allowed_token_ids")]
    fn allowed_token_ids(&self) -> SetMapper<EgldOrEsdtTokenIdentifier<Self::Api>>;

    /** 
     * Stores the token identifiers that have been used for donations 
     * We are iterating over this when the user is claiming because some tokens might have been disabled and no longer in allowed_token_ids mapper
    **/
    #[view(getDonationTokenIds)]
    #[storage_mapper("donation_token_ids")]
    fn donation_token_ids(&self) -> SetMapper<EgldOrEsdtTokenIdentifier<Self::Api>>;

    /** 
     * Stores the total donations for each token identifier
    **/
    #[storage_mapper("total_donations")]
    fn total_donations(&self, token_id: &EgldOrEsdtTokenIdentifier) -> SingleValueMapper<BigUint<Self::Api>>;

    
    /* User storage */

    /** 
     * Stores the total donations made by an address for each token identifier
    **/
    #[storage_mapper("user_donations")]
    fn user_donations(&self, user_address: &ManagedAddress, token_id: &EgldOrEsdtTokenIdentifier) -> SingleValueMapper<BigUint<Self::Api>>;
}