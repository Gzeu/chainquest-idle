pub mod client;
use multiversx_sc::*;

#[multiversx_sc::contract]
pub trait IdleSFTContract {
    #[init]
    fn init(&self) {}

    #[endpoint(mintReward)]
    fn mint_reward(&self, attributes: ManagedBuffer) {
        // MVP stub for offchain integration tests; real minting requires ESDT/NFT module wiring
        let _caller = self.blockchain().get_caller();
        let _attr = attributes;
    }
}
