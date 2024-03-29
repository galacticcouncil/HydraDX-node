# pallet-referrals

## Referrals pallet

Support for referrals, referral codes and rewards distribution.

### Overview

Referrals give an opportunity to users to earn some rewards from trading activity if the trader
used their referral code to link their account to the referrer account.

The trader can get back part of the trade fee too if configured.

Pallet also provides support for volume-based tiering. Referrer can reached higher Level based on the total amount generated by users of the referrer code.
The higher level, the better reward.

Rewards are accumulated in the pallet's account and if it is not RewardAsset, it is converted to RewardAsset prior to claim.

//! ### Terminology

* **Referral code:**  a string of certain size that identifies the referrer. Must be alphanumeric and upper case.
* **Referrer:**  user that registered a code
* **Trader:**  user that does a trade
* **Reward Asset:**  id of an asset which rewards are paid in. Usually native asset.


License: Apache-2.0
