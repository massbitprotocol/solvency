use std::marker::PhantomData;
use std::time::Instant;

use super::OrchardNullifier;
use crate::merkle_sum_tree::{Hasher, TreeItem};
use crate::{Field, Fp};
pub use halo2_gadgets::poseidon::{
    primitives::{ConstantLength, Hash as PrimitiveHash, Spec},
    PoseidonSpongeInstructions, Pow5Chip,
};
pub type PoseidonHasher = PrimitiveHash<Fp, OrchardNullifier, ConstantLength<2>, 3, 2>;
use lazy_static::lazy_static;
lazy_static! {
    static ref HASHER: PoseidonHasher = PoseidonHasher::init();
}

#[derive(Clone, Default)]
pub struct PoseidonAlgorithm<F: Field> {
    _marker: PhantomData<F>,
}

impl Hasher for PoseidonAlgorithm<Fp> {
    type Hash = TreeItem<Fp>;

    fn concat_and_hash(left: &Self::Hash, right: Option<&Self::Hash>) -> Self::Hash {
        //     let mut concatenated: Vec<u8> = (*left).into();
        match right {
            Some(right_node) => {
                let TreeItem { hash, sum } = left as &TreeItem<Fp>;
                let hashed_value = HASHER.hash([hash.clone(), right_node.hash.clone()]);
                let sum_value = sum + right_node.sum;
                TreeItem {
                    hash: hashed_value,
                    sum: sum_value,
                }
            }
            None => *left,
        }
    }

    fn hash_size() -> usize {
        std::mem::size_of::<Self::Hash>()
    }

    // fn hash(data: &[u8]) -> MPTItem {
    //     assert!(data.len() == SIZE || data.len() == SIZE2);
    //     //println!("Hash data: {:?}", data);
    //     let mut left = Fr::zero();
    //     let mut right = Fr::zero();
    //     let mut cur_ind = 0;
    //     if data.len() >= SIZE {
    //         let mut values = [0u64; 4];
    //         for i in 0..4 {
    //             let mut val = [0u8; 8];
    //             for j in 0..8 {
    //                 val[j] = data[cur_ind];
    //                 cur_ind += 1;
    //             }
    //             values[i] = u64::from_le_bytes(val);
    //         }
    //         left = Fr::from_raw(values);
    //     }
    //     if data.len() >= SIZE2 {
    //         let mut values = [0u64; 4];
    //         for i in 0..4 {
    //             let mut val = [0u8; 8];
    //             for j in 0..8 {
    //                 val[j] = data[cur_ind];
    //                 cur_ind += 1;
    //             }
    //             values[i] = u64::from_le_bytes(val);
    //         }
    //         right = Fr::from_raw(values);
    //     }
    //     let hashed_value = PoseidonHasher::init().hash([left.clone(), right.clone()]);
    //     // println!(
    //     //     "Hash result: {:?} + {:?} -> {:?}",
    //     //     left, right, hashed_value
    //     // );
    //     MPTItem::try_from(hashed_value.to_repr()).unwrap()
    // }
}
