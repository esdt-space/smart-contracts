use crate::structs::{Plan, UserPlan};

elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait StorageModule {
    /**
     * Stores the contract status [true/false]
    **/
    #[view(isEnabled)]
    #[storage_mapper("enabled")]
    fn enabled(&self) -> SingleValueMapper<bool>;

    /**
     * Stores the address where to send the payments
    **/
    #[storage_mapper("payment_address")]
    fn payment_address(&self) -> SingleValueMapper<ManagedAddress<Self::Api>>;

    /**
     * Stores the subscription plan ids
    **/
    #[view(getPlanIds)]
    #[storage_mapper("plan_ids")]
    fn plan_ids(&self) -> SetMapper<ManagedBuffer<Self::Api>>;

    /**
     * Stores the subscription plan info
    **/
    #[view(getPlanInfo)]
    #[storage_mapper("plans")]
    fn plans(&self, plan_id: &ManagedBuffer) -> SingleValueMapper<Plan<Self::Api>>;

    /**
     * Stores the tokens available for plan payment
     *
     * Example:
     * monthly [EGLD, USDC]
     * yearly: [EGLD, SPACE]
     * lifetime: [EGLD]
     **/
    #[storage_mapper("plan_payment_tokens")]
    fn plan_tokens(&self, plan: &ManagedBuffer<Self::Api>) -> SetMapper<EgldOrEsdtTokenIdentifier<Self::Api>>;

    /**
     * Stores the plan price for each token available
     *
     * monthly - egld = 0.1
     * monthly - usdc = 5
     * yearly - egld = 0.2
     * yearly - space = 100
     * lifetime - egld = 0.5
    **/
    #[storage_mapper("plan_prices")]
    fn plan_prices(&self, plan: &ManagedBuffer, token_id: &EgldOrEsdtTokenIdentifier) -> SingleValueMapper<BigUint<Self::Api>>;

    /**
     * Stores all the users that activated a plan
    **/
    #[view(getUsers)]
    #[storage_mapper("users")]
    fn users(&self) -> SetMapper<ManagedAddress<Self::Api>>;

    /**
     * Stores the token identifiers that have been used for payments
    **/
    #[view(getpaymentTokenIds)]
    #[storage_mapper("payment_token_ids")]
    fn payment_token_ids(&self) -> SetMapper<EgldOrEsdtTokenIdentifier<Self::Api>>;

    /**
     * Stores the total payments made to this smart contract [for each token identifier]
    **/
    #[storage_mapper("payment_token_amounts")]
    fn payment_token_amounts(&self, token_id: &EgldOrEsdtTokenIdentifier) -> SingleValueMapper<BigUint<Self::Api>>;

    /* User storage */

    /**
     * Stores the user subscription plan ids
    **/
    #[view(getUserPlanIds)]
    #[storage_mapper("user_plan_ids")]
    fn user_plan_ids(&self, user_address: &ManagedAddress) -> SetMapper<ManagedBuffer<Self::Api>>;

    /**
     * Stores the user subscription plan info
    **/
    #[view(getUserPlanInfo)]
    #[storage_mapper("plans")]
    fn user_plans(&self, user_address: &ManagedAddress, plan_id: &ManagedBuffer) -> SingleValueMapper<UserPlan<Self::Api>>;

    /**
     * Stores the total payments made by an address for each token identifier
    **/
    #[storage_mapper("user_payment_tokens")]
    fn user_payment_tokens(&self, user_address: &ManagedAddress) -> SetMapper<EgldOrEsdtTokenIdentifier<Self::Api>>;

    /**
     * Stores the total payments made by an address for each token identifier
    **/
    #[storage_mapper("user_payment_amounts")]
    fn user_payment_amounts(&self, user_address: &ManagedAddress, token_id: &EgldOrEsdtTokenIdentifier) -> SingleValueMapper<BigUint<Self::Api>>;
}
