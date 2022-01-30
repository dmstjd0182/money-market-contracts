use crate::*;

#[near_bindgen]
impl Contract {
    pub(crate) fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner,
            "This method can only be called by {}",
            self.owner
        );
    }

    pub(crate) fn internal_get_bid(&self, bidder: &AccountId) -> Bid {
        self.bids.get(bidder).expect("No bids with the specified information exist")
    }

    /// updates price response at every function call
    pub(crate) fn internal_ping(&mut self) {
        self.internal_update_price_response(self.requester_contract);
    }
}