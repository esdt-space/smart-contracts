elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, PartialEq, Clone)]
pub enum PlanStatus {
    Enabled,
    Disabled
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Plan<M: ManagedTypeApi> {
    pub id: ManagedBuffer<M>,
    pub status: PlanStatus,

    pub validity: u64,
    pub allows_refund: bool,
    pub refund_period: u64,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct UserPlan<M: ManagedTypeApi> {
    pub plan_id: ManagedBuffer<M>,
    pub expires_at: u64,
    pub first_subscribed: u64,
    pub last_subscribed: u64,
}