#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub mod views;
pub mod storage;

pub mod structs;

use crate::structs::{Plan, UserPlan, PlanStatus};

#[elrond_wasm::contract]
pub trait Subscriptions: 
    views::ViewsModule +
    storage::StorageModule
{
    #[init]
    fn init(&self) {
        self.enabled().set(false);
    }

    /**
     * It allows the smart contract owner to set the enabled status [true/false]
     * 
     * Validation
     * [x] It should check that the caller is the SC owner
     * 
     * Actions
     * [x] It should set the status to storage
     */
    #[only_owner]
    #[endpoint(setStatus)]
    fn set_status(&self, status: bool) {
        if status == true {
            require!(!self.payment_address().is_empty(), "Payment address is not configured");
        }

        self.enabled().set(false);
    }

    /**
     * It allows the smart contract owner to set the address where the payments should be sent to
     * 
     * Validation
     * [x] It should check that the caller is the SC owner
     * 
     * Actions
     * [x] It should set the address to storage
     */
    #[only_owner]
    #[endpoint(setPaymentAddress)]
    fn set_payment_address(&self, address: ManagedAddress<Self::Api>) {
        self.payment_address().set(address);
    }

    /**
     * It allows the smart contract owner to add a subscription plan
     * 
     * Validation
     * [x] It should check that the caller is the SC owner
     * [x] It should check that the subscription plan does not exist
     * 
     * Actions
     * [x] It should add the subscription plan to storage
     */
    #[only_owner]
    #[endpoint(addSubscriptionPlan)]
    fn add_plan(&self, plan_id: ManagedBuffer<Self::Api>, validity: u64) {
        require!(!self.plan_ids().contains(&plan_id), "This plan already exists");
        self.plan_ids().insert(plan_id.clone());

        let plan: Plan<Self::Api> = Plan {
            id: plan_id.clone(),
            validity,
            status: PlanStatus::Enabled,
            
            /* not used at this time */
            refund_period: 0,
            allows_refund: false,
        };

        self.plans(&plan_id).set(plan);
    }

    /**
     * It allows the smart contract owner to remove a subscription plan
     * 
     * Validation
     * [x] It should check that the caller is the SC owner
     * [x] It should check that the subscription plan exists
     * 
     * Actions
     * [x] It should remove the subscription plan ID from storage
     * [x] It should remove the subscription plan from storage
     * [x] It should remove the subscription plan token prices from storage
     */
    #[only_owner]
    #[endpoint(removeSubscriptionPlan)]
    fn remove_plan(&self, plan_id: ManagedBuffer<Self::Api>) {
        require!(self.plan_ids().contains(&plan_id), "This plan does not exist");

        self.plan_ids().remove(&plan_id);
        self.plans(&plan_id).clear();

        for token_identifier in self.plan_tokens(&plan_id).iter() {
            self.plan_prices(&plan_id, &token_identifier).clear();
        }

        self.plan_tokens(&plan_id).clear();
    }

    /**
     * It allows the smart contract owner to enable a subscription plan
     * 
     * Validation
     * [x] It should check that the caller is the SC owner
     * [x] It should check that the subscription plan exists
     * 
     * Actions
     * [x] It should update the plan status to Enabled
     */
    #[only_owner]
    #[endpoint(enableSubscriptionPlan)]
    fn enable_subscription_plan(&self, plan_id: ManagedBuffer<Self::Api>) {
        require!(self.plan_ids().contains(&plan_id), "This plan does not exist");

        let mut plan = self.plans(&plan_id).get();
        plan.status = PlanStatus::Enabled;

        self.plans(&plan_id).set(plan);
    }

    /**
     * It allows the smart contract owner to disable a subscription plan
     * 
     * Validation
     * [x] It should check that the caller is the SC owner
     * [x] It should check that the subscription plan exists
     * 
     * Actions
     * [x] It should update the plan status to Disabled
     */
    #[only_owner]
    #[endpoint(disableSubscriptionPlan)]
    fn disable_subscription_plan(&self, plan_id: ManagedBuffer<Self::Api>) {
        require!(self.plan_ids().contains(&plan_id), "This plan does not exist");

        let mut plan = self.plans(&plan_id).get();
        plan.status = PlanStatus::Disabled;

        self.plans(&plan_id).set(plan);
    }

    /**
     * It allows the smart contract owner to set the subscription payment amount for a token
     * 
     * Validation
     * [x] It should check that the caller is the SC owner
     * [x] It should check that the subscription plan exists
     * 
     * Actions
     * [x] It should add the token to `plan_tokens` storage if it does not exist
     * [x] It should set the price on `plan_prices` storage
     */
    #[only_owner]
    #[endpoint(setSubscriptionPlanPrice)]
    fn set_subscription_plan_price(
        &self, 
        plan_id: ManagedBuffer<Self::Api>, 
        token_identifier: &EgldOrEsdtTokenIdentifier<Self::Api>, 
        price: BigUint<Self::Api>
    ) {
        require!(self.plan_ids().contains(&plan_id), "This plan does not exist");
        
        if !self.plan_tokens(&plan_id).contains(&token_identifier) {
            self.plan_tokens(&plan_id).insert(token_identifier.clone());
        }

        self.plan_prices(&plan_id, token_identifier).set(price);
    }

    /**
     * It allows the smart contract owner remove a token from plan payments
     * 
     * Validation
     * [x] It should check that the caller is the SC owner
     * [x] It should check that the subscription plan exists
     * [x] It should check that the there token is enabled for this subscription plan
     * 
     * Actions
     * [x] It should remove the token from `plan_tokens` storage
     * [x] It should remove the token prices from `plan_prices` storage
     */
    #[only_owner]
    #[endpoint(removeSubscriptionPlanToken)]
    fn remove_subscription_plan_token(
        &self, 
        plan_id: ManagedBuffer<Self::Api>, 
        token_identifier: &EgldOrEsdtTokenIdentifier<Self::Api>
    ) {
        require!(self.plan_ids().contains(&plan_id), "This plan does not exist");
        require!(self.plan_tokens(&plan_id).contains(token_identifier), "No price configured for this plan/token combination");

        self.plan_tokens(&plan_id).remove(token_identifier);
        self.plan_prices(&plan_id, token_identifier).clear();
    }

    /**
     * It allows anyone to pay with EGLD
     */
    #[payable("EGLD")]
    #[endpoint(payWithEgld)]
    fn pay_with_egld(&self, plan_id: ManagedBuffer<Self::Api>) {
        self.register_payment(plan_id);
    }

    /**
     * It allows anyone to pay with ESDT
     */
    #[payable("*")]
    #[endpoint(payWithEsdt)]
    fn pay_with_esdt(&self, plan_id: ManagedBuffer<Self::Api>) {
        self.register_payment(plan_id);
    }

    /**
     * It checks and registers the subscription payments
     * 
     * Validation
     * [x] It should check that the plan exists
     * [x] It should check that the contract is enabled
     * [x] It should check that the subscription plan is enabled
     * [x] It should check that the price is correct for this token
     * [x] It should check that the payment token is enabled for this plan
     * 
     * Actions
     * [x] It should call `activate_user_plan` function
     * [x] It should call `register_historic_data` function
     * [x] It should send the payment to `payment_address`
     */
    #[inline]
    fn register_payment(&self, plan_id: ManagedBuffer<Self::Api>) {
        let payment = self.call_value().egld_or_single_esdt();

        require!(!self.enabled().is_empty(), "Contract is not enabled");
        require!(self.plan_ids().contains(&plan_id), "This plan does not exist");
        require!(self.plan_tokens(&plan_id).contains(&payment.token_identifier), "This token is not enabled");
        require!(&self.plan_prices(&plan_id, &payment.token_identifier).get() == &payment.amount, "Invalid payment amount");

        let plan = self.plans(&plan_id).get();
        let caller = self.blockchain().get_caller();

        require!(plan.status == PlanStatus::Enabled, "Subscription plan is disabled");

        self.activate_user_plan(&caller, plan);
        self.register_historic_data(&caller, &payment);

        self.send().direct(
            &self.payment_address().get(), 
            &payment.token_identifier, 
            payment.token_nonce, 
            &payment.amount
        );
    }

    /**
     * It activates the user subscription
     * 
     * Actions
     * [x] It should add the user address to the `users` storage
     * [x] It should increase the user's subscription expiration date
     * [x] It should update the user's subscription `last_activated` date
     */
    #[inline]
    fn activate_user_plan(&self, user_address: &ManagedAddress, plan: Plan<Self::Api>) {
        let current_timestamp = self.blockchain().get_block_timestamp();

        if !self.users().contains(&user_address) {
            self.users().insert(user_address.clone());
        }

        let mut user_plan: UserPlan<Self::Api>;

        if !self.user_plan_ids(&user_address).contains(&plan.id) {
            self.user_plan_ids(&user_address).insert(plan.id.clone());

            user_plan = UserPlan {
                plan_id: plan.id.clone(),
                expires_at: current_timestamp + plan.validity,
                first_subscribed: current_timestamp,
                last_subscribed: current_timestamp,
            };
        } else {
            user_plan = self.user_plans(&user_address, &plan.id).get();
            if user_plan.expires_at > current_timestamp {
                user_plan.expires_at += plan.validity;
            } else {
                user_plan.expires_at = current_timestamp + plan.validity;
            }

            user_plan.last_subscribed = current_timestamp;
        }

        self.user_plans(&user_address, &plan.id).set(user_plan);
    }

    /**
     * It computes and saves historical data
     * 
     * Actions
     * [x] It should add the payment token to the `payment_token_ids` storage
     * [x] It should save the payment amount to the `payment_token_amounts` storage
     * [x] It should add the payment token to the `user_payment_tokens` storage
     * [x] It should save the payment amount to the `user_payment_amounts` storage
     */
    #[inline]
    fn register_historic_data(
        &self, 
        caller: &ManagedAddress, 
        payment: &EgldOrEsdtTokenPayment<Self::Api>
    ) {
        let mut existing_user_payment = BigUint::zero();
        let mut existing_token_payment = BigUint::zero();

        if !self.payment_token_ids().contains(&payment.token_identifier) {
            existing_token_payment = self.payment_token_amounts(&payment.token_identifier).get();
        } else {
            self.payment_token_ids().insert(payment.token_identifier.clone());
        }

        if self.user_payment_tokens(&caller).contains(&payment.token_identifier) {
            existing_user_payment = self.user_payment_amounts(&caller, &payment.token_identifier).get();
        } else {
            self.user_payment_tokens(&caller).insert(payment.token_identifier.clone());
        }

        self.payment_token_amounts(&payment.token_identifier).set(existing_token_payment + &payment.amount);
        self.user_payment_amounts(&caller, &payment.token_identifier).set(existing_user_payment + &payment.amount);
    }
}
