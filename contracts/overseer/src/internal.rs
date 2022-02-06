use crate::*;

#[near_bindgen]
impl Contract {
  pub(crate) fn assert_owner(&self) {
    assert_eq!(
      env::predecessor_account_id(),
      self.config.owner_id,
      "Can only be called by the owner"
    );
  }
}
