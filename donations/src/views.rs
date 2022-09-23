elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait ViewsModule: 
    crate::storage::StorageModule
{
    /**
     * It returns the total amount of tokens raised
     */
    #[view(getTotalDonations)]
    fn get_total_donations(&self) -> MultiValueEncoded<(EgldOrEsdtTokenIdentifier, BigUint)> {
        let mut items_vec = MultiValueEncoded::new();


        for token_identifier in self.donation_token_ids().iter() {
            let token_donation = self.total_donations(&token_identifier);

            if !token_donation.is_empty() {
                items_vec.push((token_identifier, token_donation.get()));
            }
        }

        items_vec
    }

    /**
     * It returns the total amount of tokens donated by an address
     */
    #[view(getUserDonations)]
    fn get_user_donations(&self, address: &ManagedAddress) -> MultiValueEncoded<(EgldOrEsdtTokenIdentifier, BigUint)> {
        let mut items_vec = MultiValueEncoded::new();

        if !self.donors().contains(address) {
            return items_vec
        }

        for token_identifier in self.donation_token_ids().iter() {
            let user_token_donation = self.user_donations(address, &token_identifier);

            if !user_token_donation.is_empty() {
                items_vec.push((token_identifier, user_token_donation.get()));
            }
        }

        items_vec
    }
}
