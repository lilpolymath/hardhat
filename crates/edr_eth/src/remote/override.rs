use revm_primitives::{Address, HashMap, U256};

use crate::{serde::ZeroXPrefixedBytes, state::Storage};

/// Options for overriding account information.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountOverrideOptions {
    /// Account balance override.
    pub balance: Option<U256>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde::optional_u64"
    )]
    /// Account nonce override.
    pub nonce: Option<u64>,
    /// Account code override.
    pub code: Option<ZeroXPrefixedBytes>,
    /// Account storage override. Mutually exclusive with `storage_diff`.
    #[serde(rename = "state")]
    pub storage: Option<Storage>,
    /// Account storage diff override. Mutually exclusive with `storage`.
    #[serde(rename = "stateDiff")]
    pub storage_diff: Option<Storage>,
}

/// Type representing a full set of overrides for account information.
pub type StateOverrideOptions = HashMap<Address, AccountOverrideOptions>;