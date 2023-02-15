//use crate::merkle_tree::hasher::Hasher;
pub use halo2_gadgets::poseidon::{
    primitives::{ConstantLength, Domain, Hash as PrimitiveHash, Spec},
    Hash, PoseidonSpongeInstructions, Pow5Chip,
};

// use halo2_proofs::{
//     arithmetic::FieldExt,
//     circuit::{AssignedCell, Chip, Layouter},
//     plonk::Error,
// };
//use merkletree::hash::{Algorithm, Hashable};

// pub struct MPTHasher {}
// impl MPTHasher {
//     pub fn new() -> MPTHasher {
//         MPTHasher {}
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
//         for x in bytes {
//             self.data.0[self.i & 15] ^= *x;
//             self.i += 1;
//         }
//     }
// }
// impl Algorithm<MPTItem> for MPTHasher {
//     fn hash(&mut self) -> MPTItem {
//         PoseidonHasher::init().hash(message);
//         //self.data
//     }
// }

// impl<
//         F: FieldExt,
//         PoseidonChip: PoseidonSpongeInstructions<F, S, D, T, RATE>,
//         S: Spec<F, T, RATE>,
//         D: Domain<F, RATE>,
//         const T: usize,
//         const RATE: usize,
//     > Hasher<F> for Hash<F, PoseidonChip, S, D, T, RATE>
// {
//     fn hash_bits<I: IntoIterator<Item = bool>>(&self, value: I) -> F {
//         todo!()
//     }

//     fn hash_elements<I: IntoIterator<Item = F>>(&self, elements: I) -> F {
//         todo!()
//     }

//     fn compress(&self, lhs: &F, rhs: &F, i: usize) -> F {
//         todo!()
//     }
// }
