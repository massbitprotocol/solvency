pub mod field;
//pub mod fr;
pub mod account;
pub mod circuits;
//pub mod crypto;
pub mod merkle_sum_tree;
pub mod merkle_tree;
pub mod poseidon;
pub mod types;
//pub mod p128pow5t3;

pub use field::{Field, FieldExt};
//pub use p128pow5t3::P128Pow5T3;
pub use ff_ce::{PrimeField, PrimeFieldRepr};

pub use crate::merkle_sum_tree::{OrchardNullifier, PoseidonAlgorithm, TreeItem};
pub use account::Account;
pub use circuits::*;
pub use ethers_core::types::{
    transaction::{eip2930::AccessList, response::Transaction},
    Address, Block, Bytes, Signature, H160, H256, H64, U256, U64,
};

use lazy_static::lazy_static;
pub use types::MerkleTree;
pub use types::{Fp, Fq};
pub type CurveEngine = halo2_proofs::halo2curves::bn256::Bn256;
pub type G1Affine = halo2_proofs::halo2curves::bn256::G1Affine;
pub type PoseidonHasher =
    poseidon::PrimitiveHash<Fp, OrchardNullifier, poseidon::ConstantLength<2>, 3, 2>;
//pub type AccountTree = merkle_tree::SparseMerkleTree<Account, types::Fp, Hasher>;

lazy_static! {
    // /// Mock coinbase value
    // pub static ref MOCK_COINBASE: Address =
    //     address!("0x00000000000000000000000000000000c014ba5e");
    /// Mock chain ID value
    pub static ref MOCK_CHAIN_ID: U256 = U256::from(1338u64);
}
