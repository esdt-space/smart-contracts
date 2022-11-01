#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::contract]
pub trait PaymentProxy {
    #[init]
    fn init(&self) {}

    #[payable("EGLD")]
    #[endpoint(payWithEgld)]
    fn pay_with_egld(&self) {
        self.forward_payment();
    }

    #[payable("*")]
    #[endpoint(payWithEsdt)]
    fn pay_with_esdt(&self) {
        self.forward_payment();
    }

    #[inline]
    fn forward_payment(&self) {
        let payment = self.call_value().egld_or_single_esdt();
        self.send().direct(
            &self.blockchain().get_owner_address(), 
            &payment.token_identifier, 
            payment.token_nonce, 
            &payment.amount
        );
    }
}
