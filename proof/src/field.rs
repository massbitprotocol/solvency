use crate::{Fp, Fq};
pub use halo2_proofs::{
    arithmetic::{Field as Halo2Field, FieldExt},
    halo2curves::{group::ff::PrimeField, pairing::Engine},
};
use halo2curves::pasta::{EpAffine, EqAffine};

//use ff_ce::Field;
/// Trait used to reduce verbosity with the declaration of the [`FieldExt`]
/// trait and its repr.
pub trait Field: FieldExt + Halo2Field + PrimeField<Repr = [u8; 32]> {}

impl Field for Fp {}
// // Impl custom `Field` trait for BN256 Fr to be used and consistent with the
// // rest of the workspace.
// impl FieldExt for Fp {
//     const MODULUS: &'static str;

//     const ROOT_OF_UNITY_INV: Self;

//     const DELTA: Self;

//     const TWO_INV: Self;

//     const ZETA: Self;

//     fn from_u128(v: u128) -> Self {
//         todo!()
//     }

//     fn from_bytes_wide(bytes: &[u8; 64]) -> Self {
//         todo!()
//     }

//     fn get_lower_128(&self) -> u128 {
//         todo!()
//     }
// }

pub struct PastaEngine {}

// impl Engine for PastaEngine {
//     type Scalar = Fp;

//     type G1 = Fp;

//     type G1Affine = EpAffine;

//     type G2 = Fq;

//     type G2Affine = EqAffine;

//     type Gt;

//     fn pairing(p: &Self::G1Affine, q: &Self::G2Affine) -> Self::Gt {
//         todo!()
//     }
// }
