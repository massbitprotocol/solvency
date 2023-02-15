use crate::Account;
use crate::Fr;
use crate::PoseidonHasher;
use ethers_core::k256::elliptic_curve::PrimeField;
use ethers_core::types::Address;
use halo2_gadgets::poseidon::primitives::ConstantLength;
use merkletree::hash::Algorithm;
use merkletree::merkle::{
    get_merkle_tree_len_generic, get_merkle_tree_row_count, Element, FromIndexedParallelIterator,
    MerkleTree,
};
use merkletree::store::{
    DiskStore, LevelCacheStore, MmapStore, Store, StoreConfig, VecStore, SMALL_TREE_BUILD,
};
use std::hash::Hasher;
// pub type PoseidonHasher =
//     halo2_gadgets::poseidon::primitives::Hash<u8, crate::OrchardNullifier, ConstantLength<2>, 3, 2>;
pub const SIZE: usize = 0x20;
pub const SIZE2: usize = 0x40;
/// Implementation of Element abstraction that we use in our integration tests
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct MPTItem([u8; SIZE]);
impl Default for MPTItem {
    fn default() -> Self {
        Self([0u8; SIZE])
    }
}
impl MPTItem {
    pub fn from_bytes(bytes: [u8; SIZE]) -> Self {
        Self(bytes)
    }
}
impl From<&Account> for MPTItem {
    fn from(account: &Account) -> MPTItem {
        let bytes = account.as_bytes();
        let mut buffers = [0; SIZE];
        for i in 0..bytes.len() {
            if i < SIZE {
                buffers[i] = bytes[i]
            }
        }
        MPTItem(buffers)
    }
}
impl From<&Address> for MPTItem {
    fn from(address: &Address) -> MPTItem {
        let bytes = address.as_bytes();
        let mut buffers = [0; SIZE];
        for i in 0..bytes.len() {
            if i < SIZE {
                buffers[i] = bytes[i]
            }
        }
        MPTItem(buffers)
    }
}
impl AsRef<[u8]> for MPTItem {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Element for MPTItem {
    fn byte_len() -> usize {
        SIZE
    }
    fn from_slice(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), Self::byte_len());
        let mut el = [0u8; SIZE];
        el[..].copy_from_slice(bytes);
        MPTItem(el)
    }
    fn copy_to_slice(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.0);
    }
}

pub struct MPTHasher {
    buffer: [u8; SIZE2],
}
impl MPTHasher {
    pub fn new() -> MPTHasher {
        MPTHasher {
            buffer: [0u8; SIZE2],
        }
    }
}
impl Default for MPTHasher {
    fn default() -> Self {
        MPTHasher::new()
    }
}

impl Hasher for MPTHasher {
    fn finish(&self) -> u64 {
        // FIXME: contract is broken by design
        unimplemented!(
            "Hasher's contract (finish function is not used) is deliberately broken by design"
        )
    }
    fn write(&mut self, bytes: &[u8]) {
        println!("Self: {:?}; bytes {:?}", &self.buffer, bytes);
        assert!(self.buffer.len() >= bytes.len());
        for i in 0..bytes.len() {
            self.buffer[i] = bytes[i];
        }
    }
}
impl Algorithm<MPTItem> for MPTHasher {
    fn hash(&mut self) -> MPTItem {
        println!("Self buffer when hashing {:?}", &self.buffer);
        let mut left = [0u64; 4];
        let mut right = [0u64; 4];
        let mut cur_ind = 0;
        let offset = 32;
        for i in 0..4 {
            let mut val_l = [0u8; 8];
            let mut val_r = [0u8; 8];
            for j in 0..8 {
                val_l[j] = self.buffer[cur_ind];
                val_r[j] = self.buffer[cur_ind + offset];
                cur_ind += 1;
            }
            left[i] = u64::from_le_bytes(val_l);
            right[i] = u64::from_le_bytes(val_r);
        }
        let message = [Fr::from_raw(left), Fr::from_raw(right)];
        let hashed_value = PoseidonHasher::init().hash(message);
        println!("Hashed result: {:?}", hashed_value.to_repr());
        MPTItem(hashed_value.to_repr())
    }
}

pub struct AccountTree {
    pub accounts: Vec<Account>,
    pub merkle_tree: MerkleTree<MPTItem, MPTHasher, VecStore<MPTItem>>,
}
impl AccountTree {
    pub fn new(accounts: Vec<Account>) -> Self {
        let curve_values: Vec<MPTItem> = accounts.iter().map(|acc| MPTItem::from(acc)).collect();
        let merkle_tree = MerkleTree::new(curve_values).unwrap();
        Self {
            accounts,
            merkle_tree,
        }
    }
}
