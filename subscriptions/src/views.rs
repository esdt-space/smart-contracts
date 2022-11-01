elrond_wasm::imports!();

use crate::structs::{Plan, UserPlan};

#[elrond_wasm::module]
pub trait ViewsModule: 
    crate::storage::StorageModule
{
    /**
     * It returns the subscription plans available
     */
    #[view(getSubscriptionPlans)]
    fn get_subscription_plans(&self) -> MultiValueEncoded<Plan<Self::Api>> {
        let mut items_vec = MultiValueEncoded::new();

        if self.plan_ids().is_empty() {
            return items_vec
        }

        for plan_id in self.plan_ids().iter() {
           items_vec.push(self.plans(&plan_id).get());
        }

        items_vec
    }

    /**
     * It returns the prices for a specific subscription plan
     */
    #[view(getSubscriptionPlanPrices)]
    fn get_subscription_plan_prices(
        &self, 
        plan_id: &ManagedBuffer<Self::Api>
    ) -> MultiValueEncoded<(EgldOrEsdtTokenIdentifier<Self::Api>, BigUint<Self::Api>)> {
        let mut prices_vec = MultiValueEncoded::new();

        if self.plan_tokens(&plan_id).is_empty() {
            return prices_vec
        }

        for token_id in self.plan_tokens(&plan_id).iter() {
            prices_vec.push((token_id.clone(), self.plan_prices(&plan_id, &token_id).get()));
        }

        prices_vec
    }

    /**
     * It returns the active user plans
     */
    #[view(getUserPlans)]
    fn get_user_plans(&self, address: &ManagedAddress) -> MultiValueEncoded<UserPlan<Self::Api>> {
        let mut items_vec = MultiValueEncoded::new();

        if !self.users().contains(&address) {
            return items_vec
        }

        for plan_id in self.user_plan_ids(&address).iter() {
           items_vec.push(self.user_plans(&address, &plan_id).get());
        }

        items_vec
    }
}
