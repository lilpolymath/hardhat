use rethnet_eth::{
    state::{storage_root2, Storage},
    trie::KECCAK_NULL_RLP,
    B256, U256,
};

#[derive(Clone, Debug, Default)]
pub struct RethnetStorage {
    /// Index -> Value
    slots: Storage,
    /// Cached storage root
    storage_root: Option<B256>,
}

impl RethnetStorage {
    pub fn get(&self, index: &U256) -> Option<&U256> {
        self.slots.get(index)
    }

    pub fn insert(&mut self, index: U256, value: U256) {
        self.mark_dirty();
        self.slots.insert(index, value);
    }

    pub fn remove(&mut self, index: &U256) -> Option<U256> {
        self.mark_dirty();
        self.slots.remove(index)
    }

    pub fn storage_root(&mut self) -> B256 {
        if let Some(storage_root) = self.storage_root {
            storage_root
        } else {
            let storage_root = if self.slots.is_empty() {
                KECCAK_NULL_RLP
            } else {
                storage_root2(&self.slots)
            };
            self.storage_root = Some(storage_root);
            storage_root
        }
    }

    fn mark_dirty(&mut self) {
        self.storage_root = None;
    }
}

impl From<Storage> for RethnetStorage {
    fn from(slots: Storage) -> Self {
        Self {
            slots,
            storage_root: None,
        }
    }
}