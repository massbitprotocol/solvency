///
/// Merkle tree using rs_merkle lib
///
use crate::Account;
use crate::PoseidonHasher;
use crate::{Fp, Fq};
use ethers_core::k256::elliptic_curve::PrimeField;
use ethers_core::types::Address;
use std::convert::TryInto;
// use merkletree::hash::Algorithm;
// use merkletree::merkle::{
//     get_merkle_tree_len_generic, get_merkle_tree_row_count, Element, FromIndexedParallelIterator,
//     MerkleTree,
// };
// use merkletree::store::{
//     DiskStore, LevelCacheStore, MmapStore, Store, StoreConfig, VecStore, SMALL_TREE_BUILD,
// };
use rs_merkle::{Error, Hasher};
use std::convert::TryFrom;

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
impl Into<Vec<u8>> for MPTItem {
    fn into(self) -> Vec<u8> {
        self.0.to_vec()
    }
}
impl Into<Fp> for MPTItem {
    fn into(self) -> Fp {
        let mut values = [0u64; 4];
        let mut cur_ind = 0;
        for i in 0..4 {
            let mut bytes = [0u8; 8];
            for j in 0..8 {
                bytes[j] = self.0[cur_ind];
                cur_ind += 1;
            }
            values[i] = u64::from_le_bytes(bytes);
        }
        Fp::from_raw(values)
    }
}
impl TryFrom<Vec<u8>> for MPTItem {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() == SIZE {
            let buffer = value.try_into().unwrap();
            Ok(MPTItem(buffer))
        } else {
            Err(Self::Error::new(
                rs_merkle::ErrorKind::HashConversionError,
                String::from("Data size is not matched"),
            ))
        }
    }
}
impl TryFrom<[u8; SIZE]> for MPTItem {
    type Error = Error;

    fn try_from(value: [u8; SIZE]) -> Result<Self, Self::Error> {
        Ok(Self(value))
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

// impl Element for MPTItem {
//     fn byte_len() -> usize {
//         SIZE
//     }
//     fn from_slice(bytes: &[u8]) -> Self {
//         assert_eq!(bytes.len(), Self::byte_len());
//         let mut el = [0u8; SIZE];
//         el[..].copy_from_slice(bytes);
//         MPTItem(el)
//     }
//     fn copy_to_slice(&self, bytes: &mut [u8]) {
//         bytes.copy_from_slice(&self.0);
//     }
// }
#[derive(Clone)]
pub struct MPTHasher {}
impl Hasher for MPTHasher {
    type Hash = MPTItem;

    fn hash(data: &[u8]) -> MPTItem {
        assert!(data.len() == SIZE || data.len() == SIZE2);
        //println!("Hash data: {:?}", data);
        let mut left = Fp::zero();
        let mut right = Fp::zero();
        let mut cur_ind = 0;
        if data.len() >= SIZE {
            let mut values = [0u64; 4];
            for i in 0..4 {
                let mut val = [0u8; 8];
                for j in 0..8 {
                    val[j] = data[cur_ind];
                    cur_ind += 1;
                }
                values[i] = u64::from_le_bytes(val);
            }
            left = Fp::from_raw(values);
        }
        if data.len() >= SIZE2 {
            let mut values = [0u64; 4];
            for i in 0..4 {
                let mut val = [0u8; 8];
                for j in 0..8 {
                    val[j] = data[cur_ind];
                    cur_ind += 1;
                }
                values[i] = u64::from_le_bytes(val);
            }
            right = Fp::from_raw(values);
        }
        let hashed_value = PoseidonHasher::init().hash([left.clone(), right.clone()]);
        // println!(
        //     "Hash result: {:?} + {:?} -> {:?}",
        //     left, right, hashed_value
        // );
        MPTItem::try_from(hashed_value.to_repr()).unwrap()
    }

    // fn concat_and_hash(left: &Self::Hash, right: Option<&Self::Hash>) -> Self::Hash {
    //     let mut concatenated: Vec<u8> = (*left).into();

    //     match right {
    //         Some(right_node) => {
    //             let mut right_node_clone: Vec<u8> = (*right_node).into();
    //             concatenated.append(&mut right_node_clone);
    //             Self::hash(&concatenated)
    //         }
    //         None => *left,
    //     }
    // }

    fn hash_size() -> usize {
        std::mem::size_of::<Self::Hash>()
    }
}
// impl MPTHasher {
//     pub fn new() -> MPTHasher {
//         MPTHasher {
//             buffer: [0u8; SIZE2],
//         }
//     }
// }
// impl Default for MPTHasher {
//     fn default() -> Self {
//         MPTHasher::new()
//     }
// }

// impl Hasher for MPTHasher {
//     fn finish(&self) -> u64 {
//         // FIXME: contract is broken by design
//         unimplemented!(
//             "Hasher's contract (finish function is not used) is deliberately broken by design"
//         )
//     }
//     fn write(&mut self, bytes: &[u8]) {
//         println!("Self: {:?}; bytes {:?}", &self.buffer, bytes);
//         assert!(self.buffer.len() >= bytes.len());
//         for i in 0..bytes.len() {
//             self.buffer[i] = bytes[i];
//         }
//     }
// }
// impl Algorithm<MPTItem> for MPTHasher {
//     fn hash(&mut self) -> MPTItem {
//         println!("Self buffer when hashing {:?}", &self.buffer);
//         let mut left = [0u64; 4];
//         let mut right = [0u64; 4];
//         let mut cur_ind = 0;
//         let offset = 32;
//         for i in 0..4 {
//             let mut val_l = [0u8; 8];
//             let mut val_r = [0u8; 8];
//             for j in 0..8 {
//                 val_l[j] = self.buffer[cur_ind];
//                 val_r[j] = self.buffer[cur_ind + offset];
//                 cur_ind += 1;
//             }
//             left[i] = u64::from_le_bytes(val_l);
//             right[i] = u64::from_le_bytes(val_r);
//         }
//         let message = [Fr::from_raw(left), Fr::from_raw(right)];
//         let hashed_value = PoseidonHasher::init().hash(message);
//         println!("Hashed result: {:?}", hashed_value.to_repr());
//         MPTItem(hashed_value.to_repr())
//     }
// }

// pub struct AccountTree {
//     pub accounts: Vec<Account>,
//     pub merkle_tree: MerkleTree<MPTItem, MPTHasher, VecStore<MPTItem>>,
// }
// impl AccountTree {
//     pub fn new(accounts: Vec<Account>) -> Self {
//         let curve_values: Vec<MPTItem> = accounts.iter().map(|acc| MPTItem::from(acc)).collect();
//         let merkle_tree = MerkleTree::new(curve_values).unwrap();
//         Self {
//             accounts,
//             merkle_tree,
//         }
//     }
// }
